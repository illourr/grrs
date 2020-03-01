use std::fmt;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

impl fmt::Display for Cli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pattern: {}, path:{}",
            self.pattern,
            self.path.to_string_lossy()
        )
    }
}

fn main() {
    let args = Cli::from_args();
    println!("{}", args);
}
