use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde_json::Value;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Config {
    name: Option<String>,
    autostart: Option<Autostart>,
    igt: Option<InGameTime>,
    #[serde(default = "HashMap::new")]
    alias: HashMap<String, Value>,
    definitions: Vec<Split>
}

#[derive(Deserialize, Debug)]
struct Autostart {
    active: String,
    address: String,
    value: String,
    r#type: String,
    next:  Option<Vec<Split>>,
    more:  Option<Vec<Split>>,
}

#[derive(Deserialize, Debug)]
struct Split {
    name: Option<String>,
    note: Option<String>,
    address: String,
    value: String,
    r#type: String,
    next: Option<Vec<Split>>,
    more: Option<Vec<Split>>,

}

#[derive(Deserialize, Debug)]
struct InGameTime {
    active: String,
    framesAddress: String,
    secondsAddress: String,
    minutesAddress: String,
    hoursAddress: String,
}

pub fn is_active(entry: &String) -> bool {
    if entry == "1" {
        return true;
    }
    false
}

fn parse_splitter_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let split = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(split)
}

fn parser() {
    
    

    
    // for definition in conf.definitions {
    //     if definition.more.is_some() {
    //         dbg!(&definition.name);
    //     }

    //     if definition.next.is_some() {
    //         dbg!(&definition.name);
    //     }
    // }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn autostart_sm_is_active() {
        let conf = parse_splitter_file("data/SuperMetroid.json").unwrap();
        let autostart_is_set = match conf.autostart {
            Some(i) => is_active(&i.active),
            None => false
        };
        assert!(autostart_is_set);
    }

    #[test]
    fn autostart_actraiser_is_inactive() {
        let conf = parse_splitter_file("data/actraiser.json").unwrap();
        let autostart_is_set = match conf.autostart {
            Some(i) => is_active(&i.active),
            None => false
        };
        assert!(!autostart_is_set);
    }

    #[test]
    fn igt_sm_is_set() {
        let conf = parse_splitter_file("data/SuperMetroid.json").unwrap();
        let igt_is_set = match conf.igt {
            Some(i) => is_active(&i.active),
            None => false
        };
        assert!(igt_is_set);
    
    }
}