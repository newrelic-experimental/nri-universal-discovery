use super::{handler, Opts};
use bytes::Bytes;
use futures::join;
use hyper::{Body, Client, Method, Request};
use serde_json::Value;
use std::{env, fs};

static NERDGRAPH_URL: &str = "https://api.newrelic.com/graphql";
pub static INTEGRATIONS_DIR_NIX: &str = "/etc/newrelic-infra/integrations.d";
pub static INTEGRATIONS_DIR_WIN: &str =
    "C:\\Program Files\\New Relic\\newrelic-infra\\integrations.d";

#[derive(Debug)]
// IntegrationsFile named with plural as multiple integrations can be in a single file
pub struct IntegrationsFile {
    pub name: String,
    pub path: String,
    pub found_in_collection: bool,
}

impl IntegrationsFile {
    pub fn found_file(&mut self) {
        self.found_in_collection = true;
    }
}

#[derive(Debug)]
pub struct SyncStats {
    pub files_written: i32,
    pub files_updated: i32,
    pub files_already_updated: i32,
    pub files_deleted: i32,
    pub files_in_collection: usize,
    pub errors_writing: i32,
    pub errors_deleting: i32,
    pub errors_reading: i32,
}

impl SyncStats {
    pub fn add_written(&mut self) {
        self.files_written += 1;
    }
    pub fn add_updated(&mut self) {
        self.files_updated += 1;
    }
    pub fn add_already_updated(&mut self) {
        self.files_already_updated += 1;
    }
    pub fn add_deleted(&mut self) {
        self.files_deleted += 1;
    }
    pub fn add_err_writing(&mut self) {
        self.errors_writing += 1;
    }
    pub fn add_err_deleting(&mut self) {
        self.errors_deleting += 1;
    }
    pub fn add_err_reading(&mut self) {
        self.errors_reading += 1;
    }
}

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

    // collect local files, and retrieve collection async
    let nrsync_files = discover_nrsync_files();
    let collection = fetch_collection(&opts.account_id, opts.uuid, opts.api_key, &opts.collection);

    // wait for futures to resolve
    let results = join!(nrsync_files, collection);
    let files: Vec<IntegrationsFile> = results.0;
    let nerdgraph_bytes: Bytes = results.1;

    // handle the requested collection and files on disk
    let sync_stats = handler::start(files, nerdgraph_bytes);

    // report infrastructure metrics event
    report_event(sync_stats, &opts.collection, &opts.account_id);
}

// fetch collection
// fetch config collection from nerdpacks nerdstorage
async fn fetch_collection(
    account_id: &String,
    uuid: String,
    api_key: String,
    collection: &String,
) -> Bytes {
    info!("fetching collection: {}", collection);

    let https = hyper_rustls::HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let collection_query = format!(
        r#"{{
            "operationName":"AccountCollectionStorage",
            "variables":{{"accountId":{},"collection":"{}"}},
            "query":"query AccountCollectionStorage($accountId: Int\u0021, $collection: String\u0021) {{  actor {{    account(id: $accountId) {{      id      nerdStorage {{        collection(collection: $collection) {{          id          document        }}      }}    }}  }}}}"}}"#,
        account_id, collection
    );

    let req = Request::builder()
        .method(Method::POST)
        .uri(NERDGRAPH_URL)
        .header("API-Key", api_key)
        .header("content-type", "application/json")
        .header("newrelic-package-id", uuid)
        .body(Body::from(collection_query))
        .unwrap();

    let mut resp = client.request(req).await.expect("nerdgraph request failed");

    let body = hyper::body::to_bytes(resp.body_mut()).await.unwrap();

    return body;
}

// discover_nrsync_files
// find nri-sync config files locally
async fn discover_nrsync_files() -> Vec<IntegrationsFile> {
    let mut nrsync_files: Vec<IntegrationsFile> = Vec::new();

    let integrations_dir = match env::consts::OS {
        "linux" => INTEGRATIONS_DIR_NIX,
        "windows" => INTEGRATIONS_DIR_WIN,
        os => {
            panic! {"{} unsupported",os}
        }
    };

    info!("collecting files on disk: {}", integrations_dir);

    let integration_files =
        fs::read_dir(integrations_dir).expect("unable to read integrations directory");

    for file in integration_files {
        let file_data = file.unwrap();
        let file_name = file_data.file_name().to_str().unwrap().to_string();
        if file_name.ends_with(".nri-sync.yml") {
            let integrations_file = IntegrationsFile {
                name: file_name,
                path: file_data.path().to_str().unwrap().to_string(),
                found_in_collection: false,
            };
            nrsync_files.push(integrations_file);
        }
    }

    return nrsync_files;
}

// report event
// create infrastructure integrations payload with sync stats
fn report_event(sync_stats: SyncStats, collection: &String, account_id: &String) {
    let event_type = "NriSyncSample";

    let event: String = format!(
        r#"{{
        "name": "com.newrelic.{}",
        "protocol_version": "3",
        "integration_version": "{}",
        "data": [
            {{
                "metrics": [
                    {{
                        "event_type": "{}",
                        "collection": "{}",
                        "collectionAccountId": {},
                        "filesWritten": {},
                        "filesUpdated": {},
                        "filesAlreadyUpdated": {},
                        "filesDeleted": {},
                        "filesInCollection": {},
                        "errorsWriting": {},
                        "errorsDeleting": {},
                        "errorsReading": {}
                    }}
                ],
                "add_hostname": true
            }}
        ]
    }}"#,
        crate_name!(),
        crate_version!(),
        event_type,
        collection,
        account_id,
        sync_stats.files_written,
        sync_stats.files_updated,
        sync_stats.files_already_updated,
        sync_stats.files_deleted,
        sync_stats.files_in_collection,
        sync_stats.errors_writing,
        sync_stats.errors_deleting,
        sync_stats.errors_reading,
    );

    let json: Value = serde_json::from_str(&event).expect("unable to create json");

    println!("{}", json);
}
