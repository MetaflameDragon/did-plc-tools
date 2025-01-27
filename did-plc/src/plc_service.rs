use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlcService {
    pub r#type: String,
    pub endpoint: String, // Not validated to be a URL (but should usually be a URL?)
}

impl PlcService {
    pub fn new_atproto_pds(pds_endpoint: String) -> Self {
        PlcService {
            r#type: "AtprotoPersonalDataServer".to_string(),
            endpoint: pds_endpoint,
        }
    }
}
