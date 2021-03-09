use std::fs;
use std::collections::HashMap;

struct Bag {
    count: u32,
    color: String,
}

fn main(){ 
    let input = fs::read_to_string("input")
        .expect("Failed to read input.");
    let raw_rules = input.lines().collect::<Vec<&str>>();
    
    let (dictionary, index) = dictionary_builder(raw_rules);
    
    let mut counts = 0; //amount of outside bags that can hold shiny gold bags

    let mut i = 0;
    for j in &dictionary {
        if bag_check(i, &dictionary, &index) {
            counts += 1;
        }
        i += 1;
    }

    println!("The bags that can eventually hold a shiny gold bag is {}.", counts - 1);
    println!("Shiny gold bag must contain {} bags.", total_bags(*index.get("shiny gold").unwrap(), &dictionary, &index));

}

fn dictionary_builder(raw: Vec<&str>) -> (Vec<Vec<Bag>>, HashMap<&str, usize>) {
    let mut dictionary: Vec<Vec<Bag>> = Vec::new();
    let mut index = HashMap::new();
    
    let mut loop_index: usize = 0;
    for i in raw {
        let mut term: Vec<Bag> = Vec::new();

        let halves = i.split(" contain ").collect::<Vec<&str>>();
        //println!("{:?}", halves);
        
        index.insert(halves[0], loop_index);

        let definition = halves[1].split(", ").collect::<Vec<&str>>();
        //println!("{:?}", definition);

        for j in definition {
            let parts: Vec<&str> = j.split_whitespace().collect::<Vec<&str>>();
            
            let entry = Bag {
              count: parts[0].parse::<u32>().unwrap(),
              color: format!("{} {}", parts[1], parts[2]),
            };
            term.push(entry);
        }


        loop_index += 1;
        dictionary.push(term);
    }

    (dictionary, index)
    
}

fn bag_check(dic_index: usize, dictionary: &Vec<Vec<Bag>>, index: &HashMap<&str, usize>) -> bool {
    let shiny_gold = *index.get("shiny gold").unwrap();

    if dictionary[dic_index][0].color == "no other" {
        false
    }else if dic_index == shiny_gold {
        true
    }else{
        for i in &dictionary[dic_index]{
            let color: &str  = &i.color;
            let location = *index.get(&color).unwrap();
            let contains_shiny_gold = bag_check(location, &dictionary, &index); 
            if contains_shiny_gold {
                return true;
            }
        }
        return false;
    }
    
}

fn total_bags(dic_index: usize, dictionary: &Vec<Vec<Bag>>, index: &HashMap<&str, usize>) -> u32 {
    let mut bags: u32 = 0;
    for i in &dictionary[dic_index] {
        let color: &str  = &i.color;
        let count = &i.count;
        bags += count;
        if color != "no other" {
            let location = *index.get(&color).unwrap();
            bags +=  count * total_bags(location, &dictionary, &index);
        }
    }

    bags
}
