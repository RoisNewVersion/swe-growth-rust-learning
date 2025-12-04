mod input;
mod utils;
fn main() {
    println!("Palindrome Checker!");
    println!("Masukkan string untuk memeriksa apakah itu palindrome:");

    // fungsi input dipisah
    let input = input::read_input();

    let cleaned_input = utils::cleaned_string(&input);

    if cleaned_input.is_empty() {
        println!("Masukkan string yang valid dan tidak kosong");
        return;
    }

    if utils::is_polindrome(&cleaned_input) {
        println!("'{}' adalah palindrome!", input.trim());
    } else {
        println!("'{}' bukan palindrome!", input.trim());
    }
}
