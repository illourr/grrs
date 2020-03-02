pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            match writeln!(writer, "{}", line) {
                Err(why) => eprintln!("could not write: {}", why),
                Ok(wrote) => wrote,
            }
        }
    }
}
