use yew::{function_component, Html, html};

#[function_component(TodoList)]
pub fn todo_list() -> Html {
    html! {
        <ul class="todo-list">
            <li>
                <div class="view">
                    <input type="checkbox" class="toggle" />
                    <label>{"TODO 1a"}</label>
                    <button class="destroy" />
                </div>
                <input class="edit" type="text" value={"TODO 1a"} hidden={true} />
            </li>
            <li class={"completed"}>
                <div class="view">
                    <input type="checkbox" class="toggle" checked={true} />
                    <label>{"TODO 2a"}</label>
                    <button class="destroy" />
                </div>
                <input class="edit" type="text" value={"TODO 2a"} hidden={true} />
            </li>
            <li class={"editing"}>
                <div class="view">
                    <input type="checkbox" class="toggle" />
                    <label>{"TODO 3a"}</label>
                    <button class="destroy" />
                </div>
                <input class="edit" type="text" value={"TODO 3a"} />
            </li>
        </ul>
    }
}
