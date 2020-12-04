use super::nrsync;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::fs;
extern crate base64;

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    actor: Actor,
}

#[derive(Serialize, Deserialize, Debug)]
struct Actor {
    account: Account,
}
#[derive(Serialize, Deserialize, Debug)]
struct Account {
    id: i32,
    #[serde(rename = "nerdStorage")]
    nerd_storage: NerdStorage,
}

#[derive(Serialize, Deserialize, Debug)]
struct NerdStorage {
    collection: Vec<Collection>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Collection {
    document: Document,
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Document {
    config: String,
}

pub fn start(
    mut files: Vec<nrsync::IntegrationsFile>,
    nerdgraph_bytes: Bytes,
) -> nrsync::SyncStats {
    let nerdgraph_json =
        String::from_utf8(nerdgraph_bytes.to_vec()).expect("unable to convert bytes to string");

    let nerdgraph_data: Payload = serde_json::from_str(nerdgraph_json.as_str()).expect(&format!(
        "unable to deserialize successful nerdgraph response, err: {}",
        nerdgraph_json
    ));

    let collections = nerdgraph_data
        .data
        .actor
        .account
        .nerd_storage
        .collection
        .into_iter();

    let config_file_count = collections.len();
    info!("{} file(s) to sync", config_file_count);

    // collect some statistics about the sync
    let mut sync_stats = nrsync::SyncStats {
        files_written: 0,
        files_updated: 0,
        files_already_updated: 0,
        files_deleted: 0,
        files_in_collection: config_file_count,
        errors_writing: 0,
        errors_deleting: 0,
        errors_reading: 0,
    };

    for doc in collections {
        let file_name = format!("{}.nri-sync.yml", doc.id);
        let decoded_config = base64::decode(&doc.document.config).unwrap();
        let mut file_found = false;
        let mut write_file = false;

        for file in &mut files {
            // compare base64, if different write aka update
            if file.name == file_name {
                let contents = match fs::read_to_string(file.path.as_str()) {
                    Err(e) => {
                        warn!("{} read failed error: {}, skipping.", file.path, e);
                        sync_stats.add_err_reading();
                        break;
                    }
                    Ok(result) => result,
                };

                // convert config string to base64
                let file_b64 = base64::encode(contents);

                // compare configs as base64 strings to determine if update required
                if &file_b64 != &doc.document.config {
                    write_file = true;
                    sync_stats.add_updated();
                } else {
                    info!("{} up to date", file.name);
                    sync_stats.add_already_updated();
                }

                file_found = true;
                file.found_file();
                break;
            }
        }

        // write file to disk
        if write_file || !file_found {
            let config_file = String::from_utf8(decoded_config).unwrap();
            write_config_file(&mut sync_stats, file_name, config_file);
        }
    }

    // if any integration files have not been found, it means they have been removed from the collection
    // delete the file from disk
    for file in &files {
        if !file.found_in_collection {
            match fs::remove_file(&file.path) {
                Ok(_) => {
                    info!("{} deleted", file.name);
                    sync_stats.add_deleted();
                }
                Err(e) => {
                    error!("{} failed to be deleted, err: {}", file.name, e);
                    sync_stats.add_err_deleting();
                }
            }
        }
    }

    return sync_stats;
}

fn write_config_file(sync_stats: &mut nrsync::SyncStats, file_name: String, config_file: String) {
    let file_path = match std::env::consts::OS {
        "linux" => format!("{}/{}", nrsync::INTEGRATIONS_DIR_NIX, file_name),
        "windows" => format!("{}\\{}", nrsync::INTEGRATIONS_DIR_WIN, file_name),
        os => {
            panic! {"{} unsupported",os}
        }
    };

    match fs::write(file_path, config_file) {
        Ok(_) => {
            info!("{} synced to disk", file_name);
            sync_stats.add_written();
        }
        Err(e) => {
            error!("{} failed to write, err: {}", file_name, e);
            sync_stats.add_err_writing();
        }
    };
}
