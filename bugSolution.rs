fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Solution 1: Cloning the vector before modification
    let vec_clone = vec.clone();
    vec_clone.push(4);
    let mut iter = vec.iter();
    println!("First element: {:?}", iter.next());
    println!("Second element: {:?}", iter.next());

    // Solution 2: Iterating over a copy of the vector
    for element in vec.clone(){
        println!("Element: {}", element);
    }
    vec.push(4);
    
    //Solution 3: Using iter().collect() to create a new vector
    let new_vec = vec.iter().collect::<Vec<&i32>>();
    for element in new_vec{
        println!("Element: {}", element);
    }
    vec.push(4);
}