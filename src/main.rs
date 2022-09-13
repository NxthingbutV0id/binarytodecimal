use std::io;

fn main() {
    println!("Please input a binary number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading lines");
    match u32::from_str_radix(input.trim(), 2) {
        Ok(bin_number) => {
            println!("Binary: {input}Decimal: {bin_number}");
        },
        Err(_) => {
            println!("Error, not a binary number.");
        }
    }
}
