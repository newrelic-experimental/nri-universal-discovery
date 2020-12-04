use super::{handler, request, Opts};

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

pub async fn start(opts: Opts) {
    info!("starting {} v:{}", crate_name!(), crate_version!());

    let (opts, mode) = determine_mode(opts);

    match mode.as_str() {
        "nrql" => {
            let nerdgraph_data = request::nrql(opts).await;
            println!("{:?}", nerdgraph_data);
            // handler::nrql(json);
        }
        "entity" => {
            let nerdgraph_data = request::entity(opts).await;
            // println!("{:?}", nerdgraph_data);
        }
        _ => panic!("unknown mode"),
    }

    println!("{}", mode);
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
