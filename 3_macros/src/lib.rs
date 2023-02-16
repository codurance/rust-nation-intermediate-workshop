use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

macro_rules! json {
    (null) => { Json::Null };
    ([ $( $value:tt ),* ]) => {
        {
            let array = vec![$( json!($value) ),*];
            Json::Array(array)
        }
    };
    ({ $( $key:tt : $value:tt ),+ }) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key.to_string(), json!($value));
            )+
            Json::Object(
                map
            )
        }
    };
    ($value:expr) => {
        Json::from($value)
    };
}

impl From<bool> for Json {
    fn from(value: bool) -> Self {
        Json::Boolean(value)
    }
}

impl From<&str> for Json {
    fn from(value: &str) -> Self {
        Json::String(value.into())
    }
}

macro_rules! impl_from_for_numbers {
    ($( $type:ty ),+) => {
        $(
            impl From<$type> for Json {
                fn from(value: $type) -> Self {
                    Json::Number(value as f64)
                }
            }
        )+
    }
}

impl_from_for_numbers!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_a_null_value() {
        assert_eq!(json!(null), Json::Null);
    }

    #[test]
    fn parse_a_valid_boolean() {
        assert_eq!(json!(true), Json::Boolean(true));
    }

    #[test]
    fn parse_a_valid_string() {
        assert_eq!(json!("Hello"), Json::String(String::from("Hello")));
    }

    #[test]
    fn parse_a_valid_array() {
        assert_eq!(
            json!(["a", "b", "c"]),
            Json::Array(vec![
                Json::String(String::from("a")),
                Json::String(String::from("b")),
                Json::String(String::from("c")),
            ])
        );
    }

    #[test]
    fn parse_a_valid_array_of_arrays() {
        assert_eq!(
            json!([["a", "b"], ["c"]]),
            Json::Array(vec![
                Json::Array(vec![
                    Json::String(String::from("a")),
                    Json::String(String::from("b"))
                ]),
                Json::Array(vec![Json::String(String::from("c"))]),
            ])
        );
    }

    #[test]
    fn parse_a_valid_object() {
        assert_eq!(json!({
            "Hello": "world",
            "Test": 1,
            "Names": [
                "John",
                "Doe"
            ]
        }), Json::Object(
            vec![
                (String::from("Hello"), Json::String(String::from("world"))),
                (String::from("Test"), Json::Number(1.0)),
                (String::from("Names"), Json::Array(vec![
                    Json::String(String::from("John")),
                    Json::String(String::from("Doe")),
                ])),
            ].into_iter().collect()
        ));
    }

    #[test]
    fn parse_a_valid_number() {
        assert_eq!(json!(1), Json::Number(1.0));
        assert_eq!(json!(1u8), Json::Number(1.0));
        assert_eq!(json!(1u16), Json::Number(1.0));
        assert_eq!(json!(1u32), Json::Number(1.0));
        assert_eq!(json!(1u64), Json::Number(1.0));
        assert_eq!(json!(1u128), Json::Number(1.0));
        assert_eq!(json!(1usize), Json::Number(1.0));
        assert_eq!(json!(1i8), Json::Number(1.0));
        assert_eq!(json!(1i16), Json::Number(1.0));
        assert_eq!(json!(1i32), Json::Number(1.0));
        assert_eq!(json!(1i64), Json::Number(1.0));
        assert_eq!(json!(1i128), Json::Number(1.0));
        assert_eq!(json!(1isize), Json::Number(1.0));
    }

    #[test]
    fn parse_a_valid_float() {
        assert_eq!(json!(1.0f32), Json::Number(1.0));
        assert_eq!(json!(1.0f64), Json::Number(1.0));
    }
}
