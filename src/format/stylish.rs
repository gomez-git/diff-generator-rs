use crate::tree::{Node, Status};
use json::JsonValue;

pub fn format_stylish(node: &Node) -> String {
    match node.status {
        Status::Add => format!("  + {}: {}", node.name, format_value(&node.new_value)),
        Status::Remove => format!("  - {}: {}", node.name, format_value(&node.initial_value)),
        Status::Equal => format!("    {}: {}", node.name, format_value(&node.initial_value)),
        Status::NotEqual => {
            let line1 = format!("  - {}: {}", node.name, format_value(&node.initial_value));
            let line2 = format!("  + {}: {}", node.name, format_value(&node.new_value));
            format!("{line1}\n{line2}")
        }
    }
}

fn format_value(value: &Option<JsonValue>) -> String {
    let value = value.as_ref().unwrap();

    match value.is_string() {
        true => value.to_string(),
        false => value.dump(),
    }
}
