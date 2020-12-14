use super::{discovery, utils, Opts};
use discovery::DiscoveryItem;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
extern crate sys_info;

static COLLECTOR_HOSTNAME: &str = "collector.hostname";
static COLLECTOR_OS_RELEASE: &str = "collector.operatingSystemRelease";
static COLLECTOR_OS: &str = "collector.operatingSystem";
static COLLECTOR_VERSION: &str = "collector.version";

macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecorationsFile {
    defaults: Map<String, Value>,
    decorations: Vec<DecoratorItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecoratorItem {
    pub overwrite: Option<bool>,
    pub matches: Option<Map<String, Value>>,
    pub variables: Option<Map<String, Value>>,
}

pub fn decorate_discovery_items(
    raw_items: Vec<Map<String, Value>>,
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
        debug!("processing decorations");

        let decorations_file: DecorationsFile = serde_json::from_str(&decorator_contents).expect(
            &format!("unable to deserialize decorations file: {}", decorator_file),
        );

        let decorations = decorations_file.decorations;

        for mut raw_item in raw_items {
            for decoration in &decorations {
                match &decoration.matches {
                    Some(matches) => {
                        let mut found_matches: usize = 0;

                        for key in matches.keys() {
                            match raw_item.get(key) {
                                Some(item_value) => {
                                    let the_regex = matches.get(key).expect("regex undefined");

                                    let re = Regex::new(the_regex.as_str().unwrap())
                                        .expect(&format!("bad regex: {}", the_regex));

                                    if re.is_match(item_value.as_str().unwrap()) {
                                        found_matches += 1;
                                    }
                                }
                                _ => {}
                            }
                        }

                        // ensure all matches are found before applying
                        if matches.len() == found_matches {
                            for vars in &decoration.variables {
                                for var in vars.keys() {
                                    // do not override existing keys
                                    if !raw_item.contains_key(var) {
                                        let decoration_value =
                                            vars.get(var).unwrap_or(&json!("")).to_owned();

                                        raw_item.insert(var.to_string(), decoration_value);
                                    }
                                }
                            }
                        }

                        // apply default attributes
                        for key in decorations_file.defaults.keys() {
                            // do not override existing keys
                            if !raw_item.contains_key(key) {
                                let decoration_value =
                                    decorations_file.defaults.get(key).unwrap().to_owned();
                                raw_item.insert(key.to_string(), decoration_value);
                            }
                        }
                    }
                    _ => error!("no matches defined"),
                }
            }

            let item = DiscoveryItem {
                variables: apply_collector_attributes(raw_item, &opts),
            };

            discovery_items.push(item);
        }
    } else {
        for raw_item in raw_items {
            let item = DiscoveryItem {
                variables: apply_collector_attributes(raw_item, &opts),
            };
            discovery_items.push(item);
        }
    }

    return discovery_items;
}

fn apply_collector_attributes(mut raw_item: Map<String, Value>, opts: &Opts) -> Map<String, Value> {
    // collector sys variables
    let hostname = sys_info::hostname().unwrap();
    let os_release = sys_info::os_release().unwrap();
    let os_type = sys_info::os_type().unwrap();
    let version = crate_version!();

    // insert into main items
    raw_item.insert(COLLECTOR_HOSTNAME.to_string(), json!(hostname));
    raw_item.insert(COLLECTOR_OS_RELEASE.to_string(), json!(os_release));
    raw_item.insert(COLLECTOR_OS.to_string(), json!(os_type));
    raw_item.insert(COLLECTOR_VERSION.to_string(), json!(version));

    // generate and insert meta
    let meta_value = generate_meta(&raw_item, &opts);
    raw_item.insert("discoveryMeta".to_string(), meta_value);

    return raw_item;
}

// generate_meta generates meta based on the configured blacklist or whitelist
fn generate_meta(raw_item: &Map<String, Value>, opts: &Opts) -> Value {
    let mut meta: Map<String, Value> = Map::new();

    let whitelist = &opts.meta_whitelist.to_owned().unwrap_or("".to_string());
    let blacklist = &opts.meta_blacklist.to_owned().unwrap_or("".to_string());

    if whitelist.as_str() != "" {
        let mut metakeys: Vec<&str> = whitelist.split(",").collect();

        // apply collector variables to meta by default
        metakeys.push(COLLECTOR_HOSTNAME);
        metakeys.push(COLLECTOR_OS);
        metakeys.push(COLLECTOR_OS_RELEASE);
        metakeys.push(COLLECTOR_VERSION);

        for mkey in metakeys {
            match raw_item.get(mkey) {
                Some(v) => {
                    meta.insert(mkey.to_string(), v.to_owned());
                }
                _ => {}
            }
        }
    } else if blacklist.as_str() != "" {
        let metakeys: Vec<&str> = blacklist.split(",").collect();

        for rkey in raw_item.keys() {
            if let Some(_) = metakeys.iter().find(|&s| *s == rkey) {
            } else {
                match raw_item.get(rkey) {
                    Some(v) => {
                        debug!("adding {} to meta", rkey);
                        meta.insert(rkey.to_string(), v.to_owned());
                    }
                    _ => {
                        debug!("cannot add {} to meta, not found", rkey);
                    }
                }
            }
        }
    }

    // convert to string (to allow discovery to pass correctly)
    let meta_str = serde_json::to_string(&meta).unwrap();
    // convert to Value to match type
    let meta_value: Value = serde_json::to_value(meta_str).unwrap();

    return meta_value;
}
