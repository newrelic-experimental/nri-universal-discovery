use super::{discovery, Opts};
use discovery::DiscoveryItem;
use serde_json::{Map, Value};
extern crate base64;

pub fn decorate_discovery_items(
    raw_discovery_items: Vec<Map<String, Value>>,
    opts: &Opts,
) -> Vec<DiscoveryItem> {
    let mut discovery_items: Vec<DiscoveryItem> = vec![];

    match &opts.decorator_file {
        Some(decorator_file) => {
            for raw_item in raw_discovery_items {
                let item = DiscoveryItem {
                    variables: raw_item,
                };
                discovery_items.push(item);
            }
        }
        _ => {
            for raw_item in raw_discovery_items {
                let item = DiscoveryItem {
                    variables: raw_item,
                };
                discovery_items.push(item);
            }
        }
    }

    return discovery_items;
}
