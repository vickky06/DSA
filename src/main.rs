use anagramCheck::{WordGrouping, Word_Grouping_Trait};

mod anagramCheck;
fn main() {
    let mut word_grouping = WordGrouping::new();
    let word_list = vec![
        "cat".to_string(),
        "dog".to_string(),
        "tac".to_string(),
        "god".to_string(),
    ];
    word_grouping.create_word_grouping(word_list);
    word_grouping.show_word_grouping();
    println!("{:?}",word_grouping.search_word_grouping("cat".to_string()))
}
