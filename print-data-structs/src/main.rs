use std::collections::*;

fn main() {
    println!("Common Rust Collections:");

    // Sequences
    println!("\n\tSequences:");

    println!("\t\tVec: https://doc.rust-lang.org/std/vec/struct.Vec.html");
    let mut vec = Vec::new();
    vec.push("hello");
    vec.push("Michoacan");
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], "hello");

    assert_eq!(vec.pop(), Some("Michoacan"));
    assert_eq!(vec.len(),  1);

    vec.extend(["a","b","c"]);
    for x in &vec {
        println!("{x}");
    }
    assert_eq!(vec, ["hello", "a", "b", "c"]);
    // The vec! macro is provided for convenient initialization
    let mut vec1 = vec![0;5];
    assert_eq!(vec1, [0,0,0,0,0]);


    println!("\t\tVecDeque: https://doc.rust-lang.org/std/collections/struct.VecDeque.html");
    //use std::collections::VecDeque;
    let mut deq = VecDeque::from([5, 6, 7]);
    deq.push_back(-1);
    deq.push_front(-1);
    for x in &deq {
        println!("{x}");
    }


    println!("\t\tLinkedList: https://doc.rust-lang.org/std/collections/struct.LinkedList.html");
    use std::collections::LinkedList;
    let mut ll = LinkedList::from([0,1,2]);
    for x in &ll {
        println!("{x}");
    }
    let a = ll.back();
    println!("{:?}",a);
    ll.clear();

    // Maps
    println!("\n\tMaps:");
    println!("\t\tHashMap: https://doc.rust-lang.org/std/collections/struct.HashMap.html");
    println!("\t\tBTreeMap: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html");
    //use std::collections::HashMap;
    let mut book_reviews = HashMap::new();
    book_reviews.insert("Cien años de soledad", "Cool");
    book_reviews.insert("Orlando", "Malo livro");
    book_reviews.insert("Como vender una casa embrujada", "Tedioso");
    book_reviews.insert("Mi Lucha", "Interesante");

    // Check for a specific one.
    // When collections store owned values (Strings), they can still be
    // queried using references (&str).
    if !book_reviews.contains_key("El Zarco") {
        println!("Tenemos {} reviews, pero \"El Zarco\" no esta en la lista",
        book_reviews.len());
    }
    // Este review tiene muchas faltas de ortografia, hay que eliminarlo
    book_reviews.remove("Orlando");

    // Busquemos un libro valido
    let to_find = ["Mi Lucha", "Cien años de soledad"];
    for book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{book}: {review}"),
            None => println!("{book} is unreviewed"),
        }
    }

    // Sets
    println!("\n\tSets:");
    println!("\t\tHashSet: https://doc.rust-lang.org/std/collections/struct.HashSet.html");
    println!("\t\tBTreeSet: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html");
    let mut a: HashSet<i32> = vec![1i32, 2, 3, 4].into_iter().collect();
    let mut b: HashSet<i32> = vec![1i32, 2, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));
    // `HashSet::insert()` returns false if 
    // there was a value already present.
    // assert!(b.insert(4), "Value 4 is already in set B!");
    let mut x: BTreeSet<i32> = vec![1, 2, 3].into_iter().collect();
    x.insert(6);


    // Misc
    println!("\n\tMisc:");
    println!("\t\tBinaryHeap: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html");
    let mut heap = BinaryHeap::new();
    heap.push(1);
    heap.push(5);
    heap.push(2);

    assert_eq!(heap.peek(), Some(&5));

}
