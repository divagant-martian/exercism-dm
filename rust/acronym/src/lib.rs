pub fn abbreviate(phrase: &str) -> String {
    for part in phrase.split_whitespace() {
        if part.ends_with(':') {
            // FIXME: quitar el :
            return part;
        }
    }
    unimplemented!("Given the phrase '{}', return its acronym", phrase);
}
