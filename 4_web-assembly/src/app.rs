use yew::prelude::*;

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
                    <ul class="todo-list">
                        <li>
                            <div class="view">
                                <input type="checkbox" class="toggle" />
                                <label>{"TODO 1"}</label>
                                <button class="destroy" />
                            </div>
                            <input class="edit" type="text" value={"TODO 1"} hidden={true} />
                        </li>
                        <li class={"completed"}>
                            <div class="view">
                                <input type="checkbox" class="toggle" checked={true} />
                                <label>{"TODO 2"}</label>
                                <button class="destroy" />
                            </div>
                            <input class="edit" type="text" value={"TODO 2"} hidden={true} />
                        </li>
                        <li class={"editing"}>
                            <div class="view">
                                <input type="checkbox" class="toggle" />
                                <label>{"TODO 3"}</label>
                                <button class="destroy" />
                            </div>
                            <input class="edit" type="text" value={"TODO 3"} />
                        </li>
                    </ul>
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
