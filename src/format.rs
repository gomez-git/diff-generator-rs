mod stylish;

use self::stylish::format_stylish;
use crate::cli::Options;
use crate::tree::Node;

#[derive(Debug, PartialEq)]
pub enum Format {
    Stylish,
}

pub struct Formatter {
    formatter: fn(Node, usize) -> String,
}

impl Formatter {
    fn new(formatter: fn(Node, usize) -> String) -> Self {
        Self { formatter }
    }

    pub fn get_formatter(options: Options) -> Self {
        match options.format {
            Format::Stylish => Formatter::new(format_stylish),
        }
    }

    pub fn format(&self, node: Node) -> String {
        (self.formatter)(node, 1)
    }
}
