pub struct Roman { decimal:String }

impl Roman{
    pub fn from(n:u32) -> Roman { Roman { decimal:n.to_string() }}
    pub fn to_string(&self) -> String {
        self.decimal
            .chars()
            .fold((String::new(),self.decimal.len()),
                  |(mut acc, pos), dig|{
                      acc += &convert(pos,dig);
                      (acc, pos-1)
                  })
            .0
    }
}
// utils
fn convert(p:usize, d:char) -> String {
    if d == '0'|| p==0 { return "".to_string(); }
    let (c0,c1,c2) = match p {
        1 => ("I","V","X"),
        2 => ("X","L","C"),
        3 => ("C","D","M"),
        _ => ("M","","")    // not needed
    };
    if d <= '3' { c0.repeat(d.to_digit(10).unwrap() as usize) }
    else if d == '4' { c0.to_string() + c1 }
    else if d <= '8' {
        c1.to_string()+ &c0.repeat(d.to_digit(10).unwrap() as usize - 5)
    }
    else { c0.to_string() + c2 }
}
