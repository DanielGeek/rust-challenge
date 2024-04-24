fn reverse_words(s: &str) -> String {
    let mut result = String::new();
    let mut word = String::new();

    // Helper to reverse the current word and append it to result
    let mut append_reversed = |word: &mut String, result: &mut String| {
        if !word.is_empty() {
            result.push_str(&word.chars().rev().collect::<String>());
            word.clear();
        }
    };

    for ch in s.chars() {
        if ch.is_alphanumeric() {
            word.push(ch);
        } else {
            append_reversed(&mut word, &mut result);
            result.push(ch); // Add the non-alphanumeric character as is
        }
    }
    // Append the last word if any
    append_reversed(&mut word, &mut result);

    result
}

fn main() {
    let test_str = "String; 2be reversed...";
    let expected = "gnirtS; eb2 desrever...";
    assert_eq!(reverse_words(test_str), expected);
    println!("Test passed!");
}
