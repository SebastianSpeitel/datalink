use core::any::TypeId;
use core::marker::PhantomData;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Sorted<T, C: ?Sized = Box<[T]>> {
    element: PhantomData<T>,
    container: C,
}

pub type SortedArray<T, const N: usize> = Sorted<T, [T; N]>;
pub type SortedSliceBox<T> = Sorted<T, Box<[T]>>;
pub type SortedSliceRef<'a, T> = Sorted<T, &'a [T]>;
pub type SortedSlice<T> = Sorted<T, [T]>;
pub type SortedVec<T> = Sorted<T, Vec<T>>;

impl<T, C: ?Sized> Sorted<T, C> {
    #[inline]
    #[must_use]
    pub fn new(mut container: C) -> Self
    where
        T: Ord,
        C: AsMut<[T]> + Sized,
    {
        container.as_mut().sort_unstable();
        Self {
            element: PhantomData,
            container,
        }
    }

    /// # Safety
    /// The container must be sorted.
    #[inline]
    #[must_use]
    pub const unsafe fn new_unchecked(container: C) -> Self
    where
        C: Sized,
    {
        Self {
            element: PhantomData,
            container,
        }
    }

    #[inline]
    pub fn contains(&self, value: impl core::borrow::Borrow<T>) -> bool
    where
        T: Ord,
        C: AsRef<[T]>,
    {
        self.container
            .as_ref()
            .binary_search(value.borrow())
            .is_ok()
    }

    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, T>
    where
        C: AsRef<[T]>,
    {
        self.container.as_ref().iter()
    }
}

impl<T, C> Default for Sorted<T, C>
where
    C: Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            element: PhantomData,
            container: Default::default(),
        }
    }
}

impl<T, C> core::fmt::Debug for Sorted<T, C>
where
    C: core::fmt::Debug + ?Sized,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.container.fmt(f)
    }
}

impl<T, C> core::ops::Deref for Sorted<T, C> {
    type Target = C;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

impl<T, C> IntoIterator for Sorted<T, C>
where
    C: IntoIterator<Item = T>,
{
    type Item = T;
    type IntoIter = C::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.container.into_iter()
    }
}

impl<T, C> FromIterator<T> for Sorted<T, C>
where
    T: Ord,
    C: FromIterator<T> + AsMut<[T]>,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut container = iter.into_iter().collect::<C>();
        container.as_mut().sort_unstable();
        Self {
            element: PhantomData,
            container,
        }
    }
}

impl<C> TypeFilter for Sorted<TypeId, C>
where
    C: AsRef<[TypeId]> + ?Sized,
{
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        self.contains(type_id)
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        self.container.as_ref().into()
    }
    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q>
    where
        Self: Sized + 'q,
    {
        self.container.as_ref().into()
    }
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum EraseError {
    #[error("no implementation")]
    NoImpl,
    #[error("copy of dynamic filter")]
    CopyOfDynamic,
    #[error("inverse of dynamic filter")]
    InverseOfDynamic,
    #[error("merging dynamic filters")]
    MergeDynamic,
    #[error("inverse of callback filter")]
    InverseOfCallback,
    #[error("todo")]
    Todo,
}

#[derive(Debug, Default)]
pub enum ErasedFilter<'q, E = EraseError, const N: usize = 1> {
    #[default]
    Any,
    None,
    Only(TypeId),
    Not(TypeId),
    AnyOfStack(SortedArray<TypeId, N>),
    NoneOfStack(SortedArray<TypeId, N>),
    AnyOfHeap(SortedSliceBox<TypeId>),
    NoneOfHeap(SortedSliceBox<TypeId>),
    Dynamic(Box<dyn TypeFilter + 'q>),
    Callback(fn(TypeId) -> bool),
    Invalid(E),
}

pub type ValidErasedFilter<'q> = ErasedFilter<'q, core::convert::Infallible>;

