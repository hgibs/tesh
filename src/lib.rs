use unicode_segmentation::{Graphemes, UnicodeSegmentation};

mod segments;

#[derive(Debug, Copy, Clone)]
pub enum SelectionType {
    MultipleGraphemes,
    SingleGrapheme,
}

#[derive(Debug, Copy, Clone)]
pub enum TranspositionMethod {
    UpperCase,
    // SnakeCase,
}

#[derive(Debug, Copy, Clone)]
pub struct TranslationTarget {
    operate_on: SelectionType,
    transpose_to: TranspositionMethod,
}

// available targets
pub const UPPERCASE: TranslationTarget = TranslationTarget {
    operate_on: SelectionType::SingleGrapheme,
    transpose_to: TranspositionMethod::UpperCase,
};
pub const TITLECASE: TranslationTarget = TranslationTarget {
    operate_on: SelectionType::MultipleGraphemes,
    transpose_to: TranspositionMethod::UpperCase,
};

impl TranslationTarget {
    pub fn translate(&self, source: String) -> String {
        let gs = segments::GraphemeSegments::new(source.as_str(), self.operate_on);
        // match &self.operate_on {
        // SelectionType::MultipleGraphemes => todo!(),
        // // SelectionType::Segment => segments(source.as_str()),
        // SelectionType::SingleGrapheme => {
        //     segments::GraphemeSegments::from_graphemes(source.graphemes(true))
        // }

        return String::from("TO DO");
    }
}

// fn segments<'a>(query: &str) -> impl Iterator<Item = &'a str> {
//     String::from(query).graphemes(true).collect::<&str>()
// }

// #[must_use]
// pub fn translate(source: String, target_type: TranslationTarget) -> String {
//     let graphemes = source.graphemes(true);
//     let mut result = String::new();

//     for g in graphemes {
//         result.push_str(transpose_grapheme(g, &target_type).as_str());
//     }

//     result
// }

// fn select_first_byte(graph: &str) -> char {
//     let byte_list = graph.as_bytes();
//     byte_list[0] as char
// }

// fn transpose_grapheme(g: &str, target_type: &TranslationTarget) -> String {
//     print!("{}", g);
//     let converted_g: String = if g.len() == 1 {
//         // Basic Latin
//         let new_value: char = match target_type {
//             TranslationTarget::UpperCase => transpose_grapheme_to_uppercase(g),
//             // TranslationTarget::SnakeCase => transpose_grapheme_to_uppercase(g),
//             // TranslationTarget::UpperCase =>
//         };
//         new_value.to_string()
//     } else {
//         String::from(g)
//     };

//     println!(" -> {}", converted_g);

//     converted_g
// }

// fn transpose_grapheme_to_uppercase(g: &str) -> char {
//     let b = select_first_byte(g) as u8;

//     if 97 <= b && b <= 122 {
//         // Latin Uppercase
//         b.saturating_sub(32) as char
//     } else {
//         b as char
//     }
// }

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    // #[test]
    // fn test_uppercase() {
    //     static TGT = UPPERCASE;
    //     assert_eq!(

    //     )
    // }

    // #[test]
    // fn test_translate_uppercase() {
    //     static TGT: TranslationTarget = TranslationTarget::UpperCase;
    //     assert_eq!(
    //         translate(String::from("abcdev"), TGT),
    //         String::from("ABCDEV")
    //     );
    //     assert_eq!(
    //         translate(String::from("zyx🤣w123"), TGT),
    //         String::from("ZYX🤣W123")
    //     );
    // }
}
