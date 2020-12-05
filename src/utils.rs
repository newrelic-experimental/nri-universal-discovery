use std::fs;

pub fn read_file(file: &String) -> String {
    debug!("reading file: {}", file);
    match fs::read_to_string(file) {
        Err(e) => {
            error!("{} read failed error: {}.", file, e);
            String::from("")
        }
        Ok(result) => result,
    }
}
