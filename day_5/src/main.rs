use std::fs;

fn main() {
    
    let input = fs::read_to_string("input")
        .expect("Failed to read input");
    let raw_passes = input.lines().collect::<Vec<&str>>();
    
    let mut passes: Vec<Vec<char>> = Vec::new();  
 
    for i in raw_passes {
        
        let pass: Vec<char> = i.chars().collect();
        passes.push(pass);
    }
    let mut largest = 0;
    for i in passes {
        let id = determine_id(i);
        if id > largest {
            largest = id;
        }
    }

    println!("The largest ID is: {}", largest);
}

fn determine_id (pass: Vec<char>) -> u64 {
    let mut min: u64 = 0;
    let mut max: u64 = 127;
    
    for i in 0..7 {
        let diff = max - min;
        let split = diff / 2;
        if pass[i] == 'F' {
            max = max - split - 1;
        } else if pass[i] == 'B' {
            min = min + split + 1;
        }
    }
    let mut row: u64 = 0;
    if pass[6] == 'F' {
        row = min;
    } else if pass[6] == 'B' {
        row = max;
    }
    
    min = 0;
    max = 7;
    
    for i in 7..10 {

        let diff = max - min;
        let split = diff / 2;
        if pass[i] == 'L' {
            max = max - split - 1;
        } else if pass[i] == 'R' {
            min = min + split + 1;
        }
    }
    let mut column: u64 = 0;
    if pass[9] == 'L' {
        column = min;
    } else if pass[9] == 'R' {
        column = max;
    }

    row * 8 + column

}
