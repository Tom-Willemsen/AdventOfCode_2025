pub mod impl_2025_01;

pub use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub input: String,
}
