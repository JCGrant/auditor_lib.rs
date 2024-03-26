use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, default_value_t = 10)]
    pub max_token_length: usize,
}
