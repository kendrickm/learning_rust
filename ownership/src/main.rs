//Rust Book Chapter 4.1: Understanding Ownership: What is Ownership
fn main() {
    let mut s = String::from("Hello, ");
    let a = s.clone();

    let x = 5;
    s.push_str("world!");
    println!(r#"{}"#,s);
    println!(r#"{}"#,a);

    takes_ownership(s);
    //println!(r#"{}"#,s); //Fails because s is moved

    makes_copy(x);
    println!("{}",x);


    let s1 = gives_ownership();
    let s2 = String::from("Hi");
    let s3 = takes_and_gives_back(s2);

    println!("{}", s3)
}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} 


fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

a_string  // a_string is returned and moves out to the calling function
}