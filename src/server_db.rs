use dioxus::prelude::*;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Task {
    pub id: u8,
    pub description: String,
    pub due_by: String,
    pub is_done: bool,
}

#[server]
async fn init_db(filepath: String) -> Result<(), ServerFnError> {
    let conn = rusqlite::Connection::open(filepath).expect("Failed to connect to the database");
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task TEXT NOT NULL,
            is_done INTEGER DEFAULT 0,
=            due_by DATETIME DEFAULT NULL
    )",
        [],
    )
    .unwrap();
    Ok(())
}

#[server]
pub async fn add_todo(task: String, due: String) -> Result<(), ServerFnError> {
    init_db("todos.db".to_string()).await?;
    let conn = rusqlite::Connection::open("todos.db").expect("Failed to connect to the database");
    conn.execute(
        "INSERT INTO todos (task, due_by) VALUES (?1, ?2)",
        rusqlite::params![task, due],
    )
    .unwrap();
    Ok(())
}

#[server]
pub async fn get_todos() -> Result<Vec<Task>, ServerFnError> {
    init_db("todos.db".to_string()).await?;
    let conn = rusqlite::Connection::open("todos.db").expect("Failed to connect to the database");
    let mut stmt = conn
        .prepare("SELECT id, task, due_by, is_done FROM todos")
        .unwrap();
    let todo_iter = stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get::<_, i64>(3)? > 0,
            ))
        })
        .unwrap();
    let todos: Vec<Task> = todo_iter
        .map(|res| {
            let (id, task, due_by, is_done): (u8, String, String, bool) = res.unwrap();
            Task {
                id,
                description: task,
                due_by,
                is_done,
            }
        })
        .collect();
    Ok(todos)
}

#[server]
pub async fn mark_done(id: u8) -> Result<(), ServerFnError> {
    let conn = rusqlite::Connection::open("todos.db").expect("Failed to connect to the database");
    conn.execute("UPDATE todos SET is_done = 1 WHERE id = ?1", [id])
        .unwrap();
    Ok(())
}

#[server]
pub async fn mark_undone(id: u8) -> Result<(), ServerFnError> {
    let conn = rusqlite::Connection::open("todos.db").expect("Failed to connect to the database");
    conn.execute("UPDATE todos SET is_done = 0 WHERE id = ?1", [id])
        .unwrap();
    Ok(())
}

#[server]
pub async fn delete_task(id: u8) -> Result<(), ServerFnError> {
    let conn = rusqlite::Connection::open("todos.db").expect("Failed to connect to the database");
    conn.execute("DELETE FROM todos where id = ?1", [id])
        .unwrap();
    Ok(())
}
