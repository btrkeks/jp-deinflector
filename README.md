[![Crates.io](https://img.shields.io/crates/v/jp-deinflector.svg)](https://crates.io/crates/jp-deinflector)
[![Documentation](https://docs.rs/jp-deinflector/badge.svg)](https://docs.rs/jp-deinflector/)
[![Codecov](https://codecov.io/github/btrkeks/jp-deinflector/coverage.svg?branch=main)](https://codecov.io/gh/btrkeks/jp-deinflector)
[![Dependency status](https://deps.rs/repo/github/btrkeks/jp-deinflector/status.svg)](https://deps.rs/repo/github/btrkeks/jp-deinflector)

# jp-deinflector

## Introduction
This is a Rust crate for deinflecting Japanese words optimized for 
maximum performance. 
Currently, it has a function `deinflect(word: &str) -> Vec<String>` 
that will output a list of **possible** deinflections 
for the input word. Since it doesn't do any dictionary lookups,
it cannot guarantee that the returned words are actual Japanese words, but
the deinflection will always be correct (i.e. if the word were to be a real word
it would be deinflected correctly).

This crate is meant for use in dictionary applications to obtain a list of
possible deinflections that can then be looked up in a dictionary.

There is also a function `kata_to_hira(kata: &str)` that converts
all katakana characters in `kata` into their hiragana counterparts.

## Performance
This crate uses a perfect hash table to store the deinflection rules, 
which means that lookup can be performed very quickly in constant time.
The time required for a single deinflection is usually in the
nanosecond range.
