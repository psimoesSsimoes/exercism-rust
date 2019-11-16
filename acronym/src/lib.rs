pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    let mut flag = false;
    let mut last_char: char = ' ';

    for mut ch in phrase.chars() {
        if ch == '-' {
            ch = ' ';
        }

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
