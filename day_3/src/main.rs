use std::fs;

fn main() {

    //turns puzzle input into vector of &str
    let input = fs::read_to_string("input")
        .expect("Failed to read 'input'");
    let map = input.lines().collect::<Vec<&str>>();
    
    println!("{}", (solver(&map, 1, 1) * solver(&map, 3, 1) * solver(&map, 5, 1) * solver(&map, 7, 1) * solver(&map, 1, 2)));

}

fn solver(map_ref: &Vec<&str>, x_inc: usize, y_inc: usize) -> u64 {
    let t = 30; //vertical length of input - 1, was using for testing
    let map = &*map_ref;
    let mut y_pos = 0;
    let mut x_pos = 0;
    let mut index = 0;
    let mut trees: u64 = 0;
    for i in map {
        if (index != 0 && 
            if(y_inc == 2){
                index % 2 == 0
            }else{
                index % 1 == 0
            }) {
                let chars: Vec<char> = i.chars().collect();
                if(chars[x_pos] == '#'){
                    trees += 1;
                }
                else{
                }
       }if 
            if (y_inc == 2){
                index % 2 == 0
            }else{
                true
            }{     
                y_pos += y_inc;
                
                let standard = x_pos;
                for j in (t-(x_inc-1))..(t+1) {
                    if x_pos == j{
                        x_pos = x_pos + x_inc - 1 -t;
                    }
                }
                if (x_pos == standard){
                    x_pos += x_inc;
                }

        }
        index += 1;
    }
    trees
}
