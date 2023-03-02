fn main() {
    println!("Hello, world!");
}

fn init(v: Vec<usize>, v2: [Vec<usize>]) -> Vec<usize>{

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let v= vec![1, 5, 2, 6, 3, 7, 4];
        let vec_list= vec![vec![2usize, 5, 3], vec![4usize, 4, 1], vec![1usize, 7, 3]];
        
        init(v, vec_list);
        
    }
}
