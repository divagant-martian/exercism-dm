pub fn reply(message: &str) -> &str {
    let m = message.trim();

    if m.is_empty() {
        return "Fine. Be that way!";
    }

    let (mayus, has_stuff) = m.chars().fold((true, false), |acc, c| {
        (
            acc.0 && (!c.is_alphabetic() || c.is_uppercase()),
            acc.1 || c.is_alphabetic(),
        )
    });
    let is_question = m.ends_with('?');

    if mayus && has_stuff {
        if is_question {
            return "Calm down, I know what I'm doing!";
        }
        return "Whoa, chill out!";
    }
    if is_question {
        return "Sure.";
    }

    "Whatever."
}
