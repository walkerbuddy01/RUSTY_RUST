fn main() {
    let my_name = String::from("RUSTYBUDDY");

    let first_word = get_first_word(my_name);


    print!(" {} ", first_word);
} 

fn get_first_word(name: String) -> char {
    return name.chars().nth(0).unwrap();
}
