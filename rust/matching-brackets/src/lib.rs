pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::<char>::new();
    string.chars().all(|c| match c {
        '{' => {
            stack.push('}');
            true
        }
        '[' => {
            stack.push(']');
            true
        }
        '(' => {
            stack.push(')');
            true
        }
        '}' | ']' | ')' => {
            if let Some(last) = stack.pop() {
                return last == c;
            }
            false
        }
        _ => true,
    }) && stack.is_empty()
}
