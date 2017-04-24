
pub fn raindrops(x:i32) -> String{
    let mut r = "".to_string();
    if x%3 == 0 {
        r += "Pling";
    }
    if x%5 == 0 {
        r += "Plang";
    }
    if x%7 == 0 {
        r += "Plong";
    }
    if r == ""{
        r = x.to_string()
    }
    r
}
