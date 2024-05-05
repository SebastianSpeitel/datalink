#[cfg(feature = "unique")]
use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasher,
};

use super::{Links, MaybeKeyed, Result, BREAK, CONTINUE};
#[cfg(feature = "unique")]
use crate::data::unique::{Fixed, MaybeUnique};
use crate::data::BoxedData;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Linked {
    #[default]
    No,
    Yes,
}

impl Links for Linked {
    #[inline]
    fn push(&mut self, _target: BoxedData, _key: Option<BoxedData>) -> Result {
        *self = Linked::Yes;
        BREAK
    }

    #[inline]
    fn push_unkeyed(&mut self, _target: BoxedData) -> Result {
        *self = Linked::Yes;
        BREAK
    }

    #[inline]
    fn push_keyed(&mut self, _target: BoxedData, _key: BoxedData) -> Result {
        *self = Linked::Yes;
        BREAK
    }
}

impl Links for Vec<MaybeKeyed<BoxedData, BoxedData>> {
    #[inline]
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result {
        self.push(MaybeKeyed::new(key, target));
        CONTINUE
    }

    #[inline]
    fn push_unkeyed(&mut self, target: BoxedData) -> Result {
        self.push(MaybeKeyed::Unkeyed(target));
        CONTINUE
    }

    #[inline]
    fn push_keyed(&mut self, target: BoxedData, key: BoxedData) -> Result {
        self.push(MaybeKeyed::Keyed(key, target));
        CONTINUE
    }
}

impl Links for Vec<(Option<BoxedData>, BoxedData)> {
    #[inline]
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result {
        self.push((key, target));
        CONTINUE
    }
}

impl Links for Vec<(BoxedData, BoxedData)> {
    #[inline]
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result {
        if let Some(key) = key {
            self.push((key, target));
        }
        CONTINUE
    }

    #[inline]
    fn push_unkeyed(&mut self, _target: BoxedData) -> Result {
        CONTINUE
    }

    #[inline]
    fn push_keyed(&mut self, target: BoxedData, key: BoxedData) -> Result {
        self.push((key, target));
        CONTINUE
    }
}

impl Links for Vec<BoxedData> {
    #[inline]
    fn push(&mut self, target: BoxedData, _key: Option<BoxedData>) -> Result {
        self.push(target);
        CONTINUE
    }

    #[inline]
    fn push_unkeyed(&mut self, target: BoxedData) -> Result {
        self.push(target);
        CONTINUE
    }

    #[inline]
    fn push_keyed(&mut self, target: BoxedData, _key: BoxedData) -> Result {
        self.push(target);
        CONTINUE
    }
}

impl Links for Option<(BoxedData, BoxedData)> {
    #[inline]
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result {
        if self.is_some() {
            return BREAK;
        }
        if let Some(key) = key {
            self.replace((key, target));
        }
        BREAK
    }
}

impl Links for Option<(Option<BoxedData>, BoxedData)> {
    #[inline]
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result {
        if self.is_some() {
            return BREAK;
        }
        self.replace((key, target));
        BREAK
    }
}

impl Links for Option<BoxedData> {
    #[inline]
    fn push(&mut self, target: BoxedData, _key: Option<BoxedData>) -> Result {
        if self.is_some() {
            return BREAK;
        }
        self.replace(target);
        BREAK
    }
}

#[cfg(feature = "unique")]
impl<S: BuildHasher> Links for HashMap<Fixed<BoxedData>, BoxedData, S> {
    #[inline]
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result {
        if let Some(key) = key {
            if let Ok(key) = key.try_into_unique() {
                self.insert(key, target);
            }
        }
        CONTINUE
    }
    #[inline]
    fn push_unkeyed(&mut self, _target: BoxedData) -> Result {
        CONTINUE
    }
    #[inline]
    fn push_keyed(&mut self, target: BoxedData, key: BoxedData) -> Result {
        if let Ok(key) = key.try_into_unique() {
            self.insert(key, target);
        }
        CONTINUE
    }
}

#[cfg(feature = "unique")]
impl<S: BuildHasher> Links for HashSet<Fixed<BoxedData>, S> {
    #[inline]
    fn push(&mut self, target: BoxedData, _key: Option<BoxedData>) -> Result {
        if let Ok(target) = target.try_into_unique() {
            self.insert(target);
        }
        CONTINUE
    }
    #[inline]
    fn push_unkeyed(&mut self, target: BoxedData) -> Result {
        if let Ok(target) = target.try_into_unique() {
            self.insert(target);
        }
        CONTINUE
    }
    #[inline]
    fn push_keyed(&mut self, target: BoxedData, _key: BoxedData) -> Result {
        if let Ok(target) = target.try_into_unique() {
            self.insert(target);
        }
        CONTINUE
    }
}
