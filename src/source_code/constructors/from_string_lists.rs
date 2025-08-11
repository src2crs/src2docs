use super::SourceCode;

impl From<Vec<String>> for SourceCode {
    fn from(value: Vec<String>) -> Self {
        Self { lines: value }
    }
}

impl From<&Vec<String>> for SourceCode {
    fn from(value: &Vec<String>) -> Self {
        Self {
            lines: value.clone(),
        }
    }
}

impl From<&[String]> for SourceCode {
    fn from(value: &[String]) -> Self {
        Self {
            lines: value.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl From<&[&str]> for SourceCode {
    fn from(value: &[&str]) -> Self {
        Self {
            lines: value.iter().map(|s| s.to_string()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_CONTENT: &str = "fn main() {\n  println!(\"Hello World\")\n}";

    #[test]
    fn from_vec() {
        let input_lines = vec![
            "fn main() {".to_string(),
            "  println!(\"Hello World\")".to_string(),
            "}".to_string(),
        ];

        let code = SourceCode::from(input_lines);
        assert_eq!(code.to_string(), EXPECTED_CONTENT);
    }

    #[test]
    fn from_vec_ref() {
        let input_lines = vec![
            "fn main() {".to_string(),
            "  println!(\"Hello World\")".to_string(),
            "}".to_string(),
        ];

        let code = SourceCode::from(&input_lines);
        assert_eq!(code.to_string(), EXPECTED_CONTENT);
    }

    #[test]
    fn from_str_slice() {
        let input_lines = ["fn main() {", "  println!(\"Hello World\")", "}"];

        let code = SourceCode::from(input_lines.as_slice());
        assert_eq!(code.to_string(), EXPECTED_CONTENT);
    }

    #[test]
    fn from_string_slice() {
        let input_lines = [
            "fn main() {".to_string(),
            "  println!(\"Hello World\")".to_string(),
            "}".to_string(),
        ];

        let code = SourceCode::from(input_lines.iter().as_slice());
        assert_eq!(code.to_string(), EXPECTED_CONTENT);
    }
}
