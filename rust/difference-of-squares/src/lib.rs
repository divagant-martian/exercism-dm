pub fn difference(n:i32) -> i32 {
    (n-1)*n*(n+1)*(3*n+2)/12
}

pub fn square_of_sum(n:i32) -> i32 {
    n.pow(2)*(n+1).pow(2)/4
}

pub fn sum_of_squares(n:i32) -> i32 {
    n*(n+1)*(2*n+1)/6
}
