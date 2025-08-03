use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher};
use clap::Parser;
use rand::rngs::OsRng;

/// Simple CLI tool to hash passwords using Argon2id
#[derive(Parser, Debug)]
#[command(name = "argon2-hasher")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "Hash passwords securely with Argon2", long_about = None)]
struct Args {
    /// The plaintext password to hash
    #[arg(short, long)]
    password: String,
}

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Hashing failed")
        .to_string();
    hash
}

fn main() {
    let args = Args::parse();

    let hashed = hash_password(&args.password);

    println!("🔐 Hashed password:");
    println!("{}", hashed);
}
