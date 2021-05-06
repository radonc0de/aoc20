use std::fs;
use std::collections::HashMap;


fn main() {
    let mut rules: HashMap<u64, Vec<Vec<u64>>> = HashMap::new();
    
    let input = fs::read_to_string("input")
        .expect("Failed to read input");
    let lines = input.lines().collect::<Vec<&str>>();
    
    //reusing day 4 code here
    
    let mut rules_messages: Vec<Vec<&str>> = Vec::new();
    let mut entry = 0;
    
    rules_messages.push(Vec::new());

    for i in lines {
        if i == "" {
            entry += 1;
            rules_messages.push(Vec::new());
        }else{
            rules_messages[entry].push(i);
        }
    }   
    //println!("{:?}", rules_messages[0]); 
    for i in &rules_messages[0] {
        let index_rule = i.split(":").collect::<Vec<&str>>();
        //println!("{:?}", index_rule);
        //let index = parse index_rule[0] into u64 to use as hashmap key;
        let paths = index_rule[1].split("|").collect::<Vec<&str>>();
        for j in paths {
            let nums = j.split_whitespace().collect::<Vec<&str>>();
            println!("Options: {:?}", nums);
            
        }
    }
    let mut messages: Vec<&str> = Vec::new();
    for i in &rules_messages[1] {
        //println!("{}", i);        
        messages.push(i);
    }

    //println!("{:?}", messages);

    // break lines into rules vec and messages vec
    // make rules into something indexable and accessable everywhere

    //println!("Valid Messages: {}", message_handler(messages));
    
}
/*
fn message_handler(messagelist: Vec<&str>) -> u64 {
    let mut valid_messages = 0;
    for i in messagelist {
        if rule_tester(0, i)=='' {
            valid_messages += 1;
        }
    }
    valid_messages

}

fn rule_tester(rule: u64, message: &str) -> &str {
    //find direct subrules for $rule -> Vec<u64> 
    // run message tester with message for each one
    
    let mut new_message: &str;
    for i in subrules {
        new_message = message_tester(i);
    }

}*/
