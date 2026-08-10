#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationPlan {
    pub backend: &'static str,
    pub notes: Vec<&'static str>,
}

pub fn sqlite_bootstrap_plan() -> MigrationPlan {
    MigrationPlan {
        backend: "sqlite",
        notes: vec![
            "enable WAL",
            "enable foreign_keys",
            "set busy_timeout",
            "create append-oriented ledger tables",
            "store decision context and prompt version metadata",
        ],
    }
}
