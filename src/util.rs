use core::convert::Infallible;
use core::{marker::PhantomData, ops::Deref};

#[allow(clippy::inline_always)]
#[inline(always)]
pub(crate) fn cast_opt<T: 'static>(opt: &mut Option<impl Sized + 'static>) -> Option<T> {
    <dyn core::any::Any>::downcast_mut(opt).and_then(Option::take)
}
#[allow(clippy::inline_always)]
#[inline(always)]
pub(crate) fn cast_ref<T: 'static>(r#ref: &(impl Sized + 'static)) -> Option<&T> {
    <dyn core::any::Any>::downcast_ref(r#ref)
}

pub trait Eraser<T> {
    type Erased: 'static;
    fn erase(data: T) -> Self::Erased;
}

impl<T: 'static> Eraser<T> for () {
    type Erased = T;
    #[inline]
    fn erase(data: T) -> Self::Erased {
        data
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EraseToOwned<T>(PhantomData<T>);

impl<T> Default for EraseToOwned<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> EraseToOwned<T> {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}
impl<T> Eraser<T> for EraseToOwned<T>
where
    T: Deref<Target: ToOwned<Owned: 'static>>,
{
    type Erased = <T::Target as ToOwned>::Owned;
    #[inline]
    fn erase(data: T) -> Self::Erased {
        data.deref().to_owned()
    }
}

#[inline]
pub const fn always<T>(value: T) -> Result<T, Infallible> {
    Ok(value)
}

pub trait Maybe<T: Copy> {
    fn get(&self) -> Result<T, impl Sized>;

    #[inline]
    fn eq<U>(&self, other: U) -> bool
    where
        T: PartialEq<U>,
        Self: Sized,
    {
        self.get().is_ok_and(|v| v == other)
    }
}

impl<T: Copy> Maybe<T> for Option<T> {
    #[inline]
    fn get(&self) -> Result<T, impl Sized> {
        self.ok_or(())
    }
}

impl<T: Copy, E: Copy> Maybe<T> for Result<T, E> {
    #[inline]
    fn get(&self) -> Result<T, impl Sized> {
        *self
    }
    #[inline]
    fn eq<U>(&self, other: U) -> bool
    where
        T: PartialEq<U>,
    {
        self.is_ok_and(|v| v == other)
    }
}

impl<T: Copy> Maybe<T> for () {
    #[inline]
    fn get(&self) -> Result<T, impl Sized> {
        Err(())
    }
    #[inline]
    fn eq<U>(&self, _other: U) -> bool {
        false
    }
}

impl<T: Copy> Maybe<T> for Infallible {
    #[inline]
    fn get(&self) -> Result<T, impl Sized> {
        Err(*self)
    }
    #[inline]
    fn eq<U>(&self, _other: U) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Union<T, U>(pub T, pub U);

impl<A: crate::Data, B: crate::Data> crate::Data for Union<A, B> {
    #[inline]
    fn query(&self, mut request: impl crate::Request) {
        self.0.query(request.by_ref());
        self.1.query(request);
    }
    #[inline]
    fn query_owned(self, mut request: impl crate::Request) {
        self.0.query_owned(request.by_ref());
        self.1.query_owned(request);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Intersection<T, U>(pub T, pub U);

#[derive(Debug, Clone, Copy)]
pub enum Bool<T = (), F = ()> {
    True(T),
    False(F),
}

impl<T: Default, F: Default> Bool<T, F> {
    #[inline]
    #[must_use]
    pub fn r#true() -> Self {
        Self::True(T::default())
    }

    #[inline]
    #[must_use]
    pub fn r#false() -> Self {
        Self::False(F::default())
    }
}

impl<T, F> PartialEq<bool> for Bool<T, F> {
    #[inline]
    fn eq(&self, other: &bool) -> bool {
        match self {
            Self::True(_) => *other,
            Self::False(_) => !*other,
        }
    }
}
impl<T, F> PartialEq<bool> for &Bool<T, F> {
    #[inline]
    fn eq(&self, other: &bool) -> bool {
        match self {
            Bool::True(_) => *other,
            Bool::False(_) => !*other,
        }
    }
}
pub type True = Bool<(), Infallible>;
pub type False = Bool<Infallible, ()>;

impl Default for True {
    #[inline]
    fn default() -> Self {
        Self::True(())
    }
}
impl Default for False {
    #[inline]
    fn default() -> Self {
        Self::False(())
    }
}

pub const TRUE: True = True::True(());
pub const FALSE: False = False::False(());

#[derive(Debug, Clone, Copy)]
pub enum Never {}

impl PartialEq<bool> for Never {
    #[inline]
    fn eq(&self, _: &bool) -> bool {
        false
    }
}

#[allow(clippy::copy_iterator)]
impl Iterator for Never {
    type Item = core::any::TypeId;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// #[doc(hidden)]
// pub unsafe trait IntoTrait<T: ?Sized>: Sized {
//     #[doc(hidden)]
//     #[inline]
//     fn into_link_schema(self) -> impl crate::schema::LinkValidator
//     where
//         T: crate::schema::LinkValidator,
//     {
//         TRUE
//     }
//     #[doc(hidden)]
//     #[inline]
//     fn into_data_schema(self) -> impl crate::schema::DataValidator
//     where
//         T: crate::schema::DataValidator,
//     {
//         TRUE
//     }
//     #[doc(hidden)]
//     #[inline]
//     fn into_value_schema(self) -> impl crate::schema::ValueValidator
//     where
//         T: crate::schema::ValueValidator,
//     {
//         TRUE
//     }
//     #[doc(hidden)]
//     #[inline]
//     fn into_type_schema(self) -> impl crate::schema::TypeValidator
//     where
//         T: crate::schema::TypeValidator,
//     {
//         TRUE
//     }
// }

// /// A kind of iterator which can change its type as it iterates.
// pub trait Snek<M: ?Sized> {
//     type Head: IntoTrait<M>;
//     type Tail: Snek<M>;

//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail);

//     fn len(&self) -> usize;

//     #[inline]
//     fn is_empty(&self) -> bool {
//         self.len() == 0
//     }

//     #[inline]
//     fn for_each<F: SnekFn<M>>(self, mut f: F)
//     where
//         Self: Sized,
//     {
//         if let (Ok(head), tail) = self.split() {
//             f.call(head);
//             tail.for_each(f);
//         }
//     }

//     #[inline]
//     fn try_for_each<F, B>(self, mut f: F) -> ControlFlow<B>
//     where
//         Self: Sized,
//         F: SnekFn<M, Output = ControlFlow<B>>,
//         B: Default,
//     {
//         if let (Ok(head), tail) = self.split() {
//             f.call(head)?;
//             tail.try_for_each(f)
//         } else {
//             ControlFlow::Break(B::default())
//         }
//     }
// }

// pub trait SnekFn<M: ?Sized> {
//     type Output;
//     fn call(&mut self, item: impl IntoTrait<M>) -> Self::Output;
// }

// impl<M> Snek<M> for Infallible
// where
//     Self: IntoTrait<M>,
// {
//     type Head = Self;
//     type Tail = Self;

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (Err::<Self, _>(()), self)
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         0
//     }
// }

// impl<T, M> Snek<M> for Option<T>
// where
//     T: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T;
//     type Tail = ();
//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         self.map_or_else(|| (Err(()), ()), |h| (Ok(h), ()))
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         match self {
//             Some(_) => 1,
//             None => 0,
//         }
//     }
// }

// impl<T, M> Snek<M> for Vec<T>
// where
//     T: IntoTrait<M>,
// {
//     type Head = T;
//     type Tail = Self;

//     #[inline]
//     fn split(mut self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         match self.pop() {
//             Some(head) => (Ok(head), self),
//             None => (Err(()), self),
//         }
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         self.len()
//     }
// }

// impl<T, M> Snek<M> for Box<[T]>
// where
//     T: IntoTrait<M>,
// {
//     type Head = T;
//     type Tail = Vec<T>;

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         let mut vec = self.into_vec();
//         match vec.pop() {
//             Some(head) => (Ok(head), vec),
//             None => (Err(()), vec),
//         }
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         self.as_ref().len()
//     }
// }

// impl<'a, T, M> Snek<M> for std::borrow::Cow<'a, [T]>
// where
//     T: Clone,
//     std::borrow::Cow<'a, T>: IntoTrait<M>,
// {
//     type Head = std::borrow::Cow<'a, T>;
//     type Tail = Self;

//     #[inline]
//     #[allow(clippy::option_if_let_else)]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         use std::borrow::Cow;
//         match self {
//             Self::Borrowed([]) => (Err(()), Self::Borrowed(&[])),
//             Self::Borrowed([first, rest @ ..]) => (Ok(Cow::Borrowed(first)), Self::Borrowed(rest)),
//             Self::Owned(mut vec) => match vec.pop() {
//                 Some(head) => (Ok(Cow::Owned(head)), Self::Owned(vec)),
//                 None => (Err(()), Self::Borrowed(&[])),
//             },
//         }
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         self.as_ref().len()
//     }
// }

// impl<M, T> Snek<M> for [T; 0]
// where
//     Infallible: IntoTrait<M>,
// {
//     type Head = Infallible;
//     type Tail = ();

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (Err(()), ())
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         0
//     }
// }
// impl<M, T> Snek<M> for [T; 1]
// where
//     T: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T;
//     type Tail = ();

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         let [head] = self;
//         (Ok::<_, Infallible>(head), ())
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         1
//     }
// }
// macro_rules! impl_array_snek {
//     ($($l:literal->$t:literal)*) => {
//         $(
//             impl<M, T> Snek<M> for [T; $l]
//             where
//                 T: IntoTrait<M>,
//                 Infallible: IntoTrait<M>,
//             {
//                 type Head = T;
//                 type Tail = [T; $t];

//                 #[inline]
//                 fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//                     let [head, tail @ ..] = self;
//                     (Ok::<_, Infallible>(head), tail)
//                 }
//                 #[inline]
//                 fn len(&self) -> usize {
//                     $l
//                 }
//             }
//         )*
//     };
// }
// impl_array_snek! {
//     2->1
//     3->2
//     4->3
//     5->4
//     6->5
//     7->6
//     8->7
//     9->8
//     10->9
//     11->10
//     12->11
//     13->12
//     14->13
//     15->14
// }

// impl<'a, T, M> Snek<M> for &'a [T]
// where
//     &'a T: IntoTrait<M>,
// {
//     type Head = &'a T;
//     type Tail = Self;

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         match self.split_first() {
//             Some((head, tail)) => (Ok(head), tail),
//             None => (Err(()), &[]),
//         }
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         (*self).len()
//     }
// }

// impl<M> Snek<M> for ()
// where
//     Infallible: IntoTrait<M>,
// {
//     type Head = Infallible;
//     type Tail = Self;

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (Err(()), self)
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         0
//     }
// }

// impl<T, M> Snek<M> for (T,)
// where
//     T: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T;
//     type Tail = ();

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (Ok::<_, Infallible>(self.0), ())
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         1
//     }
// }

// impl<T0, T1, M> Snek<M> for (T0, T1)
// where
//     T0: IntoTrait<M>,
//     T1: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T0;
//     type Tail = (T1,);

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (Ok::<_, Infallible>(self.0), (self.1,))
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         2
//     }
// }

// impl<T0, T1, T2, M> Snek<M> for (T0, T1, T2)
// where
//     T0: IntoTrait<M>,
//     T1: IntoTrait<M>,
//     T2: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T0;
//     type Tail = (T1, T2);

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (Ok::<_, Infallible>(self.0), (self.1, self.2))
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         3
//     }
// }

// impl<T0, T1, T2, T3, M> Snek<M> for (T0, T1, T2, T3)
// where
//     T0: IntoTrait<M>,
//     T1: IntoTrait<M>,
//     T2: IntoTrait<M>,
//     T3: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T0;
//     type Tail = (T1, T2, T3);

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (Ok::<_, Infallible>(self.0), (self.1, self.2, self.3))
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         4
//     }
// }

// impl<T0, T1, T2, T3, T4, M> Snek<M> for (T0, T1, T2, T3, T4)
// where
//     T0: IntoTrait<M>,
//     T1: IntoTrait<M>,
//     T2: IntoTrait<M>,
//     T3: IntoTrait<M>,
//     T4: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T0;
//     type Tail = (T1, T2, T3, T4);

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (
//             Ok::<_, Infallible>(self.0),
//             (self.1, self.2, self.3, self.4),
//         )
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         5
//     }
// }

// impl<T0, T1, T2, T3, T4, T5, M> Snek<M> for (T0, T1, T2, T3, T4, T5)
// where
//     T0: IntoTrait<M>,
//     T1: IntoTrait<M>,
//     T2: IntoTrait<M>,
//     T3: IntoTrait<M>,
//     T4: IntoTrait<M>,
//     T5: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T0;
//     type Tail = (T1, T2, T3, T4, T5);

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (
//             Ok::<_, Infallible>(self.0),
//             (self.1, self.2, self.3, self.4, self.5),
//         )
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         6
//     }
// }

// impl<T0, T1, T2, T3, T4, T5, T6, M> Snek<M> for (T0, T1, T2, T3, T4, T5, T6)
// where
//     T0: IntoTrait<M>,
//     T1: IntoTrait<M>,
//     T2: IntoTrait<M>,
//     T3: IntoTrait<M>,
//     T4: IntoTrait<M>,
//     T5: IntoTrait<M>,
//     T6: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T0;
//     type Tail = (T1, T2, T3, T4, T5, T6);

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (
//             Ok::<_, Infallible>(self.0),
//             (self.1, self.2, self.3, self.4, self.5, self.6),
//         )
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         7
//     }
// }
// impl<T0, T1, T2, T3, T4, T5, T6, T7, M> Snek<M> for (T0, T1, T2, T3, T4, T5, T6, T7)
// where
//     T0: IntoTrait<M>,
//     T1: IntoTrait<M>,
//     T2: IntoTrait<M>,
//     T3: IntoTrait<M>,
//     T4: IntoTrait<M>,
//     T5: IntoTrait<M>,
//     T6: IntoTrait<M>,
//     T7: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T0;
//     type Tail = (T1, T2, T3, T4, T5, T6, T7);

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (
//             Ok::<_, Infallible>(self.0),
//             (self.1, self.2, self.3, self.4, self.5, self.6, self.7),
//         )
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         8
//     }
// }
// impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, M> Snek<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
// where
//     T0: IntoTrait<M>,
//     T1: IntoTrait<M>,
//     T2: IntoTrait<M>,
//     T3: IntoTrait<M>,
//     T4: IntoTrait<M>,
//     T5: IntoTrait<M>,
//     T6: IntoTrait<M>,
//     T7: IntoTrait<M>,
//     T8: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T0;
//     type Tail = (T1, T2, T3, T4, T5, T6, T7, T8);

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (
//             Ok::<_, Infallible>(self.0),
//             (
//                 self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8,
//             ),
//         )
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         9
//     }
// }

// impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M> Snek<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
// where
//     T0: IntoTrait<M>,
//     T1: IntoTrait<M>,
//     T2: IntoTrait<M>,
//     T3: IntoTrait<M>,
//     T4: IntoTrait<M>,
//     T5: IntoTrait<M>,
//     T6: IntoTrait<M>,
//     T7: IntoTrait<M>,
//     T8: IntoTrait<M>,
//     T9: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T0;
//     type Tail = (T1, T2, T3, T4, T5, T6, T7, T8, T9);

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (
//             Ok::<_, Infallible>(self.0),
//             (
//                 self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9,
//             ),
//         )
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         10
//     }
// }

// impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M> Snek<M>
//     for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
// where
//     T0: IntoTrait<M>,
//     T1: IntoTrait<M>,
//     T2: IntoTrait<M>,
//     T3: IntoTrait<M>,
//     T4: IntoTrait<M>,
//     T5: IntoTrait<M>,
//     T6: IntoTrait<M>,
//     T7: IntoTrait<M>,
//     T8: IntoTrait<M>,
//     T9: IntoTrait<M>,
//     T10: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T0;
//     type Tail = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (
//             Ok::<_, Infallible>(self.0),
//             (
//                 self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
//             ),
//         )
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         11
//     }
// }
// impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M> Snek<M>
//     for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
// where
//     T0: IntoTrait<M>,
//     T1: IntoTrait<M>,
//     T2: IntoTrait<M>,
//     T3: IntoTrait<M>,
//     T4: IntoTrait<M>,
//     T5: IntoTrait<M>,
//     T6: IntoTrait<M>,
//     T7: IntoTrait<M>,
//     T8: IntoTrait<M>,
//     T9: IntoTrait<M>,
//     T10: IntoTrait<M>,
//     T11: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T0;
//     type Tail = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (
//             Ok::<_, Infallible>(self.0),
//             (
//                 self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
//                 self.11,
//             ),
//         )
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         12
//     }
// }
// impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M> Snek<M>
//     for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
// where
//     T0: IntoTrait<M>,
//     T1: IntoTrait<M>,
//     T2: IntoTrait<M>,
//     T3: IntoTrait<M>,
//     T4: IntoTrait<M>,
//     T5: IntoTrait<M>,
//     T6: IntoTrait<M>,
//     T7: IntoTrait<M>,
//     T8: IntoTrait<M>,
//     T9: IntoTrait<M>,
//     T10: IntoTrait<M>,
//     T11: IntoTrait<M>,
//     T12: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T0;
//     type Tail = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (
//             Ok::<_, Infallible>(self.0),
//             (
//                 self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
//                 self.11, self.12,
//             ),
//         )
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         13
//     }
// }
// impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M> Snek<M>
//     for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
// where
//     T0: IntoTrait<M>,
//     T1: IntoTrait<M>,
//     T2: IntoTrait<M>,
//     T3: IntoTrait<M>,
//     T4: IntoTrait<M>,
//     T5: IntoTrait<M>,
//     T6: IntoTrait<M>,
//     T7: IntoTrait<M>,
//     T8: IntoTrait<M>,
//     T9: IntoTrait<M>,
//     T10: IntoTrait<M>,
//     T11: IntoTrait<M>,
//     T12: IntoTrait<M>,
//     T13: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T0;
//     type Tail = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (
//             Ok::<_, Infallible>(self.0),
//             (
//                 self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
//                 self.11, self.12, self.13,
//             ),
//         )
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         14
//     }
// }
// impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> Snek<M>
//     for (
//         T0,
//         T1,
//         T2,
//         T3,
//         T4,
//         T5,
//         T6,
//         T7,
//         T8,
//         T9,
//         T10,
//         T11,
//         T12,
//         T13,
//         T14,
//     )
// where
//     T0: IntoTrait<M>,
//     T1: IntoTrait<M>,
//     T2: IntoTrait<M>,
//     T3: IntoTrait<M>,
//     T4: IntoTrait<M>,
//     T5: IntoTrait<M>,
//     T6: IntoTrait<M>,
//     T7: IntoTrait<M>,
//     T8: IntoTrait<M>,
//     T9: IntoTrait<M>,
//     T10: IntoTrait<M>,
//     T11: IntoTrait<M>,
//     T12: IntoTrait<M>,
//     T13: IntoTrait<M>,
//     T14: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T0;
//     type Tail = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (
//             Ok::<_, Infallible>(self.0),
//             (
//                 self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
//                 self.11, self.12, self.13, self.14,
//             ),
//         )
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         15
//     }
// }
// impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> Snek<M>
//     for (
//         T0,
//         T1,
//         T2,
//         T3,
//         T4,
//         T5,
//         T6,
//         T7,
//         T8,
//         T9,
//         T10,
//         T11,
//         T12,
//         T13,
//         T14,
//         T15,
//     )
// where
//     T0: IntoTrait<M>,
//     T1: IntoTrait<M>,
//     T2: IntoTrait<M>,
//     T3: IntoTrait<M>,
//     T4: IntoTrait<M>,
//     T5: IntoTrait<M>,
//     T6: IntoTrait<M>,
//     T7: IntoTrait<M>,
//     T8: IntoTrait<M>,
//     T9: IntoTrait<M>,
//     T10: IntoTrait<M>,
//     T11: IntoTrait<M>,
//     T12: IntoTrait<M>,
//     T13: IntoTrait<M>,
//     T14: IntoTrait<M>,
//     T15: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T0;
//     type Tail = (
//         T1,
//         T2,
//         T3,
//         T4,
//         T5,
//         T6,
//         T7,
//         T8,
//         T9,
//         T10,
//         T11,
//         T12,
//         T13,
//         T14,
//         T15,
//     );

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (
//             Ok::<_, Infallible>(self.0),
//             (
//                 self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
//                 self.11, self.12, self.13, self.14, self.15,
//             ),
//         )
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         16
//     }
// }
// impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> Snek<M>
//     for (
//         T0,
//         T1,
//         T2,
//         T3,
//         T4,
//         T5,
//         T6,
//         T7,
//         T8,
//         T9,
//         T10,
//         T11,
//         T12,
//         T13,
//         T14,
//         T15,
//         T16,
//     )
// where
//     T0: IntoTrait<M>,
//     T1: IntoTrait<M>,
//     T2: IntoTrait<M>,
//     T3: IntoTrait<M>,
//     T4: IntoTrait<M>,
//     T5: IntoTrait<M>,
//     T6: IntoTrait<M>,
//     T7: IntoTrait<M>,
//     T8: IntoTrait<M>,
//     T9: IntoTrait<M>,
//     T10: IntoTrait<M>,
//     T11: IntoTrait<M>,
//     T12: IntoTrait<M>,
//     T13: IntoTrait<M>,
//     T14: IntoTrait<M>,
//     T15: IntoTrait<M>,
//     T16: IntoTrait<M>,
//     Infallible: IntoTrait<M>,
// {
//     type Head = T0;
//     type Tail = (
//         T1,
//         T2,
//         T3,
//         T4,
//         T5,
//         T6,
//         T7,
//         T8,
//         T9,
//         T10,
//         T11,
//         T12,
//         T13,
//         T14,
//         T15,
//         T16,
//     );

//     #[inline]
//     fn split(self) -> (Result<Self::Head, impl Sized>, Self::Tail) {
//         (
//             Ok::<_, Infallible>(self.0),
//             (
//                 self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10,
//                 self.11, self.12, self.13, self.14, self.15, self.16,
//             ),
//         )
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         17
//     }
// }