impl<'q> TryFrom<ErasedFilter<'q>> for ValidErasedFilter<'q> {
    type Error = EraseError;

    #[inline]
    fn try_from(value: ErasedFilter<'q>) -> Result<Self, Self::Error> {
        use ErasedFilter as F;
        let f = match value {
            F::Invalid(e) => return Err(e),
            F::Any => F::Any,
            F::None => F::None,
            F::Only(id) => F::Only(id),
            F::Not(id) => F::Not(id),
            F::AnyOfStack(ids) => F::AnyOfStack(ids),
            F::NoneOfStack(ids) => F::NoneOfStack(ids),
            F::AnyOfHeap(ids) => F::AnyOfHeap(ids),
            F::NoneOfHeap(ids) => F::NoneOfHeap(ids),
            F::Dynamic(filter) => F::Dynamic(filter),
            F::Callback(f) => F::Callback(f),
        };
        Ok(f)
    }
}

impl<E> ErasedFilter<'_, E> {
    #[inline]
    #[must_use]
    pub fn only<T: 'static + ?Sized>() -> Self {
        Self::Only(TypeId::of::<T>())
    }
    #[inline]
    pub fn try_contains_id(&self, type_id: TypeId) -> Option<bool> {
        use ErasedFilter as F;
        Some(match *self {
            F::Any => true,
            F::None => false,
            F::Only(id) => id == type_id,
            F::Not(id) => id != type_id,
            F::AnyOfStack(ref ids) => ids.accepts_id(type_id),
            F::NoneOfStack(ref ids) => !ids.accepts_id(type_id),
            F::AnyOfHeap(ref ids) => ids.accepts_id(type_id),
            F::NoneOfHeap(ref ids) => !ids.accepts_id(type_id),
            F::Dynamic(ref filter) => filter.accepts_id(type_id),
            F::Callback(f) => f(type_id),
            F::Invalid(_) => return None,
        })
    }
}

impl<const N: usize, E, const M: usize> From<[TypeId; N]> for ErasedFilter<'_, E, M> {
    #[inline]
    fn from(ids: [TypeId; N]) -> Self {
        if N == 0 {
            Self::None
        } else if N == 1 {
            Self::Only(ids[0])
        } else if N <= M {
            let mut arr: [TypeId; M] = [ids[0]; M];
            arr[..N].copy_from_slice(&ids);
            Self::AnyOfStack(Sorted::new(arr))
        } else {
            Self::AnyOfHeap(Sorted::new(ids.into()))
        }
    }
}

impl<E, const N: usize> From<&[TypeId]> for ErasedFilter<'_, E, N> {
    #[inline]
    fn from(ids: &[TypeId]) -> Self {
        match ids.len() {
            0 => Self::None,
            1 => Self::Only(ids[0]),
            l if l <= N => {
                let mut arr: [TypeId; N] = [ids[0]; N];
                arr.copy_from_slice(ids);
                Self::AnyOfStack(Sorted::new(arr))
            }
            _ => Self::AnyOfHeap(Sorted::new(ids.into())),
        }
    }
}

impl<E, const N: usize> FromIterator<TypeId> for ErasedFilter<'_, E, N> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = TypeId>>(iter: I) -> Self {
        use core::mem::MaybeUninit;
        let mut iter = iter.into_iter();
        match iter.size_hint() {
            (.., Some(0)) => Self::None,
            (.., Some(1)) => iter.next().map_or(Self::None, Self::Only),
            (lo, ..) if lo > N || N <= 1 => Self::AnyOfHeap(iter.collect()),
            (.., Some(up)) if up <= N => {
                let Some(first) = iter.next() else {
                    return Self::None;
                };
                let mut ids = [MaybeUninit::uninit(); N];
                ids[0].write(first);
                let mut i = 1;
                for id in iter {
                    ids[i].write(id);
                    i += 1;
                }
                assert!(i <= N);
                ids[i..].fill(MaybeUninit::new(first));
                let ids = unsafe { ids.map(|id| id.assume_init()) };
                Self::AnyOfStack(Sorted::new(ids))
            }
            (lo, up) => {
                let mut ids = [MaybeUninit::uninit(); N];
                for i in 0..N {
                    if let Some(id) = iter.next() {
                        ids[i].write(id);
                    } else if i == 0 {
                        return Self::None;
                    } else if i == 1 {
                        return Self::Only(unsafe { ids[0].assume_init() });
                    } else {
                        let first = ids[0];
                        ids[i..].fill(first);
                        let ids = unsafe { ids.map(|id| id.assume_init()) };
                        return Self::AnyOfStack(Sorted::new(ids));
                    }
                }
                let ids = unsafe { ids.map(|id| id.assume_init()) };

                let Some(next) = iter.next() else {
                    return Self::AnyOfStack(Sorted::new(ids));
                };

                let cap = up.unwrap_or(lo).min(N + 1);
                let mut vec = Vec::with_capacity(cap);
                vec.extend_from_slice(&ids);
                vec.push(next);
                vec.extend(iter);
                Self::AnyOfHeap(Sorted::new(vec.into_boxed_slice()))
            }
        }
    }
}

