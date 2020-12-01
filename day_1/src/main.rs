use std::fs;

fn main() {
    
    //turns puzzle input into vector of &str's    
    let input = fs::read_to_string("input")
        .expect("Failed");
    let stars = input.lines().collect::<Vec<&str>>();
    
    //gets and prints solutions
    println!("Answers:");
    let solutions = solver(stars);
    println!("Part 1: {}, Part 2: {}", solutions.0.unwrap(), solutions.1.unwrap());

}

fn solver(stars: Vec<&str>) -> (Option<u64>, Option<u64>){
    
    //initializes values to store answers in
    let mut part_1 : Option<u64> = None;
    let mut part_2 : Option<u64> = None;
    
    //index of for loops
    let mut index = [0, 0, 0];
    
    for i in &stars{
        for j in &stars{
            for k in &stars{
                //turn values being compared in &str vector into u64's
                let x : u64 = i.parse::<u64>().unwrap();
                let y : u64 = j.parse::<u64>().unwrap();  
                let z : u64 = k.parse::<u64>().unwrap();
                
                //check if values add to 2020, have diff indexes, and the answer hasn't been found yet
                if (x + y == 2020) && (index[0] != index[1]) && (part_1 == None) {
                    part_1 = Some(x*y);
                }   
                if (x + y + z == 2020) && (index[0] != index[1] && index[1] != index[2] && index[0] != index[2]) && (part_2 == None) {
                    part_2 = Some(x*y*z);
                }

                index[2] += 1;
            }
            index[1] += 1;
        }
        index[0] += 1;
    }
    
    //return Option enum containg answer
    (part_1, part_2)

}
