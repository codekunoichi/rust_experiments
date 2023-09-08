/* Calculator that uses an immutable vector
Lets make our vector a constant so that we can use it in our tests
*/

// a vector of immutable integers
const NUMBERS: [i32; 4] = [1, 2, 3, 4];

// a function that returns the sum of the vector
pub fn sum() -> i32 {
    let mut sum = 0;
    for n in NUMBERS.iter() {
        sum += n;
        // println!("{}", n);
    }
    println!("{}", sum);
    sum
}

/** These are the tests for the calculator */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum() {
        assert_eq!(sum(), 10);
        assert_eq!(sum(), 10);
    }
}
