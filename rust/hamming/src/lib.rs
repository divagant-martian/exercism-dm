pub fn hamming_distance(x:&str, y:&str) -> Result<i32,&'static str> {
    if x.len() != y.len() {Err("carajo")}
    else if x == y {Ok(0)}
    else{
        let mut xx = x.chars();
        let mut yy = y.chars();
        let mut r = 0;
        for i in 0..x.len(){
            if xx.next() != yy.next(){
                r += 1;
            }
        }
        Ok(r)
    }
}
