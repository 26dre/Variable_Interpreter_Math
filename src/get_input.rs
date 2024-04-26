pub fn get_input() -> String{
    let mut ret_str  = String::new(); 
    println! ("Enter input: ");
    std::io::stdin().read_line(&mut ret_str).unwrap();
    ret_str.truncate(ret_str.len() - 1);
    ret_str

}

pub fn get_input_continuously() {
    let mut str = String::new();
    println!("Input a string ");
    while str != ":q" {
        str = get_input();
    }
}
