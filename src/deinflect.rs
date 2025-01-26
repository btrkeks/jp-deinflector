use crate::deinflect::RuleType::AdjI;
use crate::deinflection_rules::{DEINFLECTION_RULES, MAX_SUFFIX_LENGTH};
use fxhash::FxHashSet;

fn concatenate(a: &str, b: &str) -> String {
    let mut str = String::with_capacity(a.len() + b.len());
    str.push_str(a);
    str.push_str(b);
    str
}

/// The grammatical type of the word, which determines
/// the rules that can be applied for deinflection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleType {
    AdjI,
    Iru,
    V1,
    V5,
    Vk,
    Vs,
    Vz,
}

impl RuleType {
    pub fn is_verb(&self) -> bool {
        !AdjI.eq(self)
    }

    pub fn is_adjective(&self) -> bool {
        AdjI.eq(self)
    }
}

#[derive(Debug, Clone)]
pub struct DeinflectionRule {
    pub kana_out: &'static str,
    pub rules_in: &'static [RuleType],
    pub rules_out: &'static [RuleType],
}

impl DeinflectionRule {
    fn can_apply_to(&self, word: &DeinflectedWord) -> bool {
        word.get_types().is_empty()
                // If we know what the type might be, it should match
            || self.rules_in.iter().any(|r| word.types.contains(r))
    }

