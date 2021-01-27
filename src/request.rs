use super::Opts;
use serde::{Deserialize, Serialize};
use serde_json::Value;
extern crate base64;
use async_recursion::async_recursion;
use reqwest::header::USER_AGENT;

#[derive(Serialize, Deserialize, Debug)]
pub struct NerdgraphPayload {
    pub data: Option<Data>,
    pub errors: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub actor: Actor,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Actor {
    pub account: Option<Account>,
    #[serde(rename = "entitySearch")]
    pub entity_search: Option<EntitySearch>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub nrql: Option<Nrql>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Nrql {
    pub results: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntitySearch {
    pub results: Option<Results>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Results {
    pub entities: Vec<Value>,
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<String>,
}

macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

pub async fn nrql(opts: &Opts) -> Vec<NerdgraphPayload> {
    let account_id = match &opts.account_id {
        Some(v) => v.as_str(),
        _ => "",
    };

    let query = format!(
        r#"{{"query":"{{\n  actor {{\n    account(id: {}) {{\n      nrql(query: \"{}\") {{\n        results\n      }}\n    }}\n  }}\n}}\n"}}"#,
        account_id,
        &opts.query.to_owned().unwrap()
    );

    let nerdgraph_json = fetch_data(&opts, &query).await;

    let nerdgraph_data: NerdgraphPayload =
        serde_json::from_str(nerdgraph_json.as_str()).expect(&format!(
            "unable to deserialize successful nerdgraph response, err: {}",
            nerdgraph_json
        ));

    // return as vector to simplify
    let mut nerdgraph_payloads: Vec<NerdgraphPayload> = Vec::new();
    nerdgraph_payloads.push(nerdgraph_data);

    return nerdgraph_payloads;
}

pub async fn entity(opts: &Opts) -> Vec<NerdgraphPayload> {
    let mut nerdgraph_payloads: Vec<NerdgraphPayload> = Vec::new();
    let cursor = String::from("");

    nerdgraph_payloads = recursive_entity_query(&opts, cursor, nerdgraph_payloads).await;

    debug!(
        "entity search request count: {:?}",
        nerdgraph_payloads.len()
    );

    return nerdgraph_payloads;
}

#[async_recursion]
async fn recursive_entity_query(
    opts: &Opts,
    cursor: String,
    mut nerdgraph_payloads: Vec<NerdgraphPayload>,
) -> Vec<NerdgraphPayload> {
    let query = build_search_query(&opts.query.to_owned().unwrap(), cursor);

    let nerdgraph_json = fetch_data(&opts, &query).await;
    let nerdgraph_data: NerdgraphPayload =
        serde_json::from_str(nerdgraph_json.as_str()).expect(&format!(
            "unable to deserialize successful nerdgraph response, err: {}",
            nerdgraph_json
        ));

    match &nerdgraph_data.errors {
        Some(errors) => {
            if errors.as_array().unwrap().len() > 0 {
                error!("nerdgraph error: {} - query: {}", errors, query);
                return nerdgraph_payloads;
            }
        }
        _ => {}
    }

    match nerdgraph_data
        .data
        .clone()
        .unwrap()
        .actor
        .entity_search
        .unwrap()
        .results
        .unwrap()
        .next_cursor
    {
        Some(cursor) => {
            nerdgraph_payloads.push(nerdgraph_data);
            return recursive_entity_query(opts, cursor, nerdgraph_payloads).await;
        }
        _ => {
            nerdgraph_payloads.push(nerdgraph_data);
            return nerdgraph_payloads;
        }
    };
}

fn build_search_query(query: &str, cursor: String) -> String {
    if cursor != "" {
        return format!(
            concat!(
                r#"{{"query":"{{\n"#,
                r#"actor {{\n"#,
                r#"entitySearch(query: \"{}\") {{\n"#,
                r#"results(cursor: \"{}\") {{\n"#,
                r#"nextCursor\n entities {{\n"#,
                r#"guid\n"#,
                r#"name\n"#,
                r#"domain\n"#,
                r#"type\n"#,
                r#"entityType\n"#,
                r#"tags {{\n            key\n            values\n          }}\n"#,
                r#"}}\n"#,
                r#"}}\n"#,
                r#"}}\n"#,
                r#"}}\n"#,
                r#"}}\n""#,
                r#"}}"#
            ),
            query, cursor
        );
    }

    return format!(
        concat!(
            r#"{{"query":"{{\n"#,
            r#"actor {{\n"#,
            r#"entitySearch(query: \"{}\") {{\n"#,
            r#"results {{\n"#,
            r#"nextCursor\n entities {{\n"#,
            r#"guid\n"#,
            r#"name\n"#,
            r#"domain\n"#,
            r#"type\n"#,
            r#"entityType\n"#,
            r#"tags {{\n            key\n            values\n          }}\n"#,
            r#"}}\n"#,
            r#"}}\n"#,
            r#"}}\n"#,
            r#"}}\n"#,
            r#"}}\n""#,
            r#"}}"#
        ),
        query
    );
}

async fn fetch_data(opts: &Opts, query: &String) -> String {
    info!("fetching {:?} data", &opts.mode);
    debug!("nerdgraph query: {}", &query);

    let proxy_url = &opts.proxy_url.to_owned().unwrap_or("".to_string());
    let client = if proxy_url.as_str() != "" {
        reqwest::Client::builder()
            .proxy(reqwest::Proxy::all(proxy_url).expect("failed to set proxy"))
            .build()
            .expect("failed to build client with proxy")
    } else {
        reqwest::Client::new()
    };

    let res = client
        .post(&opts.nerdgraph_url)
        .body(query.to_string())
        .header("API-Key", &opts.api_key.to_owned().unwrap())
        .header("content-type", "application/json")
        .header(
            USER_AGENT,
            format!("nri-universal-discovery/{}", crate_version!()),
        )
        .send()
        .await
        .expect("nerdgraph request failed");

    let body_bytes = res.bytes().await.unwrap();

    let nerdgraph_json =
        String::from_utf8(body_bytes.to_vec()).expect("unable to convert bytes to string");

    return nerdgraph_json;
}
