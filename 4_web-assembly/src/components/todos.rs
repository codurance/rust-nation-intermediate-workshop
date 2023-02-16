use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Mode {
    New,
    Completed,
    Editing,
}

#[derive(PartialEq, Clone)]
pub struct Todo {
    label: String,
    mode: Mode,
}

impl Todo {
    pub fn new(label: &str, mode: Mode) -> Self {
        Self {
            label: label.into(),
            mode,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TodoListProps {
    pub todos: Vec<Todo>,
}

#[function_component(TodoList)]
pub fn todo_list(props: &TodoListProps) -> Html {
    html! {
        <ul class="todo-list">
            { for props.todos.iter().map(|t| {
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

#[derive(Properties, PartialEq)]
pub struct NewTodoProps {
    pub on_todo_entered: Callback<String>,
}

#[function_component(NewTodo)]
pub fn new_todo(props: &NewTodoProps) -> Html {
    let on_todo_entered = props.on_todo_entered.clone();
    let onkeypress = Callback::from(move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            let input = e.target_unchecked_into::<HtmlInputElement>();

            let value = input.value();
            on_todo_entered.emit(value);
            input.set_value("");
        }
    });

    html! {
        <input class="new-todo" placeholder="What needs to be done?" {onkeypress} />
    }
}
