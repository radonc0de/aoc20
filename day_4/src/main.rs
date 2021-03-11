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
        Some(x) =>  {
            let val = x.parse::<u32>().unwrap();
            if val < 1920 || val > 2002 {
                valid = false;
            }
        },
        None => valid = false,
    } 
    match passport.get("iyr") {
        Some(x) => {
            let val = x.parse::<u32>().unwrap();
            if val < 2010 || val > 2020 {
                valid = false;
            }
        },
        None => valid = false,
    }
    match passport.get("eyr") {
        Some(x) => { 
            let val = x.parse::<u32>().unwrap();
            if val < 2020 || val > 2030 {
                valid = false;
            }
        },
        None => valid = false,
    }
    match passport.get("hgt") {
        Some(x) => {
            if x.contains("cm"){
                let hgt = x.split("cm").collect::<Vec<&str>>();
                let hgt = hgt[0].parse::<u32>().unwrap();
                if hgt < 150 || hgt > 193 {
                    valid = false;
                }
            }else if x.contains("in"){
                let hgt = x.split("in").collect::<Vec<&str>>();
                let hgt = hgt[0].parse::<u32>().unwrap();
                if hgt < 59 || hgt > 76 {
                    valid = false;
                }
            }else{
                valid = false;
            } 
        },
        None => valid = false,
    }
    //above works
    match passport.get("hcl") {
        Some(x) => {
            let chars: Vec<char> = x.chars().collect();
            if chars.len() != 7 {
                valid = false;
            }
            for c in chars {
                if !c.is_digit(16) && c != '#' {
                    valid = false;
                }
            }
        },
        None => valid = false,
    }
    //below works
    match passport.get("ecl") {
        Some(x) => {
            if x != &"amb" && x != &"blu" && x != &"brn" && x != &"gry" && x != &"grn" && x != &"hzl" && x != &"oth"{
                valid = false;
            }
        },
        None => valid = false,
    }
    match passport.get("pid") {
        Some(x) => {
            match x.parse::<u64>() {
                Ok(y) => {
                    if y > 999999999 || y < 000000000 {
                        valid = false;
                    }
                },
                Err(_e) => valid = false,
            }
            let chars: Vec<char> = x.chars().collect();
            if chars.len() != 9 {
                valid = false;
            }
        },
        None => valid = false,
    }
    
    if !valid {
        println!("Invalid passport: {:?}", passport);
    }
    valid

}
