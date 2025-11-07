# pyrmm.cli package initializer
# Try to import the native extension if available; otherwise provide a clear fallback.

try:
    # The compiled extension module (built by maturin) will be named `rmmcore`.
    from .rmmcore import RmmCore
    from . import rmmcore as _rmmcore  # keep module object available
except Exception:  # pragma: no cover - only executed when extension is not present
    _rmmcore = None

    class RmmCore:  # simple stub to give nicer error when extension missing
        def __init__(self, *args, **kwargs) -> None:
            raise RuntimeError(
                "rmmcore native extension is not available. Build the extension with maturin to use RmmCore."
            )

__all__ = ["RmmCore", "rmmcore"]
