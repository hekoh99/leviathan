#[derive(Debug, Clone, PartialEq)]
pub struct AccountSnapshot {
    pub buying_power: f64,
    pub equity: f64,
}

pub trait Broker {
    fn name(&self) -> &'static str;
    fn get_account(&self) -> Result<AccountSnapshot, String>;
    fn get_positions(&self) -> Result<Vec<String>, String>;
    fn submit_order(&self, symbol: &str, quantity: f64) -> Result<String, String>;
}

#[derive(Debug, Default)]
pub struct PaperBroker;

impl Broker for PaperBroker {
    fn name(&self) -> &'static str {
        "paper"
    }

    fn get_account(&self) -> Result<AccountSnapshot, String> {
        Ok(AccountSnapshot {
            buying_power: 10_000_000.0,
            equity: 10_000_000.0,
        })
    }

    fn get_positions(&self) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }

    fn submit_order(&self, symbol: &str, quantity: f64) -> Result<String, String> {
        Ok(format!("paper-order:{}:{quantity}", symbol))
    }
}
