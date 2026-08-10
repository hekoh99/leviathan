use leviathan_app::run_scaffold;
use leviathan_goal::sample_goal;

fn main() {
    let goal = sample_goal();
    let summary = run_scaffold(goal).expect("scaffold run should succeed");

    println!("Leviathan scaffold");
    println!("==================");
    println!("Flow: goal -> feasibility -> risk budget -> screened signal -> target -> execution");
    println!(
        "Goal: initial={} target={} horizon={}d risk={}",
        summary.goal.initial_capital.major_units(),
        summary.goal.target_value.major_units(),
        summary.goal.target_days,
        summary.goal.risk_profile
    );
    println!("Goal required CAGR: {:.2}%", summary.feasibility.required_cagr * 100.0);
    println!(
        "Estimated success probability: {:.0}%",
        summary.feasibility.estimated_success_probability * 100.0
    );
    println!(
        "Risk budget: single_position={}, cash_floor={}, max_drawdown={}",
        summary.risk_budget.max_single_position,
        summary.risk_budget.cash_floor,
        summary.risk_budget.max_drawdown_limit
    );
    println!("Screened candidates: {}", summary.screened_candidates);
    println!("Constructed cash weight: {}", summary.constructed_cash_weight);
    println!("Execution status: {}", summary.execution_status);
    println!(
        "Broker [{}] account: equity={:.0}, buying_power={:.0}",
        summary.broker_name,
        summary.broker_account.equity,
        summary.broker_account.buying_power
    );
    println!("Storage backend: {}", summary.storage_backend);
    println!("Next: implement SQLite schema, screening pipeline, and contracts generation.");
}
