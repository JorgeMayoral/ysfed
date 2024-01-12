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
            let ybf_file = Ybf::create_protected(&password, input_data.as_bytes().to_vec());
            ybf::write_file(&ybf_file, output_path.into()).unwrap();
        }
        Commands::Decrypt { file, output } => {
            let password = rpassword::prompt_password("Enter password: ").unwrap();
            let ybf_file = ybf::read_file(file.into(), Some(&password));
            let data = ybf_file.unwrap().decrypt_data(&password);
            let data_string = String::from_utf8(data).unwrap();
            let output_filename = &output.unwrap();
            let output_path = std::path::Path::new(output_filename);
            fs::write(output_path, data_string).unwrap();
        }
    }
}
