//Rust Book Chapter 3: Variables, Datatypes and other Common Concepts

fn main() {
    let mut tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of my tulple element is {}", tup.0);
    tup.0 = 200;
    println!("The value of my tulple element is {}", tup.0);
    another_function(&tup);
    println!("The value of my tulple element is {}", tup.0);
}

fn another_function(mut x: &(i32, f64, u8)){
    println!("The value of my tulple element is {}", x.0);
}