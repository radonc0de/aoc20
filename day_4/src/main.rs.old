use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input")
        .expect("Failed to read input.");
    let raw_passports = input.lines().collect::<Vec<&str>>();
    
    let mut passports: Vec<Vec<&str>> = Vec::new();
    let mut entry = 0;

    passports.push(Vec::new());

    for i in raw_passports {
        if i == "" {
            println!("--");
            entry += 1;
            passports.push(Vec::new());
        }else{
            passports[entry].push(i);
        }
    }
    
    let mut validpass = 0;

    for i in passports {
        //gen_passport(i);
        if check_passport(gen_passport(i)) {
            validpass += 1;
            println!("Valid one found");
        }
    }

    println!("Valid passports: {}", validpass);


}

fn gen_passport(lines: Vec<&str>) -> HashMap<&str, &str>{
    let mut passport: HashMap<&str, &str> = HashMap::new(); 
    for i in lines{
        let fields = i.split_whitespace().collect::<Vec<&str>>();
        for j in fields{
            let ind_field = j.split(":").collect::<Vec<&str>>();
            passport.insert(ind_field[0], ind_field[1]);
        }
    }
    passport

}

fn check_passport(passport: HashMap<&str, &str>) -> bool {
    let mut valid = true;
    match passport.get("byr") {
        Some(x) => println!("{}", x),
        None => valid = false,
        _ => println!("Oops. Just shid myself"),
    } 
    match passport.get("iyr") {
        Some(x) => println!("{}", x),
        None => valid = false,
        _ => println!("Oops. Just shid myself"),
    }
    match passport.get("eyr") {
        Some(x) => println!("{}", x),
        None => valid = false,
        _ => println!("Oops. Just shid myself"),
    }
    match passport.get("hgt") {
        Some(x) => println!("{}", x),
        None => valid = false,
        _ => println!("Oops. Just shid myself"),
    }
    match passport.get("hcl") {
        Some(x) => println!("{}", x),
        None => valid = false,
        _ => println!("Oops. Just shid myself"),
    }
    match passport.get("ecl") {
        Some(x) => println!("{}", x),
        None => valid = false,
        _ => println!("Oops. Just shid myself"),
    }
    match passport.get("pid") {
        Some(x) => println!("{}", x),
        None => valid = false,
        _ => println!("Oops. Just shid myself"),
    }

    valid

}
