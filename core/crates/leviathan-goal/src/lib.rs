use leviathan_domain::{GoalSpec, Money, RiskBudget, RiskProfile, Weight};

#[derive(Debug, Clone, PartialEq)]
pub struct GoalFeasibility {
    pub required_cagr: f64,
    pub estimated_success_probability: f64,
}

pub fn required_cagr(goal: &GoalSpec) -> f64 {
    let ratio = goal.target_value.minor_units() as f64 / goal.initial_capital.minor_units() as f64;
    ratio.powf(365.0 / goal.target_days as f64) - 1.0
}

pub fn assess_goal(goal: &GoalSpec) -> GoalFeasibility {
    let cagr = required_cagr(goal);
    let base_probability: f64 = match goal.risk_profile {
        RiskProfile::Conservative => 0.20,
        RiskProfile::Moderate => 0.35,
        RiskProfile::Aggressive => 0.48,
    };

    let adjusted_probability = if cagr <= 0.10 {
        (base_probability + 0.25).min(0.95)
    } else if cagr <= 0.20 {
        (base_probability + 0.10).min(0.90)
    } else if cagr <= 0.35 {
        base_probability
    } else {
        (base_probability - 0.15).max(0.05)
    };

    GoalFeasibility {
        required_cagr: cagr,
        estimated_success_probability: adjusted_probability,
    }
}

pub fn baseline_risk_budget(goal: &GoalSpec) -> RiskBudget {
    match goal.risk_profile {
        RiskProfile::Conservative => RiskBudget {
            max_single_position: Weight::from_percent(8).expect("valid conservative single position"),
            cash_floor: Weight::from_percent(20).expect("valid conservative cash floor"),
            max_drawdown_limit: Weight::from_percent(8).expect("valid conservative drawdown"),
        },
        RiskProfile::Moderate => RiskBudget {
            max_single_position: Weight::from_percent(12).expect("valid moderate single position"),
            cash_floor: Weight::from_percent(12).expect("valid moderate cash floor"),
            max_drawdown_limit: Weight::from_percent(12).expect("valid moderate drawdown"),
        },
        RiskProfile::Aggressive => RiskBudget {
            max_single_position: Weight::from_percent(16).expect("valid aggressive single position"),
            cash_floor: Weight::from_percent(8).expect("valid aggressive cash floor"),
            max_drawdown_limit: Weight::from_percent(18).expect("valid aggressive drawdown"),
        },
    }
}

pub fn sample_goal() -> GoalSpec {
    GoalSpec {
        initial_capital: Money::from_major_units(10_000_000),
        target_value: Money::from_major_units(12_000_000),
        target_days: 365,
        risk_profile: RiskProfile::Moderate,
    }
}
