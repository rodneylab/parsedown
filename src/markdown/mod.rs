#[cfg(test)]
mod tests;

use deunicode::deunicode;

pub fn slugified_title(title: &str) -> String {
    let deunicoded_title = deunicode(title);
    let mut result = String::with_capacity(deunicoded_title.len());
    let mut last_was_replaced = true;
    let remove_characters = "?'`:[]()!";
    let replace_characters = " -/.,";
    for chars in deunicoded_title.chars() {
        if replace_characters.contains(chars) {
            if !last_was_replaced {
                last_was_replaced = true;
                result.push('-');
            }
        } else if !remove_characters.contains(chars) {
            last_was_replaced = false;
            result.push_str(&chars.to_lowercase().to_string());
        }
    }
    result
}
