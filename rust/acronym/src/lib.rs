pub fn abbreviate(s:&str) -> String {

    let ss :String = s.split(':').take(1).collect(); // PHP ... ¬¬
    if ss.to_uppercase().eq(&ss){
        return ss;
    }
    ss.chars()
        .fold((String::new(),' '), |(mut acc, last_char), x| {
            if last_char == ' '|| (x.is_uppercase() && last_char.is_lowercase()){
                acc.push(x.to_uppercase().collect::<Vec<_>>()[0]);
                return (acc,x);
            }
            if x=='-' {
                return (acc, ' ');
            }
            (acc, x)
        })
        .0
}
