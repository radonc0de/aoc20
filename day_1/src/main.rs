use std::fs;

fn main() {
    
    //turns puzzle input into vector of &str's    
    let input = fs::read_to_string("input")
        .expect("Failed");
    let stars = input.lines().collect::<Vec<&str>>();
    
    //index of for loops
    let mut index = [0, 0, 0];
    for i in &stars{
        for j in &stars{
            for k in &stars{
                //turn values being compared in array into u64's
                let x : u64 = i.parse::<u64>().unwrap();
                let y : u64 = j.parse::<u64>().unwrap();  
                let z : u64 = k.parse::<u64>().unwrap();
                //check if values add to 2020 and have diff indexes
                if (x + y + z == 2020) && (index[0] != index[1] && index[1] != index[2] && index[0] != index[2]) {

                    println!("Answer is: {}", x * y * z);
                
                }   
            index[2] += 1;
            }
        index[1] += 1;
        }
    index[0] += 1;
    }
}
