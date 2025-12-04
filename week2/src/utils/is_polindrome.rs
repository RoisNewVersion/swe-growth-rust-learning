pub fn is_polindrome(input: &str) -> bool {
    input == input.chars().rev().collect::<String>()
}