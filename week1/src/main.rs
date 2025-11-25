use std::io;

fn main() {
    println!("Palindrome Checker!");
    println!("Masukkan string untuk memeriksa apakah itu polindrome:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Gagal membaca input");

    let cleaned_input = cleaned_string(&input);

    if cleaned_input.is_empty() {
        println!("Masukkan string yang valid dan tidak kosong");
        return;
    }

    if is_polindrome(&cleaned_input) {
        println!("'{}' adalah polindrome!", input.trim());
    } else {
        println!("'{}' bukan polindrome!", input.trim());
    }
}

fn cleaned_string(input: &str) -> String {
    input
        .chars() // iterasi setiap karakter
        .filter(|c| c.is_alphanumeric()) // hanya menyimpan huruf dan angka
        .map(|c| c.to_lowercase().to_string()) // mengubah ke huruf kecil
        .collect::<String>() // mengumpulkan menjadi string baru
}

// periksa apakah string yang sudah dibersihkan adalah polindrome
fn is_polindrome(input: &str) -> bool {
    input == input.chars().rev().collect::<String>()
}