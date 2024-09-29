use std::io;

fn main() {
    let value_a = read_value("a");
    let value_b = read_value("b");
    let value_c = read_value("c");

    let total: u32 = value_a + value_b;
    let greater_c = if total > value_c { true } else { false };

    println!("-------------------------");
    println!("sum of values a and b: {}", total);
    println!("sum is greater than c ? {}", greater_c);
}

fn read_value(option: &str) -> u32 {
    let mut value = String::new();
    println!("-------------------------");

    println!("Enter a number for {}", option);
    io::stdin()
        .read_line(&mut value)
        .expect("failed to read line");

    let value: u32 = match value.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("not valid number");
            return read_value(option);
        }
    };
    value
}
