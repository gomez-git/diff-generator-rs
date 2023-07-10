use crate::cli::Options;
use crate::format::Formatter;
use crate::tree::{make_ast, Node};
use json::JsonValue;
use std::error::Error;
use std::fs;

pub fn generate_difference(options: Options) -> Result<String, Box<dyn Error>> {
    let files: (String, String) = get_files(&options)?;
    let objects: (JsonValue, JsonValue) = get_objects(files)?;

    let formatter: Formatter = Formatter::get_formatter(options);
    let ast = make_ast(objects);

    let difference: String = ast
        .map(|node: Node| formatter.format(&node))
        .collect::<Vec<String>>()
        .join("\n");

    Ok(format!("{{\n{difference}\n}}"))
}

fn get_files(options: &Options) -> Result<(String, String), Box<dyn Error>> {
    let file1 = fs::read_to_string(&options.filepaths[0])?;
    let file2 = fs::read_to_string(&options.filepaths[1])?;

    Ok((file1, file2))
}

fn get_objects(files: (String, String)) -> Result<(JsonValue, JsonValue), Box<dyn Error>> {
    let obj1 = json::parse(&files.0)?;
    let obj2 = json::parse(&files.1)?;

    Ok((obj1, obj2))
}

#[cfg(test)]
mod tests {
    use super::generate_difference;
    use crate::cli::Options;
    use crate::format::Format;
    use std::fs;

    #[test]
    fn it_works() {
        let options = Options {
            format: Format::Stylish,
            filepaths: vec![
                "fixtures/file1.json".to_string(),
                "fixtures/file2.json".to_string(),
            ],
        };

        let difference = generate_difference(options).unwrap();

        assert_eq!(difference, fs::read_to_string("fixtures/file.txt").unwrap());
    }

    #[test]
    #[should_panic]
    fn zero_filepaths() {
        let options = Options {
            format: Format::Stylish,
            filepaths: vec![],
        };

        let _ = generate_difference(options);
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn files_dont_exists() {
        let options = Options {
            format: Format::Stylish,
            filepaths: vec!["_".to_string()],
        };

        let _ = generate_difference(options).unwrap();
    }
}
