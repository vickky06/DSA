use std::{collections::HashMap, fmt::Error};

#[derive(Debug)]
pub enum PopularityTrend {
    INCREASING,
    DECREASING,
    FLUCTUATING,
}
#[derive(Debug)]
pub struct Popularity {
    pub popularity: HashMap<String, Vec<i32>>,
}

impl Popularity {
    pub fn new() -> Self {
        return Self {
            popularity: HashMap::new(),
        };
    }

    pub fn add_popularity( &mut self, word: &str, score: i32)  {
        let word = word.to_string();
        self.popularity.entry(word).or_insert(Vec::new()).push(score);
    }
    pub fn check_popularity(&self, word: &String) -> Result<PopularityTrend, Error> {
        let mut increasing = true;
        let mut decreasing = true;
        let popularity_vector = self.popularity.get(word);
        match popularity_vector {
            Some(score) => {
                for i in 0..score.len() - 1 {
                    if score[i] > score[i + 1] {
                        increasing = false;
                    }
                    if score[i] < score[i + 1] {
                        decreasing = false;
                    }
                }
                // [10, 20, 15, 30, 25, 40] || [10,10,10,10,10,10]
                if (increasing && decreasing ) || (!increasing && !decreasing) {
                    return Result::Ok(PopularityTrend::FLUCTUATING);
                }
                //[10, 20, 30, 40, 50]
                else if increasing {
                    return Result::Ok(PopularityTrend::INCREASING)
                }
                // [50, 40, 30, 20, 10]
                else {
                    return  Result::Ok(PopularityTrend::DECREASING);
                }
            }
            None => {
                return Result::Err(Error);
            }
        }
    }
}
