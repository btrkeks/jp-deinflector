use crate::deinflection_rules::{DEINFLECTION_RULES, MAX_SUFFIX_LENGTH};

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

#[derive(Debug, Clone)]
pub struct DeinflectionRule {
    pub kana_out: &'static str,
    pub rules_in: &'static [RuleType],
    pub rules_out: &'static [RuleType],
}

impl DeinflectionRule {
    pub fn apply(&self, word: &str, suffix_len: usize) -> String {
        let stem = &word[..word.len() - suffix_len];
        format!("{}{}", stem, self.kana_out)
    }
}

pub fn deinflect_one_iteration(
    word: &str,
    current_rules: &[RuleType],
) -> Vec<(String, Vec<RuleType>)> {
    let mut results = Vec::new();
    let word_len = word.len();

    for suffix_len in (1..=MAX_SUFFIX_LENGTH).rev() {
        if suffix_len > word_len {
            continue;
        }

        let Some(suffix) = word.get(word_len - suffix_len..) else {
            continue;
        };

        // TODO:
        // Need some struct for word + rule types and then a function in DefinitionRule to check
        // if the rule can be applied to the word

        if let Some(rules) = DEINFLECTION_RULES.get(suffix) {
            for rule in rules.iter() {
                if current_rules.is_empty()
                    || rule.rules_in.iter().any(|r| current_rules.contains(r))
                {
                    let deinflected = rule.apply(word, suffix_len);
                    results.push((deinflected, rule.rules_out.to_vec()));
                }
            }
        }
    }

    results
}

// TODO: Add infinite loop detection
pub fn deinflect(word: &str) -> Vec<String> {
    let mut deinflections = deinflect_one_iteration(word, &[]);

    let mut i = 0;
    while i < deinflections.len() {
        let current_deinflection = &deinflections[i];
        let new_deinflections =
            deinflect_one_iteration(&current_deinflection.0, &current_deinflection.1);
        deinflections.extend(new_deinflections);

        i += 1;
    }

    deinflections.into_iter().map(|(s, _)| s.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_deinflects_to(input: &str, expected: &str) {
        let result = deinflect(input);
        let deinflected = result.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
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
        let deinflected: Vec<&str> = results.iter().map(|s| s.as_str()).collect();
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
            ("読む", vec![
                "読めば",
                "読まなければ",
                "読んだら",
                "読まなかったら",
            ]),
            ("食べる", vec![
                "食べれば",
                "食べなければ",
                "食べたら",
                "食べなかったら",
            ]),
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
}
