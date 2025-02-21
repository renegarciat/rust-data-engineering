use clap::Parser;
use cli_salad::create_fruit_salad;
/* Try running ./cli-salad -n 3 -f 'myfruit1 myfruit2 myfruit3' */
#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
    #[clap(short, long, value_delimiter = ' ', num_args = 1..)]
    pub fruits: Option<Vec<String>>,
}

fn print_fruits(fruits: &Vec<String>) {
    // locally sort the fruits
    let mut local_fruits = fruits.clone();
    local_fruits.sort_by_key(|word| word.to_lowercase());
    println!("Sorted fruits: {:?}", local_fruits);
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    // Get the inputed fruits
    let mut in_fruits = opts.fruits.unwrap();
    println!("Input fruits: {:?}", in_fruits);
    // Create the fruit salad
    let mut my_salad = create_fruit_salad(num_fruits);
    my_salad.append(&mut in_fruits);

    // Print the fruit salad in human readable format with a count of fruits used
    println!(
        "Created Fruit salad with {} fruits: {:?}",
        num_fruits, my_salad
    );

    // Sort and print the fruits
    print_fruits(&my_salad);
}
