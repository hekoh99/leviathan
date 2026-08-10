from __future__ import annotations

from pathlib import Path
import json
from typing import Any


def schema_directory() -> Path:
    return Path(__file__).resolve().parents[3] / "schemas" / "generated"


def load_schema(name: str) -> dict[str, Any]:
    return json.loads((schema_directory() / name).read_text())
