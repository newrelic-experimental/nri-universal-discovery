use super::Opts;
use bytes::Bytes;
use hyper::{Body, Client, Method, Request};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, fs};
extern crate base64;
use async_recursion::async_recursion;

#[derive(Serialize, Deserialize, Debug)]
pub struct NerdgraphPayload {
    data: Option<Data>,
    errors: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    actor: Actor,
}

#[derive(Serialize, Deserialize, Debug)]
struct Actor {
    account: Option<Account>,
    #[serde(rename = "entitySearch")]
    entity_search: Option<EntitySearch>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Account {
    nrql: Option<Nrql>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Nrql {
    results: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct EntitySearch {
    results: Option<Results>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Results {
    entities: Vec<Value>,
    #[serde(rename = "nextCursor")]
    next_cursor: Option<String>,
}

pub async fn nrql(opts: Opts) -> NerdgraphPayload {
    let account_id = match &opts.account_id {
        Some(v) => v.as_str(),
        _ => "",
    };

    let query = format!(
        r#"{{"query":"{{\n  actor {{\n    account(id: {}) {{\n      nrql(query: \"{}\") {{\n        results\n      }}\n    }}\n  }}\n}}\n"}}"#,
        account_id, &opts.query
    );

    let nerdgraph_json = fetch_data(&opts, &query).await;

    let nerdgraph_data: NerdgraphPayload =
        serde_json::from_str(nerdgraph_json.as_str()).expect(&format!(
            "unable to deserialize successful nerdgraph response, err: {}",
            nerdgraph_json
        ));

    return nerdgraph_data;
}

pub async fn entity(opts: Opts) -> Vec<NerdgraphPayload> {
    let mut nerdgraph_payloads: Vec<NerdgraphPayload> = Vec::new();
    let cursor = String::from("");

    nerdgraph_payloads = recursive_entity_query(opts, cursor, nerdgraph_payloads).await;

    println!("{:?}", nerdgraph_payloads.len());

    return nerdgraph_payloads;
}

#[async_recursion]
async fn recursive_entity_query(
    opts: Opts,
    cursor: String,
    mut nerdgraph_payloads: Vec<NerdgraphPayload>,
) -> Vec<NerdgraphPayload> {
    let query = build_search_query(&opts.query, cursor);

    let nerdgraph_json = fetch_data(&opts, &query).await;
    let nerdgraph_data: NerdgraphPayload =
        serde_json::from_str(nerdgraph_json.as_str()).expect(&format!(
            "unable to deserialize successful nerdgraph response, err: {}",
            nerdgraph_json
        ));
    nerdgraph_payloads.push(nerdgraph_data);

    let nerdgraph_data_match: NerdgraphPayload = serde_json::from_str(nerdgraph_json.as_str())
        .expect(&format!(
            "unable to deserialize successful nerdgraph response, err: {}",
            nerdgraph_json
        ));

    match nerdgraph_data_match.errors {
        Some(errors) => {
            if errors.as_array().unwrap().len() > 0 {
                error!("nerdgraph error: {} - query: {}", errors, query);
                return nerdgraph_payloads;
            }
        }
        _ => {}
    }

    match nerdgraph_data_match
        .data
        .unwrap()
        .actor
        .entity_search
        .unwrap()
        .results
        .unwrap()
        .next_cursor
    {
        Some(cursor) => {
            return recursive_entity_query(opts, cursor, nerdgraph_payloads).await;
        }
        _ => return nerdgraph_payloads,
    }
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

    let https = hyper_rustls::HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let req = Request::builder()
        .method(Method::POST)
        .uri(&opts.nerdgraph_url)
        .header("API-Key", &opts.api_key)
        .header("content-type", "application/json")
        .body(Body::from(query.to_string()))
        .unwrap();

    let mut resp = client.request(req).await.expect("nerdgraph request failed");

    let body_bytes = hyper::body::to_bytes(resp.body_mut()).await.unwrap();

    let nerdgraph_json =
        String::from_utf8(body_bytes.to_vec()).expect("unable to convert bytes to string");

    return nerdgraph_json;
}
