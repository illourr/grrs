use exitfailure::ExitFailure;
use failure::ResultExt;
use std::fmt;
use std::fs;
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

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            match writeln!(writer, "{}", line) {
                Err(why) => eprintln!("could not write: {}", why),
                Ok(wrote) => wrote,
            }
        }
    }
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let content = fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}
