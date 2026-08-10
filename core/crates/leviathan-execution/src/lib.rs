use std::fmt::{Display, Formatter};

use leviathan_domain::{CycleSignal, ExecutionMode, PortfolioTarget, RiskBudget, Symbol, Weight};
use leviathan_risk::{validate_target, TargetValidationError};

#[derive(Debug, Clone, PartialEq)]
pub struct TradeProposal {
    pub target: PortfolioTarget,
    pub mode: ExecutionMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstructTargetError {
    CashHintInverted {
        min_cash_bps: u16,
        max_cash_bps: u16,
    },
    CashFloorExceedsHintMax {
        cash_floor_bps: u16,
        cash_hint_max_bps: u16,
    },
    CandidateHasZeroConviction {
        symbol: String,
    },
    CandidateBandInverted {
        symbol: String,
        min_weight_bps: u16,
        max_weight_bps: u16,
    },
    CandidateLowerExceedsEffectiveUpper {
        symbol: String,
        min_weight_bps: u16,
        effective_upper_bps: u16,
    },
    NoFeasibleAllocation {
        min_required_cash_bps: u16,
        max_allowed_cash_bps: u16,
    },
    WeightOutOfRange {
        basis_points: u32,
    },
    Validation(TargetValidationError),
}

impl Display for ConstructTargetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CashHintInverted {
                min_cash_bps,
                max_cash_bps,
            } => write!(
                f,
                "cash hint is inverted: min {} bps exceeds max {} bps",
                min_cash_bps, max_cash_bps
            ),
            Self::CashFloorExceedsHintMax {
                cash_floor_bps,
                cash_hint_max_bps,
            } => write!(
                f,
                "cash floor {} bps exceeds signal cash hint upper bound {} bps",
                cash_floor_bps, cash_hint_max_bps
            ),
            Self::CandidateHasZeroConviction { symbol } => {
                write!(f, "candidate {symbol} has zero conviction")
            }
            Self::CandidateBandInverted {
                symbol,
                min_weight_bps,
                max_weight_bps,
            } => write!(
                f,
                "candidate {symbol} has inverted weight band: {} bps > {} bps",
                min_weight_bps, max_weight_bps
            ),
            Self::CandidateLowerExceedsEffectiveUpper {
                symbol,
                min_weight_bps,
                effective_upper_bps,
            } => write!(
                f,
                "candidate {symbol} lower bound {} bps exceeds effective upper {} bps",
                min_weight_bps, effective_upper_bps
            ),
            Self::NoFeasibleAllocation {
                min_required_cash_bps,
                max_allowed_cash_bps,
            } => write!(
                f,
                "no feasible cash allocation exists: minimum required cash {} bps exceeds maximum allowed cash {} bps",
                min_required_cash_bps, max_allowed_cash_bps
            ),
            Self::WeightOutOfRange { basis_points } => {
                write!(f, "weight basis points out of range: {basis_points}")
            }
            Self::Validation(error) => error.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
struct CandidateAllocation {
    symbol: Symbol,
    conviction: u8,
    upper_bps: u16,
    assigned_bps: u16,
}

pub fn construct_target(
    signal: &CycleSignal,
    budget: &RiskBudget,
) -> Result<PortfolioTarget, ConstructTargetError> {
    let (cash_hint_min, cash_hint_max) = signal.cash_weight_hint;
    if cash_hint_min > cash_hint_max {
        return Err(ConstructTargetError::CashHintInverted {
            min_cash_bps: cash_hint_min.basis_points(),
            max_cash_bps: cash_hint_max.basis_points(),
        });
    }
    if budget.cash_floor > cash_hint_max {
        return Err(ConstructTargetError::CashFloorExceedsHintMax {
            cash_floor_bps: budget.cash_floor.basis_points(),
            cash_hint_max_bps: cash_hint_max.basis_points(),
        });
    }

    let mut allocations = Vec::with_capacity(signal.candidates.len());
    let mut min_invested_bps = 0_u32;
    let mut max_invested_bps = 0_u32;
    let mut total_conviction = 0_u32;

    for candidate in &signal.candidates {
        if candidate.conviction == 0 {
            return Err(ConstructTargetError::CandidateHasZeroConviction {
                symbol: candidate.symbol.0.clone(),
            });
        }

        let (lower_band, upper_band) = candidate.suggested_weight_band;
        if lower_band > upper_band {
            return Err(ConstructTargetError::CandidateBandInverted {
                symbol: candidate.symbol.0.clone(),
                min_weight_bps: lower_band.basis_points(),
                max_weight_bps: upper_band.basis_points(),
            });
        }

        let effective_upper = upper_band.min(budget.max_single_position);
        if lower_band > effective_upper {
            return Err(ConstructTargetError::CandidateLowerExceedsEffectiveUpper {
                symbol: candidate.symbol.0.clone(),
                min_weight_bps: lower_band.basis_points(),
                effective_upper_bps: effective_upper.basis_points(),
            });
        }

        min_invested_bps += u32::from(lower_band.basis_points());
        max_invested_bps += u32::from(effective_upper.basis_points());
        total_conviction += u32::from(candidate.conviction);

        allocations.push(CandidateAllocation {
            symbol: Symbol(candidate.symbol.0.clone()),
            conviction: candidate.conviction,
            upper_bps: effective_upper.basis_points(),
            assigned_bps: lower_band.basis_points(),
        });
    }

    let min_cash_bps = u32::from(budget.cash_floor.max(cash_hint_min).basis_points())
        .max(u32::from(Weight::FULLY_INVESTED_BPS).saturating_sub(max_invested_bps));
    let max_cash_bps = u32::from(cash_hint_max.basis_points())
        .min(u32::from(Weight::FULLY_INVESTED_BPS).saturating_sub(min_invested_bps));

    if min_cash_bps > max_cash_bps {
        return Err(ConstructTargetError::NoFeasibleAllocation {
            min_required_cash_bps: min_cash_bps as u16,
            max_allowed_cash_bps: max_cash_bps as u16,
        });
    }

    let cash_weight =
        weight_from_u32(min_cash_bps).map_err(|_| ConstructTargetError::WeightOutOfRange {
            basis_points: min_cash_bps,
        })?;

    if allocations.is_empty() {
        let target = PortfolioTarget {
            weights: Vec::new(),
            cash_weight,
        };
        validate_target(&target, budget).map_err(ConstructTargetError::Validation)?;
        return Ok(target);
    }

    if total_conviction == 0 {
        return Err(ConstructTargetError::NoFeasibleAllocation {
            min_required_cash_bps: min_cash_bps as u16,
            max_allowed_cash_bps: max_cash_bps as u16,
        });
    }

    let target_invested_bps = u32::from(Weight::FULLY_INVESTED_BPS) - min_cash_bps;
    let mut remaining_bps = target_invested_bps.saturating_sub(min_invested_bps);

    while remaining_bps > 0 {
        let mut distributed_in_pass = 0_u32;

        for allocation in &mut allocations {
            if remaining_bps == 0 {
                break;
            }

            let capacity_bps = u32::from(allocation.upper_bps - allocation.assigned_bps);
            if capacity_bps == 0 {
                continue;
            }

            let share_bps = ((remaining_bps * u32::from(allocation.conviction)) / total_conviction)
                .max(1)
                .min(capacity_bps)
                .min(remaining_bps);

            allocation.assigned_bps += share_bps as u16;
            remaining_bps -= share_bps;
            distributed_in_pass += share_bps;
        }

        if distributed_in_pass == 0 {
            return Err(ConstructTargetError::NoFeasibleAllocation {
                min_required_cash_bps: min_cash_bps as u16,
                max_allowed_cash_bps: max_cash_bps as u16,
            });
        }
    }

    let target = PortfolioTarget {
        weights: allocations
            .into_iter()
            .map(|allocation| {
                weight_from_u32(u32::from(allocation.assigned_bps))
                    .map(|weight| (allocation.symbol, weight))
                    .map_err(|_| ConstructTargetError::WeightOutOfRange {
                        basis_points: u32::from(allocation.assigned_bps),
                    })
            })
            .collect::<Result<Vec<_>, _>>()?,
        cash_weight,
    };

    validate_target(&target, budget).map_err(ConstructTargetError::Validation)?;
    Ok(target)
}

pub fn prepare_execution(
    proposal: &TradeProposal,
    budget: &RiskBudget,
) -> Result<String, TargetValidationError> {
    validate_target(&proposal.target, budget)?;

    let action = match proposal.mode {
        ExecutionMode::Paper => "paper execution permitted",
        ExecutionMode::LiveRequiresApproval => "approval required before live execution",
        ExecutionMode::LiveLimitedAuto => "limited live automation permitted",
        ExecutionMode::LiveAuto => "full live automation permitted",
    };

    Ok(action.to_string())
}

fn weight_from_u32(basis_points: u32) -> Result<Weight, ()> {
    let bps = u16::try_from(basis_points).map_err(|_| ())?;
    Weight::from_basis_points(bps).map_err(|_| ())
}

#[cfg(test)]
mod tests {
    use super::{construct_target, ConstructTargetError};
    use leviathan_domain::{
        CandidateSignal, CycleSignal, MarketRegime, RiskBudget, Symbol, Weight,
    };

