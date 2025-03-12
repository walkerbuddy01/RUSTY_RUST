fn main() {
    let mut x: i16 = 89;
    // let age:u8 = 100;
    // let price:f32 = 1000.909;

    for i in 0..10 {
        x = x + 100;
        println!(" {} Hello, world! {} ", i, x);
    }

    let name: String = String::from("Hello World in the string");
    // let name1: &str    = "Hello World in the string";
    let char1: Option<char> = name.chars().nth(0);

    // Using unwrap is like we are saying to compiler that we are okay with run time exception
    println!(" {}  ", char1.unwrap());

    // print!(" {} Hello, world! {} ",age, x);
    // println!(" {} Hello, world! {} ",age, x);

    let is_even: bool = true;

    if is_even {
        println!("Yes the provided number is even");
    }
}
