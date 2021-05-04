use std::fs;

fn main() {
    
    let input = fs::read_to_string("input")
        .expect("Failed to read input");
    let lines = input.lines().collect::<Vec<&str>>();

    // break lines into rules vec and messages vec
    // make rules into something indexable and accessable everywhere

    println!("Valid Messages: {}", message_handler(messages));
    
}

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

}