    fn weight(percent: u16) -> Weight {
        Weight::from_percent(percent).unwrap()
    }

    fn budget() -> RiskBudget {
        RiskBudget {
            max_single_position: weight(20),
            cash_floor: weight(10),
            max_drawdown_limit: weight(12),
        }
    }

    #[test]
    fn enforces_lower_bounds_in_constructed_target() {
        let signal = CycleSignal {
            market_regime: MarketRegime::Neutral,
            candidates: vec![
                CandidateSignal {
                    symbol: Symbol("AAA".into()),
                    conviction: 2,
                    suggested_weight_band: (weight(10), weight(40)),
                    horizon_days: 10,
                    thesis: String::new(),
                    key_risks: Vec::new(),
                },
                CandidateSignal {
                    symbol: Symbol("BBB".into()),
                    conviction: 1,
                    suggested_weight_band: (weight(8), weight(30)),
                    horizon_days: 10,
                    thesis: String::new(),
                    key_risks: Vec::new(),
                },
            ],
            cash_weight_hint: (weight(15), weight(70)),
            confidence: 0.5,
        };

        let target = construct_target(&signal, &budget()).unwrap();
        assert!(target.weights[0].1 >= weight(10));
        assert!(target.weights[1].1 >= weight(8));
    }

    #[test]
    fn rejects_inverted_candidate_bands() {
        let signal = CycleSignal {
            market_regime: MarketRegime::Neutral,
            candidates: vec![CandidateSignal {
                symbol: Symbol("AAA".into()),
                conviction: 1,
                suggested_weight_band: (weight(12), weight(10)),
                horizon_days: 10,
                thesis: String::new(),
                key_risks: Vec::new(),
            }],
            cash_weight_hint: (weight(15), weight(40)),
            confidence: 0.5,
        };

        let error = construct_target(&signal, &budget()).unwrap_err();
        assert!(matches!(error, ConstructTargetError::CandidateBandInverted { .. }));
    }

    #[test]
    fn rejects_when_lower_bounds_cannot_fit_with_cash_constraints() {
        let permissive_budget = RiskBudget {
            max_single_position: weight(60),
            cash_floor: weight(10),
            max_drawdown_limit: weight(12),
        };
        let signal = CycleSignal {
            market_regime: MarketRegime::Neutral,
            candidates: vec![
                CandidateSignal {
                    symbol: Symbol("AAA".into()),
                    conviction: 1,
                    suggested_weight_band: (weight(50), weight(50)),
                    horizon_days: 10,
                    thesis: String::new(),
                    key_risks: Vec::new(),
                },
                CandidateSignal {
                    symbol: Symbol("BBB".into()),
                    conviction: 1,
                    suggested_weight_band: (weight(45), weight(45)),
                    horizon_days: 10,
                    thesis: String::new(),
                    key_risks: Vec::new(),
                },
            ],
            cash_weight_hint: (weight(10), weight(10)),
            confidence: 0.5,
        };

        let error = construct_target(&signal, &permissive_budget).unwrap_err();
        assert!(matches!(error, ConstructTargetError::NoFeasibleAllocation { .. }));
    }
}
