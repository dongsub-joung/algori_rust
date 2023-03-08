use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn solution(parti: Vec<String>, competi: Vec<String>) -> String{
    let mut parti_hash: HashMap<String, i32>= HashMap::new();
    let mut competi_hash: HashMap<String, i32>= HashMap::new();
    
    let mut anwser= String::new();

    for value in parti.clone(){
        parti_hash.insert(value, 1); 
    }
    for value in competi.clone(){
        competi_hash.insert(value, 1);
    }
   
    for value in parti{
        let (pk, pv)= parti_hash.get_key_value(&value).unwrap();  
        let (ck, cv)= competi_hash.get_key_value(&value).unwrap();
        let result= pv.clone() - cv.clone();
        if result == 0 {
            anwser= pk.clone();
            break;
        }
    }
    return anwser;
}
