use serde_json::{Map, Value};

use super::{processor, request, Opts};

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
#[derive(Debug)]
pub struct DiscoveryItem {
    variables: Map<String, Value>,
}

pub async fn start(opts: Opts) {
    info!("starting {} v:{}", crate_name!(), crate_version!());

    let (opts, mode) = determine_mode(opts);

    let discovery_items = match mode.as_str() {
        "nrql" => {
            let nerdgraph_data = request::nrql(opts).await;
            processor::handle_nerdgraph_payloads(nerdgraph_data, "nrql")
        }
        "entity" => {
            let nerdgraph_data = request::entity(opts).await;
            processor::handle_nerdgraph_payloads(nerdgraph_data, "entity")
        }
        _ => {
            let empty: Vec<DiscoveryItem> = vec![];
            empty
        }
    };

    println!("{:?}", discovery_items);
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
            let query_lower = &opts.query;
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
