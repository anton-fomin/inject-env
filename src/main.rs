use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs};

use clap::Parser;

/// Simple program to dump environment variables as JSON
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The output file. Output to STDOUT if not specified
    #[clap(short, long, value_parser)]
    out: Option<PathBuf>,

    /// Whether it should replace a placeholder in file instead of replacing the whole file
    #[clap(short, long, value_parser)]
    replace: Option<String>,

    /// A prefix the environment variable should start with
    #[clap(short, long, value_parser)]
    prefix: Option<String>,

    /// Remove prefix from environment variable name if it is defined
    #[clap(short, long, value_parser, default_value_t = true)]
    strip_prefix: bool,

    /// Output json as encoded string with proper escape of quotes
    #[clap(short, long, value_parser, default_value_t = false)]
    as_encoded_string: bool,

    /// Format value. Usefull to set it to variagle (eg. "window.APP_ENV = {}") 
    #[clap(short, long, value_parser, default_value = "{}")]
    format: String
}

fn main() {
    let args = Args::parse();
    let env_vars = collect_env_vars(&args.prefix, args.strip_prefix);
    let json = encode_vars(&env_vars, args.as_encoded_string, &args.format);
    match (args.out, args.replace) {
        (Some(path), Some(replace)) => {
            let content = fs::read_to_string(&path).expect("Unable to read file");
            let content = content.replace(&replace, &json);
            fs::write(&path, &content).expect("Unable to write file");
        }
        (Some(path), None) => fs::write(path, json).expect("Unable to write file"),
        (None, Some(_)) => panic!("--replace can only be used with --out"),
        (None, None) => println!("{}", json),
    }
}

fn collect_env_vars(prefix: &Option<String>, strip_prefix: bool) -> HashMap<String, String> {
    let mut env_vars: Vec<(String, String)> = env::vars().collect();

    prefix.iter().for_each(|prefix| {
        env_vars = env_vars
            .iter()
            .filter(|(key, _)| key.starts_with(prefix))
            .map(|(key, value)| {
                if strip_prefix {
                    (
                        key.strip_prefix(prefix).unwrap_or(key).to_string(),
                        value.to_string(),
                    )
                } else {
                    (key.to_string(), value.to_string())
                }
            })
            .collect()
    });

    let mut result = HashMap::new();
    for (key, value) in env_vars {
        result.insert(key, value);
    }
    result
}

fn encode_vars(vars: &HashMap<String, String>, escape_string: bool, format: &str) -> String {
    let mut json = serde_json::to_string(vars).expect("Unable to generate json");
    if escape_string {
        json = serde_json::to_string(&json).expect("Unable to generate escape string")
    }
    
    format.replace("{}", &json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_vars() {
        let vars = HashMap::from([
            ("FOO".to_string(), "foo".to_string()),
            ("BAR".to_string(), "bar".to_string()),
        ]);
        let json_string = encode_vars(&vars, false, "{}");
        let x: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(&json_string).expect("encode_vars didn't produce valid json");
        assert!(x.contains_key("FOO"));
        assert_eq!(x["FOO"], "foo");
        assert!(x.contains_key("BAR"));
        assert_eq!(x["BAR"], "bar");
    }

    #[test]
    fn test_format_the_output() {
        let vars = HashMap::from([
            ("FOO".to_string(), "foo".to_string()),
            ("BAR".to_string(), "bar".to_string()),
        ]);
        let result = encode_vars(&vars, false, "window.APP_ENV = {}");
        assert!(result.starts_with("window.APP_ENV = {"));

        let json_string = result.replace("window.APP_ENV = ", "");
        let x: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(&json_string).expect("encode_vars didn't produce valid json");
        assert!(x.contains_key("FOO"));
        assert_eq!(x["FOO"], "foo");
        assert!(x.contains_key("BAR"));
        assert_eq!(x["BAR"], "bar");
    }
}
