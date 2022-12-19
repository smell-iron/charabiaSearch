use jieba_rs::Jieba;
use once_cell::sync::Lazy;

use crate::segmenter::Segmenter;

/// Chinese Script specialized [`Segmenter`].
///
/// This Segmenter uses [`Jieba`] internally to segment the provided text
/// using HMM feature.
pub struct ChineseSegmenter;

impl Segmenter for ChineseSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
       // let segmented1 = to_segment.split("");  // 按字切割加入索引
//        let l1 = segmented1.len();
//         let l1 = 0
//        let segmented = JIEBA.cut(to_segment, false); // disable Hidden Markov Models. 按分词切割
//        let l2 = segmented.len();
//        let l = l1 + l2 + 1;
//        let arr = [str, l];
//         for index in 0..l1{
//             arr[index] = segmented1[index];
//         }
//         for index in l1..l-1{
//             arr[index] = segmented[index];
//         }
//         arr[l - 1] = to_segment; // 将其本身不经分割也加入索引
        let segmented = [to_segment];
//         segmented[0] = to_segment;
        Box::new(segmented.into_iter())
    }
}

static JIEBA: Lazy<Jieba> = Lazy::new(Jieba::new);

#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    // Original version of the text.
    const TEXT: &str =
        "人人生而自由﹐在尊嚴和權利上一律平等。他們賦有理性和良心﹐並應以兄弟關係的精神互相對待。";

    // Segmented version of the text.
    const SEGMENTED: &[&str] = &[
        "人人",
        "生而自由",
        "﹐",
        "在",
        "尊嚴",
        "和",
        "權利上",
        "一律平等",
        "。",
        "他們",
        "賦有",
        "理性",
        "和",
        "良心",
        "﹐",
        "並應",
        "以",
        "兄弟",
        "關",
        "係",
        "的",
        "精神",
        "互相",
        "對待",
        "。",
    ];

    // Segmented and normalized version of the text.
    const TOKENIZED: &[&str] = &[
        "人人",
        "生而自由",
        "﹐",
        "在",
        "尊严",
        "和",
        "权利上",
        "一律平等",
        "。",
        "他们",
        "赋有",
        "理性",
        "和",
        "良心",
        "﹐",
        "并应",
        "以",
        "兄弟",
        "关",
        "系",
        "的",
        "精神",
        "互相",
        "对待",
        "。",
    ];

    // Macro that run several tests on the Segmenter.
    test_segmenter!(ChineseSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Cj, Language::Cmn);
}