impl TypeFilter for ValidErasedFilter<'_> {
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        match *self {
            Self::Any => true,
            Self::None => false,
            Self::Only(id) => id == type_id,
            Self::Not(id) => id != type_id,
            Self::AnyOfHeap(ref ids) => ids.accepts_id(type_id),
            Self::NoneOfHeap(ref ids) => !ids.accepts_id(type_id),
            Self::AnyOfStack(ref ids) => ids.accepts_id(type_id),
            Self::NoneOfStack(ref ids) => !ids.accepts_id(type_id),
            Self::Dynamic(ref filter) => {
                eprintln!("WARN: dynamic filter used");
                filter.accepts_id(type_id)
            }
            Self::Callback(f) => f(type_id),
            Self::Invalid(e) => match e {},
        }
    }

    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        match *self {
            Self::Any => ErasedFilter::Any,
            Self::None => ErasedFilter::None,
            Self::Only(id) => ErasedFilter::Only(id),
            Self::Not(id) => ErasedFilter::Not(id),
            Self::AnyOfHeap(ref ids) => ErasedFilter::AnyOfHeap(ids.clone()),
            Self::NoneOfHeap(ref ids) => ErasedFilter::NoneOfHeap(ids.clone()),
            Self::AnyOfStack(ids) => ErasedFilter::AnyOfStack(ids),
            Self::NoneOfStack(ids) => ErasedFilter::NoneOfStack(ids),
            Self::Dynamic(_) => ErasedFilter::Invalid(EraseError::CopyOfDynamic),
            Self::Callback(f) => ErasedFilter::Callback(f),
            Self::Invalid(e) => match e {},
        }
    }

    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q>
    where
        Self: 'q,
    {
        self
    }
}

impl<E> core::ops::Not for ErasedFilter<'_, E>
where
    E: From<EraseError>,
{
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        use ErasedFilter as F;
        match self {
            F::Any => F::None,
            F::None => F::Any,
            F::Only(id) => F::Not(id),
            F::Not(id) => F::Only(id),
            F::AnyOfStack(ids) => F::NoneOfStack(ids),
            F::NoneOfStack(ids) => F::AnyOfStack(ids),
            F::AnyOfHeap(ids) => F::NoneOfHeap(ids),
            F::NoneOfHeap(ids) => F::AnyOfHeap(ids),
            F::Dynamic(_) => F::Invalid(EraseError::InverseOfDynamic.into()),
            F::Callback(..) => F::Invalid(EraseError::InverseOfCallback.into()),
            f @ F::Invalid(..) => f,
        }
    }
}

impl<E> core::ops::BitOr for ErasedFilter<'_, E>
where
    E: From<EraseError>,
{
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        use ErasedFilter as F;
        match (self, rhs) {
            (F::Any, ..) | (.., F::Any) => F::Any,
            (F::None, f) | (f, F::None) => f,
            (f @ F::Only(id1), F::Only(id2)) if id1 == id2 => f,
            (f, F::Only(id)) | (F::Only(id), f) if f.try_contains_id(id) == Some(true) => f,
            (F::AnyOfStack(ids1), F::AnyOfStack(ids2)) => ids1.into_iter().chain(ids2).collect(),
            (F::NoneOfStack(ids1), F::NoneOfStack(ids2)) => {
                !ids1.into_iter().chain(ids2).collect::<Self>()
            }
            (F::AnyOfStack(inc), F::NoneOfStack(exc))
            | (F::NoneOfStack(exc), F::AnyOfStack(inc)) => {
                inc.into_iter().filter(|id| !exc.contains(id)).collect()
            }
            (F::Dynamic(..), F::Dynamic(..)) => F::Invalid(EraseError::MergeDynamic.into()),
            (F::Invalid(e), ..) | (.., F::Invalid(e)) => F::Invalid(e),
            _ => Self::Invalid(EraseError::Todo.into()),
        }
    }
}

