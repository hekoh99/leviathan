use std::fmt::{Display, Formatter};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RunId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Symbol(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RiskProfile {
    Conservative,
    Moderate,
    Aggressive,
}

impl Display for RiskProfile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Conservative => "conservative",
            Self::Moderate => "moderate",
            Self::Aggressive => "aggressive",
        };
        f.write_str(label)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum MarketRegime {
    RiskOff,
    Neutral,
    RiskOn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum ExecutionMode {
    Paper,
    LiveRequiresApproval,
    LiveLimitedAuto,
    LiveAuto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct Money {
    minor_units: i64,
}

impl Money {
    pub fn from_minor_units(minor_units: i64) -> Self {
        Self { minor_units }
    }

    pub fn from_major_units(major_units: i64) -> Self {
        Self {
            minor_units: major_units,
        }
    }

    pub fn minor_units(self) -> i64 {
        self.minor_units
    }

    pub fn major_units(self) -> i64 {
        self.minor_units
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct Weight {
    basis_points: u16,
}

impl Weight {
    pub const FULLY_INVESTED_BPS: u16 = 10_000;

    pub fn from_basis_points(basis_points: u16) -> Result<Self, String> {
        if basis_points > Self::FULLY_INVESTED_BPS {
            return Err(format!(
                "weight basis points must be <= {}, got {}",
                Self::FULLY_INVESTED_BPS,
                basis_points
            ));
        }

        Ok(Self { basis_points })
    }

    pub fn from_percent(percent: u16) -> Result<Self, String> {
        Self::from_basis_points(percent.saturating_mul(100))
    }

    pub fn basis_points(self) -> u16 {
        self.basis_points
    }

    pub fn ratio(self) -> f64 {
        self.basis_points as f64 / Self::FULLY_INVESTED_BPS as f64
    }
}

impl Display for Weight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}%", self.ratio() * 100.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GoalSpec {
    pub initial_capital: Money,
    pub target_value: Money,
    pub target_days: u32,
    pub risk_profile: RiskProfile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CandidateSignal {
    pub symbol: Symbol,
    pub conviction: u8,
    pub suggested_weight_band: (Weight, Weight),
    pub horizon_days: u32,
    pub thesis: String,
    pub key_risks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CycleSignal {
    pub market_regime: MarketRegime,
    pub candidates: Vec<CandidateSignal>,
    pub cash_weight_hint: (Weight, Weight),
    pub confidence: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RiskBudget {
    pub max_single_position: Weight,
    pub cash_floor: Weight,
    pub max_drawdown_limit: Weight,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PortfolioTarget {
    pub weights: Vec<(Symbol, Weight)>,
    pub cash_weight: Weight,
}
