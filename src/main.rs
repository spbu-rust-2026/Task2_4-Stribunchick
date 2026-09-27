use std::{
    cmp::min,
    io::{self, Read},
};

fn main() {
    let mut text = String::new();
    io::stdin()
        .read_to_string(&mut text)
        .expect("failed to read input");

    let text = text.trim_end_matches(|character| character == '\n' || character == '\r');
    println!("{}", longest_palindromic_substring(text));
}

fn longest_palindromic_substring(text: &str) -> &str {
    if text.is_empty() {
        return "";
    }
    let chars: Vec<char> = text.chars().collect();

    let mut s2: Vec<char> = vec!['#'; chars.len() * 2 + 1];
    for i in 0..chars.len() {
        s2[i * 2 + 1] = chars[i];
    }

    let mut p: Vec<i32> = vec![0; s2.len()];

    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let mut index: i32 = 0;
    let mut i_mir: i32;
    let mut max_len: i32 = 1;

    for i in 1..(s2.len() - 1) {
        let i_i32 = i as i32;
        i_mir = 2 * c - i_i32;

        if r > i_i32 {
            p[i] = min(p[i_mir as usize], r - i_i32);
        } else {
            p[i] = 0;
        }

        while i_i32 > p[i]
            && (i_i32 + p[i] + 1) < (s2.len() as i32)
            && s2[(i_i32 - p[i] - 1) as usize] == s2[(i_i32 + p[i] + 1) as usize]
        {
            p[i] += 1;
        }

        if p[i] + i_i32 > r {
            c = i_i32;
            r = i_i32 + p[i];
        }

        if max_len < p[i] {
            max_len = p[i];
            index = i_i32;
        }
    }
    let start_char = ((index - max_len) / 2) as usize;
    let len_char = max_len as usize;
    let start_byte: usize = chars[..start_char].iter().map(|c| c.len_utf8()).sum();
    let end_byte: usize = chars[..start_char + len_char]
        .iter()
        .map(|c| c.len_utf8())
        .sum();

    &text[start_byte..end_byte]
}

#[cfg(test)]
#[path = "../tests/unit/palindrome.rs"]
mod palindrome_tests;
