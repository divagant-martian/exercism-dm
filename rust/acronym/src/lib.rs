pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    for part in phrase.split_whitespace() {
        if part.ends_with(':') {
            acronym = part.to_string();
            acronym.pop();
            return acronym;
        }
        let (current, _) =
            part.chars()
                .fold((String::new(), true), |(mut acc, must_include), c| {
                    if c.is_ascii_alphabetic() {
                        if must_include || c.is_ascii_uppercase() {
                            acc.push(c.to_ascii_uppercase());
                            return (acc, false);
                        }
                        return (acc, false);
                    }
                    if c == '-' || c == '_' {
                        return (acc, true);
                    }
                    return (acc, must_include);
                });
        if current.len() == part.len() {
            acronym.push(current.chars().next().unwrap())
        } else {
            acronym += &current;
        }
    }
    acronym
}
