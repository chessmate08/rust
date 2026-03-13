fn main() {
    let test1 = "   I love Rust!   ";
    let test2 = "Hello World!   ";
    let test3 = "   ";
    let test4 = "";
    let test5 = "  🚀  ";
    let tests:[&str; 5] = [test1, test2, test3, test4, test5];
    for i in tests {
        println!("||{}||", trim_spaces(i))
    }
}
fn trim_spaces(str: &str) -> &str {
    let mut start = 0;
    for (index, i) in str.chars().enumerate() {
        if i != ' ' {
            start = index;
            break;
        }
        
    }
    let mut end = 0;
    for (index, i) in str.chars().rev().enumerate() {
        if i !=' ' {
            end = str.len() - index; break;
        }
        
    }
    &str[start..end]
}
