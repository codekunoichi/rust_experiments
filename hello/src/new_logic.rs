/*
This is a library that randomly returns fruits native to Portugal
*/

use Rand::Rng;

pub fn fruit() -> String {
    let mut rng = rand::thread_rng();
    let fruits = vec!["banana", "maça", "morango", "uva"];
    let index = rng.gen_range(0, fruits.len());
    String::from(fruits[index])
}

/*
Create a function that returns a random fruit native to Portugal.
Define a constant vector of fruits of size atleast 15.

We then use this const in a function that latter get called in the main.rs file as a CLI.
The CLI should support the following:

// the quantity of fruits to be returned
--count=5
The function takes the count of fruits as a parameter to return.
*/

// a vector of immutabe strings that represents fruits native to Portugal and the Azores
const FRUITS: [&str; 10] = ["banana", "maça", "morango", "uva", "abacate", "melancia", "abacaxi", "laranja", "pera", "manga", "kiwi", "péssima"];

// a function that returns a random fruit native to Portugal and Azores and accepts the count of fruits as a parameter
pub fn fruit_azores(count: u32) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut fruit_azores = Vec::new();
    for _ in 0..count {
        let index = rng.gen_range(0, FRUITS.len());
        fruit_azores.push(String::from(FRUITS[index]));
        println!("{}", fruit_azores[index]);
        assert!(fruit_azores[index].len() > 0);
        assert!(fruit_azores[index].len() < 10);
        assert!(fruit_azores[index].contains("a"));
        assert!(fruit_azores[index].contains("m"));
        assert!(fruit_azores[index].contains("u"));
    }
    rng
}
    
// write the test for this function
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fruit_azores() {
        let fruit_azores = fruit_azores(5);
        assert!(fruit_azores.len() == 5);
        assert!(fruit_azores[0].contains("a"));
        assert!(fruit_azores[0].contains("m"));
        assert!(fruit_azores[0].contains("u"));
        assert!(fruit_azores[0].contains("n"));
    }
}

/*
Lets write unit tests for our function
*/
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fruit() {
        let fruit = fruit();
        assert!(fruit == "banana" || fruit == "maça" || fruit == "morango" || fruit == "uva");
        println!("{}", fruit);
        assert!(fruit.len() > 0);
        assert!(fruit.len() < 10);
        assert!(fruit.contains("a"));
        assert!(fruit.contains("m"));
        assert!(fruit.contains("u"));
        assert!(fruit.contains("n"));
        assert!(fruit.contains("b"));
        assert!(fruit.contains("o"));
        assert!(fruit.contains("avacado"));
    }
}