use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn create_tokens() -> HashMap<String, f64> {
    let tokens = vec![
        ("$size-spacing-0", 0.0),
        ("$size-spacing-1", 0.25),
        ("$size-spacing-2", 0.5),
        ("$size-spacing-3", 0.75),
        ("$size-spacing-4", 1.0),
        ("$size-spacing-5", 1.25),
        ("$size-spacing-6", 1.5),
        ("$size-spacing-7", 1.75),
        ("$size-spacing-8", 2.0),
        ("$size-spacing-9", 2.25),
        ("$size-spacing-10", 2.5),
        ("$size-spacing-11", 2.75),
        ("$size-spacing-12", 3.0),
        ("$size-spacing-14", 3.5),
        ("$size-spacing-16", 4.0),
        ("$size-spacing-20", 5.0),
        ("$size-spacing-24", 6.0),
        ("$size-spacing-28", 7.0),
        ("$size-spacing-32", 8.0),
        ("$size-spacing-36", 9.0),
        ("$size-spacing-40", 10.0),
        ("$size-spacing-44", 11.0),
        ("$size-spacing-48", 12.0),
        ("$size-spacing-52", 13.0),
        ("$size-spacing-56", 14.0),
        ("$size-spacing-60", 15.0),
        ("$size-spacing-64", 16.0),
        ("$size-spacing-72", 18.0),
        ("$size-spacing-80", 20.0),
        ("$size-spacing-96", 24.0),
        ("$size-spacing-px", 0.0625),
        ("$size-spacing-0-5", 0.125),
    ];

    tokens
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect()
}

fn find_nearest_token(value: f64, tokens: &HashMap<String, f64>) -> String {
    let mut min_diff = f64::MAX;
    let mut nearest_token = String::new();

    for (token, token_value) in tokens {
        let diff = (*token_value - value).abs();
        if diff < min_diff {
            min_diff = diff;
            nearest_token = token.clone();
        }
    }

    nearest_token
}

fn process_file(file_path: &str, tokens: &HashMap<String, f64>, output_file: &mut File) {
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let px_pattern = regex::Regex::new(r"([\w-]+)\s*:\s*([\d\s]*(?:\d+px\b\s*)+)")
        .expect("Unable to compile regex");

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.expect("Unable to read line");

        for capture in px_pattern.captures_iter(&line) {
            let style_name = &capture[1];
            let px_value_string = &capture[2];
            let px_values = px_value_string.split_whitespace().collect::<Vec<_>>();

            let recommendations = px_values
                .iter()
                .map(|px_value| {
                    let value: f64 = px_value
                        .replace("px", "")
                        .parse()
                        .expect("Unable to parse pixel value");
                    find_nearest_token(value / 16.0, &tokens)
                })
                .collect::<Vec<String>>()
                .join(", ");

            println!(
                "Line: {} - {}: {} | Recommendations: {}",
                line_number + 1,
                style_name,
                px_value_string,
                recommendations
            );

            writeln!(
                output_file,
                "Line: {} - {}: {} | Recommendations: {}",
                line_number + 1,
                style_name,
                px_value_string,
                recommendations
            )
            .expect("Unable to write to output file");
        }
    }
}

fn traverse_dir(dir_path: &Path, tokens: &HashMap<String, f64>, output_file: &mut File) {
    for entry in fs::read_dir(dir_path).expect("Unable to read directory") {
        let entry = entry.expect("Unable to read entry");
        let entry_path = entry.path();

        if entry_path.is_file() {
            let ext = entry_path.extension().unwrap_or_default();

            if ext == "css" || ext == "scss" || ext == "svelte" {
                let file_path = entry_path
                    .to_str()
                    .expect("Unable to convert path to string");
                println!("\nFile: {}", file_path);
                writeln!(output_file, "\nFile: {}", file_path)
                    .expect("Unable to write to output file");
                process_file(file_path, &tokens, output_file);
            }
        } else if entry_path.is_dir() {
            traverse_dir(&entry_path, tokens, output_file);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <starting_directory>", args[0]);
        return;
    }

    let start_path = Path::new(&args[1]);
    if !start_path.exists() || !start_path.is_dir() {
        println!("Error: Invalid starting directory");
        return;
    }

    let file_output_path = "output.txt";
    let mut output_file = File::create(file_output_path).expect("Unable to create file");

    let tokens = create_tokens();
    traverse_dir(&start_path, &tokens, &mut output_file);
}
