use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.into_iter().flat_map(|(score, vec)|{
        vec.into_iter().map(|c| (c.to_ascii_lowercase(), *score)).collect::<Vec<_>>()
    }).collect()
}
