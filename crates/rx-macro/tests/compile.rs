use rx_derive::*;
use rx_macro::ReactiveState;

#[derive(ReactiveState)]
pub struct MyStruct {
    pub a: i32,
    pub b: String,
    pub c: Vec<i32>,
}

#[derive(ReactiveState)]
pub struct MyStruct2 {
    #[rx(nested)]
    pub c: MyStruct,
    #[rx(nested)]
    pub d: RxVec<i32>,
}
