fn main() {
    let strs= "Hello, world!";
    println!("{}", strs);

    let str_list: Vec<&str>= strs.split(",").collect();
    println!("{:?}", str_list);
}
