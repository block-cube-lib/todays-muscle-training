use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Unit {
    Count,
    Second,
    LeftAndRightCount,
    LeftAndRightSecond,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraningSetting {
    id: String,
    name: String,
    description: String,
    min_amount: usize,
    increase_amount: usize,
    unit: Unit,
}

impl TraningSetting {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn description(&self) -> &String {
        &self.description
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Traning {
    pub traning_setting: TraningSetting,
    pub load: usize,
}

impl Traning {
    pub fn amount(&self) -> usize {
        let setting = &self.traning_setting;
        setting.min_amount + setting.increase_amount * (self.load - 1)
    }
    
    pub fn amount_with_unit(&self) -> String {
        let amount = self.amount();
        match self.traning_setting.unit {
            Unit::Count => format!("{}回", amount),
            Unit::Second => format!("{}秒", amount),
            Unit::LeftAndRightCount=> format!("左右{}回ずつ", amount),
            Unit::LeftAndRightSecond => format!("左右{}秒ずつ", amount),
        }
    }
}

impl Display for Traning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let setting = &self.traning_setting;
        let amount = self.amount();
        let unit = match self.traning_setting.unit {
            Unit::Count => format!("{}回", amount),
            Unit::Second => format!("{}秒", amount),
            Unit::LeftAndRightCount=> format!("左右{}回ずつ", amount),
            Unit::LeftAndRightSecond => format!("左右{}秒ずつ", amount),
        };
        write!(f, "{}を{}", setting.name, unit)
    }
}