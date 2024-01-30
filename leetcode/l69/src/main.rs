fn main() {
    let val = my_sqrt(8);
    println!("{}", val);
}

pub fn my_sqrt(x: i32) -> i32 {
    if x <= 1{
        return x;
    }
    for temp in 1..=x {
        let temp = temp as i64;
        if temp*temp > x as i64 {
            return temp as i32 -1;
        }
    }
    -1
}


