pub fn is_pangram(sent:&str) -> bool {
    let mut letters = "thequickbrownfoxjumpsoverthelazydog".chars(); // im as lazy as the dog
    letters.all(|x| sent.to_lowercase().contains(x))
}
