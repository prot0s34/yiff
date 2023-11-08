use serde_yaml::Value;
use difference::{Changeset, Difference};
use std::fs;

fn main() {
    // Load your YAML files into structured data.
    let yaml1_content = fs::read_to_string("file1.yaml").expect("Failed to read file1.yaml");
    let yaml2_content = fs::read_to_string("file2.yaml").expect("Failed to read file2.yaml");

    let yaml1: Value = serde_yaml::from_str(&yaml1_content).expect("Failed to parse file1.yaml");
    let yaml2: Value = serde_yaml::from_str(&yaml2_content).expect("Failed to parse file2.yaml");

    // Traverse and construct full nested paths for changes.
    let mut paths = vec![];
    find_paths(&yaml1, &mut paths, "".to_string());

    // Compare the two data structures.
    let changeset = Changeset::new(
        &serde_yaml::to_string(&yaml1).unwrap(),
        &serde_yaml::to_string(&yaml2).unwrap(),
        "\n",
    );

    println!("");
    println!("#### YAML STRUCTURE ####");
    println!("");
    for path in paths {
        println!("{}", path);
    }

    println!("");
    println!("#### CHANGES ####");
    println!("");
    for change in &changeset.diffs {
        match change {
            Difference::Add(value) => {
                // Get the full path for the added value.
                let path = find_path(&yaml1, &yaml2, value);
                println!("ADDED: {}", path);
            }
            Difference::Rem(value) => {
                // Get the full path for the removed value.
                let path = find_path(&yaml1, &yaml2, value);
                println!("REMOVED: {}", path);
            }
            _ => {}
        }
    }
}

fn find_path(data1: &Value, data2: &Value, current_key: &str) -> String {
    if data1 == data2 {
        return String::new();
    }

    match (data1, data2) {
        (Value::Mapping(map1), Value::Mapping(map2)) => {
            for (k, v1) in map1.iter() {
                if let Some(v2) = map2.get(k) {
                    let diff_path = find_path(v1, v2, current_key);
                    if !diff_path.is_empty() {
                        return diff_path;
                    }
                }
            }
        }
        (Value::Sequence(seq1), Value::Sequence(seq2)) => {
            for (i, v1) in seq1.iter().enumerate() {
                if let Some(v2) = seq2.get(i) {
                    let diff_path = find_path(v1, v2, current_key);
                    if !diff_path.is_empty() {
                        return diff_path;
                    }
                }
            }
        }
        _ => {
            if data1 != data2 {
                return current_key.to_string();
            }
        }
    }

    String::new()
}

fn find_paths(data: &Value, paths: &mut Vec<String>, current_path: String) {
    if let Value::Mapping(map) = data {
        for (key, value) in map {
            let mut new_path = current_path.clone();
            if !new_path.is_empty() {
                new_path.push('.');
            }
            new_path.push_str(key.as_str().unwrap_or(""));
            paths.push(new_path.clone());
            find_paths(value, paths, new_path);
        }
    } else if let Value::Sequence(seq) = data {
        for (index, value) in seq.iter().enumerate() {
            let new_path = format!("{}[{}]", current_path, index);
            paths.push(new_path.clone());
            find_paths(value, paths, new_path);
        }
    }
}
