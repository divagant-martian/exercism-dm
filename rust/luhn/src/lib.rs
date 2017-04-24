pub fn is_valid(x:&str) -> bool {
    let xx = x.replace(" ","");
    if xx.len() < 2 || !xx.chars().all(|x| x.is_numeric()){
        return false;
    }
    let suma = xx.chars()
        .zip((0..xx.len()).rev())
        .fold(0,|acc, (y,z)|acc +
              if z%2 == 0{
                  y.to_digit(10).unwrap()
              }
              else {
                  let p = 2*y.to_digit(10).unwrap();
                  (p-p%10)/10 + p%10     
              }
        )%10;
    suma == 0
}
