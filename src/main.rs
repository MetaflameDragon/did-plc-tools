use std::collections::HashMap;
use url::Url;

fn main() {
    let handle = "scalytooth.metaflame.dev";
    let endpoint = "https://scalytooth.metaflame.dev";

    let unsigned_op = UnsignedPlcOperation {
        r#type: "plc_operation".to_string(),
        rotation_keys: vec![],
        verification_methods: HashMap::from([("".to_string(), DidKey("".to_string()))]),
        also_known_as: vec![AkaUri(format!("at://{handle}").to_string())],
        services: HashMap::from([(
            "atproto_pds".to_string(),
            PlcService {
                r#type: "AtprotoPersonalDataServer".to_string(),
                endpoint: Url::parse(endpoint).expect("invalid endpoint URL"),
            },
        )]),
        prev: None,
    };
}

struct DidKey(String);

struct Signature(String);

struct AkaUri(String);

struct PlcService {
    r#type: String,
    endpoint: Url,
}

struct PlcOperation<'a> {
    inner: &'a UnsignedPlcOperation<'a>,
    sig: Signature,
}

struct UnsignedPlcOperation<'a> {
    r#type: String,
    rotation_keys: Vec<DidKey>,
    verification_methods: HashMap<String, DidKey>,
    also_known_as: Vec<AkaUri>,
    services: HashMap<String, PlcService>,
    prev: Option<&'a PlcOperation<'a>>,
}
