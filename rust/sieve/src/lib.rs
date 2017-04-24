pub fn primes_up_to(i:i32) -> Vec<i32>{
    (2..i+1).fold((2..i+1).collect::<Vec<i32>>(), |vec, x|{
        vec.into_iter().filter(|y| *y % x !=0 || *y==x ).collect()
    })
}

