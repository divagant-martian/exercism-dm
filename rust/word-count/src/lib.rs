use std::collections::HashMap;
pub fn word_count(s:&str) ->  HashMap<String, u32> {
    let ss:String = s.to_lowercase().chars()
        .filter_map(|x|
                    if x.is_alphabetic() || x.is_whitespace() || x.is_numeric() {
                        Some(x)
                    }
                    else {
                        None
                    })
        .collect();
    ss.split_whitespace()
        .fold(HashMap::new(), |mut acc, w|{
            *acc.entry(w.to_string()).or_insert(0) += 1;           
            acc
        })
}
