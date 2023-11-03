use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use crane_webserver::*;

fn main() {
    let list = Arc::new(Mutex::new(TodoList::new()));

    list.lock().unwrap().add_item("item 1".to_string());
    list.lock().unwrap().add_item("item 2".to_string());
    list.lock().unwrap().add_item("item 3".to_string());
    list.lock().unwrap().add_item("item 4".to_string());

    let server = WebServer::bind("127.0.0.1:8888", move |path, query| {
        let not_found = ResponseBuilder::new().status(HttpStatus::Not_Found).build();
        match path.as_str() {
            "/todos" => todos(&mut list.lock().unwrap()),

            // please forgive me....
            "/api/add" => {
                let Some(items) = query.get("item") else {
                    return ResponseBuilder::new()
                        .header("Content-Type", "text/json")
                        .status(HttpStatus::OK)
                        .body(r#"{ error: "Item parameter not found" }"#)
                        .build();
                };

                add_item(&items, &mut list.lock().unwrap())
            }

            // again
            "/api/remove" => {
                let Some(items) = query.get("item") else {
                    return ResponseBuilder::new()
                        .header("Content-Type", "text/json")
                        .status(HttpStatus::OK)
                        .body(r#"{ error: "Item parameter not found" }"#)
                        .build();
                };

                remove_item(&items, &mut list.lock().unwrap())
            }

            // and in future also :)
            // ...
            _ => not_found,
        }
    })
    .unwrap();

    server.start();
}

fn add_item(items: &[String], list: &mut TodoList) -> Response {
    for item in items {
        list.add_item(item.to_string());
    }

    ResponseBuilder::new()
        .status(HttpStatus::OK)
        .body(r#"{ error: "none" }"#)
        .header("Access-Control-Allow-Origin", "*")
        .header("Content-Type", "text/json")
        .build()
}

fn remove_item(items: &[String], list: &mut TodoList) -> Response {
    for item in items {
        list.remove_item(item.to_string());
    }

    ResponseBuilder::new()
        .status(HttpStatus::OK)
        .body(r#"{ error: "none" }"#)
        .header("Access-Control-Allow-Origin", "*")
        .header("Content-Type", "text/json")
        .build()
}

fn todos(list: &mut TodoList) -> Response {
    let json = list
        .get_items()
        .iter()
        .map(|(status, item)| format!("{{ \"name\": \"{item}\", \"checked\": {status} }}"))
        .collect::<Vec<_>>()
        .join(", ");

    ResponseBuilder::new()
        .status(HttpStatus::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Content-Type", "application/json")
        .body(&format!("[{json}]"))
        .build()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum TodoStatus {
    Finished,
    Pending,
}

impl std::fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Finished => "true",
                Self::Pending => "false",
            }
        )
    }
}

struct TodoList {
    todos: HashSet<(TodoStatus, String)>,
}

impl TodoList {
    fn new() -> Self {
        Self {
            todos: HashSet::new(),
        }
    }

    fn add_item(&mut self, todo: String) -> bool {
        self.todos.insert((TodoStatus::Pending, todo))
    }

    fn get_items<'a>(&self) -> &HashSet<(TodoStatus, String)> {
        &self.todos
    }

    fn set_finished(&mut self, todo: String) {
        let _ = self.todos.remove(&(TodoStatus::Pending, todo.clone()));
        self.todos.insert((TodoStatus::Finished, todo));
    }

    fn remove_item(&mut self, todo: String) {
        if !self.todos.remove(&(TodoStatus::Pending, todo.clone())) {
            self.todos.remove(&(TodoStatus::Finished, todo));
        }
    }
}
