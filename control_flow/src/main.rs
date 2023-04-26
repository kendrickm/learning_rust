fn five() -> i32 {
    println!("You got 5 on it");
    5
}

fn multi_input(value: i32, ignore_it: bool) -> i32 {
    if ignore_it{
        return five();
    }
    
    value
}


fn main() {
   let x = multi_input(10, true);
   println!("{x}");
}
