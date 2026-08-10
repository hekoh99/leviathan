use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use leviathan_domain::{PortfolioTarget, RiskBudget, Weight};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetValidationError {
    CashBelowFloor {
        cash_weight_bps: u16,
        cash_floor_bps: u16,
    },
    DuplicateSymbol {
        symbol: String,
    },
    ZeroWeightPosition {
        symbol: String,
    },
    PositionExceedsMaxSingle {
        symbol: String,
        weight_bps: u16,
        max_single_position_bps: u16,
    },
    GrossWeightMismatch {
        actual_bps: u32,
        expected_bps: u16,
    },
}

impl Display for TargetValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CashBelowFloor {
                cash_weight_bps,
                cash_floor_bps,
            } => write!(
                f,
                "cash weight {} bps is below cash floor {} bps",
                cash_weight_bps, cash_floor_bps
            ),
            Self::DuplicateSymbol { symbol } => write!(f, "duplicate symbol in target: {symbol}"),
            Self::ZeroWeightPosition { symbol } => write!(f, "position {symbol} has zero weight"),
            Self::PositionExceedsMaxSingle {
                symbol,
                weight_bps,
                max_single_position_bps,
            } => write!(
                f,
                "position {symbol} exceeds max single position: {} bps > {} bps",
                weight_bps, max_single_position_bps
            ),
            Self::GrossWeightMismatch {
                actual_bps,
                expected_bps,
            } => write!(
                f,
                "target weights must sum to {} bps, got {} bps",
                expected_bps, actual_bps
            ),
        }
    }
}

pub fn validate_target(
    target: &PortfolioTarget,
    risk_budget: &RiskBudget,
) -> Result<(), TargetValidationError> {
    if target.cash_weight < risk_budget.cash_floor {
        return Err(TargetValidationError::CashBelowFloor {
            cash_weight_bps: target.cash_weight.basis_points(),
            cash_floor_bps: risk_budget.cash_floor.basis_points(),
        });
    }

    let mut seen = HashSet::new();
    for (symbol, weight) in &target.weights {
        if !seen.insert(symbol.0.as_str()) {
            return Err(TargetValidationError::DuplicateSymbol {
                symbol: symbol.0.clone(),
            });
        }

        if weight.basis_points() == 0 {
            return Err(TargetValidationError::ZeroWeightPosition {
                symbol: symbol.0.clone(),
            });
        }

        if *weight > risk_budget.max_single_position {
            return Err(TargetValidationError::PositionExceedsMaxSingle {
                symbol: symbol.0.clone(),
                weight_bps: weight.basis_points(),
                max_single_position_bps: risk_budget.max_single_position.basis_points(),
            });
        }
    }

    let gross_weight_bps: u32 = target
        .weights
        .iter()
        .map(|(_, weight)| u32::from(weight.basis_points()))
        .sum::<u32>()
        + u32::from(target.cash_weight.basis_points());
    if gross_weight_bps != u32::from(Weight::FULLY_INVESTED_BPS) {
        return Err(TargetValidationError::GrossWeightMismatch {
            actual_bps: gross_weight_bps,
            expected_bps: Weight::FULLY_INVESTED_BPS,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_target;
    use leviathan_domain::{PortfolioTarget, RiskBudget, Symbol, Weight};

    fn budget() -> RiskBudget {
        RiskBudget {
            max_single_position: Weight::from_percent(20).unwrap(),
            cash_floor: Weight::from_percent(10).unwrap(),
            max_drawdown_limit: Weight::from_percent(12).unwrap(),
        }
    }

    #[test]
    fn rejects_duplicate_symbols() {
        let target = PortfolioTarget {
            weights: vec![
                (Symbol("005930".into()), Weight::from_percent(15).unwrap()),
                (Symbol("005930".into()), Weight::from_percent(15).unwrap()),
            ],
            cash_weight: Weight::from_percent(70).unwrap(),
        };

        assert!(validate_target(&target, &budget()).is_err());
    }

    #[test]
    fn rejects_incorrect_weight_sum() {
        let target = PortfolioTarget {
            weights: vec![(Symbol("005930".into()), Weight::from_percent(20).unwrap())],
            cash_weight: Weight::from_percent(50).unwrap(),
        };

        assert!(validate_target(&target, &budget()).is_err());
    }
}
