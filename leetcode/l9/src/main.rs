
fn main() {
    let result = is_palindrome(4001);
    println!("{}",result);
}

pub fn is_palindrome(x: i32) -> bool {
    if x<0 {
        return false
    } else  {
        let temp = x.to_string();
        let num = temp.len()/2;
        let s d  &temp[..num];
        let back = &temp[temp.len()-num..];
        let t:String = back.chars().rev().collect();
        if s == t {
            return true
        } else {
            return false
        }
    }
}