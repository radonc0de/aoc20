use std::fs;

fn main() {

    let input = fs::read_to_string("input")
        .expect("Failed to read 'input'");
    let instructions = input.split_whitespace().collect::<Vec<&str>>();

    println!("Accumulator Value: {}", solver(&instructions));
}

fn solver(tions: &Vec<&str>) -> i64 {

    let used_indexes: Vec<usize> = Vec::new(); 
    
    boot_code(tions, used_indexes, 0, 0)

}

fn boot_code(tions: &Vec<&str>, mut used_indexes: Vec<usize>, mut index: usize, cum: i64) -> i64 {
    //println!("");
    //println!("Index: {}", index);
    //println!("Command:{}.", tions[index]);
    if index >= tions.len()  {
        index = index % tions.len();
    }
    let mut used = false;
    for i in &used_indexes {
        if &index == i {
            used = true;
        }
    }
    if used {
        cum
    }else{
        let arg: i64 = tions[index + 1].parse::<i64>().unwrap();
        //println!("Argument: {}", arg);
        used_indexes.push(index);
        match tions[index] {
            "acc" => boot_code(tions, used_indexes, index + 2, cum + arg),
            "jmp" => boot_code(tions, used_indexes, (index as i64 + (2 * arg)) as usize, cum),
            "nop" => boot_code(tions, used_indexes, index + 2, cum),
            _   => 0,
            }
    }
    

}


