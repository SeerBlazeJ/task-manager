use crate::server_db::{add_todo, delete_task, get_todos, mark_done, mark_undone};
use dioxus::prelude::*;
const MAIN_CSS: Asset = asset!("assets/main.css");

#[component]
pub fn App() -> Element {
    let mut new_todo_item = use_signal(|| String::new());
    let mut new_todo_due = use_signal(|| String::new());
    let mut todos = use_resource(get_todos);
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        h1 { "Todo App made using Dioxus" }
        div { id: "AddTaskHeader",
            h2 { "Add Task" }
        }
        div { id: "inputArea",
            input {
                r#type: "text",
                placeholder: "Task Name",
                value: "{new_todo_item}",
                oninput: move |e| new_todo_item.set(e.value().clone()),
            }
            input {
                r#type: "datetime-local",
                placeholder: "Enter Time",
                value: "{new_todo_due}",
                oninput: move |e| new_todo_due.set(e.value().clone()),
            }
            button {
                id: "AddTaskButton",
                name: "AddTaskButton",
                disabled: "{new_todo_item.read().is_empty()}",
                onclick: move |_| async move {
                    add_todo(new_todo_item.read().clone(), new_todo_due.read().clone())
                        .await
                        .unwrap();
                    new_todo_item.set("".to_string());
                    new_todo_due.set("".to_string());
                    todos.restart();
                },
                "Add"
            }
        }
        br {}
        div { id: "TaskContainer",
            h2 { "Current Tasks" }
            div { id: "TaskList",
                if let Some(tasks) = &*todos.read() {
                    match tasks {
                        Ok(task_vec) => {
                            if !task_vec.is_empty() {
                                let task_elements = task_vec
                                    .iter()
                                    .map(|task| {
                                        let task_id = task.id;
                                        rsx! {
                                            if !task.due_by.is_empty() {
                                                input {
                                                    class: "TaskCheck",
                                                    r#type: "checkbox",
                                                    checked: "{task.is_done}",
                                                    oninput: move |event| async move {
                                                        let is_enabled = event.value() == "true";
                                                        if is_enabled {
                                                            mark_done(task_id).await.unwrap();
                                                        } else {
                                                            mark_undone(task_id).await.unwrap();
                                                        }
                                                    },
                                                }
                                                "{task.description} due by {task.due_by}"
                                                button {
                                                    class: "DeleteButton",
                                                    onclick: move |_| async move {
                                                        delete_task(task_id).await.unwrap();
                                                        todos.restart();
                                                    },
                                                    "Delete"
                                                }
                                            } else {
                                                input {
                                                    class: "TaskCheck",
                                                    r#type: "checkbox",
                                                    checked: "{task.is_done}",
                                                    oninput: move |event| async move {
                                                        let is_enabled = event.value() == "true";
                                                        if is_enabled {
                                                            mark_done(task_id).await.unwrap();
                                                        } else {
                                                            mark_undone(task_id).await.unwrap();
                                                        }
                                                    },
                                                }
                                                "{task.description} "
                                                button {
                                                    class: "DeleteButton",
                                                    onclick: move |_| async move {
                                                        delete_task(task_id).await.unwrap();
                                                        todos.restart();
                                                    },
                                                    "Delete"
                                                }
                                            }
                                            br {}
                                        }
                                    });
                                rsx! {
                                    {task_elements}
                                }
                            } else {
                                rsx! {
                                    h4 { "No Tasks found." }
                                }
                            }
                        }
                        Err(_) => rsx! {
                            li { "Failed to load tasks" }
                        },
                    }
                } else {
                    h3 { "Loading tasks" }
                }
            }
        }
    }
}
