use crate::tree::{Node, Status};
use json::JsonValue;

pub fn format_stylish(node: Node, depth: usize) -> String {
    let indent = " ".repeat((depth - 1) * 4);

    match node.status {
        Status::Add => format!(
            "{indent}  + {}: {}",
            node.name,
            format_value(node.new_value, depth)
        ),
        Status::Remove => format!(
            "{indent}  - {}: {}",
            node.name,
            format_value(node.initial_value, depth)
        ),
        Status::Equal => format!(
            "{indent}    {}: {}",
            node.name,
            format_value(node.initial_value, depth)
        ),
        Status::NotEqual => {
            let line1 = format!(
                "{indent}  - {}: {}",
                node.name,
                format_value(node.initial_value, depth)
            );
            let line2 = format!(
                "{indent}  + {}: {}",
                node.name,
                format_value(node.new_value, depth)
            );
            format!("{line1}\n{line2}")
        }
        Status::Nested => {
            let indent = " ".repeat(depth * 4);
            let line1 = format!("{indent}{}: {{", node.name);
            let line2 = node
                .children
                .unwrap()
                .map(|node: Node| format_stylish(node, depth + 1))
                .collect::<Vec<String>>()
                .join("\n");
            let line3 = format!("{indent}}}");

            format!("{line1}\n{line2}\n{line3}")
        }
    }
}

fn format_value(value: Option<JsonValue>, spaces: usize) -> String {
    let value = value.unwrap();

    if value.is_object() {
        let line = value
            .entries()
            .map(|(key, value)| {
                let spaces = spaces + 1;
                let indent = " ".repeat(spaces * 4);
                format!(
                    "{indent}{key}: {}",
                    format_value(Some(value.clone()), spaces)
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        return format!("{{\n{line}\n{}}}", " ".repeat(spaces * 4));
    }
    match value.is_string() {
        true => value.to_string(),
        false => value.dump(),
    }
}
