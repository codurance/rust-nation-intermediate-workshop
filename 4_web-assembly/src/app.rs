use yew::prelude::*;
use crate::components::todos::TodoList;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="todomvc-wrapper">
            <section class="todoapp">
                <header class="header">
                    <h1>{"todos"}</h1>
                    <input class="new-todo" placeholder="What needs to be done?" />
                </header>
                <section class="main">
                    <input id="toggle-all" class="toggle-all" type="checkbox" />
                    <label for="toggle-all" />
                    <TodoList />
                </section>
                <footer class="footer">
                    <span class="todo-count">
                        <strong>{0}</strong> {" item(s) left"}
                    </span>
                    <ul class="filters">
                        <li><a class="selected" href="#/all">{"All"}</a></li>
                        <li><a class="not-selected" href="#/active">{"Active"}</a></li>
                        <li><a class="not-selected" href="#/completed">{"Completed"}</a></li>
                    </ul>
                    <button class="clear-completed">
                        { format!("Clear completed ({})", 0) }
                    </button>
                </footer>
            </section>
            <footer class="info">
                <p>{"Double-click to edit a todo"}</p>
            </footer>
        </div>
    }
}
