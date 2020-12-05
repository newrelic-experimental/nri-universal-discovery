use super::{discovery, Opts};
use discovery::DiscoveryItem;
use serde_json::{Map, Value};
use std::fs;
extern crate base64;

pub fn decorate_discovery_items(
    raw_discovery_items: Vec<Map<String, Value>>,
    opts: &Opts,
) -> Vec<DiscoveryItem> {
    let mut discovery_items: Vec<DiscoveryItem> = vec![];

    let contents = match &opts.decorator_file {
        Some(decorator_file) => match fs::read_to_string(decorator_file) {
            Err(e) => {
                warn!("{} read failed error: {}.", decorator_file, e);
                String::from("")
            }
            Ok(result) => result,
        },
        _ => String::from(""),
    };

    for raw_item in raw_discovery_items {
        let item = DiscoveryItem {
            variables: raw_item,
        };
        discovery_items.push(item);
    }

    return discovery_items;
}
