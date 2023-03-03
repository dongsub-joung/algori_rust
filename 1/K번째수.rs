fn main() {
}   
    pub fn init(v1: Vec<usize>, v2:[[usize; 3]; 3]) -> Vec<usize>{
        let result_v= Vec::new();

        for v in v2{
            let i= v[0];
            let j= v[1];
            let k= v[2];

            let v= v1[i..k];
            
            v.sort();

            let k_number= v[k];

            result_v.push(k_number);
        }

        result_v
    }


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let v= vec![1usize, 5, 2, 6, 3, 7, 4];
        let vec_list= vec![vec![2usize, 5, 3], vec![4usize, 4, 1], vec![1usize, 7, 3]];
        let result= vec![5usize, 6, 3];

        assert_eq!(init(v, vec_list), result); 
    }
}
