use rand::Rng;
use std::env;
use std::io::{self, Write};

// Define the character set to use when generating passwords.
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789)(*&^%$#@!~";

// generate_password takes the length of the password to generate and a string of characters to exclude.
// It returns a randomly generated password of the specified length.
fn generate_password(len: usize, exclude: &str) -> String {
    // Create a thread-local random number generator.
    let mut rng = rand::thread_rng();
    // Convert the exclude string to a byte array.
    let exclude_bytes = exclude.as_bytes();
    // Generate a password of the specified length.
    let password: String = (0..len)
        .map(|_i| {
            let mut next_char = CHARSET[rng.gen_range(0..CHARSET.len())] as char;
            while exclude_bytes.contains(&(next_char as u8)) {
                next_char = CHARSET[rng.gen_range(0..CHARSET.len())] as char;
            }
            next_char
        })
        .collect();
    // Return the generated password.
    password
}
fn main() -> io::Result<()> {
    // Get the command line arguments.
    let args: Vec<String> = env::args().collect();
    // Set the default password length and excluded characters.
    let mut len = 16;
    let mut exclude = String::new();

    // Iterate over the command line arguments and check for the --len and --exclude arguments.
    for (_i, arg) in args.iter().enumerate() {
        // If the --len argument is specified, set the password length.
        if arg.starts_with("--len=") {
            if let Some(n) = arg.split('=').nth(1) {
                len = n.parse::<usize>().unwrap();
            }
        }
        // If the --exclude argument is specified, set the excluded characters.
        if arg.starts_with("--exclude=") {
            if let Some(n) = arg.split('=').nth(1) {
                exclude = n.to_string();
            }
        }
    }

    // Generate a password.
    let password = generate_password(len, &exclude);
    // Print the password to stdout.
    writeln!(io::stdout(), "Suggested password: {}", password)?;

    // Return success.
    Ok(())
}
