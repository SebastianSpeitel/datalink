use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasher,
};

use super::{Links, Result};
use crate::data::{
    unique::{Fixed, MaybeUnique},
    BoxedData,
};

impl Links for Vec<(Option<BoxedData>, BoxedData)> {
    #[inline]
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result {
        self.push((key, target));
        Ok(())
    }
}

impl Links for Vec<(BoxedData, BoxedData)> {
    #[inline]
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result {
        if let Some(key) = key {
            self.push((key, target));
        }
        Ok(())
    }

    #[inline]
    fn push_unkeyed(&mut self, _target: BoxedData) -> Result {
        Ok(())
    }

    #[inline]
    fn push_keyed(&mut self, target: BoxedData, key: BoxedData) -> Result {
        self.push((key, target));
        Ok(())
    }
}

impl Links for Vec<BoxedData> {
    #[inline]
    fn push(&mut self, target: BoxedData, _key: Option<BoxedData>) -> Result {
        self.push(target);
        Ok(())
    }

    #[inline]
    fn push_unkeyed(&mut self, target: BoxedData) -> Result {
        self.push(target);
        Ok(())
    }

    #[inline]
    fn push_keyed(&mut self, target: BoxedData, _key: BoxedData) -> Result {
        self.push(target);
        Ok(())
    }
}

impl Links for Option<(BoxedData, BoxedData)> {
    #[inline]
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result {
        if self.is_some() {
            return Ok(());
        }
        if let Some(key) = key {
            self.replace((key, target));
        }
        Ok(())
    }
}

impl Links for Option<(Option<BoxedData>, BoxedData)> {
    #[inline]
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result {
        if self.is_some() {
            return Ok(());
        }
        self.replace((key, target));
        Ok(())
    }
}

impl Links for Option<BoxedData> {
    #[inline]
    fn push(&mut self, target: BoxedData, _key: Option<BoxedData>) -> Result {
        if self.is_some() {
            return Ok(());
        }
        self.replace(target);
        Ok(())
    }
}

impl<S: BuildHasher> Links for HashMap<Fixed<BoxedData>, BoxedData, S> {
    #[inline]
    fn push(&mut self, target: BoxedData, key: Option<BoxedData>) -> Result {
        if let Some(key) = key {
            if let Ok(key) = key.try_into_unique() {
                self.insert(key, target);
            }
        }
        Ok(())
    }
    #[inline]
    fn push_unkeyed(&mut self, _target: BoxedData) -> Result {
        Ok(())
    }
    #[inline]
    fn push_keyed(&mut self, target: BoxedData, key: BoxedData) -> Result {
        if let Ok(key) = key.try_into_unique() {
            self.insert(key, target);
        }
        Ok(())
    }
}

impl<S: BuildHasher> Links for HashSet<Fixed<BoxedData>, S> {
    #[inline]
    fn push(&mut self, target: BoxedData, _key: Option<BoxedData>) -> Result {
        if let Ok(target) = target.try_into_unique() {
            self.insert(target);
        }
        Ok(())
    }
    #[inline]
    fn push_unkeyed(&mut self, target: BoxedData) -> Result {
        if let Ok(target) = target.try_into_unique() {
            self.insert(target);
        }
        Ok(())
    }
    #[inline]
    fn push_keyed(&mut self, target: BoxedData, _key: BoxedData) -> Result {
        if let Ok(target) = target.try_into_unique() {
            self.insert(target);
        }
        Ok(())
    }
}