impl<E> core::ops::BitAnd for ErasedFilter<'_, E>
where
    E: From<EraseError>,
{
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Any, f) | (f, Self::Any) => f,
            (Self::None, ..) | (.., Self::None) => Self::None,
            (Self::Only(id1), Self::Only(id2)) if id1 == id2 => Self::Only(id1),
            (Self::Only(id), Self::AnyOfStack(ids)) | (Self::AnyOfStack(ids), Self::Only(id)) => {
                if ids.accepts_id(id) {
                    Self::Only(id)
                } else {
                    Self::None
                }
            }
            (Self::Only(id), Self::NoneOfStack(ids)) | (Self::NoneOfStack(ids), Self::Only(id)) => {
                if ids.accepts_id(id) {
                    Self::None
                } else {
                    Self::Only(id)
                }
            }
            (Self::Only(id), Self::Callback(f)) | (Self::Callback(f), Self::Only(id)) => {
                if f(id) {
                    Self::Only(id)
                } else {
                    Self::None
                }
            }
            (Self::Dynamic(..), Self::Dynamic(..)) => {
                Self::Invalid(EraseError::MergeDynamic.into())
            }
            (Self::Invalid(e), ..) | (.., Self::Invalid(e)) => Self::Invalid(e),
            _ => Self::Invalid(EraseError::Todo.into()),
        }
    }
}

pub trait TypeFilter {
    fn accepts_id(&self, type_id: TypeId) -> bool;
    #[inline]
    fn accepts_value_of<T: 'static + ?Sized>(&self) -> bool
    where
        Self: Sized,
    {
        self.accepts_id(TypeId::of::<T>())
    }
    #[inline]
    fn accepts<T: 'static + ?Sized>(&self, value: &T) -> bool
    where
        Self: Sized,
    {
        let _ = value;
        self.accepts_value_of::<T>()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        false
    }

    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        ErasedFilter::Invalid(EraseError::NoImpl)
    }
    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q>
    where
        Self: Sized + 'q,
    {
        self.as_erased()
            .try_into()
            .unwrap_or_else(|_| ErasedFilter::Dynamic(Box::new(self)))
    }
}

impl core::fmt::Debug for dyn TypeFilter + '_ {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TypeFilter")
            .field("as_erased", &self.as_erased())
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub struct Only<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized> Default for Only<T> {
    #[inline]
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: ?Sized> Clone for Only<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for Only<T> {}

impl<T> TypeFilter for Only<T>
where
    T: 'static + ?Sized,
{
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        type_id == TypeId::of::<T>()
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        ErasedFilter::only::<T>()
    }
    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q> {
        ErasedFilter::only::<T>()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Any;

impl TypeFilter for Any {
    #[inline]
    fn accepts_id(&self, _type_id: TypeId) -> bool {
        true
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        ErasedFilter::Any
    }
    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q> {
        ErasedFilter::Any
    }
}

#[derive(Debug)]
pub struct AnyOf<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized> AnyOf<T> {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: ?Sized> Default for AnyOf<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ?Sized> Clone for AnyOf<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for AnyOf<T> {}

impl TypeFilter for AnyOf<()> {
    #[inline]
    fn accepts_id(&self, _type_id: TypeId) -> bool {
        false
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        ErasedFilter::None
    }
    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q> {
        ErasedFilter::None
    }
    #[inline]
    fn is_empty(&self) -> bool {
        true
    }
}

impl<T1> TypeFilter for AnyOf<(T1,)>
where
    T1: 'static + ?Sized,
{
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        type_id == TypeId::of::<T1>()
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        ErasedFilter::only::<T1>()
    }
    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q> {
        ErasedFilter::only::<T1>()
    }
}

impl<T1, T2> TypeFilter for AnyOf<(T1, T2)>
where
    T1: 'static,
    T2: 'static,
{
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        type_id == TypeId::of::<T1>() || type_id == TypeId::of::<T2>()
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        [TypeId::of::<T1>(), TypeId::of::<T2>()].into()
    }
    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q> {
        ErasedFilter::Callback(|id| id == TypeId::of::<T1>() || id == TypeId::of::<T2>())
    }
}

impl<T1, T2, T3> TypeFilter for AnyOf<(T1, T2, T3)>
where
    T1: 'static,
    T2: 'static,
    T3: ?Sized + 'static,
{
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        type_id == TypeId::of::<T1>()
            || type_id == TypeId::of::<T2>()
            || type_id == TypeId::of::<T3>()
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        [TypeId::of::<T1>(), TypeId::of::<T2>(), TypeId::of::<T3>()].into()
    }
    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q> {
        ErasedFilter::Callback(|id| {
            id == TypeId::of::<T1>() || id == TypeId::of::<T2>() || id == TypeId::of::<T3>()
        })
    }
}

