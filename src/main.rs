use clap::Parser;
use log::error;
use quickserving_core::http::server::Server;
use simple_logger::SimpleLogger;

#[derive(Parser, Debug, Clone)]
#[command(version, about = "A simple HTTP server.")]
struct CLIConfig {
    #[arg(
        long,
        default_value = "8080",
        help = "The port that server will be listening for requests on."
    )]
    port: u16,
    #[arg(long, default_value = ".", help = "The directory that will be served.")]
    directory: String,
    #[arg(
        long,
        default_value = "index.html",
        help = "The file that will be read from requested path when user requests url ending with '/'."
    )]
    index: String,
    #[arg(
        long,
        default_value = "404.html",
        help = "The file that will be served when the file requested by user is not avaible."
    )]
    not_found: String,
}

fn main() {
    let _ = SimpleLogger::new().init();

    let cli_config = CLIConfig::parse();

    let server = Server {
        port: cli_config.port,
        directory: cli_config.directory,
        index_file: cli_config.index,
        not_found_uri: cli_config.not_found,
    };
    let setup_result = server.listen();

    if setup_result.is_err() {
        error!("Error: {}", setup_result.err().unwrap());
    }
}
