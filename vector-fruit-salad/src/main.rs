/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::io::BufReader;
use std::io::BufRead;

static MAX_FRUITS: usize = 5;
static ALL_FRUITS: [&str; MAX_FRUITS] = [
    "Banana",
    "Avocado",
    "Date",
    "Lemon",
    "Watermelon"
];

/// Adds n fruits to the fruits vector.
/// 
fn add_fruits(fruits: &mut Vec<&str>, rng: &mut ThreadRng, mut n: usize) {
    let mut chosen_fruits = Vec::new(); 
    if n > MAX_FRUITS {
        n = MAX_FRUITS;
    }
    chosen_fruits = ALL_FRUITS[0..n].to_vec();
    // Scramble local selection
    chosen_fruits.shuffle(rng);
    fruits.append(&mut chosen_fruits);
}

fn get_user_fruits() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

fn _read_stdin<R: BufRead>(reader: &mut R) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}

/// 
/// Removes the fruit at the end (last element) of the vector.
fn remove_back_fruit(fruits: &mut Vec<&str>) {
    let removed_fruit = fruits.pop();
    println!("Removed fruit: {:?}", removed_fruit);
    print_salad(fruits);
}

/// Print out the fruit salad
fn print_salad (fruit: &Vec<&str>) {
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}

fn main() {
    let dish = ("ham", "eggs", 2);
    let mut fruit = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];

    // Write the fruits you want to add to the list (separated by a space), and press enter or EOF when finished.
    println!("Please enter the fruits you want to add to the list (separated by a space), and press enter when finished: ");
    let input = get_user_fruits();
    let mut user_in: Vec<&str> = input.split_whitespace().collect();
    //Append the user input to the fruit vector.
    fruit.append(&mut user_in);

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

    print_salad(&fruit);
    
    // Select a random fruit in the salad
    let rand_fruit = fruit.choose(&mut rng);
    println!("Randomly chosen fruit: {:?}", rand_fruit);
    
    // Add n=2 random fruits from a list
    add_fruits(&mut fruit, &mut rng, 2);
    print_salad(&fruit);
    
    
    // Again...
    add_fruits(&mut fruit, &mut rng, 2);
    print_salad(&fruit);

    // Remove the lsast element from the fruit salad.
    remove_back_fruit(&mut fruit);

}