impl<T1, T2, T3, T4> TypeFilter for AnyOf<(T1, T2, T3, T4)>
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static + ?Sized,
{
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        type_id == TypeId::of::<T1>()
            || type_id == TypeId::of::<T2>()
            || type_id == TypeId::of::<T3>()
            || type_id == TypeId::of::<T4>()
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        [
            TypeId::of::<T1>(),
            TypeId::of::<T2>(),
            TypeId::of::<T3>(),
            TypeId::of::<T4>(),
        ]
        .into()
    }
    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q> {
        ErasedFilter::Callback(|id| {
            id == TypeId::of::<T1>()
                || id == TypeId::of::<T2>()
                || id == TypeId::of::<T3>()
                || id == TypeId::of::<T4>()
        })
    }
}

impl<T1, T2, T3, T4, T5, T6> TypeFilter for AnyOf<(T1, T2, T3, T4, T5, T6)>
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
{
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        type_id == TypeId::of::<T1>()
            || type_id == TypeId::of::<T2>()
            || type_id == TypeId::of::<T3>()
            || type_id == TypeId::of::<T4>()
            || type_id == TypeId::of::<T5>()
            || type_id == TypeId::of::<T6>()
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        [
            TypeId::of::<T1>(),
            TypeId::of::<T2>(),
            TypeId::of::<T3>(),
            TypeId::of::<T4>(),
            TypeId::of::<T5>(),
            TypeId::of::<T6>(),
        ]
        .into()
    }
    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q> {
        ErasedFilter::Callback(|id| {
            id == TypeId::of::<T1>()
                || id == TypeId::of::<T2>()
                || id == TypeId::of::<T3>()
                || id == TypeId::of::<T4>()
                || id == TypeId::of::<T5>()
                || id == TypeId::of::<T6>()
        })
    }
}

