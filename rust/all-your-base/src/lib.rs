pub fn convert(number: &[u32], b0: u32, b1: u32) -> Result<Vec<u32>, ()> {
    if number.iter().any(|&x| x>=b0) || b0 < 2 || b1 < 2 {return Err(());}
    Ok(div(
        number.iter().fold(
            (0,number.len()),|(n,i),xi|(n+b0.pow(i as u32 - 1)*xi,i-1)).0,
        b1,
        Vec::new()
    ))
}

fn div(n:u32, b1:u32, mut v:Vec<u32>) -> Vec<u32> {
    if n>0 {
        v.push(n%b1);
        div(n/b1, b1, v)
    }
    else{
        v.reverse();
        v
    }
}
