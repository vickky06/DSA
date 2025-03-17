use rand::{seq::SliceRandom, thread_rng, Rng};

pub trait UtilTraits {
    fn generate_random_numbers(min: Option<i32>, max: Option<i32>) -> i32;
    fn generate_random_name() -> String;
}
pub struct utils {
    // pub genete_random_numbers: fn(min: Option<i32>, max: Option<i32>) -> i32
}

impl UtilTraits for utils {
    fn generate_random_numbers(min: Option<i32>, max: Option<i32>) -> i32 {
        let min = min.unwrap_or(0);
        let max = max.unwrap_or(100);
        let mut rng = rand::thread_rng();
        // println!("min: {} || max :{}", min, max);
        rng.gen_range(min..=max)
    }

    fn generate_random_name() -> String {
        let names = [
            "Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace", "Hannah", "Ivy", "Jack",
        ];

        let mut rng = thread_rng();
        names.choose(&mut rng).unwrap().to_string()
    }
}

pub struct NameGenerator {
    names: Vec<String>,
    index: usize,
    pub max_index: usize
}

impl NameGenerator {
   pub fn new() -> Self {
        let mut names = vec![
            "Alice".to_string(), "Bob".to_string(), "Charlie".to_string(), 
            "David".to_string(), "Eve".to_string(), "Frank".to_string(), 
            "Grace".to_string(), "Hannah".to_string(), "Ivy".to_string(), "Jack".to_string(),
        ];
        let maX_len = names.len();
        // Shuffle the names to ensure randomness
        let mut rng = thread_rng();
        names.shuffle(&mut rng);

        Self { names, index: 0, max_index: maX_len }
    }

    pub fn next_name(&mut self) -> Option<String> {
        if self.index < self.names.len() {
            let name = self.names[self.index].clone();
            self.index += 1;
            Some(name)
        } else {
            None // No more unique names left
        }
    }
}
