use std::collections::HashMap;

mod dsa;
mod util;
use crate::dsa::anagramCheck::{WordGrouping, Word_Grouping_Trait};
use crate::dsa::popularityCheck;
use crate::dsa::stack_max::StockStack;
use crate::dsa::overlappingTime::overlapping_time::{Meeting, Scheduler};
use util::utils::utils as Utils;
use util::utils::UtilTraits;

// ...

fn anagram_main() -> () {
    let mut word_grouping = WordGrouping::new();
    let word_list = vec![
        "cat".to_string(),
        "dog".to_string(),
        "tac".to_string(),
        "god".to_string(),
    ];
    word_grouping.create_word_grouping(word_list);
    word_grouping.show_word_grouping();
    println!(
        "{:?}",
        word_grouping.search_word_grouping("cat".to_string())
    )
}

fn popularity_main() -> () {
    let mut popularity = popularityCheck::Popularity::new();
    let mut popularity_mapper = HashMap::<String, Vec<i32>>::new();

    popularity_mapper.insert("cat".to_string(), vec![10, 20, 15, 30, 25, 40]);
    popularity_mapper.insert("dog".to_string(), vec![10, 10, 10, 10, 10, 10]);
    popularity_mapper.insert("cow".to_string(), vec![10, 20, 30, 40, 50]);
    popularity_mapper.insert("goat".to_string(), vec![50, 40, 30, 20, 10]);
    for (k, vector) in &popularity_mapper {
        for v in vector {
            popularity.add_popularity(&k, *v);
        }
    }
    for (k, v) in &popularity_mapper {
        match popularity.check_popularity(k) {
            Ok(popularity_trend) => {
                println!("{} is {:?}:{:?}", k, popularity_trend, v);
            }
            Err(e_pop) => {
                println!("{} is not found {:?}", k, e_pop);
            }
        }
    }
}

fn stock_main() -> () {
    let mut stock = StockStack::new();
    for i in 1..11 {
        println!("Week: {}", i);
        let min = Utils::generate_random_numbers(None, Some(i));
        let max = Utils::generate_random_numbers(Some(i + 1), None);
        stock.push(Utils::generate_random_numbers(Some(min), Some(max)));
    }
    stock.print_stack();

    for _ in 0..3 {
        stock.pop();
    }
    stock.print_stack();
    for i in 1..10 {
        let tup = stock.get_max_for_week(Some(i)).unwrap();
        println!("Max for week {} is {}", tup.0, tup.1);
    }
}

fn overlaping_main()->(){
        let mut scheduler = Scheduler::new();
    
        let meetings:Vec<(&str, Meeting)> = vec![
            ("Alice", Meeting::new(10, 20).unwrap()),
            ("Alice", Meeting::new(15, 25).unwrap()),
            ("Alice", Meeting::new(1, 2).unwrap()),
            ("Bob", Meeting::new(20, 30).unwrap()),
            ("Bob", Meeting::new(25, 35).unwrap()),
            ("Zeus", Meeting::new(5, 7).unwrap()),
            ("Zeus", Meeting::new(100, 101).unwrap()),
            // ("Kratos", Meeting::new(50, 60).unwrap()),
        ];
        let mut names: Vec<&str> = Vec::new();
    
        for (name, meeting) in &meetings {
            scheduler.add_meeting(name.to_string(), *meeting);
            names.push(name);
        }
       

        // let names: Vec<&&str> = meetings.into_iter().map(|meeting| &meeting.0).collect::<Vec<_>>();    
        for name in names {
            scheduler.show_meetings(name.to_string());
        }
    
        let pairs = vec![("Alice", "Bob"), ("Zeus", "Kratos")];
    
        for (name1, name2) in pairs {
            match scheduler.get_overlaps(name1.to_string(), name2.to_string()) {
                Some(overlaps) => println!("Overlapping meetings between {} and {}: {:#?}", name1, name2, overlaps),
                None => println!("No overlapping meetings between {} and {}", name1, name2),
            }
        }
     
}
fn main() {
    overlaping_main();
    //stock_main();
    //anagram_main()
    // popularity_main();

}
