use std::collections::HashMap;

use crate::types::{Address, Bytes, U256};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Clone, Debug, Deserialize, Default)]
/// A Block Hash or Block Number
pub struct StateOverride {
    map: HashMap<Address, StateOverrideItem>,
}

impl StateOverride {
    pub fn add(mut self, address: Address, item: StateOverrideItem) -> Self {
        self.map.insert(address, item);
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct StateOverrideItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<U256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<Bytes>,
}

impl Serialize for StateOverride {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.map.serialize(serializer)
    }
}
