pub enum EngineType {
    Chromium,
    Gecko,
}

use std::fmt;

impl fmt::Display for EngineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineType::Chromium => write!(f, "chromium"),
            EngineType::Gecko => write!(f, "gecko"),
        }
    }
}