    pub fn apply(&self, deinflected_word: &DeinflectedWord, suffix_len: usize) -> Option<String> {
        if self.can_apply_to(deinflected_word) {
            let word = deinflected_word.get_word();
            debug_assert!(word.len() >= suffix_len);
            let stem = &word[..word.len() - suffix_len];
            Some(concatenate(stem, self.kana_out))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct DeinflectedWord {
    pub word: String,
    pub types: &'static [RuleType],
}

impl DeinflectedWord {
    pub fn new(word: String, types: &'static [RuleType]) -> Self {
        Self { word, types }
    }

    pub fn get_word(&self) -> &str {
        &self.word
    }

    pub fn get_types(&self) -> &'static [RuleType] {
        self.types
    }

    pub fn could_be_verb(&self) -> bool {
        self.types.iter().any(|t| t.is_verb())
    }

    pub fn could_be_adjective(&self) -> bool {
        self.types.iter().any(|t| t.is_adjective())
    }
}

/// Tracks words that have already been seen to avoid infinite loops
struct SeenWordsTracker {
    seen: FxHashSet<(usize, *const RuleType)>,
}

impl SeenWordsTracker {
    pub fn new() -> Self {
        Self {
            seen: FxHashSet::default(),
        }
    }

    /// Returns try if the word is
    pub fn check_is_new(&mut self, word: &DeinflectedWord) -> bool {
        let key = (
            fxhash::hash(word.word.as_bytes()),
            word.get_types().as_ptr(),
        );
        self.seen.insert(key)
    }
}

/// Returns an iterator over all suffixes of length <= MAX_SUFFIX_LENGTH of the word.
/// Assumes that the word only consists of Japanese characters.
/// (Otherwise there is nothing to deflect anyway, so it doesn't matter if this function breaks)
fn capped_suffixes(word: &str) -> impl Iterator<Item = &str> {
    let max_suffix_bytes = MAX_SUFFIX_LENGTH * 3; // each jap character is 3 bytes

    let start_pos = if word.len() > max_suffix_bytes {
        word.len() - max_suffix_bytes
    } else {
        0
    };

    (start_pos..word.len())
        .step_by(3)
        .filter_map(move |i| word.get(i..))
}

/// Performs a single deinflect operation, e.g.: 食べさせられたくなかった -> 食べさせられたくない
pub fn deinflect_one_iteration(deinflected_word: &DeinflectedWord) -> Vec<DeinflectedWord> {
    let mut results = Vec::new();

    for suffix in capped_suffixes(deinflected_word.get_word()) {
        if let Some(rules) = DEINFLECTION_RULES.get(suffix) {
            for rule in rules.into_iter() {
                if let Some(deinflected) = rule.apply(deinflected_word, suffix.len()) {
                    results.push(DeinflectedWord::new(deinflected, rule.rules_out));
                }
            }
        }
    }

    results
}

/// Returns a list of possible deinflections for the given word.
///
/// # Examples
/// ```
/// use jp_deinflector::deinflect;
/// let deinflections = deinflect("食べさせられなかった");
/// assert!(deinflections.iter().map(|s| s.get_word()).any(|w| w == "食べる"));
/// ```
pub fn deinflect(word: &str) -> Vec<DeinflectedWord> {
    let initial = DeinflectedWord::new(word.to_string(), &[]);
    let mut deinflections = deinflect_one_iteration(&initial);
    let mut seen_checker = SeenWordsTracker::new();

    let mut i = 0;
    while i < deinflections.len() {
        let current = &deinflections[i];
        if seen_checker.check_is_new(current) {
            let new_deinflections = deinflect_one_iteration(current);
            deinflections.extend(new_deinflections);
        }
        i += 1;
    }

    deinflections
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_deinflects_to(input: &str, expected: &str) {
        let result = deinflect(input);
        let deinflected = result.iter().map(|s| s.get_word()).collect::<Vec<&str>>();
        assert!(
            deinflected.contains(&expected),
            "Input '{}' did not deinflect to '{}'. Results: {:?}",
            input,
            expected,
            deinflected
        );
    }

    fn assert_does_not_deinflect_to(input: &str, not_expected: &str) {
        let results = deinflect(input);
        let deinflected: Vec<&str> = results.iter().map(|s| s.get_word()).collect();
        assert!(
            !deinflected.contains(&not_expected),
            "Input '{}' unexpectedly deinflected to '{}'. Results: {:?}",
            input,
            not_expected,
            deinflected
        );
    }

    #[test]
    fn test_deinflects_ichidan() {
        let cases = [
            "信じない",
            "信じます",
            "信じない",
            "信じます",
            "信じません",
            "信じた",
            "信じなかった",
            "信じました",
            "信じませんでした",
            "信じて",
            "信じなくて",
            "信じられる",
            "信じられない",
            "信じられない",
            "信じさせる",
            "信じさせない",
            "信じさせられる",
            "信じさせられない",
            "信じろ",
            "信じ",
            "信じよう",
            "信じられなかった",
        ];
        for case in cases {
            assert_deinflects_to(case, "信じる");
        }

        let cases = [
            "生きない",
            "生きます",
            "生きません",
            "生きた",
            "生きなかった",
            "生きました",
            "生きませんでした",
            "生きて",
            "生きなくて",
            "生きられる",
            "生きられない",
            "生きられない",
            "生きさせる",
            "生きさせない",
            "生きさせられる",
            "生きさせられない",
            "生きろ",
            "生き",
            "生きよう",
        ];
        for case in cases {
            assert_deinflects_to(case, "生きる");
        }
    }

    #[test]
    fn test_deinflects_godan_ru() {
        let cases = [
            "売らない",
            "売ります",
            "売りません",
            "売った",
            "売らなかった",
            "売りました",
            "売りませんでした",
            "売って",
            "売らなくて",
            "売れる",
            "売れない",
            "売られる",
            "売られない",
            "売らせる",
            "売らせない",
            "売らせられる",
            "売らせられない",
            "売れ",
            "売り",
            "売ろう",
        ];
        for case in cases {
            assert_deinflects_to(case, "売る");
        }
    }

    #[test]
    fn test_deinflects_godan_mu() {
        let cases = [
            "読まない",
            "読みます",
            "読みません",
            "読んだ",
            "読まなかった",
            "読みました",
            "読みませんでした",
            "読んで",
            "読まなくて",
            "読める",
            "読めない",
            "読まれる",
            "読まれない",
            "読ませる",
            "読ませない",
            "読ませられる",
            "読ませられない",
            "読め",
            "読み",
            "読もう",
        ];
        for case in cases {
            assert_deinflects_to(case, "読む");
        }
    }

    #[test]
    fn test_deinflects_godan_bu() {
        let cases = [
            "遊ばない",
            "遊びます",
            "遊びません",
            "遊んだ",
            "遊ばなかった",
            "遊びました",
            "遊びませんでした",
            "遊んで",
            "遊ばなくて",
            "遊べる",
            "遊べない",
            "遊ばれる",
            "遊ばれない",
            "遊ばせる",
            "遊ばせない",
            "遊ばせられる",
            "遊ばせられない",
            "遊べ",
            "遊び",
            "遊ぼう",
        ];
        for case in cases {
            assert_deinflects_to(case, "遊ぶ");
        }
    }

    #[test]
    fn test_deinflects_godan_tsu() {
        let cases = [
            "勝たない",
            "勝ちます",
            "勝ちません",
            "勝った",
            "勝たなかった",
            "勝ちました",
            "勝ちませんでした",
            "勝って",
            "勝たなくて",
            "勝てる",
            "勝てない",
            "勝たれる",
            "勝たれない",
            "勝たせる",
            "勝たせない",
            "勝たせられる",
            "勝たせられない",
            "勝て",
            "勝ち",
            "勝とう",
        ];
        for case in cases {
            assert_deinflects_to(case, "勝つ");
        }
    }

    #[test]
    fn test_deinflects_godan_su() {
        let cases = [
            "誑かさない",
            "誑かします",
            "誑かしません",
            "誑かした",
            "誑かさなかった",
            "誑かしました",
            "誑かしませんでした",
            "誑かして",
            "誑かさなくて",
            "誑かせる",
            "誑かせない",
            "誑かされる",
            "誑かされない",
            "誑かさせる",
            "誑かさせない",
            "誑かさせられる",
            "誑かさせられない",
            "誑かせ",
            "誑かし",
            "誑かそう",
        ];
        for case in cases {
            assert_deinflects_to(case, "誑かす");
        }
    }

    #[test]
    fn test_deinflects_godan_ku() {
        let cases = [
            "叩かない",
            "叩きます",
            "叩きません",
            "叩いた",
            "叩かなかった",
            "叩きました",
            "叩きませんでした",
            "叩いて",
            "叩かなくて",
            "叩ける",
            "叩けない",
            "叩かれる",
            "叩かれない",
            "叩かせる",
            "叩かせない",
            "叩かせられる",
            "叩かせられない",
            "叩け",
            "叩き",
            "叩こう",
        ];
        for case in cases {
            assert_deinflects_to(case, "叩く");
        }
    }

    #[test]
    fn test_deinflects_godan_gu() {
        let cases = [
            "泳がない",
            "泳ぎます",
            "泳ぎません",
            "泳いだ",
            "泳がなかった",
            "泳ぎました",
            "泳ぎませんでした",
            "泳いで",
            "泳がなくて",
            "泳げる",
            "泳げない",
            "泳がれる",
            "泳がれない",
            "泳がせる",
            "泳がせない",
            "泳がせられる",
            "泳がせられない",
            "泳げ",
            "泳ぎ",
            "泳ごう",
        ];
        for case in cases {
            assert_deinflects_to(case, "泳ぐ");
        }
    }

    #[test]
    fn test_deinflects_godan_u() {
        let cases = [
            "買わない",
            "買います",
            "買いません",
            "買った",
            "買わなかった",
            "買いました",
            "買いませんでした",
            "買って",
            "買わなくて",
            "買える",
            "買えない",
            "買われる",
            "買われない",
            "買わせる",
            "買わせない",
            "買わせられる",
            "買わせられない",
            "買え",
            "買い",
            "買おう",
        ];
        for case in cases {
            assert_deinflects_to(case, "買う");
        }
    }

    #[test]
    fn test_deinflects_negations() {
        let test_cases = [
            ("食べる", "食べず"),
            ("行く", "行かず"),
            ("知る", "知らん"),
            ("泣く", "泣かぬ"),
            ("信じる", "信じぬ"),
            // ("落ちる", "落ちまい"),
            // ("消える", "消えまい"),
            // ("知る", "知らせまい"),
            // ("食べる", "食べさせまい"),
            // ("言う", "言われまい"),
        ];
        for (expected, input) in test_cases {
            assert_deinflects_to(input, expected);
        }
    }

    #[test]
    fn test_deinflects_tai() {
        let test_cases = [
            ("遊ぶ", vec!["遊びたい", "遊びたかった", "遊びたければ"]),
            ("食べる", vec!["食べたい"]),
            ("旅行する", vec!["旅行したい"]),
            ("会う", vec!["会わせたい"]),
            ("寝る", vec!["寝させたい"]),
            ("思う", vec!["思われたい"]),
            ("褒める", vec!["褒められたい"]),
        ];
        for (expected, inputs) in test_cases {
            for input in inputs {
                assert_deinflects_to(input, expected);
            }
        }
    }

    #[test]
    fn test_deinflects_nagara() {
        let test_cases = [
            ("走る", "走りながら"),
            ("攪拌する", "攪拌しながら"),
            ("食べる", "食べながら"),
            ("書く", "書きながら"),
        ];
        for (expected, input) in test_cases {
            assert_deinflects_to(input, expected);
        }
    }

    #[test]
    fn test_deinflects_suru_and_compounds() {
        let suru_cases = [
            "しない",
            "します",
            "しません",
            "した",
            "しなかった",
            "しました",
            "しませんでした",
            "して",
            "しなくて",
            "される",
            "されない",
            "させる",
            "させない",
            "させられる",
            "させられない",
            "しろ",
            "し",
            "しよう",
        ];
        for case in suru_cases {
            assert_deinflects_to(case, "する");
        }

        let compound_cases = [
            "勉強しない",
            "勉強します",
            "勉強しません",
            "勉強した",
            "勉強しなかった",
            "勉強しました",
            "勉強しませんでした",
            "勉強して",
            "勉強しなくて",
            "勉強される",
            "勉強されない",
            "勉強させる",
            "勉強させない",
            "勉強させられる",
            "勉強させられない",
            "勉強しろ",
            "勉強し",
            "勉強しよう",
        ];
        for case in compound_cases {
            assert_deinflects_to(case, "勉強する");
        }
    }

    #[test]
    fn test_deinflects_conditional_forms() {
        let test_cases = [
            (
                "読む",
                vec!["読めば", "読まなければ", "読んだら", "読まなかったら"],
            ),
            (
                "食べる",
                vec!["食べれば", "食べなければ", "食べたら", "食べなかったら"],
            ),
        ];
        for (expected, inputs) in test_cases {
            for input in inputs {
                assert_deinflects_to(input, expected);
            }
        }
    }

    #[test]
    fn test_deinflects_adverb_forms() {
        assert_deinflects_to("早く", "早い");
    }

    #[test]
    fn test_deinflects_compound_verbs() {
        let tobi_cases = [
            "飛び込まない",
            "飛び込みます",
            "飛び込みません",
            "飛び込んだ",
            "飛び込まなかった",
            "飛び込みました",
            "飛び込みませんでした",
            "飛び込んで",
            "飛び込まなくて",
            "飛び込める",
            "飛び込めない",
        ];
        for case in tobi_cases {
            assert_deinflects_to(case, "飛び込む");
        }

        let kaki_cases = [
            "書き始めない",
            "書き始めます",
            "書き始めません",
            "書き始めた",
            "書き始めなかった",
            "書き始めました",
            "書き始めませんでした",
            "書き始めて",
            "書き始めなくて",
            "書き始められる",
            "書き始められない",
        ];
        for case in kaki_cases {
            assert_deinflects_to(case, "書き始める");
        }
    }

    #[test]
    fn test_wrong_deinflections() {
        assert_does_not_deinflect_to("白ける", "白い");
        assert_does_not_deinflect_to("とぼけた", "とぶ");
    }

    #[test]
    fn test_edge_cases() {
        // Empty string
        let deinflections = deinflect("");
        assert!(deinflections.is_empty());

        // Single character
        let deinflections = deinflect("あ");
        assert!(deinflections.is_empty());

        // Max length string
        let long_string = "あ".repeat(1000);
        assert!(deinflect(&long_string).is_empty());
    }

    #[test]
    fn test_chained_inflections() {
        assert_deinflects_to("食べさせられなかった", "食べる");
        assert_deinflects_to("走らされていました", "走る");
    }

    #[test]
    fn test_kuru() {
        let kuru_cases = [
            "来ない",
            "来ます",
            "来ません",
            "来た",
            "来なかった",
            "来ました",
            "来て",
            "来なくて",
            "来られない",
            "来られる",
            "来られない",
            "来させる",
            "来させない",
            "来させられる",
            "来させられない",
            "来い",
        ];
        for case in kuru_cases {
            assert_deinflects_to(case, "来る");
        }
    }

    #[test]
    fn test_get_suffixes() {
        let word = "走らされている";
        let suffixes: Vec<&str> = capped_suffixes(word).collect();
        assert_eq!(
            suffixes,
            vec![
                "走らされている",
                "らされている",
                "されている",
                "れている",
                "ている",
                "いる",
                "る"
            ]
        );

        let word = "食べさせられなかった";
        let suffixes: Vec<&str> = capped_suffixes(word).collect();
        assert_eq!(
            suffixes,
            vec![
                "せられなかった",
                "られなかった",
                "れなかった",
                "なかった",
                "かった",
                "った",
                "た"
            ]
        );
    }

    #[test]
    fn test_matsu() {
        let cases = [
            "待ちます",
            "待った",
            "待ちました",
            "待って",
            "待てる",
            "待たせる",
            "待たせられる",
            "待て",
            "待たない",
            "待ちません",
            "待たなかった",
            "待ちませんでした",
            "待たなくて",
            "待てない",
            "待たれない",
            "待たせない",
            "待たせられない",
            "待つな",
            "待てば",
            "待っちゃう",
            "待っちまう",
            "待ちなさい",
            "待ちそう",
            "待ちすぎる",
            "待ちたい",
            "待ったら",
            "待ったり",
            "待たず",
            "待たぬ",
            "待ち",
            "待ちましょう",
            "待とう",
            "待たされる",
            "待っとく",
            "待っている",
            "待っておる",
            "待ってる",
            "待ってしまう",
        ];

        for case in cases {
            assert_deinflects_to(case, "待つ");
        }
    }

    #[test]
    fn test_suru() {
        let cases = [
            "しない",
            "します",
            "しません",
            "した",
            "しなかった",
            "しました",
            "しませんでした",
            "して",
            "しなくて",
            "される",
            "されない",
            "させる",
            "させない",
            "させられる",
            "させられない",
            "しろ",
            "し",
            "しよう",
            "できる",
            "できない",
            "できます",
            "できません",
        ];

        for case in cases {
            assert_deinflects_to(case, "する");
        }
    }

    // jp-inflctions unfortunately produces wrong inflections, which is why this is unusable
    // Generate random "verbs" inflect them using 'jp-inflections', and test that they are deinflected correctly
    // proptest! {
    //     #[test]
    //     fn test_godan_inflection_deinflection_roundtrip(base_verb in "(([あ-ん]){1,5}(う|く|ぐ|す|つ|ぬ|ぶ|む|る))"
    //         .prop_filter("not_aru", |v| v != "ある")) {
    //         // Create a base verb
    //         let word = jp_inflections::Word::new(&base_verb, None);
    //         if !word.is_verb() { return Ok(()); }
    //
    //         let verb = match word.into_verb(jp_inflections::VerbType::Godan) {
    //             Ok(v) => v,
    //             Err(_) => return Ok(()),
    //         };
    //
    //         // Generate various inflections
    //         let inflected_forms = vec![
    //         ("dictionary long", verb.dictionary(jp_inflections::WordForm::Long).ok()),
    //
    //         // Negative forms
    //         ("negative short", verb.negative(jp_inflections::WordForm::Short).ok()),
    //         ("negative long", verb.negative(jp_inflections::WordForm::Long).ok()),
    //         ("negative past short", verb.negative_past(jp_inflections::WordForm::Short).ok()),
    //         ("negative past long", verb.negative_past(jp_inflections::WordForm::Long).ok()),
    //
    //         // Te forms
    //         ("te form", verb.te_form().ok()),
    //         ("negative te", verb.negative_te_form().ok()),
    //
    //         // Past forms
    //         ("past short", verb.past(jp_inflections::WordForm::Short).ok()),
    //         ("past long", verb.past(jp_inflections::WordForm::Long).ok()),
    //
    //         // Potential forms
    //         ("potential short", verb.potential(jp_inflections::WordForm::Short).ok()),
    //         ("potential long", verb.potential(jp_inflections::WordForm::Long).ok()),
    //         ("negative potential short", verb.negative_potential(jp_inflections::WordForm::Short).ok()),
    //         ("negative potential long", verb.negative_potential(jp_inflections::WordForm::Long).ok()),
    //
    //         // Passive forms
    //         ("passive", verb.passive().ok()),
    //         ("negative passive", verb.negative_passive().ok()),
    //
    //         // Causative forms
    //         ("causative", verb.causative().ok()),
    //         ("negative causative", verb.negative_causative().ok()),
    //
    //         // Causative-Passive forms
    //         // ("causative passive", verb.causative_passive().ok()),
    //         // ("negative causative passive", verb.negative_causative_passive().ok()),
    //
    //         // Imperative forms
    //         ("imperative", verb.imperative().ok()),
    //         ("negative imperative", verb.imperative_negative().ok()),
    //     ];
    //
    //     for (inflection_type, maybe_inflected) in inflected_forms {
    //         if let Some(inflected) = maybe_inflected {
    //             let deinflections = deinflect(&inflected.kana);
    //             prop_assert!(
    //                 deinflections.iter().map(|d| d.get_word()).any(|w| w == &base_verb),
    //                 "Failed to deinflect '{}' ({}) back to '{}'.\nAll deinflections: {:?}",
    //                 inflected.kana,
    //                 inflection_type,
    //                 base_verb,
    //                 deinflections
    //             );
    //         }
    //     }
    //     }
    // }
}
