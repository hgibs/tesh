pub enum TranslationTarget {
    UpperCase,
}

pub fn translate(source: String, target_type: TranslationTarget) -> String {
    match target_type {
        TranslationTarget::UpperCase => translate_uppercase(source),
    }
}

fn translate_uppercase(source: String) -> String {
    let cbytes = source.chars().into_iter().map(|x| x as u8);
    let tgt_bytes: Vec<u8> = cbytes.map(|x| x - 32).collect();
    let tgt_chars: Vec<char> = tgt_bytes.into_iter().map(|x| x as char).collect();

    tgt_chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_uppercase() {
        assert_eq!(
            translate_uppercase(String::from("abcdev")),
            String::from("ABCDEV")
        )
    }
}
