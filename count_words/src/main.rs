use std::fs;
use std::env::args;
use std::collections::HashMap;
fn main() {
    if args().nth(1) == None {
        panic!("Program needs text file as second argument.");
    }
    let file = fs::read_to_string(args().nth(1).unwrap());
    let contents = match &file {
        Result::Ok(val) => val.trim().split_whitespace(),
        Result::Err(e) => panic!("not a file or without correct permissions: {:?}", e)
    };
    let mut words:HashMap<String, u32> = HashMap::new();
    for i in contents {
        let word_amount = match words.get(i) {
             Some(val) => val + 1,
             None => 1
        };
        words.insert(String::from(i), word_amount);
    }
    let mut max_words = vec![words.iter().next()];
    if max_words[0] == None {
        panic!("No words were in file");
    }

    for (index, i) in words.keys().enumerate() {
        if index == 0 {
            continue;
        }
    
        if max_words[0].unwrap().1 == words.get(i).unwrap() {

            max_words.push(Some((i, words.get(i).unwrap())))
        } else if max_words[0].unwrap().1 < words.get(i).unwrap() {

            max_words.clear();
            max_words.push(Some((i, words.get(i).unwrap())));

        }
    }
    for i in max_words {
        println!("{{{}}} has {{{}}} instances", i.unwrap().0, i.unwrap().1)
    }
}