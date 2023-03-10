use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn solution(list: Vec<String>) -> bool{
    let mut hash_map: HashMap<String, usize>= HashMap::new();
    for i in 0..list.len(){
       hash_map.insert(list[i].clone(), i); 
    }

    for i in 0..list.len(){
        let substring= list[i].as_str();
        
        for j in 0..substring.len(){
           if(hash_map.contains_key(substring[0..j]))
               return false;
        }
    }
    
    true
}
