pub fn encode(s:&str) -> String {
    s.to_lowercase()
        .chars()
        .fold((String::new(),0),|(mut acc,i), x| {
            if x>='a' && x<='z' { // ascii range = [97,122]
                acc.push((219u8 - (x as u8)) as char );
            }                  
            else if x>='0' && x<='9'{
                acc.push(x);
            }
            else{
                return (acc,i); // else, ignore character. Counter does not increase
            }
            if i == 4 {
                acc.push(' ');
            }
            return (acc,(i+1)%5);
        })
        .0
        .trim()
        .to_string()
}

pub fn decode(s:&str) -> String {
    s.chars()
        .fold(String::new(),
              |mut acc, x| {
                  if x>='a' && x<='z' { // ascii range = [97,122]
                      acc.push((219u8 - (x as u8)) as char );
                  }
                  else if x>='0' && x<='9'{
                      acc.push(x);            
                  }
                  return acc;
              }
        )
        .to_string()
}
