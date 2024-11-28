#[cfg(test)]
mod tests;

use std::borrow::Cow;

pub fn format_line<'a, I: Into<Cow<'a, str>>>(line: I) -> Cow<'a, str> {
    let line = line.into();
    fn is_replace_character(c: char) -> bool {
        c == '\'' || c == '"'
    }

    let first = line.find(is_replace_character);
    if let Some(value) = first {
        let (mut result, rest) = match value {
            0 => match &line[0..1] {
                "\"" => (String::from('\u{201c}'), line[1..].chars()),
                "'" => (String::from('\u{2018}'), line[1..].chars()),
                _ => (String::from(&line[0..value]), line[value..].chars()),
            },
            _ => {
                if &line[(value - 1)..value] == " " {
                    (
                        String::from(&line[0..(value - 1)]),
                        line[(value - 1)..].chars(),
                    )
                } else {
                    (String::from(&line[0..value]), line[value..].chars())
                }
            }
        };
        result.reserve(line.len() - value);

        let mut preceded_by_space = false;
        for c in rest {
            match c {
                '\'' => {
                    if preceded_by_space {
                        preceded_by_space = false;
                        result.push('\u{2018}')
                    } else {
                        result.push('\u{2019}')
                    }
                }
                '"' => {
                    if preceded_by_space {
                        preceded_by_space = false;
                        result.push('\u{201c}')
                    } else {
                        result.push('\u{201d}')
                    }
                }
                ' ' => {
                    preceded_by_space = true;
                    result.push(c);
                }
                _ => {
                    preceded_by_space = false;
                    result.push(c);
                }
            }
        }
        Cow::Owned(result)
    } else {
        line
    }
}