impl<T1, T2, T3, T4, T5, T6, T7> TypeFilter for AnyOf<(T1, T2, T3, T4, T5, T6, T7)>
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
{
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        type_id == TypeId::of::<T1>()
            || type_id == TypeId::of::<T2>()
            || type_id == TypeId::of::<T3>()
            || type_id == TypeId::of::<T4>()
            || type_id == TypeId::of::<T5>()
            || type_id == TypeId::of::<T6>()
            || type_id == TypeId::of::<T7>()
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        [
            TypeId::of::<T1>(),
            TypeId::of::<T2>(),
            TypeId::of::<T3>(),
            TypeId::of::<T4>(),
            TypeId::of::<T5>(),
            TypeId::of::<T6>(),
            TypeId::of::<T7>(),
        ]
        .into()
    }
    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q> {
        ErasedFilter::Callback(|id| {
            id == TypeId::of::<T1>()
                || id == TypeId::of::<T2>()
                || id == TypeId::of::<T3>()
                || id == TypeId::of::<T4>()
                || id == TypeId::of::<T5>()
                || id == TypeId::of::<T6>()
                || id == TypeId::of::<T7>()
        })
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8> TypeFilter for AnyOf<(T1, T2, T3, T4, T5, T6, T7, T8)>
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
    T8: 'static,
{
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        type_id == TypeId::of::<T1>()
            || type_id == TypeId::of::<T2>()
            || type_id == TypeId::of::<T3>()
            || type_id == TypeId::of::<T4>()
            || type_id == TypeId::of::<T5>()
            || type_id == TypeId::of::<T6>()
            || type_id == TypeId::of::<T7>()
            || type_id == TypeId::of::<T8>()
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        [
            TypeId::of::<T1>(),
            TypeId::of::<T2>(),
            TypeId::of::<T3>(),
            TypeId::of::<T4>(),
            TypeId::of::<T5>(),
            TypeId::of::<T6>(),
            TypeId::of::<T7>(),
            TypeId::of::<T8>(),
        ]
        .into()
    }
    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q> {
        ErasedFilter::Callback(|id| {
            id == TypeId::of::<T1>()
                || id == TypeId::of::<T2>()
                || id == TypeId::of::<T3>()
                || id == TypeId::of::<T4>()
                || id == TypeId::of::<T5>()
                || id == TypeId::of::<T6>()
                || id == TypeId::of::<T7>()
                || id == TypeId::of::<T8>()
        })
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TypeFilter
    for AnyOf<(
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
    )>
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
    T8: 'static,
    T9: 'static,
    T10: 'static,
    T11: 'static,
    T12: 'static,
    T13: 'static,
    T14: 'static,
    T15: 'static,
    T16: 'static,
{
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        type_id == TypeId::of::<T1>()
            || type_id == TypeId::of::<T2>()
            || type_id == TypeId::of::<T3>()
            || type_id == TypeId::of::<T4>()
            || type_id == TypeId::of::<T5>()
            || type_id == TypeId::of::<T6>()
            || type_id == TypeId::of::<T7>()
            || type_id == TypeId::of::<T8>()
            || type_id == TypeId::of::<T9>()
            || type_id == TypeId::of::<T10>()
            || type_id == TypeId::of::<T11>()
            || type_id == TypeId::of::<T12>()
            || type_id == TypeId::of::<T13>()
            || type_id == TypeId::of::<T14>()
            || type_id == TypeId::of::<T15>()
            || type_id == TypeId::of::<T16>()
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        [
            TypeId::of::<T1>(),
            TypeId::of::<T2>(),
            TypeId::of::<T3>(),
            TypeId::of::<T4>(),
            TypeId::of::<T5>(),
            TypeId::of::<T6>(),
            TypeId::of::<T7>(),
            TypeId::of::<T8>(),
            TypeId::of::<T9>(),
            TypeId::of::<T10>(),
            TypeId::of::<T11>(),
            TypeId::of::<T12>(),
            TypeId::of::<T13>(),
            TypeId::of::<T14>(),
            TypeId::of::<T15>(),
            TypeId::of::<T16>(),
        ]
        .into()
    }
    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q> {
        ErasedFilter::Callback(|id| {
            id == TypeId::of::<T1>()
                || id == TypeId::of::<T2>()
                || id == TypeId::of::<T3>()
                || id == TypeId::of::<T4>()
                || id == TypeId::of::<T5>()
                || id == TypeId::of::<T6>()
                || id == TypeId::of::<T7>()
                || id == TypeId::of::<T8>()
                || id == TypeId::of::<T9>()
                || id == TypeId::of::<T10>()
                || id == TypeId::of::<T11>()
                || id == TypeId::of::<T12>()
                || id == TypeId::of::<T13>()
                || id == TypeId::of::<T14>()
                || id == TypeId::of::<T15>()
                || id == TypeId::of::<T16>()
        })
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct And<T: TypeFilter, U: TypeFilter>(pub T, pub U);

impl<T, U> TypeFilter for And<T, U>
where
    T: TypeFilter,
    U: TypeFilter,
{
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        self.0.accepts_id(type_id) && self.1.accepts_id(type_id)
    }
    #[inline]
    fn accepts_value_of<Typ: 'static + ?Sized>(&self) -> bool {
        self.0.accepts_value_of::<Typ>() && self.1.accepts_value_of::<Typ>()
    }
    #[inline]
    fn accepts<Typ: 'static + ?Sized>(&self, val: &Typ) -> bool {
        self.0.accepts(val) && self.1.accepts(val)
    }
    // #[inline]
    // fn as_typeset(&self) -> Option<TypeSet> {
    //     let set1 = self.0.as_typeset()?;
    //     let set2 = self.1.as_typeset()?;

    //     match (set1, set2) {
    //         (TypeSet::None, ..) | (.., TypeSet::None) => Some(TypeSet::None),
    //         (TypeSet::Any, set) | (set, TypeSet::Any) => Some(set),
    //         (
    //             TypeSet::AnyOf {
    //                 len: any_len,
    //                 ids: any_ids,
    //             },
    //             TypeSet::NoneOf {
    //                 len: none_len,
    //                 ids: none_ids,
    //             },
    //         )
    //         | (
    //             TypeSet::NoneOf {
    //                 len: none_len,
    //                 ids: none_ids,
    //             },
    //             TypeSet::AnyOf {
    //                 len: any_len,
    //                 ids: any_ids,
    //             },
    //         ) => {
    //             let mut ids = [marker::<'…'>(); 15];
    //             let mut len = 0;
    //             let any_ids = &any_ids[..any_len.get() as usize];
    //             let none_ids = &none_ids[..none_len.get() as usize];
    //             for id in any_ids {
    //                 if !none_ids.contains(id) {
    //                     ids[len] = *id;
    //                     len += 1;
    //                 }
    //             }
    //             match NonZeroU16::new(len as u16) {
    //                 None => Some(TypeSet::None),
    //                 Some(len) => Some(TypeSet::AnyOf { len, ids }),
    //             }
    //         }
    //         // (
    //         //     TypeSet::AnyOf {
    //         //         len: len1,
    //         //         ids: ids1,
    //         //     },
    //         //     TypeSet::AnyOf {
    //         //         len: len2,
    //         //         ids: ids2,
    //         //     },
    //         // ) => {
    //         //     let len = len1.saturating_add(len2.get());
    //         //     if len.get() > 15 {
    //         //         return None;
    //         //     }
    //         //     let mut ids = [marker::<'…'>(); 15];
    //         //     ids[..len1.get() as usize].copy_from_slice(&ids1[..len1.get() as usize]);
    //         //     ids[len1.get() as usize..len.get() as usize]
    //         //         .copy_from_slice(&ids2[..len2.get() as usize]);

    //         //     Some(TypeSet::AnyOf { len, ids })
    //         // }
    //         // (
    //         //     TypeSet::NoneOf {
    //         //         len: len1,
    //         //         ids: ids1,
    //         //     },
    //         //     TypeSet::NoneOf {
    //         //         len: len2,
    //         //         ids: ids2,
    //         //     },
    //         // ) => {
    //         //     todo!()
    //         // }
    //         _ => todo!(),
    //     }
    // }

    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        self.0.as_erased() & self.1.as_erased()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty() || self.1.is_empty()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Or<T: TypeFilter, U: TypeFilter>(pub T, pub U);

impl<T, U> TypeFilter for Or<T, U>
where
    T: TypeFilter,
    U: TypeFilter,
{
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        self.0.accepts_id(type_id) || self.1.accepts_id(type_id)
    }
    #[inline]
    fn accepts_value_of<Typ: 'static + ?Sized>(&self) -> bool {
        self.0.accepts_value_of::<Typ>() || self.1.accepts_value_of::<Typ>()
    }
    #[inline]
    fn accepts<Typ: 'static + ?Sized>(&self, val: &Typ) -> bool {
        self.0.accepts(val) || self.1.accepts(val)
    }
    // #[inline]
    // fn as_typeset(&self) -> Option<TypeSet> {
    //     let set1 = self.0.as_typeset()?;
    //     let set2 = self.1.as_typeset()?;

    //     match (set1, set2) {
    //         (TypeSet::Any, ..) | (.., TypeSet::Any) => Some(TypeSet::Any),
    //         (TypeSet::None, set) | (set, TypeSet::None) => Some(set),
    //         (
    //             TypeSet::AnyOf {
    //                 len: len1,
    //                 ids: ids1,
    //             },
    //             TypeSet::AnyOf {
    //                 len: len2,
    //                 ids: ids2,
    //             },
    //         ) => {
    //             let len = len1.saturating_add(len2.get());
    //             if len.get() > 15 {
    //                 return None;
    //             }
    //             let mut ids = [marker::<'…'>(); 15];
    //             ids[..len1.get() as usize].copy_from_slice(&ids1[..len1.get() as usize]);
    //             ids[len1.get() as usize..len.get() as usize]
    //                 .copy_from_slice(&ids2[..len2.get() as usize]);

    //             Some(TypeSet::AnyOf { len, ids })
    //         }
    //         _ => todo!(),
    //     }
    // }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        self.0.as_erased() | self.1.as_erased()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Not<T: TypeFilter>(pub T);

