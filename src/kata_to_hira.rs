/// Converts all katakana characters contained in the input string `kata`
/// into their hiragana counterparts. All other characters are kept as they are.
pub fn kata_to_hira(kata: &str) -> String {
    kata.chars()
        .map(|c| {
            if is_katakana(c) {
                katakana_char_to_hira(c)
            } else {
                c
            }
        })
        .collect()
}

fn is_katakana(c: char) -> bool {
    let cp = c as u32;
    cp >= 0x30A1 && cp <= 0x30F6
}

fn katakana_char_to_hira(c: char) -> char {
    char::from_u32(c as u32 - 0x60).unwrap_or(c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_all_correct() {
        let kata = "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲンガギグゲゴザジズゼゾダヂヅデドバビブベボパピプペポ";
        let hira = "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをんがぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽ";
        assert_eq!(kata_to_hira(kata), hira);
    }

    proptest! {
        #[test]
        fn non_katakana_stay_unchanged(s in any::<String>()) {
            let converted = kata_to_hira(&s);

            for (input_char, output_char) in s.chars().zip(converted.chars()) {
                if !is_katakana(input_char) {
                    assert_eq!(
                        input_char, output_char,
                        "Non-katakana character changed: {:?} -> {:?}",
                        input_char, output_char
                    );
                }
            }
        }
    }
}
