use std::collections::HashMap;

fn main() {
    let mut fruit_salad = vec!["apple", "banana", "cherry", "pomegranate", "fig", "prickly pear"];
    fruit_salad.push("pear");
    fruit_salad.pop();
    print_fruits(&fruit_salad);

    // Remove "fig"
    remove_fruit(&mut fruit_salad, "fig");
    println!("After removing fig:");
    print_fruits(&fruit_salad);

    // Sort fruits
    sort_fruits(&mut fruit_salad);
    println!("After sorting fruits:");
    print_fruits(&fruit_salad);

    // Count occurences
    let occurences = count_occurences(&fruit_salad);
}
fn print_fruits(fruit_vec: &Vec<&str>) {
    for fruit in fruit_vec {
        println!("{}", fruit);
    }
}

fn remove_fruit(fruit_vec: &mut Vec<&str>, fruit: &str) {
    // Iterate over entire structure (intensive task, consider using map instead)
    /* Here we have two options: .remove or .retain.
    .remove will remove the first occurrence of the element and return it.
    .retain will remove all occurrences of the element and return a boolean indicating if any were removed.
    */
    // Assuming the fruit can be found more than once, we can use .retain
    fruit_vec.retain(|&x| x != fruit);
}

fn sort_fruits(fruit_vec: &mut Vec<&str>) {
    fruit_vec.sort();

}

fn count_occurences(fruit_vec: &Vec<&str>) -> HashMap<&str, u32>{
    let mut occurences: HashMap<&str, u32> = HashMap::new();
    for fruit in fruit_vec {
        *occurences.entry(fruit).or_insert(0) += 1;
    }
    occurences
}