impl<T: TypeFilter> TypeFilter for Not<T> {
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        !self.0.accepts_id(type_id)
    }
    #[inline]
    fn accepts_value_of<Typ: 'static + ?Sized>(&self) -> bool {
        !self.0.accepts_value_of::<Typ>()
    }
    #[inline]
    fn accepts<Typ: 'static + ?Sized>(&self, val: &Typ) -> bool {
        !self.0.accepts(val)
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        !self.0.as_erased()
    }
}
pub type StringLike = AnyOf<(String, Box<str>, &'static str, str)>;
pub const STRING_LIKE: StringLike = AnyOf::new();

pub type BytesLike = AnyOf<(Vec<u8>, Box<[u8]>, &'static [u8], [u8])>;
pub const BYTES_LIKE: BytesLike = AnyOf::new();

#[derive(Debug)]
pub struct AcceptedBy<R>(PhantomData<R>);

impl<R> AcceptedBy<R> {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<R> Default for AcceptedBy<R> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<R> Clone for AcceptedBy<R> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<R> Copy for AcceptedBy<R> {}

impl<const N: usize> TypeFilter for [TypeId; N] {
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        self.contains(&type_id)
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        (*self).into()
    }
    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q> {
        self.into()
    }
    #[inline]
    fn is_empty(&self) -> bool {
        N == 0
    }
}

impl<R: crate::Receiver> TypeFilter for AcceptedBy<R> {
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        R::accepting().accepts_id(type_id)
    }
    #[inline]
    fn accepts_value_of<T: 'static + ?Sized>(&self) -> bool {
        R::accepting().accepts_value_of::<T>()
    }
    #[inline]
    fn accepts<T: 'static + ?Sized>(&self, value: &T) -> bool {
        R::accepting().accepts(value)
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        R::accepting().as_erased()
    }
    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q> {
        R::accepting().into_erased()
    }
    #[inline]
    fn is_empty(&self) -> bool {
        R::accepting().is_empty()
    }
}

