fn main() {
    let prices = vec![841,636,646,45,682,90,959,900,165,460,994,396,744,959,964,732,165,124,604,576,478,270,871,767,557,107,623,8,313,937,478,888,233,603,859,923,563,760,496,61,976,448,174,587,693,134,113,222,881,396,805,813,350,590,529,981,385,881,17,263,305,744,729,805,721,693,736,520,34,251,18,836,543,232,531,577];
    let result = max_profit(prices);
    println!("{}", result)
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_temp= prices[0];    // 841
    let mut max_temp= prices[0];    // 841
    let mut min = prices[0];        // 841  636
    let mut result = 0;


    let mut max = 0;        // 841
    let mut prev_value = prices[0]; // 841


    for num in prices.iter() {
        if *num - prev_value < 0 {prev_value = *num;}
        if *num - prev_value > 0 && *num - prev_value > max {max = *num - prev_value }
    }

    return max
}