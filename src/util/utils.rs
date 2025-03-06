use rand::Rng;

pub trait UtilTraits {
    fn generate_random_numbers(min: Option<i32>, max: Option<i32>) -> i32;
    
}
pub struct utils{
    // pub genete_random_numbers: fn(min: Option<i32>, max: Option<i32>) -> i32
 }

impl UtilTraits for utils{
    fn generate_random_numbers(min: Option<i32>, max: Option<i32>) -> i32 {
        let min = min.unwrap_or(0);
        let max = max.unwrap_or(100);
        let mut rng = rand::thread_rng();
        println!("min: {} || max :{}", min, max);
        rng.gen_range(min..=max)
    }
}