pub fn sum_of_multiples(x:i32, y:&Vec<i32>) -> i32 {
    (1..x).filter(
        |&i| y.iter().any(
            |j| i%j == 0)).sum::<i32>()
}
