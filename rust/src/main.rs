use clap::{Parser, CommandFactory};
use colored::*;
use std::path::PathBuf;

mod cmds;
mod core;

use cmds::{Commands, RmmBox};

/// 🚀 RMM 
#[derive(Parser)]
#[command(color = clap::ColorChoice::Always)]
#[command(styles = get_styles())]
#[command(help_template = "\
{before-help}{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
")]
struct Cli {
    #[command(subcommand)]
    /// 命令
    cmd: Option<Commands>,
}

/// 获取 clap 样式配置
fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .header(clap::builder::styling::AnsiColor::Yellow.on_default())
        .usage(clap::builder::styling::AnsiColor::Green.on_default())
        .literal(clap::builder::styling::AnsiColor::Cyan.on_default())
        .placeholder(clap::builder::styling::AnsiColor::Cyan.on_default())
        .error(clap::builder::styling::AnsiColor::Red.on_default())
        .valid(clap::builder::styling::AnsiColor::Green.on_default())
        .invalid(clap::builder::styling::AnsiColor::Red.on_default())
}

/// 更新 meta 配置中的项目列表
fn update_meta_projects(core: &core::rmm_core::RmmCore, project_id: &str, project_path: &std::path::Path) -> anyhow::Result<()> {
    let mut meta = core.get_meta_config()?;
    meta.projects.insert(project_id.to_string(), project_path.to_string_lossy().to_string());
    
    // 保存更新后的配置
    let meta_path = core.get_rmm_root().join("meta.toml");
    let meta_content = toml::to_string_pretty(&meta)?;
    std::fs::write(meta_path, meta_content)?;
    
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    
    match args.cmd {
        // 初始化命令
        Some(Commands::Init { project_id }) => {
            // 获取当前目录
            let current_dir = std::env::current_dir()?;
            
            // 处理项目ID和路径
            let (actual_project_id, project_path) = if project_id == "." {
                // 如果是 "."，使用当前目录名作为项目ID，在当前目录初始化
                let dir_name = current_dir.file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| anyhow::anyhow!("无法获取当前目录名"))?;
                (dir_name.to_string(), current_dir)
            } else {
                // 解析路径，可能是相对路径如 ./XXX/YYY
                let target_path = if project_id.starts_with('.') {
                    // 相对路径：./XXX/YYY 或 ../XXX
                    current_dir.join(&project_id).canonicalize()?
                } else {
                    // 直接名称：在当前目录下创建
                    current_dir.join(&project_id)
                };
                
                // 从最终路径提取项目ID（目录名）
                let dir_name = target_path.file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| anyhow::anyhow!("无法获取目标目录名"))?;
                
                // 如果不是相对路径，需要创建目录
                if !project_id.starts_with('.') {
                    std::fs::create_dir_all(&target_path)?;
                }
                
                (dir_name.to_string(), target_path)
            };

            // 从 meta 配置读取作者信息，如果没有则使用默认值
            let core = core::rmm_core::RmmCore::new();
            let (author_name, author_email) = match core.get_meta_config() {
                Ok(meta) => {
                    (meta.username, meta.email)
                }
                Err(_) => {
                    ("unknown".to_string(), "unknown@example.com".to_string())
                }
            };

            cmds::init::init_project(&project_path, &actual_project_id, &author_name, &author_email)?;
            
            // 更新 meta 配置中的 projects (ID = PATH)
            if let Err(e) = update_meta_projects(&core, &actual_project_id, &project_path) {
                eprintln!("⚠️ 警告: 无法更新 meta 配置: {}", e);
            }
            println!("{} 项目初始化成功！", "✅".green().bold());
        },

        // 构建命令
        Some(Commands::Build { project_path, no_auto_fix, script }) => {
            // 确定项目路径
            let target_path = if let Some(path) = project_path {
                PathBuf::from(path)
            } else {
                std::env::current_dir()?
            };
            
            // 规范化路径
            let project_path = target_path.canonicalize().unwrap_or(target_path);

            // 如果指定了脚本，运行脚本；否则运行构建
            if let Some(script_name) = script {
                let core = core::rmm_core::RmmCore::new();
                match core.run_rmake_script(&project_path, &script_name) {
                    Ok(()) => {
                        println!("{} 脚本执行成功！", "✅".green().bold());
                    }
                    Err(e) => {
                        // 如果脚本未找到，列出可用脚本
                        if e.to_string().contains("未找到") {
                            eprintln!("❌ 脚本 '{}' 未找到", script_name);
                            match core.list_rmake_scripts(&project_path) {
                                Ok(scripts) => {
                                    if scripts.is_empty() {
                                        eprintln!("📋 当前项目的Rmake.toml中没有定义任何脚本");
                                    } else {
                                        eprintln!("📋 可用脚本:");
                                        for script in scripts {
                                            eprintln!("   - {}", script);
                                        }
                                    }
                                }
                                Err(_) => {
                                    eprintln!("⚠️  无法读取Rmake.toml配置文件");
                                }
                            }
                        } else {
                            eprintln!("❌ 脚本执行失败: {}", e);
                        }
                        return Err(e);
                    }
                }
            } else {
                // 执行构建，传递自动修复参数
                let auto_fix = !no_auto_fix;  // 默认启用自动修复，除非用户明确禁用
                cmds::build::build_project_with_options(&project_path, auto_fix)?;
                println!("{} 构建成功！", "✅".green().bold());
            }
        },
        
        // 运行脚本命令
        Some(Commands::Run { project_path, script }) => {
            // 确定项目路径
            let target_path = if let Some(path) = project_path {
                PathBuf::from(path)
            } else {
                std::env::current_dir()?
            };
            
            // 规范化路径
            let project_path = target_path.canonicalize().unwrap_or(target_path);
            
            // 运行脚本
            cmds::run::run_script(&project_path, script.as_deref())?;
            
            if script.is_some() {
                println!("{} 脚本执行成功！", "✅".green().bold());
            }
        },
        
        // 同步项目元数据命令
        Some(Commands::Sync { project_name, projects_only, search_paths, max_depth }) => {
            // 转换 search_paths 为 &str 类型
            let search_paths_refs = search_paths.as_ref().map(|paths| {
                paths.iter().map(|s| s.as_str()).collect::<Vec<&str>>()
            });
            
            // 同步项目
            cmds::sync::sync_projects(
                project_name.as_deref(),
                projects_only,
                search_paths_refs,
                max_depth,
            )?;
            
            println!("{} 项目同步成功！", "✅".green().bold());
        },
        
        // 显示版本信息
        Some(Commands::Version) => {
            RmmBox::rmm_version();
        },

        // 外部命令 - 二进制版本不支持Python扩展
        Some(Commands::External(cmd)) => {
            println!("⚠️  二进制版本不支持 Python 扩展命令: {}", cmd.join(" "));
            eprintln!("💡 提示: Python 扩展命令需要通过 Python 包使用");
            std::process::exit(1);
        }
        
        // 没有提供子命令，默认显示带颜色的帮助
        None => {
            let mut cmd = Cli::command();
            cmd.print_help().ok();
        }
    }
    
    Ok(())
}
