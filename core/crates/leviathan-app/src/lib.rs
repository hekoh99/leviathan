use leviathan_broker::{AccountSnapshot, Broker, PaperBroker};
use leviathan_domain::{CandidateSignal, CycleSignal, ExecutionMode, GoalSpec, MarketRegime, Weight};
use leviathan_execution::{construct_target, prepare_execution, TradeProposal};
use leviathan_goal::{assess_goal, baseline_risk_budget, GoalFeasibility};
use leviathan_storage::sqlite_bootstrap_plan;

#[derive(Debug, Clone, PartialEq)]
pub struct RunSummary {
    pub goal: GoalSpec,
    pub feasibility: GoalFeasibility,
    pub risk_budget: leviathan_domain::RiskBudget,
    pub screened_candidates: usize,
    pub constructed_cash_weight: Weight,
    pub execution_status: String,
    pub broker_name: &'static str,
    pub broker_account: AccountSnapshot,
    pub storage_backend: &'static str,
}

pub fn run_scaffold(goal: GoalSpec) -> Result<RunSummary, String> {
    let feasibility = assess_goal(&goal);
    let risk_budget = baseline_risk_budget(&goal);
    let signal = scaffold_signal();
    let target = construct_target(&signal, &risk_budget).map_err(|error| error.to_string())?;
    let proposal = TradeProposal {
        target,
        mode: ExecutionMode::Paper,
    };
    let execution_status = prepare_execution(&proposal, &risk_budget).map_err(|error| error.to_string())?;

    let broker = PaperBroker;
    let broker_account = broker.get_account().map_err(|error| format!("broker account unavailable: {error}"))?;
    let storage = sqlite_bootstrap_plan();

    Ok(RunSummary {
        goal,
        feasibility,
        risk_budget,
        screened_candidates: signal.candidates.len(),
        constructed_cash_weight: proposal.target.cash_weight,
        execution_status,
        broker_name: broker.name(),
        broker_account,
        storage_backend: storage.backend,
    })
}

fn scaffold_signal() -> CycleSignal {
    CycleSignal {
        market_regime: MarketRegime::Neutral,
        candidates: vec![
            CandidateSignal {
                symbol: leviathan_domain::Symbol("005930".to_string()),
                conviction: 4,
                suggested_weight_band: (
                    Weight::from_percent(5).expect("valid lower band"),
                    Weight::from_percent(12).expect("valid upper band"),
                ),
                horizon_days: 20,
                thesis: "Large-cap liquidity anchor in the initial shortlist".to_string(),
                key_risks: vec!["earnings disappointment".to_string()],
            },
            CandidateSignal {
                symbol: leviathan_domain::Symbol("000660".to_string()),
                conviction: 3,
                suggested_weight_band: (
                    Weight::from_percent(5).expect("valid lower band"),
                    Weight::from_percent(12).expect("valid upper band"),
                ),
                horizon_days: 20,
                thesis: "Second shortlist candidate from the scaffolded screening step".to_string(),
                key_risks: vec!["cycle slowdown".to_string()],
            },
        ],
        cash_weight_hint: (
            Weight::from_percent(76).expect("valid cash hint min"),
            Weight::from_percent(90).expect("valid cash hint max"),
        ),
        confidence: 0.71,
    }
}
