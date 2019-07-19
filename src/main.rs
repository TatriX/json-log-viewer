use std::io::{self, BufRead};
use std::collections::BTreeMap;
use serde::Deserialize;
use serde_json::Value;
use colored::*;
use unescape::unescape;
use std::fmt;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
struct Message {
    level: Level,
    msg: String,
    time: DateTime<Utc>,
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Level {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Level::Trace => "white".cyan(),
            Level::Debug => "white".cyan(),
            Level::Info => "info".white(),
            Level::Warning => "warn".purple(),
            Level::Error => "error".red(),
        })
    }
}


fn main() -> io::Result<()> {
    for line in  io::stdin().lock().lines().map(|l| l.unwrap()) {
        match serde_json::from_str::<Message>(&line) {
            Ok(message) => {
                print!(" {} ┊{}┊ {}", message.level, message.time.format("%H:%M:%S"), message.msg);

                message.extra.iter().for_each(|(key, value)| {
                    print!(" {}: ", key.white());

                    match value {
                        Value::String(s) => {
                            match unescape(&s).and_then(|raw| serde_json::from_str::<Value>(&raw).ok()) {
                                Some(value) => print!("{}", value),
                                None => print!("{}", s)
                            }
                        },
                        other => print!("{}", other),
                    }
                });
                println!();
            },
            Err(_) => {
                // if we can't parse json, just print line back
                println!("{}", line);
            }
        }
    }
    Ok(())
}
