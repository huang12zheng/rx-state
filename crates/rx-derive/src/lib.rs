//! souce from https://github.com/framesurge/perseus/blob/main/packages/perseus-macro/src/rx_state.rs

mod rx_collections;
use leptos_reactive::Scope;
pub use rx_collections::*;

/// A trait for `struct`s that can be made reactive. Typically, this will be
/// derived with the `#[make_rx]` macro, though it can be implemented manually
/// if you have more niche requirements.
pub trait MakeRx {
    /// The type of the reactive version that we'll convert to. By having this
    /// as an associated type, we can associate the reactive type with the
    /// unreactive, meaning greater inference and fewer arguments that the
    /// user needs to provide to macros.
    type Rx: MakeUnrx;

    /// Transforms an instance of the `struct` into its reactive version.
    fn make_rx(self, scope: Scope) -> Self::Rx;
}

/// A trait for reactive `struct`s that can be made un-reactive. This is the
/// opposite of `MakeRx`, and is intended particularly for state freezing. Like
/// `MakeRx`, this will usually be derived automatically with the `#[make_rx]`
/// macro, but you can also implement it manually.
///
/// The types that implement this are typically referred to as the *intermediate
/// state* types, as they are rendered far more ergonomic to use by being put
/// through Sycamore's `create_ref()` function.
pub trait MakeUnrx {
    /// The type of the unreactive version that we'll convert to.
    type Unrx: MakeRx;
    /// Transforms an instance of the `struct` into its unreactive version. By
    /// having this as an associated type, we can associate the reactive type
    /// with the unreactive, meaning greater inference and fewer arguments
    /// that the user needs to provide to macros.
    fn make_unrx(self) -> Self::Unrx;
}
