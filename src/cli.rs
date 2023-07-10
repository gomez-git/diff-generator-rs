use crate::format::Format;
use std::process;

#[derive(Debug, PartialEq)]
pub struct Options {
    pub format: Format,
    pub filepaths: Vec<String>,
}

impl Options {
    fn new() -> Self {
        Self {
            format: Format::Stylish,
            filepaths: vec![],
        }
    }

    pub fn build<T>(arguments: T) -> Result<Self, &'static str>
    where
        T: Iterator<Item = String>,
    {
        let mut options = Options::new();
        let mut arguments = arguments;

        loop {
            let argument = arguments.next();

            if argument.is_none() {
                break options.check_filepaths();
            };

            let argument = argument.unwrap();

            match argument.as_str() {
                "-f" | "--format" => {
                    let format = arguments.next().unwrap_or("".to_string());

                    options.format = match format.as_str() {
                        "stylish" => Format::Stylish,
                        _ => {
                            break Err("Unknown format
Perhaps you forgot to specify the format?
Try one of this: stylish (default), pretty, json.");
                        }
                    }
                }
                "-V" | "--version" => {
                    println!("0.1.0");
                    process::exit(0);
                }
                "-h" | "--help" => {
                    println!(
                        "
    Usage: gendiff [options] <filepath1> <filepath2>

    Compares two configuration files and shows a difference.

    Options:
        -f, --format <type>  output format
        -V, --version        output the version number
        -h, --help           output usage information
"
                    );
                    process::exit(0);
                }
                _ => {
                    options.filepaths.push(argument);
                }
            }
        }
    }

    fn check_filepaths(self) -> Result<Self, &'static str> {
        let filepaths = &self.filepaths;

        if filepaths.is_empty() {
            return Err("First file path is missing!");
        }
        if filepaths.len() == 1 {
            return Err("Second file path is missing!");
        }
        if filepaths[0] == filepaths[1] {
            return Err("File paths are the same, try another.");
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Options;
    use crate::cli::Format;

    #[test]
    fn it_works() {
        let arguments = ["-f", "stylish", "file1.json", "file2.json"]
            .iter()
            .map(|s| s.to_string());
        let options = Options::build(arguments).unwrap();

        assert_eq!(
            options,
            Options {
                format: Format::Stylish,
                filepaths: vec!["file1.json".to_string(), "file2.json".to_string()],
            }
        );
    }

    #[test]
    fn it_works_with_defaults() {
        let arguments = ["file1.json", "file2.json"].iter().map(|s| s.to_string());
        let options = Options::build(arguments).unwrap();

        assert_eq!(
            options,
            Options {
                format: Format::Stylish,
                filepaths: vec!["file1.json".to_string(), "file2.json".to_string()],
            }
        );
    }

    #[test]
    #[should_panic(expected = "First file path is missing!")]
    fn missing_filepaths() {
        let arguments = ["-f", "stylish"].iter().map(|s| s.to_string());
        let _ = Options::build(arguments).unwrap();
    }

    #[test]
    #[should_panic(expected = "Second file path is missing!")]
    fn missing_filepath_2() {
        let arguments = ["file1.json"].iter().map(|s| s.to_string());
        let _ = Options::build(arguments).unwrap();
    }

    #[test]
    #[should_panic(expected = "File paths are the same, try another.")]
    fn same_filepaths() {
        let arguments = ["file1.json", "file1.json"].iter().map(|s| s.to_string());
        let _ = Options::build(arguments).unwrap();
    }

    #[test]
    #[should_panic(expected = "Unknown format")]
    fn incorrect_format() {
        let arguments = ["-f", "blank"].iter().map(|s| s.to_string());
        let _ = Options::build(arguments).unwrap();
    }
}
