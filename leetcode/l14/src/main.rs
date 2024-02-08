use std::collections::HashMap;

fn main() {
    let strs:Vec<String> = vec![String::from("flower"),String::from("flowr"),String::from("flight")];
    let result = longest_common_prefix(strs);
    println!("{}", result);
}

/*
    0 -> f
    1 -> l
    2 -> o
    3 -> w
    4 -> e
    5 -> r
 */
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut standard = HashMap::new();
    let first_str = &strs[0];

    for (index, str) in strs[0].chars().enumerate() {
        standard.entry(index).or_insert(str);
    }

    let mut first_length = first_str.len();

    for s in strs.iter().skip(1) {
        let mut current_length = 0;
        for (index, ch) in s.chars().enumerate() {
            if let Some(&expected_ch) = standard.get(&index) {
                println!("ex :{}, s : {}", expected_ch, s );
                if ch == expected_ch {
                    current_length += 1;
                } else {
                    println!("1 first : {}, current : {}", first_length, current_length);
                    first_length = first_length.min(current_length);
                    break;
                }
            } else {
                first_length = first_length.min(current_length);
                break;
            }
        }
        first_length = first_length.min(current_length);
    }
    first_str[..first_length].to_string()
}