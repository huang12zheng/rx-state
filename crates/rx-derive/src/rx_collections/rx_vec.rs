use std::ops::Deref;

use leptos_reactive::{create_rw_signal, RwSignal, Scope, SignalGetUntracked};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{MakeRx, MakeUnrx};

/// A reactive version of [`Vec`] that uses nested reactivity on its elements.
/// This requires nothing by `Clone + 'static` of the elements inside the
/// vector, and it wraps them in `RwSignal`s to make them reactive. If you want
/// to store nested reactive types inside the vector (e.g. `String`s), you
/// should use [`super::RxVecNested`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RxVec<T>(Vec<T>);
/// The reactive version of [`RxVec`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RxVecRx<T: 'static>(RwSignal<Vec<RwSignal<T>>>);
// --- Reactivity implementations ---
/// Clone is due to `self.id.with_no_subscription(self.runtime, Clone::clone)`
impl<T: Clone + 'static> MakeRx for RxVec<T> {
    type Rx = RxVecRx<T>;

    fn make_rx(self, scope: Scope) -> Self::Rx {
        RxVecRx(create_rw_signal(
            scope,
            self.0
                .into_iter()
                .map(|x| create_rw_signal(scope, x))
                .collect(),
        ))
    }
}
impl<T: Clone> MakeUnrx for RxVecRx<T> {
    type Unrx = RxVec<T>;

    fn make_unrx(self) -> Self::Unrx {
        let vec = self.0.get_untracked();
        RxVec(vec.into_iter().map(|x| (x.get_untracked())).collect())
    }
}
// --- Dereferencing ---
impl<T> Deref for RxVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> Deref for RxVecRx<T> {
    type Target = RwSignal<Vec<RwSignal<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// --- Conversion implementation ---
impl<T> From<Vec<T>> for RxVec<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}
