#![allow(dead_code)]
#![allow(unused_variables)]

use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;


pub fn string_to_u32(hex_address: &str) -> Result<u32, Box<dyn Error>> {
    Ok(u32::from_str_radix(hex_address, 16)?)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ComparisonTypes {
    Bit,
    Eq,
    Gte,
    Gt,
    Lte,
    Lt,
    Wbit,
    Weq,
    Wgt,
    Wgte,
    Wlte,
    Wlt,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    name: Option<String>,
    autostart: Option<Autostart>,
    igt: Option<InGameTime>,
    #[serde(default = "HashMap::new")]
    alias: HashMap<String, Value>,
    definitions: Vec<SplitDefinition>,
}

#[derive(Deserialize, Debug)]
pub struct Autostart {
    active: String,
    address: String,
    value: String,
    r#type: String,
    next: Option<Vec<SplitDefinition>>,
    more: Option<Vec<SplitDefinition>>,
}

impl Autostart {
    pub fn is_active(self) -> bool {
        if self.active == "1" {
            return true;
        }
        false
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SplitDefinition {
    name: Option<String>,
    note: Option<String>,
    address: String,
    value: String,
    r#type: ComparisonTypes,
    next: Option<Vec<SplitDefinition>>,
    more: Option<Vec<SplitDefinition>>,
}

impl SplitDefinition {
    pub fn new(
        name: String,
        note: Option<String>,
        address: String,
        value: String,
        r#type: ComparisonTypes,
        next: Option<Vec<SplitDefinition>>,
        more: Option<Vec<SplitDefinition>>,
    ) -> Self {
        Self {
            name: Some(name),
            note,
            address,
            value,
            r#type,
            next,
            more,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InGameTime {
    active: String,
    frames_address: String,
    seconds_address: String,
    minutes_address: String,
    hours_address: String,
}

impl InGameTime {
    pub fn new(
        active: String,
        frames_address: String,
        seconds_address: String,
        minutes_address: String,
        hours_address: String,
    ) -> Self {
        Self {
            active,
            frames_address,
            seconds_address,
            minutes_address,
            hours_address,
        }
    }

    pub fn is_active(self) -> bool {
        if self.active == "1" {
            return true;
        }
        false
    }
}

pub fn parse_splitter_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let split = serde_json::from_reader(reader)?;
    Ok(split)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn autostart_sm_is_active() {
        let conf = parse_splitter_file("data/SuperMetroid.json").unwrap();
        let autorstart_active = match conf.autostart {
            Some(c) => c.is_active(),
            None => false,
        };
        assert!(autorstart_active);
    }

    #[test]
    fn autostart_actraiser_is_inactive() {
        let conf = parse_splitter_file("data/actraiser.json").unwrap();
        let autorstart_active = match conf.autostart {
            Some(c) => c.is_active(),
            None => false,
        };
        assert!(!autorstart_active);
    }

    #[test]
    fn igt_sm_is_set() {
        let conf = parse_splitter_file("data/SuperMetroid.json").unwrap();
        assert!(conf.igt.is_some());
    }

    #[test]
    fn igt_alttp_is_not_set() {
        let conf = parse_splitter_file("data/alttp-subplits.json").unwrap();
        dbg!(&conf.igt);
        let igt_is_set = match conf.igt {
            Some(i) => i.is_active(),
            None => false,
        };
        assert!(!igt_is_set);
    }

    #[test]
    fn new_split_definition() {
        let split = SplitDefinition::new(
            "Test".to_string(),
            None,
            "0x6969".to_string(),
            "2".to_string(),
            ComparisonTypes::Eq,
            None, 
            None
        );

        assert_eq!(&split.address, "0x6969")
    }

    #[test]
    fn string_to_u32_validation() {
        let address = "0x6969".to_string();
        let num = string_to_u32(address.trim_start_matches("0x")).unwrap();
        assert_eq!(num, 0x6969);
    }
}
