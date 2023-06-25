use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;

use leptos_reactive::{create_rw_signal, RwSignal, Scope, SignalGetUntracked};
#[cfg(feature = "serde")]
use serde::{Deserialize, DeserializeOwned, Serialize};

use crate::{MakeRx, MakeUnrx};

/// A reactive version of [`Vec`] that uses nested reactivity on its elements.
/// This requires nothing by `Clone + 'static` of the elements inside the map,
/// and it wraps them in `RwSignal`s to make them reactive. If you want to store
/// nested reactive types inside the map (e.g. `String`s), you should
/// use [`super::RxHashMapNested`].
// #[derive(Clone, Debug, PartialEq, Eq)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RxHashMap<K, V>(HashMap<K, V>);

/// The reactive version of [`RxHashMap`].
#[derive(Clone, Debug)]
// #[derive(Clone, Debug, PartialEq, Eq)]
pub struct RxHashMapRx<K: 'static, V: 'static>(RwSignal<HashMap<K, RwSignal<V>>>);

// --- Reactivity implementations ---
impl<K: 'static + Eq + Hash + Clone, V: 'static + Clone> MakeRx for RxHashMap<K, V> {
    type Rx = RxHashMapRx<K, V>;

    fn make_rx(self, scope: Scope) -> Self::Rx {
        RxHashMapRx(create_rw_signal(
            scope,
            self.0
                .into_iter()
                .map(|(k, v)| (k, create_rw_signal(scope, v)))
                .collect(),
        ))
    }
}
impl<K: Eq + Hash + Clone, V: Clone> MakeUnrx for RxHashMapRx<K, V> {
    type Unrx = RxHashMap<K, V>;

    fn make_unrx(self) -> Self::Unrx {
        let map = self.0.get_untracked();
        RxHashMap(
            map.into_iter()
                .map(|(k, v)| (k, v.get_untracked()))
                .collect(),
        )
    }

    #[cfg(any(client, doc))]
    fn compute_suspense(&self, _cx: Scope) {}
}
// --- Dereferencing ---
impl<K: Hash, V> Deref for RxHashMap<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<K, V> Deref for RxHashMapRx<K, V> {
    type Target = RwSignal<HashMap<K, RwSignal<V>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// --- Conversion implementation ---
impl<K: Hash, V> From<HashMap<K, V>> for RxHashMap<K, V> {
    fn from(value: HashMap<K, V>) -> Self {
        Self(value)
    }
}
