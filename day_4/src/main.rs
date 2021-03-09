use std::fs;

fn main() {
    let input = fs::read_to_string("example")
        .expect("Failed to read input.");
    let raw_passports = input.lines().collect::<Vec<&str>>();

    for i in raw_passports {
        if (i == ""){
            println!("--");
        }else{
            println!("{}", i);
        }
    }
}
