[![Crates.io](https://img.shields.io/crates/v/jp-deinflector.svg)](https://crates.io/crates/jp-deinflector)
[![Documentation](https://docs.rs/jp-deinflector/badge.svg)](https://docs.rs/jp-deinflector/)
[![Codecov](https://codecov.io/github/btrkeks/jp-deinflector/coverage.svg?branch=main)](https://codecov.io/gh/btrkeks/jp-deinflector)
[![Dependency status](https://deps.rs/repo/github/btrkeks/jp-deinflector/status.svg)](https://deps.rs/repo/github/btrkeks/jp-deinflector)

# jp-deinflector

## Introduction
This is a (work in progress) Rust crate for deinflecting Japanese words optimized for 
maximum performance. 
Currently, it has one function `deinflect(word: &str)` that will output a list of 
_possible_ deinflections for the input word. Since it doesn't do any dictionary lookups,
the list may (and will) contain false deinflections.

This crate is meant for use in dictionary applications to obtain a list of
possible deinflections that can then be looked up in a dictionary.

## Performance
This crate uses a perfect hash table to store the deinflection rules, which means that
lookup can be performed very quickly in constant time. \
A lookup will currently in the worst case consist of 7 hash table lookups
per deinflection iteration.