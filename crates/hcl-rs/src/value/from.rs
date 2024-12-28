use super::{Map, Number, Value};
use std::borrow::Cow;

macro_rules! impl_from_integer {
    ($($ty:ty),*) => {
        $(
            impl<Capsule> From<$ty> for Value<Capsule> {
                fn from(n: $ty) -> Self {
                    Self::Number(n.into())
                }
            }
        )*
    };
}

impl_from_integer!(i8, i16, i32, i64, isize);
impl_from_integer!(u8, u16, u32, u64, usize);

impl<Capsule> From<f32> for Value<Capsule> {
    fn from(f: f32) -> Self {
        From::from(f as f64)
    }
}

impl<Capsule> From<f64> for Value<Capsule> {
    fn from(f: f64) -> Self {
        Number::from_f64(f).map_or(Value::Null, Value::Number)
    }
}

impl<Capsule> From<Number> for Value<Capsule> {
    fn from(num: Number) -> Self {
        Self::Number(num)
    }
}

impl<Capsule> From<bool> for Value<Capsule> {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl<Capsule> From<String> for Value<Capsule> {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl<Capsule> From<&str> for Value<Capsule> {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl<'a, Capsule> From<Cow<'a, str>> for Value<Capsule> {
    fn from(s: Cow<'a, str>) -> Self {
        Self::String(s.into_owned())
    }
}

impl<Capsule> From<Map<String, Value<Capsule>>> for Value<Capsule> {
    fn from(f: Map<String, Value<Capsule>>) -> Self {
        Self::Object(f)
    }
}

impl<Capsule, T: Into<Value<Capsule>>> From<Vec<T>> for Value<Capsule> {
    fn from(f: Vec<T>) -> Self {
        Self::Array(f.into_iter().map(Into::into).collect())
    }
}

impl<'a, Capsule, T: Clone + Into<Value<Capsule>>> From<&'a [T]> for Value<Capsule> {
    fn from(f: &'a [T]) -> Self {
        Self::Array(f.iter().cloned().map(Into::into).collect())
    }
}

impl<Capsule, T: Into<Value<Capsule>>> FromIterator<T> for Value<Capsule> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::Array(iter.into_iter().map(Into::into).collect())
    }
}

impl<Capsule, K: Into<String>, V: Into<Value<Capsule>>> FromIterator<(K, V)> for Value<Capsule> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Self::Object(
            iter.into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

impl<Capsule> From<()> for Value<Capsule> {
    fn from((): ()) -> Self {
        Self::Null
    }
}
