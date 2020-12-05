use super::{discovery, utils, Opts};
use discovery::DiscoveryItem;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct DecoratorItem {
    pub overwrite: Option<bool>,
    pub variables: Map<String, Value>,
    pub sensitive: Map<String, Value>,
}

pub fn decorate_discovery_items(
    raw_discovery_items: Vec<Map<String, Value>>,
    opts: &Opts,
) -> Vec<DiscoveryItem> {
    let mut discovery_items: Vec<DiscoveryItem> = vec![];

    let decorator_file = &opts.decorator_file.to_owned().unwrap_or(String::from(""));
    let decorator_contents = if &decorator_file.to_string() != "" {
        utils::read_file(decorator_file)
    } else {
        debug!("decorator file not in use");
        String::from("")
    };

    if decorator_contents != "" {
        debug!("attempting decorations");
    }

    for raw_item in raw_discovery_items {
        let item = DiscoveryItem {
            variables: raw_item,
        };
        discovery_items.push(item);
    }

    return discovery_items;
}
