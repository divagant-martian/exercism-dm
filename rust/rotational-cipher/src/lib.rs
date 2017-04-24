pub fn rotate(s:&str, i:u8) -> String{
    let a:u8 = 'a' as u8;
    let z:u8 = 'z' as u8;
    let am:u8 = 'A' as u8;
    let zm:u8 = 'Z' as u8;
    s.chars()
        .fold(String::new(),|mut acc, x|{
            if x >= 'a' && x <= 'z'{
                acc.push( (((x as u8) - a + i)%(z+1-a) + a) as char);
            }
            else if x >= 'A' && x <= 'Z'{
                acc.push( (((x as u8) - am + i)%(zm+1-am) + am) as char);
            }
            else{
                acc.push(x);
            }
            acc
        })
}
