use std::fs;
use std::collections::HashMap;

fn main() {

    let input = fs::read_to_string("input")
        .expect("Failed to read 'input'");
    let lines = input.lines().collect::<Vec<&str>>();

    println!("Sum: {}", bitmask_system(lines));
}

fn bitmask_system(input: Vec<&str>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut bitmask: [Option<bool>; 36] = [None; 36];
    for i in input {
        
        let j = i.split_whitespace().collect::<Vec<&str>>();
        if j[0] == "mask" {
            bitmask = bitmask_builder(j[2]);
            //bitmask_reader(&bitmask);
        }else{
            let index = j[0].split("[").nth(1).unwrap().split("]").next().unwrap().parse::<u64>().unwrap();
            let new_value = value_modifier(j[2].parse::<u64>().unwrap(), bitmask);
            //println!("Index: {}", index);
            //println!("{} -> {}", j[2], new_value); 
            memory.insert(index, new_value);
        }
    }
    let mut sum = 0;
    for (_j, k) in &memory { 
        sum += k;
    }
    sum
}

//bitmask string -> bitmask array
fn bitmask_builder(string: &str) -> [Option<bool>; 36] {
    let mut bitmask: [Option<bool>; 36] = [None; 36];
    let bits = string.chars().collect::<Vec<char>>();
    let mut j = 0;
    for i in bits {
        match i {
            '0' => bitmask[j] = Some(false), 
            '1' => bitmask[j] = Some(true),
            'X' => bitmask[j] = None,
             _  => println!("Sheeesh. Don't know what to do"),
        }
        j += 1;
    }

    bitmask

}

//reads bitmask arrays
fn bitmask_reader(bitmask: &[Option<bool>; 36]) {
    for i in 0..36 {
        print!("{}: ", i);
        match bitmask[i] {
            Some(true) => println!("1"),
            Some(false) => println!("0"),
            None => println!("X"),            
        }        
    }
}

//function takes decimals and bitmask and outputs new decimal value
fn value_modifier(val: u64, bitmask: [Option<bool>; 36]) -> u64 {
    let mut value = val;
    let mut bitvalue: [bool; 36] = [false; 36];
    for i in 0..36 {
        let exp = u64::pow(2, 35 - (i as u32));
        //println!("exp = {}", exp);
        if value >= exp {
            value = value - exp;
            bitvalue[i] = bitmask[i].unwrap_or(true);
        } else {
            bitvalue[i] = bitmask[i].unwrap_or(false);
        }
        //println!("value is equal to {}", value);
    }
    /*Testing
    println!("Displaying Binary Value");
    for i in 0..36 {
        print!("{}: ", i);
        println!("{}", bitvalue[i]);
    }
    */
    let mut decvalue = 0;
    for i in 0..36 {
        if bitvalue[i] {
            decvalue += 1 * u64::pow(2, 35 - (i as u32));
        } 
    }
    decvalue as u64
}
