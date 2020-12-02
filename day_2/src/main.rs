use std::fs;

fn main() {

    //turns puzzle input into vector of &str
    let input = fs::read_to_string("input")
        .expect("Could not find 'input'");
    let passwords = input.lines().collect::<Vec<&str>>();
    
    let validpasswords = solver(passwords);
    println!("Valid Passwords According to: ");
    println!("Sled Rental Place: {}", validpasswords.0);
    println!("the OTCAS: {}", validpasswords.1);
    
        
}

fn solver(passwords: Vec<&str>) -> (u64, u64) {
    let mut p1_validpass: u64 = 0;
    let mut p2_validpass: u64 = 0;
    
    for i in passwords {
        
        let split: Vec<&str> = i.split_whitespace().collect();
        let limits: Vec<&str> = split[0].split('-').collect();
        let a = limits[0].parse::<u64>().unwrap();
        let b = limits[1].parse::<u64>().unwrap();
        let letter = split[1].chars().next().unwrap();
        let mut letter_count: u64 = 0;
        let mut index = 0;
        let mut letter_matches = 0;
        for j in split[2].chars() {
            if j == letter {
                letter_count += 1;
            }
            index += 1;
            if (index == a || index == b) && j == letter {
                letter_matches += 1;
            }
        }
        if letter_count >= a && letter_count <= b {
            p1_validpass += 1;
        }
        if letter_matches == 1 {
            p2_validpass += 1;
        }
    }
    (p1_validpass, p2_validpass)
}
