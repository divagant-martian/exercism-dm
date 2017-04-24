// use std::collections::HashMap;

pub fn score(word:&str) -> i32{
    let mut r = 0;
    for ch in word.to_lowercase().chars() {
        r += value(ch);
    }
    r
}

fn value(x:char) -> i32 {
    match x {
        'a'|'e'|'i'|'o'|'u'|'l'|'n'|'r'|'s'|'t' => 1,
        'd'|'g' => 2,
        'b'|'c'|'m'|'p' => 3,
        'f'|'h'|'v'|'w'|'y' => 4,
        'k' => 5,
        'j'|'x' => 8,
        'q'|'z' => 10,
        _ => 0
    }
}
