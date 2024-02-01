use std::collections::HashMap;

fn main() {
    let s = String::from("bbbaaaba");
    let t = String::from("aaabbbba");
    let val = is_isomorphic(s,t);
    println!("{}", val);
}



pub fn is_isomorphic(s: String, t: String) -> bool {
    let (s,t) = (s.into_bytes(), t.into_bytes());
    let mut s_char_counts = HashMap::new();
    let mut t_char_counts = HashMap::new();

    for ch in 0..s.len() {
        let mut val = s_char_counts.entry(s[ch]).or_insert(t[ch]);
        if *val != t[ch] {
            return false
        }

        let mut val2 = t_char_counts.entry(t[ch]).or_insert(s[ch]);
        if *val2 != s[ch] {
            return false
        }
    }
    true
}



// pub fn is_isomorphic(s: String, t: String) -> bool {
//     let mut s_v: Vec<usize> = Vec::new();
//     let mut t_v: Vec<usize> = Vec::new();
//
//     let mut s_char_counts = HashMap::new();
//     let mut t_char_counts = HashMap::new();
//
//     for ch in s.chars() {
//         let count = s_char_counts.entry(ch).or_insert(0);
//         s_v.push(*count);
//         *count += 1;
//     }
//
//     for ch in t.chars() {
//         let count = t_char_counts.entry(ch).or_insert(0);
//         t_v.push(*count);
//         *count += 1;
//     }
//
//     println!("{:?}",s_v);
//     println!("{:?}",t_v);
//
//     if s_v == t_v {
//         true
//     } else {
//         false
//     }
// }