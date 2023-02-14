# Parsing a JSON like structure

This exercise is inspired by the macro example in the [Programming Rust - Blandy, Orendorff & Tindall](LINK) book.

The goal is to create a declarative macro that provides a convenient way to create a Json structure in Rust from a JSON like string.

To go through the exercise, uncomment each test individually and try to make them pass.

## Hints

- For this exercise, here are the token types you would need:
  - `expr`
  - `tt`

- `expr` can only be followed by some specific characters such as `,` `;` etc...
- Help with usage of From trait to convert a value of a certain type into a Json value
    ```rust
    impl From<&str> for Json {
        fn from(value: &str) -> Self {
            todo!()
        }
    }
    ```
- When you need to return an expression after a series of statements you can use a block with `{}` to return the expected expression.
- Avoid repetition for each integer type by defining a macro that treat them all at once.
