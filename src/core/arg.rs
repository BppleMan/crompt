use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Arg {
    pub name: String,
    pub short: Option<String>,
    pub long: Option<String>,
    pub help: Option<String>,
    pub required: bool,
    pub takes_value: bool,
    pub default: Option<String>,
    pub validator: Option<String>,
    pub allowed: Vec<String>,
    pub multiple: bool,
    pub position: Option<u32>,
}
