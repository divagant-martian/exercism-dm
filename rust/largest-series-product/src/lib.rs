use std::collections::VecDeque;


pub fn lsp(s:&str,i:usize) -> Result<u32,&'static str> {
    if s.chars().any(|x| !x.is_numeric()) || s.len() < i{
        return Err("bad request");
    }
    let mut window: VecDeque<u32> = VecDeque::with_capacity(i);
    let mut nums = s.chars();
    let mut lsp = 1;                // product identity
    for c in nums.by_ref().take(i){ // initial window
        let d = c.to_digit(10).unwrap();
        lsp *= d;
        window.push_front(d);
    }
    let mut maybe_lsp = lsp;
    for c in nums {
        let d = c.to_digit(10).unwrap();
        window.push_front(d);
        let last = window.pop_back().unwrap();
        maybe_lsp *= d;
        if last == 0 {
            maybe_lsp = window.iter().fold(1, |acc, x| acc*x);
        }
        else{
            maybe_lsp /= last;
        }
        if maybe_lsp > lsp {
            lsp = maybe_lsp;
        }        
    }
    Ok(lsp)
}
