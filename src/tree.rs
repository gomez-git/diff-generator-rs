use json::JsonValue;

pub struct Node {
    pub status: Status,
    pub name: String,
    pub initial_value: Option<JsonValue>,
    pub new_value: Option<JsonValue>,
    pub children: Option<Box<dyn Iterator<Item = Node>>>,
}

impl Node {
    fn new(
        status: Status,
        initial_value: Option<JsonValue>,
        new_value: Option<JsonValue>,
        children: Option<Box<dyn Iterator<Item = Node>>>,
        name: String,
    ) -> Self {
        Self {
            status,
            name,
            initial_value,
            new_value,
            children,
        }
    }
}

pub enum Status {
    Add,
    Remove,
    Equal,
    NotEqual,
    Nested,
}

pub fn make_ast(objects: (JsonValue, JsonValue)) -> impl Iterator<Item = Node> {
    let union_uniq_keys: Vec<String> = get_union_uniq_keys(&objects.0, &objects.1);

    Box::new(union_uniq_keys.into_iter().map(move |key| {
        if !objects.0.has_key(&key) {
            return Node::new(Status::Add, None, Some(objects.1[&key].clone()), None, key);
        }
        if !objects.1.has_key(&key) {
            return Node::new(
                Status::Remove,
                Some(objects.0[&key].clone()),
                None,
                None,
                key,
            );
        }
        if objects.0[&key].is_object() && objects.1[&key].is_object() {
            let objects = (objects.0[&key].clone(), objects.1[&key].clone());

            return Node::new(
                Status::Nested,
                None,
                None,
                Some(Box::new(make_ast(objects))),
                key,
            );
        }
        if objects.0[&key] == objects.1[&key] {
            return Node::new(
                Status::Equal,
                Some(objects.0[&key].clone()),
                Some(objects.1[&key].clone()),
                None,
                key,
            );
        }
        Node::new(
            Status::NotEqual,
            Some(objects.0[&key].clone()),
            Some(objects.1[&key].clone()),
            None,
            key,
        )
    }))
}

fn get_union_uniq_keys(object1: &JsonValue, object2: &JsonValue) -> Vec<String> {
    let mut vector: Vec<String> = object1
        .entries()
        .chain(object2.entries())
        .map(|(key, _)| key.to_string())
        .collect();

    vector.sort();
    vector.dedup();

    vector
}
