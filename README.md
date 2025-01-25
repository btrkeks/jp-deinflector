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