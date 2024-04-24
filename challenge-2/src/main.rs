use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use regex::Regex;

fn update_file_version(source_path: &PathBuf, filename: &str, regex_pattern: &str, replacement: &str) -> std::io::Result<()> {
    let file_path = source_path.join(filename);
    let temp_file_path = source_path.join(format!("{}1", filename));

    // Read from original file and write updates to a temporary file
    let file = File::open(&file_path)?;
    let mut temp_file = File::create(&temp_file_path)?;
    let reader = BufReader::new(file);
    let re = Regex::new(regex_pattern).unwrap();

    for line in reader.lines() {
        let line = line?;
        let updated_line = re.replace_all(&line, replacement).to_string();
        writeln!(temp_file, "{}", updated_line)?;
    }

    // Replace the original file with the updated file
    fs::remove_file(&file_path)?;
    fs::rename(&temp_file_path, &file_path)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let source_path = env::var("SourcePath").expect("SourcePath environment variable not set");
    let build_num = env::var("BuildNum").expect("BuildNum environment variable not set");

    let source_dir = PathBuf::from(source_path).join("develop/global/src");

    // SConstruct file update
    let sconstruct_pattern = r"point=\d+";
    let sconstruct_replacement = format!("point={}", build_num);
    update_file_version(&source_dir, "SConstruct", sconstruct_pattern, &sconstruct_replacement)?;

    // VERSION file update
    let version_pattern = r"ADLMSDK_VERSION_POINT=\d+";
    let version_replacement = format!("ADLMSDK_VERSION_POINT={}", build_num);
    update_file_version(&source_dir, "VERSION", version_pattern, &version_replacement)?;

    Ok(())
}
