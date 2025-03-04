use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn init_languages() -> HashMap<String, i32> {
    let mut languages = HashMap::new();
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("HTML/CSS".to_string(), 1990);
    languages.insert("Python".to_string(), 1991);
    languages.insert("SQL".to_string(), 1974);
    languages.insert("TypeScript".to_string(), 2012);
    languages.insert("Bash/Shell".to_string(), 1989);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C#".to_string(), 2000);
    languages.insert("C++".to_string(), 1985);
    languages.insert("C".to_string(), 1972);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("PowerShell".to_string(), 2006);
    languages.insert("Go".to_string(), 2007);
    languages.insert("Rust".to_string(), 2010);

    languages
}

fn calculate_weights(years_active: &mut HashMap<String, i32>) -> HashMap<String, i32> {
    // Subtract the creation year from 2024 to get the number of years active.
    for year in years_active.values_mut() {
        *year = 2025 - *year;
    }

    let min_year = *years_active.values().min().unwrap_or(&0);
    let max_year = *years_active.values().max().unwrap_or(&0);

    let mut weights = HashMap::new();

    for (language, &year) in years_active.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 + 1; // weight between 1 and 100
        weights.insert(language.to_string(), weight);
    }

    weights
}

fn get_languages() -> HashMap<String, i32> {
    let mut languages: HashMap<String, i32> = HashMap::new();
    let text = _read_stdin();
    let words: Vec<&str> = text.split_whitespace().collect();
    for word in &words {
        let parts: Vec<&str> = word.split(',').collect();
        languages.insert(parts[0].to_string(), parts[1].parse().unwrap());
    }
    languages
}

fn _read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    let line = line.trim().to_string();
    line
}

fn main() {
    let mut languages = init_languages();
    let weights = calculate_weights(&mut languages);

    println!("Language weighing from 1-100 by age (1 is newest and 100 is oldest):");
    for (language, weight) in &weights {
        println!("{}: {}", language, weight);
    }
    println!("Include new languages\n Type new languages and their year of birth, separated by spaces. Eg.\n
            Ruby,1993");
    let mut input_languages: HashMap<String, i32> = get_languages();
    let weights = calculate_weights(&mut input_languages);
    println!("Language weighing from 1-100 by age (1 is newest and 100 is oldest):");
    for (language, weight) in &weights {
        println!("{}: {}", language, weight);
    }
}
