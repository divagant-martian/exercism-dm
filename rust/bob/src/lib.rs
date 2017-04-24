pub fn reply(say:&str) -> String {
    {
        if say.is_empty() { "Fine. Be that way!" }
        else if say.to_uppercase() == say { "Whoa, chill out!" }
        else if say.ends_with("?") { "Sure." }
        else { "Whatever." }
    }.to_string()
}
