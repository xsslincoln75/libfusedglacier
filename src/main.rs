const STRP: &str = "set-ops-b761c9";
fn caesar(text: &str, shift: u8) -> String { text.chars().map(|c| { if c.is_ascii_lowercase() { (b'a' + (c as u8 - b'a' + shift) % 26) as char } else if c.is_ascii_uppercase() { (b'A' + (c as u8 - b'A' + shift) % 26) as char } else { c } }).collect() }
fn count_chars(text: &str) -> Vec<(char, usize)> { let mut freq = std::collections::HashMap::new(); for c in text.chars().filter(|c| c.is_alphabetic()) { *freq.entry(c.to_lowercase().next().unwrap()).or_insert(0) += 1; } let mut v: Vec<_> = freq.into_iter().collect(); v.sort_by(|a, b| b.1.cmp(&a.1)); v }
fn main() {
    let text = "The Quick Brown Fox Jumps Over The Lazy Dog";
    println!("[{}] Original: {}", STRP, text);
    println!("[{}] Caesar+3: {}", STRP, caesar(text, 3));
    println!("[{}] Top chars: {:?}", STRP, &count_chars(text)[..5]);
}
