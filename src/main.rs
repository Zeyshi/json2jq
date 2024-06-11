use clap::{command, Parser};
use serde_json::Value;
use std::collections::HashSet;
use std::io::{self, Read};
use std::process;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<String>,
}

fn main() {
    let args = Cli::parse();

    // Read the JSON input from stdin or from the CLI argument
    let mut input = String::new();
    if let Some(cli_input) = args.input {
        input = cli_input;
    } else {
        if let Err(err) = io::stdin().read_to_string(&mut input) {
            eprintln!("Error reading input: {}", err);
            process::exit(1);
        }
    }

    // Parse the JSON input
    let json: Value = match serde_json::from_str(&input) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            process::exit(1);
        }
    };

    // Generate jq paths and print them
    match generate_jq_paths(&json) {
        Ok(paths) => {
            for path in paths {
                println!("{}", path);
            }
        }
        Err(err) => {
            eprintln!("Error generating jq paths: {}", err);
            process::exit(1);
        }
    }
}

fn generate_jq_paths(value: &Value) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut paths = HashSet::new();
    generate_paths(value, ".".to_string(), &mut paths);
    let mut paths_vec: Vec<_> = paths.into_iter().collect();
    paths_vec.sort();
    Ok(paths_vec)
}

fn generate_paths(value: &Value, current_path: String, paths: &mut HashSet<String>) {
    match value {
        Value::Object(map) => {
            if current_path != "." {
                paths.insert(current_path.clone());
            }
            for (key, val) in map {
                let new_path = if current_path == "." {
                    format!(".{}", key)
                } else {
                    format!("{}.{}", current_path, key)
                };
                generate_paths(val, new_path, paths);
            }
        }
        Value::Array(arr) => {
            if current_path != "." {
                paths.insert(current_path.clone());
            }
            let new_path = format!("{}[]", current_path);
            for item in arr {
                generate_paths(item, new_path.clone(), paths);
            }
        }
        _ => {
            paths.insert(current_path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_generate_jq_paths_simple() {
        let data = json!({"name": "Alice", "type": "paying"});
        let paths = generate_jq_paths(&data).unwrap();
        let expected = vec![".name", ".type"];
        assert_eq!(paths, expected);
    }

    #[test]
    fn test_generate_jq_paths_nested() {
        let data = json!({"name": "Alive", "type": "paying", "features": [{"premium": true}]});
        let paths = generate_jq_paths(&data).unwrap();
        let expected = vec![".features", ".features[]", ".features[].premium", ".name", ".type"];
        assert_eq!(paths, expected);
    }

    #[test]
    fn test_generate_jq_paths_empty_array() {
        let data = json!({"items": []});
        let paths = generate_jq_paths(&data).unwrap();
        let expected = vec![".items"];
        assert_eq!(paths, expected);
    }

    #[test]
    fn test_generate_jq_paths_nested_objects() {
        let data = json!({"outer": {"inner": {"key": "value"}}});
        let paths = generate_jq_paths(&data).unwrap();
        let expected = vec![".outer", ".outer.inner", ".outer.inner.key"];
        assert_eq!(paths, expected);
    }

    #[test]
    fn test_generate_jq_paths_array_of_objects() {
        let data = json!([{"key1": "value1"}, {"key2": "value2"}]);
        let paths = generate_jq_paths(&data).unwrap();
        let expected = vec![".[]", ".[].key1", ".[].key2"];
        assert_eq!(paths, expected);
    }

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
