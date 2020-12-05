use super::{discovery, utils, Opts};
use discovery::DiscoveryItem;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

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

        let decorations: Vec<DecoratorItem> = serde_json::from_str(&decorator_contents).expect(
            &format!("unable to deserialize decorations file: {}", decorator_file),
        );

        for mut raw_item in raw_items {
            for decoration in &decorations {
                match &decoration.matches {
                    Some(matches) => {
                        for key in matches.keys() {
                            match raw_item.get(key) {
                                Some(item_value) => {
                                    let the_regex = matches.get(key).expect("regex undefined");

                                    let re = Regex::new(the_regex.as_str().unwrap())
                                        .expect(&format!("bad regex: {}", the_regex));

                                    if re.is_match(item_value.as_str().unwrap()) {
                                        for vars in &decoration.variables {
                                            for var in vars.keys() {
                                                // do not override existing keys
                                                if !raw_item.contains_key(var) {
                                                    let decoration_value =
                                                        vars.get(var).unwrap().to_owned();

                                                    raw_item
                                                        .insert(var.to_string(), decoration_value);
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => error!("no matches defined"),
                }
            }

            let item = DiscoveryItem {
                variables: raw_item,
            };

            discovery_items.push(item);
        }
    } else {
        for raw_item in raw_items {
            let item = DiscoveryItem {
                variables: raw_item,
            };
            discovery_items.push(item);
        }
    }

    return discovery_items;
}
