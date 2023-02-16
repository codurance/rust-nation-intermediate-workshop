use yew::prelude::*;

#[derive(PartialEq, Clone)]
enum Mode {
    New,
    Completed,
    Editing,
}

#[derive(PartialEq, Clone)]
struct Todo {
    label: String,
    mode: Mode,
}

impl Todo {
    fn new(label: &str, mode: Mode) -> Self {
        Self {
            label: label.into(),
            mode,
        }
    }
}

#[function_component(TodoList)]
pub fn todo_list() -> Html {
    let todos = vec![
        Todo::new("TODO 1d", Mode::New),
        Todo::new("TODO 2d", Mode::Completed),
        Todo::new("TODO 3d", Mode::Editing),
    ];

    html! {
        <ul class="todo-list">
            { for todos.iter().map(|t| {
                html! {
                    <TodoItem todo={(*t).clone()} />
                }
            })}
        </ul>
    }
}

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
    todo: Todo,
}

#[function_component(TodoItem)]
pub fn todo_item(props: &TodoItemProps) -> Html {
    let class_mode = match props.todo.mode {
        Mode::New => None,
        Mode::Completed => Some("completed"),
        Mode::Editing => Some("editing"),
    };
    html! {
        <li class={classes!(class_mode)}>
            <div class="view">
                <input type="checkbox" class="toggle"
                    checked={matches!(props.todo.mode, Mode::Completed)} />
                <label>{&props.todo.label}</label>
                <button class="destroy" />
            </div>
            <input class="edit" type="text" value={props.todo.label.clone()}
                hidden={!matches!(props.todo.mode, Mode::Editing)} />
        </li>
    }
}
