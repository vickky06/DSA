use std::collections::HashMap;

use crate::dsa::mostRecentlyPurchased::Direction;
use crate::dsa::mostRecentlyPurchased::Item_Map;
use crate::util::utils::utils as Utils;
use crate::util::utils::NameGenerator;
use crate::util::utils::UtilTraits;

use crate::dsa::anagramCheck::{WordGrouping, Word_Grouping_Trait};
use crate::dsa::binarySearchTree::BinarySearchTree;
use crate::dsa::itemRankMerging::ItemObjectsLinkedList;
use crate::dsa::overlappingTime::overlapping_time::{Meeting, Scheduler};
use crate::dsa::popularityCheck;
use crate::dsa::specialShoppingCart::ComboOffers;
use crate::dsa::specialShoppingCart::*;
use crate::dsa::stack_max::StockStack;

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
            Some(overlaps) => println!(
                "Overlapping meetings between {} and {}: {:#?}",
                name1, name2, overlaps
            ),
            None => println!("No overlapping meetings between {} and {}", name1, name2),
        }
    }
}

pub fn shopping_cart_main() -> () {
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
    println!(
        "Fetching Orange (not added): {:?}",
        item_prices.get_item("Orange")
    );

    // Remove an item
    item_prices.remove_item("Banana");
    println!(
        "Fetching Banana after removal: {:?}",
        item_prices.get_item("Banana")
    );

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
    combo_offers.set_combo_price(20, &item_prices); // Set combo price

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

pub fn bst_main() {
    let mut product_prices: Vec<i32> = Vec::new();
    for i in 0..=10 {
        product_prices.push(Utils::generate_random_numbers(Some(i), Some(i + 100)));
    }
    let max: &i32 = product_prices.iter().max().unwrap();
    let min: &i32 = product_prices.iter().min().unwrap();

    let mut bst = BinarySearchTree::new(product_prices[0]);
    for i in 1..product_prices.len() {
        bst.insert(product_prices[i]);
        print!("{} ", product_prices[i]);
    }
    println!("");
    for i in 0..1 {
        let min_t = Utils::generate_random_numbers(Some(min + 0), Some(max - 1));
        let max_t = Utils::generate_random_numbers(Some(min_t + 1), Some(max + 0));
        println!(
            "Products in the range: min: {} max: {} are: {:?}",
            min_t,
            max_t,
            bst.productsInRange(min_t, max_t)
        );
    }
}

pub fn item_rank_main() {
    const SORTED_LIST: &str = "sorted_list";
    let mut item_rank: ItemObjectsLinkedList<i32> = ItemObjectsLinkedList::new(SORTED_LIST.to_string());
    let mut generator = NameGenerator::new();
    let mut prod_list: Vec<String> = Vec::new();
    
     
    
    // Add a random number of products, each with a random number of prices.
    // The prices are in decreasing order, so the most recent price is added to the front of the list.
    // This is done to maintain the increasing order of the list, as the data structure is a min-heap or a sorted list.
    for _ in 0..3 {
        println!();
        if let Some(name) = generator.next_name() {
            // Generate a random number of prices for the product
            let range = Utils::generate_random_numbers(Some(3), Some(11));
            // Generate the initial price
            let mut last_price = Utils::generate_random_numbers(Some(90), Some(100));
            // Add the prices to the list in decreasing order
            for i in 0..range {
                item_rank.upsert_item(&name, last_price);
                // Generate the next price, which is smaller than the previous one
                last_price =
                    Utils::generate_random_numbers(Some(last_price - 11), Some(last_price -1));
            }
            // Add the product name to the list of products
            prod_list.push(name.clone());
        } else {
            println!("No more unique names available.");
        }
    }

    // println!("Printing keys");
    // // for name in prod_list {
    //     item_rank.print_list(prod_list[0].as_str());
    //     item_rank.remove_item(prod_list[0].as_str());
    //     item_rank.print_list(prod_list[0].as_str());

    // // }

    item_rank.add_linked_list(SORTED_LIST);
    let values:Vec<(i32, String)> = Vec::new();
    // let mut i = 0;
    loop{
        // println!("*********");
        let values:Vec<(i32,String)> = item_rank.get_heads();
        if values.len() == 0 {
            break;
        }
        let (min_item,min_value_item) = values.iter().min().unwrap();
        item_rank.upsert_item(SORTED_LIST, min_item+0);
        // item_rank.print_list(SORTED_LIST);
        item_rank.remove_item(&min_value_item);
    }

    item_rank.print_list(SORTED_LIST);
   
}


pub fn most_recently_purchased(){
    
    let mut tracker = Item_Map::new(2);
    tracker.purchased(10);
    tracker.purchased(20);
    tracker.print_list(Direction::Forward); // 10 20
    tracker.purchased(10);
    tracker.print_list(Direction::Forward); // 20 10
    tracker.purchased(30);
    tracker.print_list(Direction::Forward); // 10 30
    
}