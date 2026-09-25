use std::fmt::Write;



pub fn build_proverb(list: &[&str]) -> String {
    
    let mut vec: Vec<String> = Vec::new();
    if list.is_empty() {
        return "".to_string();
    }
    let result: Vec<String> = list
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let mut f = String::new();
            if (i < list.len() - 1) {
                let next = i + 1;
                write!(f, "For want of a {} the {} was lost.\n", v, list[next]);
                f.to_string()
            } else {
                f.to_string()
            }
        }).collect();
    let first = list[0];
    let str = format!("And all for the want of a {first}.");
    result.concat() + &str
}

