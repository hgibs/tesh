use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Copy, Clone)]
pub enum TranslationTarget {
    UpperCase,
    SnakeCase,
}

pub fn select_target(query: String) -> TranslationTarget {
    match query.as_str() {
        "upper" => TranslationTarget::UpperCase,
        "snake" => TranslationTarget::SnakeCase,
        _ => TranslationTarget::UpperCase,
    }
}

#[must_use]
pub fn translate(source: String, target_type: TranslationTarget) -> String {
    let graphemes = source.graphemes(true);
    let mut result = String::new();

    for g in graphemes {
        result.push_str(transpose_grapheme(g, &target_type).as_str());
    }

    result
}

fn transpose_grapheme(g: &str, target_type: &TranslationTarget) -> String {
    let converted_g: String = if g.len() == 1 {
        // Basic Latin
        let byte_list = g.as_bytes();
        let first_byte: u8 = byte_list[0];
        print!("{}", first_byte as char);
        let new_value: char = match target_type {
            TranslationTarget::UpperCase => transpose_byte_to_uppercase(first_byte),
            TranslationTarget::SnakeCase => transpose_byte_to_uppercase(first_byte),
        };
        new_value.to_string()
    } else {
        print!("{}", g);
        String::from(g)
    };

    println!(" -> {}", converted_g);

    converted_g
}

fn transpose_byte_to_uppercase(b: u8) -> char {
    if 97 <= b && b <= 122 {
        // Latin Uppercase
        b.saturating_sub(32) as char
    } else {
        b as char
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_uppercase() {
        static TGT: TranslationTarget = TranslationTarget::UpperCase;
        assert_eq!(
            translate(String::from("abcdev"), TGT),
            String::from("ABCDEV")
        );
        assert_eq!(
            translate(String::from("zyx🤣w123"), TGT),
            String::from("ZYX🤣W123")
        );
    }
}
