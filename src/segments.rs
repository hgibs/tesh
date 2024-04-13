use unicode_segmentation::{Graphemes, UnicodeSegmentation};

use crate::SelectionType;

/// External iterator for the segments remaining iterate over the next group of
/// [grapheme clusters](http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries). (i.e. type: SingleGrapheme or MultipleGraphemes)
#[derive(Debug, Clone)]
pub struct GraphemeSegments<'a> {
    string: &'a str,
    split: SelectionType,
}

impl<'a> Iterator for GraphemeSegments<'a> {
    type Item = &'a str;
    //fn size_hint(&self) -> (usize, Option<usize>) {

    fn next(&mut self) -> Option<&'a str> {
        todo!();
    }
}

impl<'a> GraphemeSegments<'a> {
    pub fn from_graphemes(g: Graphemes) -> GraphemeSegments {
        GraphemeSegments {
            string: g.as_str(),
            split: SelectionType::SingleGrapheme,
        }
    }

    pub fn new(string: &'a str, split: SelectionType) -> GraphemeSegments<'a> {
        GraphemeSegments {
            string: string,
            split: split,
        }
    }
}
