use clap::Parser;

use auditor_lib::args::Args;
use auditor_lib::auditor::Auditor;
use auditor_lib::config::Config;

fn main() {
    // Parse command line args
    let args: Args = Args::parse();

    // Parse config file
    let config = Config::from_file("config.toml");

    // Construct an auditor
    let auditor = Auditor::new(config.disallowed_strings, args.max_token_length);

    // Read text from stdin
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read from stdin");

    // Read text from stdin
    auditor
        .audit(&buffer)
        .expect("Failed to audit from text from stdin");

    // Print "ok" if everything is fine
    println!("ok");
}
