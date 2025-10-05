use say_hello_lukas::{say_hello, say_goodbye};

fn main() {
    let response = say_hello("Lukas");
    println!("{}", response);

    let response = say_goodbye("Lukas");
    println!("{}", response);
}
