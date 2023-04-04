use core::{fmt::Error, error};

fn main() {
    let new_list= pass_values();
}

pub async fn pass_values() -> Result<Vec<usize>, Box<dyn std::error::Error>>>{
    let mut list: Vec<usize>= Vec::new();
    list.push(1);
    list.push(2);

    return Ok(list);
}
