mod tests;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, serde::Deserialize)]
#[serde(untagged)]
pub enum Option3<T> {
    None,
    Null,
    Some(T),
}

impl<T: serde::Serialize> serde::Serialize for Option3<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            Option3::Some(x) => serializer.serialize_some(x),
            Option3::Null => serializer.serialize_none(),
            Option3::None => serializer.serialize_unit(),
        }
    }
}

impl<T> Default for Option3<T> {
    fn default() -> Self {
        Option3::None
    }
}

impl<T> Option3<T> {
    pub fn unwrap(self) -> T {
        match self {
            Option3::Some(val) => val,
            _ => panic!("called `Option3::unwrap()` on a `None` or `Null` value"),
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Option3::Some(x) => x,
            _ => default,
        }
    }

    pub const fn is_some(&self) -> bool {
        matches!(*self, Option3::Some(_))
    }

    pub const fn is_none(&self) -> bool {
        matches!(*self, Option3::None)
    }

    pub const fn is_null(&self) -> bool {
        matches!(*self, Option3::Null)
    }

    pub const fn as_ref(&self) -> Option3<&T> {
        match *self {
            Option3::Some(ref x) => Option3::Some(x),
            Option3::None => Option3::None,
            Option3::Null => Option3::Null,
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option3<U> {
        match self {
            Option3::Some(x) => Option3::Some(f(x)),
            Option3::None =>  Option3::None,
            Option3::Null =>  Option3::Null,
        }
    }

    pub fn expect(self, msg: &str) -> T {
        match self {
            Option3::Some(val) => val,
            Option3::None => expect_failed(msg),
            Option3::Null => expect_failed(msg),
        }
    }

    pub fn filter<P: FnOnce(&T) -> bool>(self, predicate: P) -> Option3<T> {
        if let Option3::Some(x) = self {
            if predicate(&x) {
                return Option3::Some(x);
            }
        }
        Option3::None
    }

    pub fn to_option(self) -> Option<T> {
        match self {
            Option3::Some(x) => Some(x),
            _ => None,
        }
    }

    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Option3::Some(v) => Ok(v),
            _ => Err(err),
        }
    }
}

impl<T: Clone> Option3<&T> {
    pub fn cloned(self) -> Option3<T> {
        self.map(|t| t.clone())
    }
}

impl<T: Clone> Option3<&mut T> {
    pub fn cloned(self) -> Option3<T> {
        self.map(|t| t.clone())
    }
}

impl<T> Option3<Option3<T>> {
    pub fn flatten(self) -> Option3<T> {
        match self {
            Option3::Some(Option3::Some(x)) => Option3::Some(x),
            Option3::Some(r) => r,
            Option3::None => Option3::None,
            Option3::Null => Option3::Null,
        }
    }
}

fn expect_failed(msg: &str) -> ! {
    panic!("{}", msg)
}

impl<T> From<Option<T>> for Option3<T> {
    fn from(t: Option<T>) -> Self {
        match t {
            Some(thing) => Self::Some(thing),
            None => Self::None,
        }
    }
}
pub mod option3_box {
    pub fn is_none<T>(t: &Box<crate::Option3<T>>) -> bool {
        matches!(**t, crate::Option3::None)
    }
}
