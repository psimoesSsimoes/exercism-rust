pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    let mut flag = false;
    let mut last_char: char = ' ';
    let p = phrase.replace("-", " ");
    for ch in p.chars() {
        if ch.is_whitespace() {
            flag = false;
        }

        if ch.is_alphabetic() && !flag
            || ch.is_alphabetic() && flag && ch.is_uppercase() && last_char.is_lowercase()
        {
            flag = true;
            result.push(ch.to_ascii_uppercase())
        }
        last_char = ch;
    }
    result
}
