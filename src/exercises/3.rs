use std::io;

fn main() {
    let value_one = read_value();
    let value_two = read_value();

    if value_one == value_two {
        let total = value_one + value_two;
        println!("sum of values: {}", total)
    } else {
        let total = value_one * value_two;
        println!("multiplication of values: {}", total)
    }
}

fn read_value() -> u32 {
    let mut value = String::new();

    println!("enter a integer value");
    io::stdin().read_line(&mut value).expect("");
    println!("---------------------");

    let value: u32 = match value.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("enter a valid number");
            return read_value();
        }
    };
    value
}
