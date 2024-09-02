use std::fs;

use clap::{Parser, Subcommand};
use ybf::Ybf;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encrypt the given file
    Encrypt {
        /// Path to the file to encrypt/decrypt
        #[arg(short, long)]
        file: String,
        /// Path to the output file
        #[arg(short, long, default_value = "output.ybf")]
        output: Option<String>,
    },
    /// Decrypt the given file
    Decrypt {
        /// Path to the file to encrypt/decrypt
        #[arg(short, long)]
        file: String,
        /// Path to the output file
        #[arg(short, long, default_value = "output")]
        output: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let Cli { command } = cli;

    match command {
        Commands::Encrypt { file, output } => {
            let password = rpassword::prompt_password("Enter password: ").unwrap();
            let input_data = fs::read_to_string::<&str>(&file).unwrap();
            let output_filename = &output.unwrap();
            let output_path = std::path::Path::new(output_filename);
            let ybf_file = Ybf::create(&password, input_data.as_bytes().to_vec());
            ybf_file.write_file(output_path.into()).unwrap();
        }
        Commands::Decrypt { file, output } => {
            let password = rpassword::prompt_password("Enter password: ").unwrap();
            let ybf = Ybf::from_file(file.into()).unwrap();
            let data = ybf.decrypt_data(&password).unwrap();
            let output_filename = &output.unwrap();
            let output_path = std::path::Path::new(output_filename);
            fs::write(output_path, data.as_bytes()).unwrap();
        }
    }
}
