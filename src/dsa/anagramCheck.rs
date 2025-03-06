use std::collections::HashMap;

pub trait Word_Grouping_Trait{
    fn create_word_grouping(&mut self, word_list: Vec<String>)->();
    fn show_word_grouping(&self)->();
    fn search_word_grouping(&self, word: String)->Vec<String>;
}

pub struct WordGrouping{
    pub word_grouping:HashMap<String, Vec<String>>
}

impl WordGrouping{
    pub fn new()->Self{
        return Self{
            word_grouping:HashMap::new(),
        }   
    }
}
impl Word_Grouping_Trait for WordGrouping{
   
    fn create_word_grouping(&mut self, word_list: Vec<String>) ->(){
        let mut char_freq= vec![0;26];
        for cur_word in word_list{
            for c in cur_word.to_lowercase().chars(){
                char_freq[(c as u32 - 'a' as u32)as usize] += 1;
            }
            let key:String = char_freq.into_iter().map(|f| f.to_string()).collect();
            self.word_grouping.entry(key).or_insert(Vec::new()).push(cur_word);
            char_freq = vec![0;26];
        }   
    }

    fn show_word_grouping(&self)->() {
        for (key,value ) in &self.word_grouping{
            println!("{}:{:?}",key,value);
        }
    }


    fn search_word_grouping(&self, word: String)->Vec<String>{
        let mut char_freq = vec![0;26];
        for c in word.to_lowercase().chars(){
            char_freq[(c as u32 - 'a' as u32)as usize] += 1;
        }
        let key:String = char_freq.into_iter().map(|f| f.to_string()).collect();
        return self.word_grouping.get(&key).unwrap().clone();
        
    }
}


pub fn greet(name:&str)->String{
    format!("Hello {}",name)
}