impl<T: TypeFilter + ?Sized> TypeFilter for &T {
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        T::accepts_id(self, type_id)
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        T::as_erased(self)
    }
    #[inline]
    fn is_empty(&self) -> bool {
        T::is_empty(self)
    }
}

impl<T: TypeFilter + ?Sized> TypeFilter for Box<T> {
    #[inline]
    fn accepts_id(&self, type_id: TypeId) -> bool {
        self.as_ref().accepts_id(type_id)
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        self.as_ref().as_erased()
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }
}

impl TypeFilter for core::convert::Infallible {
    #[inline]
    fn accepts_id(&self, _: TypeId) -> bool {
        match *self {}
    }
    #[inline]
    fn as_erased(&self) -> ErasedFilter<'static> {
        match *self {}
    }
    #[inline]
    fn into_erased<'q>(self) -> ValidErasedFilter<'q> {
        match self {}
    }
    #[inline]
    fn is_empty(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_only() {
        let set = Only::<u8>::default();
        assert!(set.accepts_value_of::<u8>());
        assert!(!set.accepts_value_of::<u16>());
    }

    #[test]
    fn stringlike() {
        let set = STRING_LIKE;
        assert!(set.accepts_value_of::<String>());
        assert!(set.accepts_value_of::<&str>());
        assert!(!set.accepts_value_of::<u8>());
    }

    #[test]
    fn byteslike() {
        let set = BYTES_LIKE;
        assert!(set.accepts_value_of::<Vec<u8>>());
        assert!(set.accepts_value_of::<&[u8]>());
        assert!(!set.accepts_value_of::<u8>());
    }
}
