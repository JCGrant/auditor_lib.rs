use auditor_lib::auditor::Auditor;
use auditor_lib::config::Config;

fn main() {
    // Parse the config
    let config = Config::load_all("config.toml");

    // Construct an auditor
    let auditor = Auditor::new(config.disallowed_strings, config.max_token_length);

    // Read text from stdin
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read from stdin");

    // Audit text from stdin
    auditor
        .audit(&buffer)
        .expect("Failed to audit from text from stdin");

    // Print "ok" if everything is fine
    println!("ok");
}
