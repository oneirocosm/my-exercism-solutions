#[macro_export(local_inner_macros)]
macro_rules! hashmap {
    ( $( $key:expr => $val:expr,)+ ) => { $crate::hashmap!( $( $key => $val),+ ) };
    ( $( $key:expr => $val:expr),* ) => {
        {
            let mut temp_hashmap = ::std::collections::HashMap::new();
            $(
                temp_hashmap.insert($key, $val);
            )*
            temp_hashmap
        }
    };
}
