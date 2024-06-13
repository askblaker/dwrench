use clap::{Parser, Subcommand, ValueEnum};
use dwrench::{df_from_csv, df_to_parquet, normalize_df_colnames};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "dwrench")]
#[command(about = "A prototype that sometimes come in handy", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, ValueEnum)]
enum CsvConvertToFormats {
    Parquet,
}
#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Csvconvert {
        to: CsvConvertToFormats,
        input_path: PathBuf,
        output_path: PathBuf,

        #[arg(short, long)]
        separator: Option<String>,

        #[arg(short, long)]
        normalize_colnames: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Csvconvert {
            to,
            input_path,
            output_path,
            separator: _,
            normalize_colnames,
        } => {
            println!("to: {:?}, in: {:?}, out: {:?}", to, input_path, output_path);
            let mut df = df_from_csv(&input_path);
            println!("{}", df);

            match normalize_colnames {
                Some(_normalize) => {
                    normalize_df_colnames(&mut df);
                }
                None => {}
            }

            df_to_parquet(&mut df, &output_path)
        }
    }
}
