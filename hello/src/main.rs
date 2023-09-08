fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// add three numbers together
fn add_three_numbers(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

/*
Uses my_logi.rs to add all the elements of a vector together
*/
mod my_logic;

fn main() {
    println!(
        " The sum of the elements of the vector is: {}",
        my_logic::sum()
    );

    let a = 3;
    let b = 4;
    let c = add_two_numbers(a, b);
    println!("{} + {} = {}", a, b, c);
    println!("Hello, world!");

    // add three numbers together
    let d = 5;
    let e = 6;
    let f = 7;
    let g = add_three_numbers(d, e, f);
    println!("{} + {} + {} = {}", d, e, f, g);
}
