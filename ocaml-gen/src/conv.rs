//! Implementations of [`crate::OCamlDesc`] for types
//! that have natural equivalents in OCaml.
//! The OCaml description should be the corresponding data types in OCaml
//! This should correspond to the mapping defined in the ocaml-rs book:
//! `<https://github.com/zshipko/ocaml-rs/blob/v1.0.0-beta.4/doc/src/02_type_conversion.md>`
//! FIXME:
//! Unsigned types like uint16, uint32 and uint64 are not implemented as OCaml
//! does not provide types in the Stdlib for it. A custom block should be used.
//! Using [Stdint](https://github.com/andrenth/ocaml-stdint/) could be a solution.

use crate::{Env, OCamlDesc};
use const_random::const_random;

impl OCamlDesc for () {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "unit".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl<T> OCamlDesc for &T
where
    T: OCamlDesc,
{
    fn ocaml_desc(env: &Env, generics: &[&str]) -> String {
        T::ocaml_desc(env, generics)
    }

    fn unique_id() -> u128 {
        T::unique_id()
    }
}

impl OCamlDesc for [u8; 32] {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "bytes".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl OCamlDesc for &[u8] {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "bytes".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl OCamlDesc for &mut [u8] {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "bytes".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl<T> OCamlDesc for Vec<T>
where
    T: OCamlDesc,
{
    fn ocaml_desc(env: &Env, generics: &[&str]) -> String {
        format!("({}) array", T::ocaml_desc(env, generics))
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl<T, E> OCamlDesc for Result<T, E>
where
    T: OCamlDesc,
    E: OCamlDesc,
{
    fn ocaml_desc(env: &Env, generics: &[&str]) -> String {
        format!(
            "({}, {}) result",
            T::ocaml_desc(env, generics),
            E::ocaml_desc(env, generics)
        )
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl<T> OCamlDesc for Option<T>
where
    T: OCamlDesc,
{
    fn ocaml_desc(env: &Env, generics: &[&str]) -> String {
        format!("({}) option", T::ocaml_desc(env, generics))
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl OCamlDesc for usize {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "int".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl OCamlDesc for ocaml::Int {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "int".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl OCamlDesc for f32 {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "float".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl OCamlDesc for f64 {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "float".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl OCamlDesc for String {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "string".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl OCamlDesc for bool {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "bool".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl OCamlDesc for i8 {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "int".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl OCamlDesc for i16 {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "int".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl OCamlDesc for i32 {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "int32".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl OCamlDesc for i64 {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "int64".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl OCamlDesc for u8 {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "int".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl OCamlDesc for u16 {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "int".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl OCamlDesc for u32 {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "int32".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl OCamlDesc for u64 {
    fn ocaml_desc(_env: &Env, _generics: &[&str]) -> String {
        "int64".to_string()
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

impl<K, V> OCamlDesc for std::collections::BTreeMap<K, V>
where
    K: OCamlDesc,
    V: OCamlDesc,
{
    fn ocaml_desc(env: &Env, generics: &[&str]) -> String {
        let k = K::ocaml_desc(env, generics);
        let v = V::ocaml_desc(env, generics);
        format!("(({} * {}) list)", k, v)
    }

    fn unique_id() -> u128 {
        const_random!(u128) + K::unique_id() + V::unique_id()
    }
}

impl<T> OCamlDesc for std::collections::LinkedList<T>
where
    T: OCamlDesc,
{
    fn ocaml_desc(env: &Env, generics: &[&str]) -> String {
        let t = T::ocaml_desc(env, generics);
        format!("(({}) list)", t)
    }

    fn unique_id() -> u128 {
        const_random!(u128) + T::unique_id()
    }
}

impl<T> OCamlDesc for ocaml::Pointer<T>
where
    T: OCamlDesc,
{
    fn ocaml_desc(env: &Env, generics: &[&str]) -> String {
        T::ocaml_desc(env, generics)
    }

    fn unique_id() -> u128 {
        const_random!(u128)
    }
}

macro_rules! impl_ocaml_desc_for_tuple {
    ($($idx:tt),+) => {
        impl<$($idx: OCamlDesc,)+> OCamlDesc for ($($idx,)+) {
            fn ocaml_desc(env: &Env, generics: &[&str]) -> String {
                let v = vec![
                    $($idx::ocaml_desc(env, generics),)+
                ];
                format!("({})", v.join(" * "))
            }

            fn unique_id() -> u128 {
                const_random!(u128)
            }
        }
    };
}

// Implement OCamlDesc for tuples of sizes 2 to 20
impl_ocaml_desc_for_tuple!(T1, T2);
impl_ocaml_desc_for_tuple!(T1, T2, T3);
impl_ocaml_desc_for_tuple!(T1, T2, T3, T4);
impl_ocaml_desc_for_tuple!(T1, T2, T3, T4, T5);
impl_ocaml_desc_for_tuple!(T1, T2, T3, T4, T5, T6);
impl_ocaml_desc_for_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_ocaml_desc_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_ocaml_desc_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_ocaml_desc_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_ocaml_desc_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_ocaml_desc_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_ocaml_desc_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_ocaml_desc_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_ocaml_desc_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_ocaml_desc_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
impl_ocaml_desc_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17
);
impl_ocaml_desc_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18
);
impl_ocaml_desc_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19
);
impl_ocaml_desc_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20
);
