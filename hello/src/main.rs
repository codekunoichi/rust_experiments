fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let a = 3;
    let b = 4;
    let c = add_two_numbers(a, b);
    println!("{} + {} = {}", a, b, c);
    println!("Hello, world!");
}
