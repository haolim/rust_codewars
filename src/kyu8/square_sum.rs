pub fn square_sum(vec: Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in vec {
        sum = sum + value*value;
    }
    sum
}
