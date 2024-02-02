fn main() {
    let s = String::from("0P");
    let a = is_palindrome(s);
    println!("{}",a);
}

pub fn is_palindrome(s: String) -> bool {
    // let re = Regex::new(r"[^a-zA-Z0-9\s]").unwrap();
    // let s = re.replace_all(&s, "").to_string();
    // let s = s.to_lowercase().replace(&['(', ')', ',', '\"', '.', ';', ':', '\'',' ','@','!','#','$','%','^','&','*','+','='][..], "");
    let mut temp = Vec::new();
    let char_vec: Vec<char> = s.chars().collect();
    for c in char_vec {
        if c.is_alphabetic()|c.is_numeric() {
            temp.push(c);
        } else{
            continue
        }
    }
    let s:String = temp.into_iter().collect();

    let s = s.to_lowercase();
    let num = s.len()/2;
    let front = &s[..num];
    let back = &s[s.len()-num..];
    let back_reverse:String = back.chars().rev().collect();
    println!("{}",front);
    println!("{}",back_reverse);
    if front == back_reverse {
        return true
    } else {
        return false
    }
}