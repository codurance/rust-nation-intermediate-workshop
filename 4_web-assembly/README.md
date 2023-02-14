# Implementing TodoMVC

The goal of this exercise is to extract the current HTML content into a dynamic application using the `Yew` framework. 

This folder contains the source files for the [TodoMVC](https://todomvc.com/) example, you can see a running example following [this link](https://todomvc.com/examples/react/#/).

To start the project locally, please run the following command:

```sh
trunk serve
```

This command is watching all the files in the project and will recompile whenever a file is saved.

## Extract components

Let's start by extracting some components from the HTML view

- **TodoList**: Component that receives a list of todo and renders a list of `TodoDetails` Component
- **TodoDetails**: Component that display a given todo to render. A Todo can be either Viewed, Completed, Editing
- **AddTodo**: Component that display an input that is used to create a new Todo

## Load a list of static todos from a state

The next step is to introduce a state containing our list of todos. First step is to use a static list of Todos that we will make dynamic by reacting to some events.

You could use a state struct like the following one. It will be simpler to pass it down to a child component.

```rust
struct TodoListState {
    todos: Vec<Todos>
}
```

## Add a new Todo

- Implement a callback to an `onkeypress` event triggered by the `<input />` element.
- This callback should emit a `NewInputEntered` event when the "Enter" key was pressed and also reset
the input to a blank value.
- The parent element of `AddTodo` listens to the event using a callback and update the state with the
list where the new todo was added.
