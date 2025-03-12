use std::collections::HashMap;

use crate::dsa::anagramCheck::{WordGrouping, Word_Grouping_Trait};
use crate::dsa::popularityCheck;
use crate::dsa::stack_max::StockStack;
use crate::dsa::overlappingTime::overlapping_time::{Meeting, Scheduler};
use crate::dsa::specialShoppingCart::*;
use crate::dsa::specialShoppingCart::ComboOffers;
use crate::util::utils::utils as Utils;
use crate::util::utils::UtilTraits;

pub fn anagram_main() {
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

pub fn popularity_main() {
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

pub fn stock_main() {
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

pub fn overlaping_main() {
    let mut scheduler = Scheduler::new();

    let meetings: Vec<(&str, Meeting)> = vec![
        ("Alice", Meeting::new(10, 20).unwrap()),
        ("Alice", Meeting::new(15, 25).unwrap()),
        ("Alice", Meeting::new(1, 2).unwrap()),
        ("Bob", Meeting::new(20, 30).unwrap()),
        ("Bob", Meeting::new(25, 35).unwrap()),
        ("Zeus", Meeting::new(5, 7).unwrap()),
        ("Zeus", Meeting::new(100, 101).unwrap()),
    ];
    let mut names: Vec<&str> = Vec::new();

    for (name, meeting) in &meetings {
        scheduler.add_meeting(name.to_string(), *meeting);
        names.push(name);
    }

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

pub fn shopping_cart_main()->(){
    let mut item_prices = ItemPrices::new();

    // Create items
    let item1 = ItemObject::new("Apple".to_string(), 10);
    let item2 = ItemObject::new("Banana".to_string(), 5);
    let item3 = ItemObject::new("Cherry".to_string(), 15);
    let item4 = ItemObject::new("Date".to_string(), 10); // Same price as Apple

    // Add items
    item_prices.add_item(item1.clone());
    item_prices.add_item(item2.clone());
    item_prices.add_item(item3.clone());
    item_prices.add_item(item4.clone());

    // Try adding duplicate item
    item_prices.add_item(item1.clone()); // Should print "Item already exists"

    // Retrieve items
    println!("Fetching Apple: {:?}", item_prices.get_item("Apple"));
    println!("Fetching Orange (not added): {:?}", item_prices.get_item("Orange"));

    // Remove an item
    item_prices.remove_item("Banana");
    println!("Fetching Banana after removal: {:?}", item_prices.get_item("Banana"));

    // Try removing a non-existent item
    item_prices.remove_item("Grapes"); // Should print "Item does not exist"

    // Update price of an existing item
    item_prices.update_price("Apple", 12);
    println!("Updated Apple price: {:?}", item_prices.get_item("Apple"));

    // Try updating price of a non-existent item
    item_prices.update_price("Grapes", 20); // Should print "Item does not exist"
    
    item_prices.add_item(item2.clone());
    item_prices.add_item(item3.clone());
    item_prices.update_price("Apple", 5);



    // Initialize combo offers
    let mut combo_offers = ComboOffers::new();
    combo_offers.set_combo_price(20,&item_prices); // Set combo price

    // Generate offers
    let offers = combo_offers.generate_offers();

    match offers {
        Some(offers) => {
            println!("Generated Offers:");
            for (item1, item2) in offers {
                println!(
                    "{} + {} = {}",
                    item1.get_name(),
                    item2.get_name(),
                    combo_offers.get_combo_price()
                );
            }
        }
        None => println!("No valid offers created."),
    }
}