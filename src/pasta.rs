use bytesize::ByteSize;
use chrono::{Datelike, Local, TimeZone, Timelike};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::args::ARGS;
use crate::util::animalnumbers::to_animal_names;
use crate::util::hashids::to_hashids;
use crate::util::syntaxhighlighter::html_highlight;

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct PastaFile {
    pub name: String,
    pub size: ByteSize,
}

impl PastaFile {
    pub fn from_unsanitized(path: &str) -> Result<Self, &'static str> {
        let path = Path::new(path);
        let name = path.file_name().ok_or("Path did not contain a file name")?;
        let name = name.to_string_lossy().replace(' ', "_");
        Ok(Self {
            name,
            size: ByteSize::b(0),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pasta {
    pub id: u64,
    #[serde(default)]
    pub custom_key: Option<String>,
    pub content: String,
    pub file: Option<PastaFile>,
    pub extension: String,
    pub private: bool,
    pub editable: bool,
    pub created: i64,
    pub expiration: i64,
    pub last_read: i64,
    pub read_count: u64,
    pub burn_after_reads: u64,
    pub pasta_type: String,
}

impl Pasta {
    pub fn public_key(&self) -> String {
        match &self.custom_key {
            Some(key) if is_valid_custom_key(key) => key.clone(),
            _ => self.id_as_animals(),
        }
    }

    pub fn id_as_animals(&self) -> String {
        generated_key(self.id)
    }

    pub fn created_as_string(&self) -> String {
        let date = Local.timestamp(self.created, 0);
        format!(
            "{:02}-{:02} {:02}:{:02}",
            date.month(),
            date.day(),
            date.hour(),
            date.minute(),
        )
    }

    pub fn expiration_as_string(&self) -> String {
        if self.expiration == 0 {
            String::from("Never")
        } else {
            let date = Local.timestamp(self.expiration, 0);
            format!(
                "{:02}-{:02} {:02}:{:02}",
                date.month(),
                date.day(),
                date.hour(),
                date.minute(),
            )
        }
    }

    pub fn last_read_time_ago_as_string(&self) -> String {
        let elapsed = self.last_read_elapsed_seconds();
        let days = (elapsed / 86400) as u16;
        if days > 1 {
            return format!("{} days ago", days);
        };

        // it's less than 1 day, let's do hours then
        let hours = (elapsed / 3600) as u16;
        if hours > 1 {
            return format!("{} hours ago", hours);
        };

        // it's less than 1 hour, let's do minutes then
        let minutes = (elapsed / 60) as u16;
        if minutes > 1 {
            return format!("{} minutes ago", minutes);
        };

        // it's less than 1 minute, let's do seconds then
        let seconds = elapsed as u16;
        if seconds > 1 {
            return format!("{} seconds ago", seconds);
        };

        // it's less than 1 second?????
        String::from("just now")
    }

    pub fn last_read_elapsed_seconds(&self) -> i64 {
        let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => {
                log::error!("SystemTime before UNIX EPOCH!");
                0
            }
        } as i64;
        (timenow - self.last_read).max(0)
    }

    pub fn last_read_days_ago(&self) -> u16 {
        // get current unix time in seconds
        let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => {
                log::error!("SystemTime before UNIX EPOCH!");
                0
            }
        } as i64;

        // get seconds since last read and convert it to days
        ((timenow - self.last_read) / 86400) as u16
    }

    pub fn content_syntax_highlighted(&self) -> String {
        html_highlight(&self.content, &self.extension)
    }

    pub fn content_not_highlighted(&self) -> String {
        html_highlight(&self.content, "txt")
    }

    pub fn content_escaped(&self) -> String {
        html_escape::encode_text(&self.content.replace('`', "\\`").replace('$', "\\$")).to_string()
    }
}

pub fn generated_key(id: u64) -> String {
    if ARGS.hash_ids {
        to_hashids(id)
    } else {
        to_animal_names(id)
    }
}

pub fn normalize_custom_key(value: &str) -> Result<Option<String>, &'static str> {
    let value = value.trim();
    if value.is_empty() {
        return Ok(None);
    }
    if !is_valid_custom_key(value) {
        return Err("invalid_key");
    }
    Ok(Some(value.to_owned()))
}

pub fn is_valid_custom_key(value: &str) -> bool {
    (3..=64).contains(&value.len())
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-' || byte == b'_'
        })
}

pub fn find_by_key(pastas: &[Pasta], key: &str) -> Option<usize> {
    pastas.iter().position(|pasta| pasta.public_key() == key)
}

pub fn key_is_available(pastas: &[Pasta], key: &str) -> bool {
    find_by_key(pastas, key).is_none()
}

#[cfg(test)]
mod tests {
    use super::{key_is_available, normalize_custom_key, Pasta};

    fn pasta(id: u64, custom_key: Option<&str>) -> Pasta {
        Pasta {
            id,
            custom_key: custom_key.map(str::to_owned),
            content: String::new(),
            file: None,
            extension: String::new(),
            private: false,
            editable: false,
            created: 0,
            expiration: 0,
            last_read: 0,
            read_count: 0,
            burn_after_reads: 0,
            pasta_type: "text".to_owned(),
        }
    }

    #[test]
    fn validates_custom_keys() {
        assert_eq!(normalize_custom_key("  my-note ").unwrap(), Some("my-note".to_owned()));
        assert_eq!(normalize_custom_key("").unwrap(), None);
        assert!(normalize_custom_key("ABCD").is_err());
        assert!(normalize_custom_key("ab").is_err());
        assert!(normalize_custom_key("a/b").is_err());
        assert!(normalize_custom_key(&"a".repeat(65)).is_err());
    }

    #[test]
    fn checks_custom_and_generated_key_collisions() {
        let pastas = vec![pasta(12, Some("my-note")), pasta(13, Some("old-key"))];
        assert_eq!(pastas[0].public_key(), "my-note");
        assert!(!key_is_available(&pastas, "my-note"));
        assert!(key_is_available(&pastas, "another-note"));
    }

    #[test]
    fn loads_records_without_custom_key() {
        let json = r#"{
            "id": 12, "content": "hello", "file": null, "extension": "txt",
            "private": false, "editable": false, "created": 0, "expiration": 0,
            "last_read": 0, "read_count": 0, "burn_after_reads": 0, "pasta_type": "text"
        }"#;
        let pasta: Pasta = serde_json::from_str(json).unwrap();
        assert_eq!(pasta.custom_key, None);
    }
}

impl fmt::Display for Pasta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
