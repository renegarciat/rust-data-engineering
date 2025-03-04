/*
This example code counts the frequency of each number in the vector.
 */
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}

fn logic_input_sentece() -> Vec<(String, u32)> {
    let text = _read_stdin();
    let words: Vec<String> = text.split_whitespace().map(String::from).collect();
    let mut frequencies: HashMap<&str, u32> = HashMap::new();

    for word in &words {
        let frequency = frequencies.entry(word).or_insert(0);
        *frequency += 1;
    }

    let mut result: Vec<(String, u32)> = Vec::new();

    for (word, frequency) in frequencies {
        result.push((word.to_string(), frequency));
    }
    result
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

fn sort_by_frequency(list: &Vec<(String, u32)>) -> Vec<(String, u32)> {
    let mut sorted_list: Vec<(String, u32)> = list.clone();
    sorted_list.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_list
}
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let result = logic(numbers);
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );
    let input_string = logic_input_sentece();
    println!(
        "The frequency of each word in the vector is: {:?}",
        input_string
    );
    let sorted_input_string = sort_by_frequency(&input_string);
    println!(
        "The frequency of each word in the vector is: {:?}",
        sorted_input_string
    );
}
