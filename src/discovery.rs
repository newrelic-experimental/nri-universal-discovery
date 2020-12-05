use super::{decorator, processor, request, Opts};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

macro_rules! crate_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryItem {
    pub variables: Map<String, Value>,
}

pub async fn start(opts: Opts) {
    info!("starting {} v:{}", crate_name!(), crate_version!());

    let (opts, mode) = determine_mode(opts);

    let raw_discovery_items = match mode.as_str() {
        "nrql" => {
            opts.account_id.to_owned().expect("account id not defined");
            opts.api_key.to_owned().expect("api key not defined");
            let nerdgraph_data = request::nrql(&opts).await;
            processor::handle_nerdgraph_payloads(nerdgraph_data)
        }
        "entity" => {
            opts.api_key.to_owned().expect("api key not defined");
            let nerdgraph_data = request::entity(&opts).await;
            processor::handle_nerdgraph_payloads(nerdgraph_data)
        }
        "file" => processor::handle_file(&opts),
        _ => {
            let empty: Vec<Map<String, Value>> = vec![];
            empty
        }
    };

    let discovery_items = decorator::decorate_discovery_items(raw_discovery_items, &opts);

    let json = serde_json::to_string(&discovery_items).expect("json conversion failed");

    println!("{}", json);
}

// determine_mode determines if we will perform a nrql or entity search query
fn determine_mode(opts: Opts) -> (Opts, String) {
    match &opts.mode {
        Some(mode) => {
            let mode_lower = mode.to_lowercase();
            if mode_lower == "nrql" {
                return (opts, mode_lower);
            }
            if mode_lower == "entity" || mode_lower == "entitysearch" {
                return (opts, String::from("entity"));
            }
            panic!("invalid query mode {}", mode);
        }
        None => {
            let query_lower = &opts.query.to_owned().unwrap_or(String::from(""));
            if query_lower.contains("SELECT ") {
                return (opts, String::from("nrql"));
            }

            match &opts.discovery_file {
                Some(_) => {
                    return (opts, String::from("file"));
                }
                None => panic!("unable to determine discovery mode"),
            }
        }
    }
}
