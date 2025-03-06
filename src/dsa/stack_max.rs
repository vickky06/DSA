
pub struct StockStack{
    pub max_stack:Vec<i32>,
    pub main_stack:Vec<i32>,
}

impl StockStack{
    pub fn new()->Self{
        return Self{
            max_stack:Vec::new(),
            main_stack:Vec::new(),
        }
    }

    pub fn push(&mut self, value:i32){
        // println!("Pushing value: {}", value);
        self.main_stack.push(value);
        if !(self.main_stack.is_empty() || self.max_stack.is_empty()){
            if value > *self.max_stack.last().unwrap(){
                self.max_stack.push(value);
            }
            else{
                self.max_stack.push(*self.max_stack.last().unwrap());
            }
        }
        else{
            self.max_stack.push(value);
            // println!("There are no stock data yet.");
        }
    }

    pub fn pop(&mut self){
        if self.main_stack.is_empty(){
            println!("There are no stock data yet.");
            return
        }
        let popped = self.main_stack.pop();
        self.max_stack.pop();
        // println!("Popped value: {}", popped.unwrap());
    }

    pub fn print_stack(&self){
        println!("Main stack: {:?}", self.main_stack);
        println!("Max stack: {:?}", self.max_stack);
    }

    pub fn get_max_for_week(&self, week: Option<usize>) -> Option<(i32,String)> {
        let week_index = week.unwrap_or(self.max_stack.len() - 1);
        // println!("week: {}", week_index);z 

        if week_index >= self.max_stack.len() {
            return Some((0,String::from("Invalid week")));
        }
        Some((self.max_stack[week_index],week_index.to_string()))
    }
}