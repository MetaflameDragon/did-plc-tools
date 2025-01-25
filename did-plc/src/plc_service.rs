use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlcService {
    pub r#type: String,
    pub endpoint: Url,
}

impl PlcService {
    pub fn new_atproto_pds(pds_endpoint: Url) -> Self {
        PlcService {
            r#type: "AtprotoPersonalDataServer".to_string(),
            endpoint: pds_endpoint,
        }
    }
}
