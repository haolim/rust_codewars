mod kyu8 {
    pub mod square_sum;
}
fn main() {
    println!("{}",kyu8::square_sum::square_sum(vec![1,2]));
}

#[test]
fn returns_expected() {
    assert_eq!(kyu8::square_sum::square_sum(vec![1, 2]), 5);
    assert_eq!(kyu8::square_sum::square_sum(vec![-1, -2]), 5);
    assert_eq!(kyu8::square_sum::square_sum(vec![5, 3, 4]), 50);
    assert_eq!(kyu8::square_sum::square_sum(vec![]), 0);
}