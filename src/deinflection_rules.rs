use crate::deinflect::{DeinflectionRule, RuleType};

// This is maximum number of suffix lengths that we check against this list
pub const MAX_SUFFIX_LENGTH: usize = 7;

pub fn get_deinflection_rules(suffix: &str) -> Option<&'static [DeinflectionRule]> {
    hashify::tiny_map! {
        suffix.as_bytes(),
        "ければ" => &[
            DeinflectionRule {
                kana_out: "い",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
        ],
        "えば" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "けば" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "げば" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "せば" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "てば" => &[
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "ねば" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "べば" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "めば" => &[
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "れば" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V1, RuleType::V5, RuleType::Vk, RuleType::Vs, RuleType::Vz],
            },
        ],
        "ちゃう" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V1],
            },
        ],
        "いじゃう" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
        ],
        "いちゃう" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
        ],
        "しちゃう" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vs],
            },
        ],
        "っちゃう" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
        ],
        "んじゃう" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
        ],
        "じちゃう" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vz],
            },
        ],
        "為ちゃう" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vs],
            },
        ],
        "きちゃう" => &[
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vk],
            },
        ],
        "来ちゃう" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來ちゃう" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vk],
            },
        ],
        "ちまう" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V1],
            },
        ],
        "いじまう" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
        ],
        "いちまう" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
        ],
        "しちまう" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vs],
            },
        ],
        "っちまう" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
        ],
        "んじまう" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
        ],
        "じちまう" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vz],
            },
        ],
        "為ちまう" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vs],
            },
        ],
        "きちまう" => &[
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vk],
            },
        ],
        "来ちまう" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來ちまう" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vk],
            },
        ],
        "てしまう" => &[
            DeinflectionRule {
                kana_out: "て",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Iru],
            },
        ],
        "でしまう" => &[
            DeinflectionRule {
                kana_out: "で",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Iru],
            },
        ],
        "なさい" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "いなさい" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "きなさい" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "ぎなさい" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "しなさい" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "ちなさい" => &[
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "になさい" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "びなさい" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "みなさい" => &[
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "りなさい" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "じなさい" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[],
                rules_out: &[RuleType::Vz],
            },
        ],
        "為なさい" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "来なさい" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來なさい" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "そう" => &[
            DeinflectionRule {
                kana_out: "い",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "いそう" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "きそう" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "ぎそう" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "しそう" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "ちそう" => &[
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "にそう" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "びそう" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "みそう" => &[
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "りそう" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "じそう" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[],
                rules_out: &[RuleType::Vz],
            },
        ],
        "為そう" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "来そう" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來そう" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "すぎる" => &[
            DeinflectionRule {
                kana_out: "い",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::AdjI],
            },
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V1],
            },
        ],
        "いすぎる" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "きすぎる" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vk],
            },
        ],
        "ぎすぎる" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "しすぎる" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vs],
            },
        ],
        "ちすぎる" => &[
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "にすぎる" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
    "びすぎる" => &[
    DeinflectionRule {
    kana_out: "ぶ",
    rules_in: &[RuleType::V1],
    rules_out: &[RuleType::V5],
    },
    ],
    "みすぎる" => &[
    DeinflectionRule {
    kana_out: "む",
    rules_in: &[RuleType::V1],
    rules_out: &[RuleType::V5],
    },
    ],
    "りすぎる" => &[
    DeinflectionRule {
    kana_out: "る",
    rules_in: &[RuleType::V1],
    rules_out: &[RuleType::V5],
    },
    ],
    "じすぎる" => &[
    DeinflectionRule {
    kana_out: "ずる",
    rules_in: &[RuleType::V1],
    rules_out: &[RuleType::Vz],
    },
    ],
    "為すぎる" => &[
    DeinflectionRule {
    kana_out: "為る",
    rules_in: &[RuleType::V1],
    rules_out: &[RuleType::Vs],
    },
    ],
    "来すぎる" => &[
    DeinflectionRule {
    kana_out: "来る",
    rules_in: &[RuleType::V1],
    rules_out: &[RuleType::Vk],
    },
    ],
    "來すぎる" => &[
    DeinflectionRule {
    kana_out: "來る",
    rules_in: &[RuleType::V1],
    rules_out: &[RuleType::Vk],
    },
    ],
    "たい" => &[
    DeinflectionRule {
    kana_out: "る",
    rules_in: &[RuleType::AdjI],
    rules_out: &[RuleType::V1],
    },
    ],
    "いたい" => &[
    DeinflectionRule {
    kana_out: "う",
    rules_in: &[RuleType::AdjI],
    rules_out: &[RuleType::V5],
    },
    ],
    "きたい" => &[
    DeinflectionRule {
    kana_out: "く",
    rules_in: &[RuleType::AdjI],
    rules_out: &[RuleType::V5],
    },
            DeinflectionRule {
    kana_out: "くる",
    rules_in: &[RuleType::AdjI],
    rules_out: &[RuleType::Vk],
    },
    ],
    "ぎたい" => &[
    DeinflectionRule {
    kana_out: "ぐ",
    rules_in: &[RuleType::AdjI],
    rules_out: &[RuleType::V5],
    },
    ],
    "したい" => &[
    DeinflectionRule {
    kana_out: "す",
    rules_in: &[RuleType::AdjI],
    rules_out: &[RuleType::V5],
    },
            DeinflectionRule {
    kana_out: "する",
    rules_in: &[RuleType::AdjI],
    rules_out: &[RuleType::Vs],
    },
    ],
    "ちたい" => &[
    DeinflectionRule {
    kana_out: "つ",
    rules_in: &[RuleType::AdjI],
    rules_out: &[RuleType::V5],
    },
    ],
    "にたい" => &[
    DeinflectionRule {
    kana_out: "ぬ",
    rules_in: &[RuleType::AdjI],
    rules_out: &[RuleType::V5],
    },
    ],
    "びたい" => &[
    DeinflectionRule {
    kana_out: "ぶ",
    rules_in: &[RuleType::AdjI],
    rules_out: &[RuleType::V5],
    },
    ],
    "みたい" => &[
    DeinflectionRule {
    kana_out: "む",
    rules_in: &[RuleType::AdjI],
    rules_out: &[RuleType::V5],
    },
    ],
    "りたい" => &[
    DeinflectionRule {
    kana_out: "る",
    rules_in: &[RuleType::AdjI],
    rules_out: &[RuleType::V5],
    },
    ],
    "じたい" => &[
    DeinflectionRule {
    kana_out: "ずる",
    rules_in: &[RuleType::AdjI],
    rules_out: &[RuleType::Vz],
    },
    ],
    "為たい" => &[
    DeinflectionRule {
    kana_out: "為る",
    rules_in: &[RuleType::AdjI],
    rules_out: &[RuleType::Vs],
    },
    ],
    "来たい" => &[
    DeinflectionRule {
    kana_out: "来る",
    rules_in: &[RuleType::AdjI],
    rules_out: &[RuleType::Vk],
    },
    ],
    "來たい" => &[
    DeinflectionRule {
    kana_out: "來る",
    rules_in: &[RuleType::AdjI],
    rules_out: &[RuleType::Vk],
    },
    ],
    "かったら" => &[
    DeinflectionRule {
    kana_out: "い",
    rules_in: &[],
    rules_out: &[RuleType::AdjI],
    },
    ],
    "たら" => &[
    DeinflectionRule {
    kana_out: "る",
    rules_in: &[],
    rules_out: &[RuleType::V1],
    },
    ],
    "いたら" => &[
    DeinflectionRule {
    kana_out: "く",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "いだら" => &[
    DeinflectionRule {
    kana_out: "ぐ",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "したら" => &[
    DeinflectionRule {
    kana_out: "す",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
            DeinflectionRule {
    kana_out: "する",
    rules_in: &[],
    rules_out: &[RuleType::Vs],
    },
    ],
    "ったら" => &[
    DeinflectionRule {
    kana_out: "う",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    DeinflectionRule {
    kana_out: "つ",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    DeinflectionRule {
    kana_out: "る",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "んだら" => &[
    DeinflectionRule {
    kana_out: "ぬ",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    DeinflectionRule {
    kana_out: "ぶ",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    DeinflectionRule {
    kana_out: "む",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "じたら" => &[
    DeinflectionRule {
    kana_out: "ずる",
    rules_in: &[],
    rules_out: &[RuleType::Vz],
    },
    ],
    "為たら" => &[
    DeinflectionRule {
    kana_out: "為る",
    rules_in: &[],
    rules_out: &[RuleType::Vs],
    },
    ],
    "きたら" => &[
    DeinflectionRule {
    kana_out: "くる",
    rules_in: &[],
    rules_out: &[RuleType::Vk],
    },
    ],
    "来たら" => &[
    DeinflectionRule {
    kana_out: "来る",
    rules_in: &[],
    rules_out: &[RuleType::Vk],
    },
    ],
    "來たら" => &[
    DeinflectionRule {
    kana_out: "來る",
    rules_in: &[],
    rules_out: &[RuleType::Vk],
    },
    ],
    "いったら" => &[
    DeinflectionRule {
    kana_out: "いく",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "おうたら" => &[
    DeinflectionRule {
    kana_out: "おう",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "こうたら" => &[
    DeinflectionRule {
    kana_out: "こう",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "そうたら" => &[
    DeinflectionRule {
    kana_out: "そう",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "とうたら" => &[
    DeinflectionRule {
    kana_out: "とう",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "行ったら" => &[
    DeinflectionRule {
    kana_out: "行く",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "逝ったら" => &[
    DeinflectionRule {
    kana_out: "逝く",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "往ったら" => &[
    DeinflectionRule {
    kana_out: "往く",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "請うたら" => &[
    DeinflectionRule {
    kana_out: "請う",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "乞うたら" => &[
    DeinflectionRule {
    kana_out: "乞う",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "恋うたら" => &[
    DeinflectionRule {
    kana_out: "恋う",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "問うたら" => &[
    DeinflectionRule {
    kana_out: "問う",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "負うたら" => &[
    DeinflectionRule {
    kana_out: "負う",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "沿うたら" => &[
    DeinflectionRule {
    kana_out: "沿う",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "添うたら" => &[
    DeinflectionRule {
    kana_out: "添う",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "副うたら" => &[
    DeinflectionRule {
    kana_out: "副う",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "厭うたら" => &[
    DeinflectionRule {
    kana_out: "厭う",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "のたもうたら" => &[
    DeinflectionRule {
    kana_out: "のたまう",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "かったり" => &[
    DeinflectionRule {
    kana_out: "い",
    rules_in: &[],
    rules_out: &[RuleType::AdjI],
    },
    ],
    "たり" => &[
    DeinflectionRule {
    kana_out: "る",
    rules_in: &[],
    rules_out: &[RuleType::V1],
    },
    ],
    "いたり" => &[
    DeinflectionRule {
    kana_out: "く",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "いだり" => &[
    DeinflectionRule {
    kana_out: "ぐ",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "したり" => &[
    DeinflectionRule {
    kana_out: "す",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
            DeinflectionRule {
    kana_out: "する",
    rules_in: &[],
    rules_out: &[RuleType::Vs],
    },
    ],
    "ったり" => &[
    DeinflectionRule {
    kana_out: "う",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    DeinflectionRule {
    kana_out: "つ",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    DeinflectionRule {
    kana_out: "る",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "んだり" => &[
    DeinflectionRule {
    kana_out: "ぬ",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    DeinflectionRule {
    kana_out: "ぶ",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    DeinflectionRule {
    kana_out: "む",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "じたり" => &[
    DeinflectionRule {
    kana_out: "ずる",
    rules_in: &[],
    rules_out: &[RuleType::Vz],
    },
    ],
    "為たり" => &[
    DeinflectionRule {
    kana_out: "為る",
    rules_in: &[],
    rules_out: &[RuleType::Vs],
    },
    ],
    "きたり" => &[
    DeinflectionRule {
    kana_out: "くる",
    rules_in: &[],
    rules_out: &[RuleType::Vk],
    },
    ],
    "来たり" => &[
    DeinflectionRule {
    kana_out: "来る",
    rules_in: &[],
    rules_out: &[RuleType::Vk],
    },
    ],
    "來たり" => &[
    DeinflectionRule {
    kana_out: "來る",
    rules_in: &[],
    rules_out: &[RuleType::Vk],
    },
    ],
    "いったり" => &[
    DeinflectionRule {
    kana_out: "いく",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "おうたり" => &[
    DeinflectionRule {
    kana_out: "おう",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "こうたり" => &[
    DeinflectionRule {
    kana_out: "こう",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "そうたり" => &[
    DeinflectionRule {
    kana_out: "そう",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "とうたり" => &[
    DeinflectionRule {
    kana_out: "とう",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "行ったり" => &[
    DeinflectionRule {
    kana_out: "行く",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "逝ったり" => &[
    DeinflectionRule {
    kana_out: "逝く",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "往ったり" => &[
    DeinflectionRule {
    kana_out: "往く",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "請うたり" => &[
    DeinflectionRule {
    kana_out: "請う",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
    "乞うたり" => &[
    DeinflectionRule {
    kana_out: "乞う",
    rules_in: &[],
    rules_out: &[RuleType::V5],
    },
    ],
        "恋うたり" => &[
            DeinflectionRule {
                kana_out: "恋う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "問うたり" => &[
            DeinflectionRule {
                kana_out: "問う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "負うたり" => &[
            DeinflectionRule {
                kana_out: "負う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "沿うたり" => &[
            DeinflectionRule {
                kana_out: "沿う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "添うたり" => &[
            DeinflectionRule {
                kana_out: "添う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "副うたり" => &[
            DeinflectionRule {
                kana_out: "副う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "厭うたり" => &[
            DeinflectionRule {
                kana_out: "厭う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "のたもうたり" => &[
            DeinflectionRule {
                kana_out: "のたまう",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "くて" => &[
            DeinflectionRule {
                kana_out: "い",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::AdjI],
            },
        ],
        "て" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V1],
            },
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "てる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "いて" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "いで" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "して" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::Vs],
            },
        ],
        "って" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "んで" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "じて" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::Vz],
            },
        ],
        "為て" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::Vs],
            },
        ],
        "きて" => &[
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::Vk],
            },
        ],
        "来て" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來て" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::Vk],
            },
        ],
        "いって" => &[
            DeinflectionRule {
                kana_out: "いく",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "おうて" => &[
            DeinflectionRule {
                kana_out: "おう",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "こうて" => &[
            DeinflectionRule {
                kana_out: "こう",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "そうて" => &[
            DeinflectionRule {
                kana_out: "そう",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "とうて" => &[
            DeinflectionRule {
                kana_out: "とう",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "行って" => &[
            DeinflectionRule {
                kana_out: "行く",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "逝って" => &[
            DeinflectionRule {
                kana_out: "逝く",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "往って" => &[
            DeinflectionRule {
                kana_out: "往く",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "請うて" => &[
            DeinflectionRule {
                kana_out: "請う",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "乞うて" => &[
            DeinflectionRule {
                kana_out: "乞う",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "恋うて" => &[
            DeinflectionRule {
                kana_out: "恋う",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "問うて" => &[
            DeinflectionRule {
                kana_out: "問う",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "負うて" => &[
            DeinflectionRule {
                kana_out: "負う",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "沿うて" => &[
            DeinflectionRule {
                kana_out: "沿う",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "添うて" => &[
            DeinflectionRule {
                kana_out: "添う",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "副うて" => &[
            DeinflectionRule {
                kana_out: "副う",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "厭うて" => &[
            DeinflectionRule {
                kana_out: "厭う",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "のたもうて" => &[
            DeinflectionRule {
                kana_out: "のたまう",
                rules_in: &[RuleType::Iru],
                rules_out: &[RuleType::V5],
            },
        ],
        "ず" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "かず" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "がず" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "さず" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "たず" => &[
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "なず" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "ばず" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "まず" => &[
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "らず" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "わず" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "ぜず" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[],
                rules_out: &[RuleType::Vz],
            },
        ],
        "せず" => &[
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "為ず" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "こず" => &[
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "来ず" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來ず" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "ぬ" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "かぬ" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "がぬ" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "さぬ" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "たぬ" => &[
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "なぬ" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "ばぬ" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "まぬ" => &[
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "らぬ" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "わぬ" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "ぜぬ" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[],
                rules_out: &[RuleType::Vz],
            },
        ],
        "せぬ" => &[
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "為ぬ" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "こぬ" => &[
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "来ぬ" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來ぬ" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "く" => &[
            DeinflectionRule {
                kana_out: "い",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
        ],
        "させる" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V1],
            },
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vs],
            },
        ],
        "かせる" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "がせる" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "たせる" => &[
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "なせる" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "ばせる" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "ませる" => &[
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "らせる" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "わせる" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
                },
        ],
        "じさせる" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vz],
            },
        ],
        "ぜさせる" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vz],
            },
        ],
        "為せる" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vs],
            },
        ],
        "せさせる" => &[
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vs],
            },
        ],
        "為させる" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vs],
            },
        ],
        "こさせる" => &[
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vk],
            },
        ],
        "来させる" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來させる" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vk],
            },
        ],
        "ろ" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "よ" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "え" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "える",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "け" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "ける",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "げ" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "げる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "せ" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "せる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "ね" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "ねる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "べ" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "べる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "め" => &[
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "める",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "れ" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "れる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "じろ" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[],
                rules_out: &[RuleType::Vz],
            },
        ],
        "ぜよ" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[],
                rules_out: &[RuleType::Vz],
            },
        ],
        "しろ" => &[
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "せよ" => &[
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "為ろ" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "為よ" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "こい" => &[
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "来い" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來い" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "な" => &[
            DeinflectionRule {
                kana_out: "",
                rules_in: &[],
                rules_out: &[RuleType::V1, RuleType::V5, RuleType::Vk, RuleType::Vs, RuleType::Vz],
            },
        ],
        "い" => &[
            DeinflectionRule {
                kana_out: "いる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "き" => &[
            DeinflectionRule {
                kana_out: "きる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
            DeinflectionRule {
                kana_out: "い",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
        ],
        "ぎ" => &[
            DeinflectionRule {
                kana_out: "ぎる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "じ" => &[
            DeinflectionRule {
                kana_out: "じる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "ぜ" => &[
            DeinflectionRule {
                kana_out: "ぜる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "ち" => &[
            DeinflectionRule {
                kana_out: "ちる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "で" => &[
            DeinflectionRule {
                kana_out: "でる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "に" => &[
            DeinflectionRule {
                kana_out: "にる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "ひ" => &[
            DeinflectionRule {
                kana_out: "ひる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "び" => &[
            DeinflectionRule {
                kana_out: "びる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "へ" => &[
            DeinflectionRule {
                kana_out: "へる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "み" => &[
            DeinflectionRule {
                kana_out: "みる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "り" => &[
            DeinflectionRule {
                kana_out: "りる",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "し" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            }
        ],
        "来" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "くない" => &[
            DeinflectionRule {
                kana_out: "い",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::AdjI],
            },
        ],
        "ない" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::V1],
            },
        ],
        "かない" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::V5],
            },
        ],
        "がない" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::V5],
            },
        ],
        "さない" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::V5],
            },
        ],
        "たない" => &[
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::V5],
            },
        ],
        "なない" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::V5],
            },
        ],
        "ばない" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::V5],
            },
        ],
        "まない" => &[
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::V5],
            },
        ],
        "らない" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::V5],
            },
        ],
        "わない" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::V5],
            },
        ],
        "じない" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::Vz],
            },
        ],
        "しない" => &[
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::Vs],
            },
        ],
        "為ない" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::Vs],
            },
        ],
        "こない" => &[
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::Vk],
            },
        ],
        "来ない" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來ない" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[RuleType::AdjI],
                rules_out: &[RuleType::Vk],
            },
        ],
        "さ" => &[
            DeinflectionRule {
                kana_out: "い",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
        ],
        "かれる" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "がれる" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "される" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vs],
            },
        ],
        "たれる" => &[
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "なれる" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "ばれる" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "まれる" => &[
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "われる" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
                },
        ],
        "られる" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5, RuleType::V1],
            }
        ],
        "じされる" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vz],
            },
        ],
            "ぜされる" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vz],
            },
        ],
        "為れる" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vs],
            },
        ],
        "こられる" => &[
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vk],
            },
        ],
         "来られる" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vk],
            },
        ],
         "來られる" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vk],
            },
        ],
        "かった" => &[
            DeinflectionRule {
                kana_out: "い",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
        ],
        "た" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "いた" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
         "いだ" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "した" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "った" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "んだ" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "じた" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[],
                rules_out: &[RuleType::Vz],
            },
        ],
          "為た" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
         "きた" => &[
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "来た" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來た" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "いった" => &[
            DeinflectionRule {
                kana_out: "いく",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "おうた" => &[
            DeinflectionRule {
                kana_out: "おう",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "こうた" => &[
            DeinflectionRule {
                kana_out: "こう",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "そうた" => &[
            DeinflectionRule {
                kana_out: "そう",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "とうた" => &[
            DeinflectionRule {
                kana_out: "とう",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "行った" => &[
            DeinflectionRule {
                kana_out: "行く",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "逝った" => &[
            DeinflectionRule {
                kana_out: "逝く",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "往った" => &[
            DeinflectionRule {
                kana_out: "往く",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
          "請うた" => &[
            DeinflectionRule {
                kana_out: "請う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "乞うた" => &[
            DeinflectionRule {
                kana_out: "乞う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "恋うた" => &[
            DeinflectionRule {
                kana_out: "恋う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "問うた" => &[
            DeinflectionRule {
                kana_out: "問う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "負うた" => &[
            DeinflectionRule {
                kana_out: "負う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "沿うた" => &[
            DeinflectionRule {
                kana_out: "沿う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "添うた" => &[
            DeinflectionRule {
                kana_out: "添う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "副うた" => &[
            DeinflectionRule {
                kana_out: "副う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "厭うた" => &[
            DeinflectionRule {
                kana_out: "厭う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
            "のたもうた" => &[
            DeinflectionRule {
                kana_out: "のたまう",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
       "ます" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "います" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "きます" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
          "ぎます" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "します" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
             DeinflectionRule {
                kana_out: "する",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "ちます" => &[
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "にます" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "びます" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "みます" => &[
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "ります" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "じます" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[],
                rules_out: &[RuleType::Vz],
            },
        ],
        "為ます" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "来ます" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來ます" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "くありません" => &[
            DeinflectionRule {
                kana_out: "い",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
        ],
        "ません" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                 rules_out: &[RuleType::V1],
            },
        ],
        "いません" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "きません" => &[
            DeinflectionRule {
                kana_out: "く",
                 rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
         "ぎません" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "しません" => &[
            DeinflectionRule {
                kana_out: "す",
                 rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "する",
                 rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "ちません" => &[
            DeinflectionRule {
                kana_out: "つ",
                 rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "にません" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "びません" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                 rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "みません" => &[
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
         "りません" => &[
            DeinflectionRule {
                kana_out: "る",
                 rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "じません" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[],
                rules_out: &[RuleType::Vz],
            },
        ],
         "為ません" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
         "来ません" => &[
            DeinflectionRule {
                kana_out: "来る",
                 rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來ません" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "ました" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
         "いました" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
          "きました" => &[
            DeinflectionRule {
                kana_out: "く",
                 rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "ぎました" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                 rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
         "しました" => &[
            DeinflectionRule {
                kana_out: "す",
                 rules_in: &[],
                rules_out: &[RuleType::V5],
            },
             DeinflectionRule {
                kana_out: "する",
                 rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "ちました" => &[
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "にました" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "びました" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "みました" => &[
            DeinflectionRule {
                kana_out: "む",
                 rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "りました" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "じました" => &[
            DeinflectionRule {
                kana_out: "ずる",
                 rules_in: &[],
                rules_out: &[RuleType::Vz],
            },
        ],
        "為ました" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "来ました" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來ました" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
         "くありませんでした" => &[
            DeinflectionRule {
                kana_out: "い",
                 rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
        ],
         "ませんでした" => &[
            DeinflectionRule {
                kana_out: "る",
                 rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
         "いませんでした" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "きませんでした" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
         "ぎませんでした" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
         "しませんでした" => &[
            DeinflectionRule {
                kana_out: "す",
                 rules_in: &[],
                rules_out: &[RuleType::V5],
            },
             DeinflectionRule {
                kana_out: "する",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "ちませんでした" => &[
            DeinflectionRule {
                kana_out: "つ",
                 rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
         "にませんでした" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "びませんでした" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                 rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
         "みませんでした" => &[
            DeinflectionRule {
                kana_out: "む",
                 rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "りませんでした" => &[
            DeinflectionRule {
                kana_out: "る",
                 rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "じませんでした" => &[
            DeinflectionRule {
                kana_out: "ずる",
                 rules_in: &[],
                rules_out: &[RuleType::Vz],
            },
        ],
        "為ませんでした" => &[
            DeinflectionRule {
                kana_out: "為る",
                 rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "来ませんでした" => &[
            DeinflectionRule {
                kana_out: "来る",
                 rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來ませんでした" => &[
            DeinflectionRule {
                kana_out: "來る",
                 rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "ましょう" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "いましょう" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "きましょう" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
         "ぎましょう" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "しましょう" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "ちましょう" => &[
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
          "にましょう" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
         "びましょう" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
         "みましょう" => &[
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "りましょう" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "じましょう" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[],
                rules_out: &[RuleType::Vz],
            },
        ],
        "為ましょう" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "来ましょう" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來ましょう" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "れる" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[RuleType::V1],
                 rules_out: &[RuleType::V1, RuleType::V5],
            },
        ],
        "える" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[RuleType::V1],
                 rules_out: &[RuleType::V5],
            },
        ],
         "ける" => &[
            DeinflectionRule {
                kana_out: "く",
                 rules_in: &[RuleType::V1],
                 rules_out: &[RuleType::V5],
            },
        ],
         "げる" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                 rules_in: &[RuleType::V1],
                 rules_out: &[RuleType::V5],
            },
        ],
         "せる" => &[
            DeinflectionRule {
                kana_out: "す",
                 rules_in: &[RuleType::V1],
                 rules_out: &[RuleType::V5],
            },
        ],
         "てる" => &[
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "て",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Iru],
            },
        ],
        "ねる" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "べる" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                 rules_in: &[RuleType::V1],
                 rules_out: &[RuleType::V5],
            },
        ],
         "める" => &[
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
         "これる" => &[
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vk],
            },
        ],
        "来れる" => &[
            DeinflectionRule {
                kana_out: "来る",
                 rules_in: &[RuleType::V1],
                 rules_out: &[RuleType::Vk],
            },
        ],
        "來れる" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vk],
            },
        ],
            "ざれる" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vz],
            },
        ],
          "ぜられる" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vz],
            },
        ],
          "せられる" => &[
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vs],
            },
        ],
         "為られる" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vs],
            },
        ],
       "よう" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "おう" => &[
            DeinflectionRule {
                kana_out: "う",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
         "こう" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "ごう" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
          "とう" => &[
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "のう" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                 rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "ぼう" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "もう" => &[
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "ろう" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "じよう" => &[
            DeinflectionRule {
                kana_out: "ずる",
                rules_in: &[],
                rules_out: &[RuleType::Vz],
            },
        ],
        "しよう" => &[
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "為よう" => &[
            DeinflectionRule {
                kana_out: "為る",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "りながら" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "きながら" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[],
                rules_out: &[RuleType::V5],
            },
        ],
        "しながら" => &[
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[],
                rules_out: &[RuleType::Vs],
            },
        ],
        "ながら" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "らん" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[],
                rules_out: &[RuleType::V1],
            },
        ],
        "こよう" => &[
            DeinflectionRule {
                kana_out: "くる",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "来よう" => &[
            DeinflectionRule {
                kana_out: "来る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "來よう" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[],
                rules_out: &[RuleType::Vk],
            },
        ],
        "かされる" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "がされる" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "たされる" => &[
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
        "なされる" => &[
             DeinflectionRule {
                kana_out: "ぬ",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
          "ばされる" => &[
            DeinflectionRule {
                kana_out: "ぶ",
                 rules_in: &[RuleType::V1],
                 rules_out: &[RuleType::V5],
            },
        ],
         "まされる" => &[
            DeinflectionRule {
                kana_out: "む",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
          "らされる" => &[
            DeinflectionRule {
                kana_out: "る",
                 rules_in: &[RuleType::V1],
                rules_out: &[RuleType::V5],
            },
        ],
          "わされる" => &[
            DeinflectionRule {
                kana_out: "う",
                 rules_in: &[RuleType::V1],
                 rules_out: &[RuleType::V5],
            },
        ],
            "とく" => &[
            DeinflectionRule {
                kana_out: "る",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V1],
            },
        ],
            "いとく" => &[
            DeinflectionRule {
                kana_out: "く",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
        ],
             "いどく" => &[
            DeinflectionRule {
                kana_out: "ぐ",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
        ],
          "しとく" => &[
            DeinflectionRule {
                kana_out: "す",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
             DeinflectionRule {
                kana_out: "する",
                 rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vs],
            },
        ],
            "っとく" => &[
            DeinflectionRule {
                kana_out: "う",
                 rules_in: &[RuleType::V5],
                 rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "つ",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
             DeinflectionRule {
                kana_out: "る",
                 rules_in: &[RuleType::V5],
                 rules_out: &[RuleType::V5],
            },
        ],
            "んどく" => &[
            DeinflectionRule {
                kana_out: "ぬ",
                 rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "ぶ",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
            DeinflectionRule {
                kana_out: "む",
                 rules_in: &[RuleType::V5],
                rules_out: &[RuleType::V5],
            },
        ],
        "じとく" => &[
            DeinflectionRule {
                kana_out: "ずる",
                 rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vz],
            },
        ],
          "為とく" => &[
            DeinflectionRule {
                kana_out: "為る",
                 rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vs],
            },
        ],
          "きとく" => &[
            DeinflectionRule {
                kana_out: "くる",
                 rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vk],
            },
        ],
          "来とく" => &[
            DeinflectionRule {
                kana_out: "来る",
                 rules_in: &[RuleType::V5],
                 rules_out: &[RuleType::Vk],
            },
        ],
          "來とく" => &[
            DeinflectionRule {
                kana_out: "來る",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Vk],
            },
        ],
          "ている" => &[
            DeinflectionRule {
                kana_out: "て",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Iru],
            },
        ],
           "ておる" => &[
            DeinflectionRule {
                kana_out: "て",
                 rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Iru],
            },
        ],
           "でいる" => &[
            DeinflectionRule {
                kana_out: "で",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Iru],
            },
        ],
           "でおる" => &[
            DeinflectionRule {
                kana_out: "で",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Iru],
            },
        ],
         "でる" => &[
            DeinflectionRule {
                kana_out: "で",
                 rules_in: &[RuleType::V1],
                 rules_out: &[RuleType::Iru],
            },
        ],
         "とる" => &[
            DeinflectionRule {
                kana_out: "て",
                rules_in: &[RuleType::V5],
                rules_out: &[RuleType::Iru],
            },
        ],
            "ないでいる" => &[
             DeinflectionRule {
                kana_out: "ない",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::AdjI],
             },
        ],
           "しげ" => &[
            DeinflectionRule {
                kana_out: "しい",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
        ],
            "ねえ" => &[
            DeinflectionRule {
                kana_out: "ない",
                 rules_in: &[],
                 rules_out: &[RuleType::AdjI],
            },
        ],
          "めえ" => &[
            DeinflectionRule {
                kana_out: "むい",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
            DeinflectionRule {
                kana_out: "まい",
                 rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
        ],
            "みい" => &[
             DeinflectionRule {
                kana_out: "むい",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
             },
        ],
           "ちぇえ" => &[
             DeinflectionRule {
                kana_out: "つい",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
             },
            DeinflectionRule {
                kana_out: "ちゃい",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
             },
        ],
          "ちい" => &[
             DeinflectionRule {
                kana_out: "つい",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
             },
        ],
         "せえ" => &[
            DeinflectionRule {
                kana_out: "すい",
                 rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
            DeinflectionRule {
                kana_out: "さい",
                 rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
        ],
        "ええ" => &[
              DeinflectionRule {
                kana_out: "いい",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
            DeinflectionRule {
                kana_out: "わい",
                 rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
              DeinflectionRule {
                kana_out: "よい",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
             },
        ],
           "いぇえ" => &[
             DeinflectionRule {
                kana_out: "よい",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
        ],
        "うぇえ" => &[
            DeinflectionRule {
                 kana_out: "わい",
                 rules_in: &[],
                 rules_out: &[RuleType::AdjI],
            },
        ],
        "けえ" => &[
            DeinflectionRule {
                kana_out: "かい",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
        ],
         "げえ" => &[
             DeinflectionRule {
                kana_out: "がい",
                 rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
             DeinflectionRule {
                kana_out: "ごい",
                 rules_in: &[],
                rules_out: &[RuleType::AdjI],
             },
        ],
        "ぜえ" => &[
             DeinflectionRule {
                kana_out: "ずい",
                rules_in: &[],
                 rules_out: &[RuleType::AdjI],
            },
        ],
         "っぜえ" => &[
             DeinflectionRule {
                kana_out: "ずい",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
        ],
         "れえ" => &[
            DeinflectionRule {
                kana_out: "らい",
                 rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
             DeinflectionRule {
                kana_out: "れい",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
             },
        ],
        "でえ" => &[
             DeinflectionRule {
                kana_out: "どい",
                rules_in: &[],
                 rules_out: &[RuleType::AdjI],
             },
        ],
          "べえ" => &[
            DeinflectionRule {
                kana_out: "ばい",
                rules_in: &[],
                rules_out: &[RuleType::AdjI],
            },
        ],
        "できる" => &[
            DeinflectionRule {
                kana_out: "する",
                rules_in: &[RuleType::V1],
                rules_out: &[RuleType::Vs],
            },
        ],
    }
}
