fn main() {
    let nums = vec![1,2,-2,-1];
    let result = three_sum(nums);
    println!("{:?}",result)
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut results = Vec::new();

    for (out_idx, &out_num) in nums.iter().enumerate() {
        for (middle_idx, &middle_num) in nums.iter().enumerate().skip(out_idx + 1) {
            for &in_num in nums.iter().skip(middle_idx + 1) {
                if out_num + middle_num + in_num == 0 {
                    results.push(vec![out_num, middle_num, in_num])
                }
            }
        }
    }
    for vec in &mut results {
        vec.sort();
    }
    results.sort();
    results.dedup();
    results
}