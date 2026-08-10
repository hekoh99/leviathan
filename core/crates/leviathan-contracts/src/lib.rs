use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use leviathan_domain::{CycleSignal, GoalSpec, PortfolioTarget, RiskBudget};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum ContractVersion {
    V1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Envelope<T> {
    pub version: ContractVersion,
    pub payload: T,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AnalyzeCycleRequest {
    pub goal: GoalSpec,
    pub risk_budget: RiskBudget,
    pub candidate_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AnalyzeCycleResponse {
    pub signal: CycleSignal,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ExplainGoalRequest {
    pub goal: GoalSpec,
    pub required_cagr: f64,
    pub success_probability: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ExplainGoalResponse {
    pub summary: String,
    pub alternatives: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ConstructPortfolioRequest {
    pub signal: CycleSignal,
    pub risk_budget: RiskBudget,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ConstructPortfolioResponse {
    pub target: PortfolioTarget,
}

pub fn generated_schemas() -> Vec<(&'static str, Value)> {
    vec![
        (
            "analyze-cycle-request.schema.json",
            schema_to_value(schema_for!(Envelope<AnalyzeCycleRequest>)),
        ),
        (
            "analyze-cycle-response.schema.json",
            schema_to_value(schema_for!(Envelope<AnalyzeCycleResponse>)),
        ),
        (
            "explain-goal-request.schema.json",
            schema_to_value(schema_for!(Envelope<ExplainGoalRequest>)),
        ),
        (
            "explain-goal-response.schema.json",
            schema_to_value(schema_for!(Envelope<ExplainGoalResponse>)),
        ),
        (
            "construct-portfolio-request.schema.json",
            schema_to_value(schema_for!(Envelope<ConstructPortfolioRequest>)),
        ),
        (
            "construct-portfolio-response.schema.json",
            schema_to_value(schema_for!(Envelope<ConstructPortfolioResponse>)),
        ),
    ]
}

fn schema_to_value<T: Serialize>(schema: T) -> Value {
    serde_json::to_value(schema).expect("schema should serialize to JSON")
}
