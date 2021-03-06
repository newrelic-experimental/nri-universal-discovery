use crate::Opts;

use super::{request, utils};
use request::NerdgraphPayload;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
extern crate base64;

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscoveryFile {
    defaults: Map<String, Value>,
    discovery_items: Vec<Map<String, Value>>,
}

pub fn handle_nerdgraph_payloads(
    nerdgraph_payloads: Vec<request::NerdgraphPayload>,
) -> Vec<Map<String, Value>> {
    let discovery_values = unpack_nerdgraph_payloads(nerdgraph_payloads);
    let raw_discovery_items = process_discovery_values(discovery_values);

    return raw_discovery_items;
}

pub fn handle_file(opts: &Opts) -> Vec<Map<String, Value>> {
    let file = opts.discovery_file.to_owned().expect("file undefined");
    let file = utils::read_file(&file);
    let discovery_file: DiscoveryFile = serde_json::from_str(&file)
        .expect(&format!("unable to deserialize discovery file: {}", file));

    let mut new_values: Vec<Map<String, Value>> = vec![];

    for raw_item in discovery_file.discovery_items {
        let mut new_object: Map<String, Value> = raw_item;

        // if default key does not exist in existing item
        // inject the default value
        for default_key in discovery_file.defaults.keys() {
            match new_object.get(default_key) {
                Some(_) => {}
                _ => {
                    let value = discovery_file.defaults.get(default_key).unwrap();
                    new_object.insert(default_key.to_owned(), value.to_owned());
                    ()
                }
            }
        }

        new_values.push(new_object);
    }

    return new_values;
}

fn process_discovery_values(discovery_values: Vec<Value>) -> Vec<Map<String, Value>> {
    let mut values: Vec<Map<String, Value>> = vec![];

    for value in discovery_values.into_iter() {
        let mut new_object: Map<String, Value> = Map::new();

        match value.as_object() {
            Some(object) => {
                for key in object.keys() {
                    let value = object.get(key).unwrap();
                    // unpack payload into flat map, do not include arrays, objects or nulls
                    if !value.is_object() && !value.is_array() && !value.is_null() {
                        new_object.insert(key.to_string(), value.to_owned());
                    }
                }

                // check for tags and unpack them
                match object.get("tags") {
                    Some(tags) => match tags.as_array() {
                        Some(tags) => {
                            for tag in tags {
                                match tag.as_object() {
                                    Some(tag_obj) => {
                                        let key = tag_obj.get("key").unwrap().as_str().unwrap();
                                        let values =
                                            tag_obj.get("values").unwrap().as_array().unwrap();

                                        for value in values {
                                            new_object.insert(
                                                format!("tags.{}", key.to_string()),
                                                value.to_owned(),
                                            );
                                            new_object.remove("tags");
                                            break;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    },
                    __ => {}
                }
            }
            _ => {}
        };

        values.push(new_object);
    }

    return values;
}

fn unpack_nerdgraph_payloads(nerdgraph_payloads: Vec<NerdgraphPayload>) -> Vec<Value> {
    let mut discovery_values: Vec<Value> = vec![];
    let payloads = nerdgraph_payloads.into_iter();

    for payload in payloads {
        match payload.data {
            Some(data) => {
                match data.actor.account {
                    Some(account) => match account.nrql {
                        Some(nrql) => {
                            let results = nrql.results.into_iter();
                            for event in results {
                                discovery_values.push(event);
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                };
                match data.actor.entity_search {
                    Some(entity_search) => match entity_search.results {
                        Some(results) => {
                            let entities = results.entities.into_iter();
                            for entity in entities {
                                discovery_values.push(entity);
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }

    return discovery_values;
}
