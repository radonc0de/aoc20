use std::fs;
use std::collections::HashMap;

fn main() {

    let input = fs::read_to_string("input")
        .expect("Failed to read 'input'");
    let lines = input.lines().collect::<Vec<&str>>();
    
    let sums = bitmask_system(lines);
    println!("Sum for Part 1: {}, Sum for Part 2: {}", sums[0], sums[1]);
}

fn bitmask_system(input: Vec<&str>) -> [u64; 2] {
    let mut memory_pt1: HashMap<u64, u64> = HashMap::new();
    let mut memory_pt2: HashMap<u64, u64> = HashMap::new();
    let mut bitmask: [Option<bool>; 36] = [None; 36];
    for i in input {
        
        let j = i.split_whitespace().collect::<Vec<&str>>();
        if j[0] == "mask" {
            bitmask = bitmask_builder(j[2]);
            //bitmask_reader(&bitmask);
        }else{
            let index_pt1 = j[0].split("[").nth(1).unwrap().split("]").next().unwrap().parse::<u64>().unwrap();
            let value_pt1 = value_modifier(j[2].parse::<u64>().unwrap(), &bitmask);
            
            let indexes_pt2 = index_modifier(index_builder(j[0].split("[").nth(1).unwrap().split("]").next().unwrap().parse::<u64>().unwrap(), bitmask));
            let value_pt2 = j[2].parse::<u64>().unwrap();
            
            memory_pt1.insert(index_pt1, value_pt1);
            for i in indexes_pt2 {
                memory_pt2.insert(i, value_pt2);
            }
            //println!("Index: {}", index);
            //println!("{} -> {}", j[2], new_value); 
        }
    }
    let mut sum_pt1 = 0;
    for (_j, k) in &memory_pt1 { 
        sum_pt1 += k;
    }
    let mut sum_pt2 = 0;
    for (_j, k) in &memory_pt2 { 
        sum_pt2 += k;
    }
    [sum_pt1, sum_pt2]
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
fn value_modifier(val: u64, bitmask_ref: &[Option<bool>; 36]) -> u64 {
    let mut value = val;
    let bitmask = *bitmask_ref;
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

fn index_builder(mut value: u64, bitmask: [Option<bool>; 36]) -> [Option<bool>; 36] {
    let mut bitvalue: [Option<bool>; 36] = [Some(false); 36];
    for i in 0..36 {
        let exp = u64::pow(2, 35 - (i as u32));
        //println!("exp = {}", exp);
        if value >= exp {
            value = value - exp;
            match bitmask[i] {
                Some(true) => bitvalue[i] = Some(true),
                Some(false) => bitvalue[i] = Some(true),
                None => bitvalue[i] = None        
            }
        } else {
            match bitmask[i] {
                Some(true) => bitvalue[i] = Some(true),
                Some(false) => bitvalue[i] = Some(false),
                None => bitvalue[i] = None
            }
        }
    }
    bitvalue
}


fn index_modifier(index_temp: [Option<bool>; 36]) -> Vec<u64> {
    let mut indexes: Vec<u64> = Vec::new();
    let mut iter = 0;
    let mut unknowns = false;
    loop {
        if iter > 35 {
            break;
        }else if index_temp[iter] == None {
            unknowns = true;
            let mut with_zero: [Option<bool>; 36] = *&index_temp;
            with_zero[iter] = Some(false);
            let indexes_zero =  index_modifier(with_zero);
            let mut with_one: [Option<bool>; 36] = *&index_temp;
            with_one[iter] = Some(true);
            let indexes_one =  index_modifier(with_one);
            indexes = vector_add(indexes_zero, indexes_one); 
            break;
        }
        iter += 1;
    }
    if !unknowns {
        let mut decvalue = 0;
        for i in 0..36 {
            if index_temp[i].unwrap() {
                decvalue += 1 * u64::pow(2, 35 - (i as u32));
            } 
        }
        indexes.push(decvalue);
    }
    indexes
}

fn vector_add(vec2: Vec<u64>, vec3: Vec<u64>) -> Vec<u64> {
    let mut vec1 = vec2;
    for i in vec3 {
        vec1.push(i);
    }
    vec1
} 

