use clap::{Parser, Subcommand};
use ysfed::{decrypt, encrypt};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Path to the file to encrypt/decrypt
    #[arg(short, long)]
    file: String,
    /// Password used to encrypt/decrypt the file
    #[arg(short, long)]
    password: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Encrypt the given file
    Encrypt {
        /// Path to the output file
        #[arg(short, long, default_value = "output.lock")]
        output: Option<String>,
    },
    /// Decrypt the given file
    Decrypt {
        /// Path to the output file
        #[arg(short, long, default_value = "output.unlock")]
        output: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let Cli {
        command,
        password,
        file,
    } = cli;

    let password = password.as_bytes();

    match command {
        Commands::Encrypt { output } => {
            let input_path = std::path::Path::new(&file);
            let output_filename = &output.unwrap();
            let output_path = std::path::Path::new(output_filename);
            encrypt::encrypt_file(password, input_path, output_path).unwrap_or_else(|e| {
                println!("{}", e);
                std::process::exit(1);
            });
        }
        Commands::Decrypt { output } => {
            let input_path = std::path::Path::new(&file);
            let output_filename = &output.unwrap();
            let output_path = std::path::Path::new(output_filename);
            decrypt::decrypt_file(password, input_path, output_path).unwrap_or_else(|e| {
                println!("{}", e);
                std::process::exit(1);
            });
        }
    }
}
