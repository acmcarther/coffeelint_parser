extern crate rustc_serialize;

use rustc_serialize::{
  Decoder,
};

#[derive(RustcDecodable, Clone, Debug)]
pub struct CoffeelintResponse {
  // type: String
  // time: GitTM
  pub message: String
}

#[derive(RustcDecodable, Clone, Debug)]
pub struct LintError {
  pub line: u32,
  pub error_text: String
}

#[derive(Clone, Debug)]
pub struct FileErrors {
  pub file_name: String,
  pub errors: Vec<LintError>
}

pub fn identify_lint_errors(lint_content: String) -> Vec<FileErrors> {
  let marker = "\u{001B}[1m";
  let green = "\u{001B}[32m";
  let red = "\u{001B}[31m";
  let m_2 = "\u{001B}[39m";
  let m_3 = "\u{001B}[22m";
  let m_4 = "\u{2717}";
  lint_content
    .lines()
    .filter(|line| line.contains("✗"))
    .filter(|line| !line.contains("Lint!")) // Remove final line
    .map(|string| {
      string
        .replace(marker, "")
        .replace(green, "")
        .replace(red, "")
        .replace(m_2, "")
        .replace(m_3, "")
        .replace(m_4, "")
        .trim()
        .to_owned()
     })
    .fold(vec![], |mut acc, line| {
      let first_char = line.chars().next().expect("Error line had no first character");
      if first_char == '#' {
        // Line error
        let mut last_file: &mut FileErrors = acc.last_mut().expect("Found line err with no file preceeding it");
        let mut contents = line.splitn(2, ":").map(|string| string.to_owned()).collect::<Vec<String>>();
        let line_num = contents.remove(0).replace("#", "");
        let error = contents.remove(0);
        last_file.errors.push(LintError {line: line_num.parse::<u32>().unwrap(), error_text: error });
      } else {
        // New File
        acc.push(FileErrors {file_name: line, errors: vec![]});
      }
      acc
    })
}

