#[derive(Debug)]
struct Todo {
    id: u64,
    title: String,
    done: bool,
}
fn main() {
    let mut todos: Vec<Box<Todo>> = Vec::new();

    add_todo(&mut todos, "Todo 2");
    mark_todo(&mut todos[0]);
    println!("{:?}", todos);
}

fn add_todo(list: &mut Vec<Box<Todo>>, title: &str) {
    list.push(Box::new(Todo {
        id: list.len() as u64,
        title: String::from(title),
        done: false,
    }));
}

fn mark_todo(todo: &mut Box<Todo>) {
    todo.done = true;
}

fn reset_todo(todo: &mut Box<Todo>) {
    // Yeni Todo ilə əvəz et
    *todo = Box::new(Todo {
        id: 99,
        title: String::from("Reset edildi"),
        done: false,
    });
}
