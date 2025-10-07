use crate::server_db::{add_todo, delete_task, get_todos, mark_done, mark_undone};
use dioxus::prelude::*;
const MAIN_CSS: Asset = asset!("assets/main.css");

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    App {},
    #[route("/add")]
    AddTask {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
pub fn route_handler() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
pub fn App() -> Element {
    // TODO: Handle all the fields of the updated task struct
    let mut todos = use_resource(get_todos);
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        h1 { "Todo" }
        div { id: "TaskContainer",
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
        div { id: "inputArea",
            button {
                onclick: move |_| async move {
                    use_navigator().push(Route::AddTask {});
                },
                "Add Task >>"
            }
        }
    }
}

#[component]
fn AddTask() -> Element {
    let mut new_todo_item_name = use_signal(|| String::new());
    let mut new_todo_item_desc: Signal<String> = use_signal(|| String::new());
    let mut new_todo_item_imp: Signal<String> = use_signal(|| String::new());
    let mut new_todo_item_due = use_signal(|| String::new());
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { id: "AddTaskHeader",
            h1 { "Add Task" }
        }
        div { id: "inputArea",
            label {
                "Task's Title :"
                input {
                    r#type: "text",
                    placeholder: "Name",
                    value: "{new_todo_item_name}",
                    oninput: move |e| new_todo_item_name.set(e.value().clone()),
                }
            }
            label {
                "Description:"
                textarea {
                    oninput: move |e| new_todo_item_desc.set(e.value().clone()),
                    value: "{new_todo_item_desc}",
                    placeholder: "Description",
                }

            }
            label {
                "Task due by:"
                input {
                    r#type: "datetime-local",
                    value: "{new_todo_item_due}",
                    oninput: move |e| new_todo_item_due.set(e.value().clone()),
                }
            }
            label {
                "Importance"
                input {
                    r#type: "range",
                    min: "1",
                    max: "10",
                    value: "{new_todo_item_imp}",
                    oninput: move |e| new_todo_item_imp.set(e.value().clone()),
                }
            }
            button {
                id: "AddTaskButton",
                name: "AddTaskButton",
                disabled: "{(new_todo_item_name.read().is_empty() || new_todo_item_desc.read().is_empty())}",
                onclick: move |_| async move {
                    add_todo(
                            new_todo_item_name.read().clone(),
                            new_todo_item_desc.read().clone(),
                            new_todo_item_imp.read().clone(),
                            new_todo_item_due.read().clone(),
                        )
                        .await
                        .unwrap();
                    new_todo_item_name.set("".to_string());
                    new_todo_item_desc.set("".to_string());
                    new_todo_item_due.set("".to_string());
                },
                "Add"
            }
            button {
                onclick: move |_| async move {
                    use_navigator().push(Route::App {});
                },
                "<< Go Back"
            }
        }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    let route_str = route.join("/");
    rsx! {
        h1 { "ERROR 404: Page not found for route \"{route_str}\"" }
    }
}
