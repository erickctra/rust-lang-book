use std::io;

fn main() {
    let mut value = String::new();

    let value = read_value(&mut value);

    if value % 2 == 0 {
        println!("the value is even");
    } else {
        println!("the value is odd");
    }

    if value > 0 {
        println!("the value is positive");
    } else {
        println!("the value is negative");
    }
}

fn read_value(value: &mut String) -> u32 {
    println!("enter any number");
    io::stdin().read_line(value).expect("enter a valid number");

    let value: u32 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return read_value(value);
        }
    };
    value
}
