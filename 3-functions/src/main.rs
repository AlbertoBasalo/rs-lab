// Use the Caesar cipher to encrypt and decrypt files using a key from the command line.

use std::env;
use std::fs;

/// A `struct` that holds the command line arguments.
/// It is used to store the _clean file name_ and the _key string_.
struct CliArgs {
    clean_file_name: String,
    key_string: String,
}

/// Reads the command line arguments.
///
/// ## Returns
/// A `CliArgs` struct that holds the command line arguments.
///
/// ## Errors
/// If the number of arguments is not 3, then the program exits with an error message.
fn read_args() -> CliArgs {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("💣 Usage: cargo run <clean_file> <key_string>");
        std::process::exit(1);
    }
    let cli_args = CliArgs {
        clean_file_name: args[1].clone(),
        key_string: args[2].clone(),
    };
    return cli_args;
}

/// Reads the text content of a file.
///
/// If the file is not found, or if the file is not valid UTF-8,
/// then returns an error message.
/// ## Arguments
/// * `filename` - A string slice that holds the name of the file to read.
///
/// ## Returns
/// * `Result<String, std::io::Error>` - The content of the file.
///
/// ## Example
/// ```
/// let content = read_file("example.txt");
/// ```
fn read_file(filename: &str) -> String {
    let content: Result<String, std::io::Error> = fs::read_to_string(filename);
    match content {
        Ok(content) => return content,
        Err(error) => {
            eprintln!("💣 Error reading file: {}", error);
            std::process::exit(1)
        }
    }
}

/// Encrypts a string using the **Caesar cipher**.
///
/// This function takes a clean string and a key password as input.
/// It applies the Caesar cipher to each character in the string,
/// using the key password as a source of shift values.
/// The shift values are rotated for each character.
/// > Note: This function is implemented using loops instead of iterators.
///
/// ## Arguments
////// * `input` - A string slice that holds the clean text to be encrypted.
/// * `key` - A string slice that holds the key password.
///
/// ## Returns
/// * `String` - The encrypted text.
///
/// ## Example
/// ```
/// let encrypted = caesar_cipher_with_password("hello", "key");
/// ```
fn caesar_cipher_text(clean_string: &str, key_string: &str) -> String {
    let mut key_index: usize = 0;
    let mut ciphered_string: String = String::new();
    for clean_char in clean_string.chars() {
        let char_shift: u8 = key_string.chars().nth(key_index).unwrap() as u8;
        let ciphered_char: char = caesar_cipher_char(clean_char, char_shift);
        ciphered_string.push(ciphered_char);
        key_index = get_next_key_index(key_index, key_string);
    }
    return ciphered_string;
}

/// Gets the next rotating index from a key string.
/// This function is used to rotate the key string for each character.
/// If the current index is at the end of the key string, then it wraps around to the start.
/// Otherwise, it increments the current index by 1.
/// ## Arguments
/// * `current_index` - A usize that holds the current index.
/// * `key_string` - A string slice that holds the key password.
/// ## Returns
/// * `usize` - The next rotating index.
fn get_next_key_index(current_index: usize, key_string: &str) -> usize {
    (current_index + 1) % key_string.len()
}

/// Encrypts a character using the **Caesar cipher**.
///
/// This function takes a clean character and a shift value as input.
/// It applies the Caesar cipher to the character using the shift value.
/// If the character is not an ASCII alphabetic character, then it is left unchanged.
///
/// ## Arguments
/// * `clean_char` - A character that holds the clean text to be encrypted.
/// * `shift` - A u8 that holds the shift value.
///
/// ## Returns
/// * `char` - The encrypted character.
///
/// ## Example
/// ```
/// let encrypted = caesar_cipher_char('a', 3);
/// ```
fn caesar_cipher_char(clean_char: char, shift: u8) -> char {
    if clean_char.is_ascii_alphabetic() == false {
        return clean_char;
    }

    let mut base_case_code: u8 = 'A' as u8;
    if clean_char.is_ascii_lowercase() {
        base_case_code = 'a' as u8;
    }
    let clean_code: u8 = clean_char as u8;
    let case_length: u8 = 26;
    let ciphered_code: u8 = ((clean_code - base_case_code + shift) % case_length) + base_case_code;
    let ciphered_char: char = ciphered_code as char;
    return ciphered_char;
}

/// The main function reads the command line arguments, reads the file content, and encrypts the text.  
/// It then prints the original and encrypted text to the console.
/// ## Example
/// ```
/// cargo run example.txt key
/// ```
fn main() {
    let cli_args = read_args();

    let clean_text = read_file(&cli_args.clean_file_name);

    let encrypted = caesar_cipher_text(&clean_text, &cli_args.key_string);

    println!("⛲ Original text:\n{}", clean_text);
    println!("🕵️‍♀️ Encrypted text:\n{}", encrypted);
}
