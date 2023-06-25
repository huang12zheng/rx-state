use rx_derive::*;
use rx_macro::ReactiveState;

#[test]
fn compile_derive() {
    use crate::*;
    #[derive(ReactiveState)]
    pub struct MyStruct {
        pub a: i32,
        pub b: String,
        pub c: Vec<i32>,
    }
}
