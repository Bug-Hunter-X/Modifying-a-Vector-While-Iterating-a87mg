fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();
    println!("First element: {:?}", iter.next());
    // Modify the vector while iterating
    vec.push(4);
    println!("Second element: {:?}", iter.next());
}