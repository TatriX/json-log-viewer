use std::io::{self, BufRead};
use std::collections::HashMap;
use serde::Deserialize;
use serde_json::Value;
use colored::*;
use std::fmt;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
struct Message {
    level: Level,
    msg: Value,
    time: DateTime<Utc>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Level::Trace => "white".cyan(),
            Level::Debug => "white".cyan(),
            Level::Info => "info".white(),
            Level::Warn => "warn".purple(),
            Level::Error => "error".red(),
        })
    }
}


fn main() -> io::Result<()> {
    for line in  io::stdin().lock().lines().map(|l| l.unwrap()) {
        let message: Message = serde_json::from_str(&line).expect("Cannot decode");
        print!(" {} ┊{}┊ ", message.level, message.time.format("%H:%M:%S"));
        match message.msg {
            Value::String(s) => print!("{}", s),
            other => print!("{}", other),
        }
        message.extra.iter().for_each(|(key, value)| {
            print!(" {}: {}", key.white(), value);
        });
        println!();
    }
    Ok(())
}
