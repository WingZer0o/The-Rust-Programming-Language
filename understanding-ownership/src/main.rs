fn main() {
    move_vs_clone();
    ownership_and_functions();
    return_values_and_scope();
}

fn return_values_and_scope() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("s1 = {}, s2 = {}, s3 = {}", s1, s3, s3);
}


fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}











fn ownership_and_functions() {
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;
    
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}


fn move_vs_clone() {
    // let s = String::from("world");
    // let k = s;
    // println!("s = {}, k = {}", s, k);

    // copy that data from the heap.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}