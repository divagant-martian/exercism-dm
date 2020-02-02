pub fn raindrops(n: u32) -> String {
    let mut rd = String::new();
    if n % 3 == 0 {
        rd.push_str("Pling");
    }
    if n % 5 == 0 {
        rd.push_str("Plang");
    }
    if n % 7 == 0 {
        rd.push_str("Plong");
    }
    if rd.is_empty() {
        rd = n.to_string();
    }
    rd
}
