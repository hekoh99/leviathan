from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import json
from typing import Any


@dataclass(slots=True)
class ServiceStatus:
    mode: str
    stateful: bool
    broker_access: bool
    db_access: bool


class ContractRegistry:
    def __init__(self, schema_dir: Path) -> None:
        self.schema_dir = schema_dir

    def load(self, name: str) -> dict[str, Any]:
        path = self.schema_dir / name
        return json.loads(path.read_text())

    def available(self) -> list[str]:
        return sorted(path.name for path in self.schema_dir.glob("*.schema.json"))


@dataclass(slots=True)
class ServiceEnvelope:
    version: str
    payload: dict[str, Any]


class IntelligenceService:
    def __init__(self, registry: ContractRegistry) -> None:
        self.registry = registry

    def analyze_cycle(self, request: ServiceEnvelope) -> ServiceEnvelope:
        self._require_supported_version(request)
        self.registry.load("analyze-cycle-request.schema.json")
        self.registry.load("analyze-cycle-response.schema.json")
        payload = {
            "signal": {
                "market_regime": "Neutral",
                "candidates": [],
                "cash_weight_hint": [1500, 2500],
                "confidence": 0.0,
            },
            "notes": ["stubbed stateless response"],
        }
        return ServiceEnvelope(version="V1", payload=payload)

    def explain_goal(self, request: ServiceEnvelope) -> ServiceEnvelope:
        self._require_supported_version(request)
        self.registry.load("explain-goal-request.schema.json")
        self.registry.load("explain-goal-response.schema.json")
        payload = {
            "summary": "Goal explanation not implemented yet.",
            "alternatives": ["Extend the target period.", "Lower the target amount."],
        }
        return ServiceEnvelope(version="V1", payload=payload)

    @staticmethod
    def _require_supported_version(request: ServiceEnvelope) -> None:
        if request.version != "V1":
            raise ValueError(f"unsupported contract version: {request.version}")


def describe_service_boundary() -> ServiceStatus:
    return ServiceStatus(
        mode="stateless",
        stateful=False,
        broker_access=False,
        db_access=False,
    )


def default_registry() -> ContractRegistry:
    schema_dir = Path(__file__).resolve().parents[3] / "schemas" / "generated"
    return ContractRegistry(schema_dir)


def main() -> None:
    status = describe_service_boundary()
    registry = default_registry()
    service = IntelligenceService(registry)
    response = service.explain_goal(ServiceEnvelope(version="V1", payload={}))
    print("Leviathan intelligence scaffold")
    print("==============================")
    print(f"mode: {status.mode}")
    print(f"stateful: {status.stateful}")
    print(f"broker_access: {status.broker_access}")
    print(f"db_access: {status.db_access}")
    print(f"loaded_contracts: {len(registry.available())}")
    print(f"sample_response_version: {response.version}")
    print("Next: add model routing, screening, grounding, and schema-generated contracts.")


if __name__ == "__main__":
    main()
