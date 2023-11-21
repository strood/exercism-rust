#[macro_export]
macro_rules! hashmap {
    (, $(,)? ) => {
        compile_error!("Invalid hashmap! macro invocation. Unexpected comma.");
    };
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut temp_hashmap = ::std::collections::HashMap::new();
            $(
                temp_hashmap.insert($key, $value);
            )*
            temp_hashmap
        }
    };
}
