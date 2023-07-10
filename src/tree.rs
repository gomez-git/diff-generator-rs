use json::JsonValue;

pub struct Node {
    pub status: Status,
    pub name: String,
    pub initial_value: Option<JsonValue>,
    pub new_value: Option<JsonValue>,
}

impl Node {
    fn new(
        status: Status,
        name: String,
        initial_value: Option<JsonValue>,
        new_value: Option<JsonValue>,
    ) -> Self {
        Self {
            status,
            name,
            initial_value,
            new_value,
        }
    }
}

pub enum Status {
    Add,
    Remove,
    Equal,
    NotEqual,
}

pub fn make_ast<'a>(objects: (JsonValue, JsonValue)) -> impl Iterator<Item = Node> + 'a {
    let union_uniq_keys: Vec<String> = get_union_uniq_keys(&objects);

    union_uniq_keys.into_iter().map(move |key| {
        if !objects.0.has_key(&key) {
            return Node::new(
                Status::Add,
                key.to_string(),
                None,
                Some(objects.1[key].clone()),
            );
        }
        if !objects.1.has_key(&key) {
            return Node::new(
                Status::Remove,
                key.to_string(),
                Some(objects.0[key].clone()),
                None,
            );
        }
        if objects.0[&key] == objects.1[&key] {
            return Node::new(
                Status::Equal,
                key.to_string(),
                Some(objects.0[&key].clone()),
                Some(objects.1[key].clone()),
            );
        }
        Node::new(
            Status::NotEqual,
            key.to_string(),
            Some(objects.0[&key].clone()),
            Some(objects.1[key].clone()),
        )
    })
}

fn get_union_uniq_keys(objects: &(JsonValue, JsonValue)) -> Vec<String> {
    let mut vector = vec![];

    for (key, _) in objects.0.entries() {
        vector.push(key.to_string());
    }

    for (key, _) in objects.1.entries() {
        vector.push(key.to_string());
    }

    vector.sort();
    vector.dedup();

    vector
}
