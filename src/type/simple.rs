use core::any::{TypeId, type_name};
use core::num::NonZeroU8;

use super::{Type, TypeSet};
use crate::util::Maybe;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
#[repr(u8)]
pub enum SimpleType {
    #[default]
    Untyped = 0,
    Id(TypeId),
    Name(&'static str),
}

#[derive(Clone, Copy)]
struct Untyped;
impl Type for Untyped {
    fn name(&self) -> impl Maybe<&'static str> {
        panic!("Untyped has no name") as ()
    }
    fn eq(&self, _other: impl Type) -> bool {
        panic!("Untyped cannot be compared") as _
    }
    fn id(&self) -> impl Maybe<TypeId> {
        panic!("Untyped has no id") as ()
    }
    fn is<T: 'static + ?Sized>(&self) -> bool {
        panic!("Untyped cannot be compared") as _
    }
}

impl SimpleType {
    #[inline]
    #[must_use]
    pub const fn of<T: 'static + ?Sized>() -> Self {
        Self::Id(TypeId::of::<T>())
    }

    #[inline]
    #[must_use]
    pub fn name_of<T: 'static + ?Sized>() -> Self {
        Self::Name(type_name::<T>())
    }
}

impl Type for SimpleType {
    #[inline]
    fn id(&self) -> impl Maybe<TypeId> {
        match *self {
            SimpleType::Id(id) => Some(id),
            _ => None,
        }
    }

    #[inline]
    fn name(&self) -> impl Maybe<&'static str> {
        match *self {
            SimpleType::Name(n) => Some(n),
            _ => None,
        }
    }

    #[inline]
    fn eq(&self, other: impl Type) -> bool {
        match *self {
            SimpleType::Id(id) => other.id().eq(id),
            SimpleType::Name(n) => other.name().eq(n),
            SimpleType::Untyped => false,
        }
    }

    #[inline]
    fn is<T: 'static + ?Sized>(&self) -> bool {
        match *self {
            SimpleType::Id(id) => id == TypeId::of::<T>(),
            SimpleType::Name(n) => n == type_name::<T>(),
            SimpleType::Untyped => false,
        }
    }
}

impl From<TypeId> for SimpleType {
    #[inline]
    fn from(id: TypeId) -> Self {
        SimpleType::Id(id)
    }
}

impl From<&'static str> for SimpleType {
    #[inline]
    fn from(name: &'static str) -> Self {
        SimpleType::Name(name)
    }
}

const MAX_TYPES: usize = 7;

#[derive(Clone, Copy)]
pub struct SimpleTypeSet<'a>(Repr<'a, MAX_TYPES>);

impl<'a> SimpleTypeSet<'a> {
    pub(crate) const ANY: Self = Self(Repr::ANY);

    #[inline]
    #[must_use]
    pub const fn any() -> Self {
        Self::ANY
    }

    #[inline]
    #[must_use]
    pub const fn empty() -> Self {
        Self(Repr::EMPTY)
    }

    #[inline]
    #[must_use]
    pub const fn new(ids: &'a [TypeId], names: &'a [&'static str]) -> Self {
        Self(Repr::new(ids, names))
    }

    #[inline]
    #[must_use]
    pub(crate) fn for_type<T: 'static + ?Sized>() -> Self {
        let mut types = [TypeRepr::UNUSED; _];
        types[0].id = TypeId::of::<T>();
        types[1].name = type_name::<T>();
        Self(Repr {
            ids: Len(1),
            names: (Len(1), Len(1)),
            types,
            ..Repr::DEFAULT
        })
    }

    // #[inline]
    // #[must_use]
    // pub const fn from_slice(slice: &'a [SimpleType]) -> Self {
    //     let mut ids = [TypeId::of::<core::convert::Infallible>(); _];
    //     let mut names = [""; _];

    //     let mut ids_idx = 0;
    //     let mut names_idx = 0;

    //     let mut slice_idx = 0;

    //     while slice_idx < slice.len() {
    //         match slice[slice_idx] {
    //             SimpleType::Id(id) => {
    //                 if ids_idx >= ids.len() {
    //                     return Self(Inner::Any);
    //                 }
    //                 ids[ids_idx] = id;
    //                 ids_idx += 1;
    //             }
    //             SimpleType::Name(name) => {
    //                 if names_idx >= names.len() {
    //                     return Self(Inner::Any);
    //                 }
    //                 names[names_idx] = name;
    //                 names_idx += 1;
    //             }
    //             SimpleType::Untyped => {}
    //         }

    //         slice_idx += 1;
    //     }

    //     Self(Inner::Some {
    //         ids: InnerSet::Array(ids),
    //         names: InnerSet::Array(names),
    //     })
    // }

    #[inline]
    pub fn from_trait(set: impl TypeSet) -> Self {
        if set.contains(Untyped) {
            return Self::ANY;
        }

        Self(set.types().into_iter().collect())
    }

    #[inline]
    fn into_owned(self) -> SimpleTypeSet<'static> {
        todo!()
    }
}

impl core::fmt::Debug for SimpleTypeSet<'_> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SimpleTypeSet")
            .field("any", &self.0.any)
            .field("ids", &self.0.ids)
            .field("names", &self.0.names)
            .field("id_slices", &self.0.id_slices)
            .field("name_slices", &self.0.name_slices)
            .field("N", &self.0.types.len())
            .finish()
    }
}

impl TypeSet for SimpleTypeSet<'_> {
    #[inline]
    fn contains(&self, typ: impl Type) -> bool {
        self.0.contains(typ)
    }
    #[inline]
    fn types(self) -> impl IntoIterator<Item: Type> {
        self
    }
    #[inline]
    fn into_simple(self) -> SimpleTypeSet<'static> {
        self.into_owned()
    }
}

impl TypeSet for &SimpleTypeSet<'_> {
    #[inline]
    fn contains(&self, typ: impl Type) -> bool {
        self.0.contains(typ)
    }
    #[inline]
    fn types(self) -> impl IntoIterator<Item: Type> {
        *self
    }
    #[inline]
    fn into_simple(self) -> SimpleTypeSet<'static> {
        self.into_owned()
    }
}

impl<'a> IntoIterator for SimpleTypeSet<'a> {
    type Item = SimpleType;
    type IntoIter = Iter<'a, MAX_TYPES>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

#[derive(Clone, Copy)]
struct InlineStr {
    data: [u8; 15],
    len: u8,
}

impl InlineStr {
    const fn as_str(&self) -> &str {
        use core::{slice::from_raw_parts, str::from_utf8_unchecked};

        unsafe { from_utf8_unchecked(from_raw_parts(self.data.as_ptr(), self.len as _)) }
    }
}

impl AsRef<str> for InlineStr {
    #[inline]
    fn as_ref(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.data[..self.len as usize]) }
    }
}

union StaticStr {
    heap: &'static str,
    inline: InlineStr,
}

impl StaticStr {
    const fn new(s: &'static str) -> Self {
        let len = s.len();
        if len == 0 {
            return Self { heap: "" };
        }
        if len <= 15 {
            let mut buf = [0u8; 15];
            let mut i = 0;
            while i < len {
                buf[i] = s.as_bytes()[i];
                i += 1;
            }
            return Self {
                inline: InlineStr {
                    data: buf,
                    len: len as u8,
                },
            };
        }
        debug_assert!(len & 0x1100_0000_0000_0000 == 0);
        Self { heap: s }
    }

    const fn len(&self) -> usize {
        let inline_len = unsafe { self.inline.len };
        if inline_len == 0 {
            return unsafe { self.heap.len() };
        }
        inline_len as _
    }

    const fn as_str(&self) -> &str {
        let inline_len = unsafe { self.inline.len };
        if inline_len == 0 {
            return unsafe { self.heap };
        }
        unsafe { self.inline.as_str() }
    }
}

impl AsRef<str> for StaticStr {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<str> for StaticStr {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.len() == other.len() && self.as_ref() == other
    }
}

#[derive(Clone, Copy)]
union TypeRepr<'a> {
    id: TypeId,
    name: &'static str,
    ids: &'a [TypeId],
    names: &'a [&'static str],
    unused: (),
}

impl TypeRepr<'_> {
    const UNUSED: Self = Self { unused: () };
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(transparent)]
struct Len<const N: usize = MAX_TYPES>(u8);

// #[repr(u8)]
// enum Flags {
//     Empty = 0b0000,
//     Id(Len) = 0b1000,
//     IdS(Len) = 0b0100,
//     Name(Len) = 0b0010,
//     NameS(Len) = 0b0001,
//     IdIdS(Len, Len) = 0b1100,
//     IdName(Len, Len) = 0b1010,
//     IdNameS(Len, Len) = 0b1001,
//     IdsName(Len, Len) = 0b0110,
//     NameNameS(Len, Len) = 0b0011,
//     IdIdSName(Len, Len, Len) = 0b1110,
//     IdNameNameS(Len, Len, Len) = 0b1101,
//     IdSNameNameS(Len, Len, Len) = 0b1011,
//     // IdIdSNameNameS(Len,Len,Len,Len) = 0b1111,
// }

#[repr(align(4))]
enum Flags3 {
    Empty,
    Any,
    Ids(u8),
    IdSlices(u8),
    Names(u8),
    NameSlices(u8),
    Inline { ids: u8, names: u8 },
    Slices { ids: u8, names: u8 },
    Compact(u16),
}

#[repr(u8)]
enum Flags2 {
    Empty,
    Id(Len),
    IdS(Len),
    Name(Len),
    NameS(Len),
    IdIdS(Len, Len),
    IdName(Len, Len),
    IdNameS(Len, Len),
    IdsName(Len, Len),
    NameNameS(Len, Len),
    IdIdSName(Len, Len, Len),
    IdNameNameS(Len, Len, Len),
    IdSNameNameS(Len, Len, Len),
    IdIdSNameNameS(Len, Len, Len, Len),
}

enum LenOf {
    Ids(Len),
    IdSlices(Len),
    Names(Len),
    NameSlices(Len),
}

struct Lenghts {
    ids: Len,
    id_slices: Len,
    names: Len,
    name_slices: Len,
}

// total: u8 => 0 => empty, >N => any
// ids: u8
// id_slices: u8
// names: u8
// name_slices: implicit

#[derive(Debug, Clone, Copy)]
enum Meta {
    Any,
    Empty,
    Ids(NonZeroU8),
    Names(NonZeroU8),
    Inline {
        ids: NonZeroU8,
        names: NonZeroU8,
    },
    Slices {
        ids: NonZeroU8,
        names: NonZeroU8,
    },
    Mixed {
        ids: u8,
        names: (u8, u8),
        id_slices: (u8, u8),
        name_slices: (u8, u8),
    },
}

#[derive(Debug, Clone, Copy)]
enum Flags {
    Empty,
    Some,
    Any,
}

struct ReprInfo {
    flags: Flags,
    ids: Len,
    names: (Len, Len),
    id_slices: (Len, Len),
    name_slices: (Len, Len),
}

#[derive(Clone, Copy)]
struct Repr<'a, const N: usize = MAX_TYPES> {
    any: bool,
    ids: Len<N>,
    names: (Len<N>, Len<N>),
    id_slices: (Len<N>, Len<N>),
    name_slices: (Len<N>, Len<N>),
    types: [TypeRepr<'a>; N],
}

impl<'a, const N: usize> Repr<'a, N> {
    const DEFAULT: Self = Self {
        any: false,
        ids: Len(0),
        names: (Len(0), Len(0)),
        id_slices: (Len(0), Len(0)),
        name_slices: (Len(0), Len(0)),
        types: [TypeRepr::UNUSED; N],
    };

    const ANY: Self = Self {
        any: true,
        ..Self::DEFAULT
    };

    const EMPTY: Self = Self::DEFAULT;

    const fn new(ids: &'a [TypeId], names: &'a [&'static str]) -> Self {
        assert!(core::mem::size_of::<TypeRepr>() == 16);
        assert!(core::mem::size_of::<[TypeRepr; N]>() == 16 * N);
        assert!(core::mem::size_of::<Self>() == 4 + 4 + 16 * N);
        assert!(core::mem::align_of::<TypeRepr>() == 8);
        let mut types = [TypeRepr::UNUSED; N];
        let ptr = types.as_mut_ptr();

        match (ids.len(), names.len()) {
            (0, 0) => Self::EMPTY,
            (i, 0) if i <= N => {
                unsafe {
                    ptr.copy_from_nonoverlapping(ids.as_ptr().cast(), i);
                }
                Self {
                    ids: Len(i as u8),
                    types,
                    ..Self::DEFAULT
                }
            }
            (0, n) if n <= N => {
                unsafe {
                    ptr.copy_from_nonoverlapping(names.as_ptr().cast(), n);
                }
                Self {
                    names: (Len(0), Len(n as u8)),
                    types,
                    ..Self::DEFAULT
                }
            }
            (i, n) if i + n <= N => {
                unsafe {
                    ptr.copy_from_nonoverlapping(ids.as_ptr().cast(), i);
                    ptr.add(i)
                        .copy_from_nonoverlapping(names.as_ptr().cast(), n);
                }
                Self {
                    ids: Len(i as u8),
                    names: (Len(i as u8), Len(n as u8)),
                    types,
                    ..Self::DEFAULT
                }
            }
            (i, ..) if i < N => {
                unsafe {
                    ptr.copy_from_nonoverlapping(ids.as_ptr().cast(), i);
                }
                types[i].names = names;
                Self {
                    ids: Len(i as u8),
                    name_slices: (Len(i as u8), Len(i as u8 + 1)),
                    types,
                    ..Self::DEFAULT
                }
            }
            (.., n) if n < N => {
                unsafe {
                    ptr.copy_from_nonoverlapping(names.as_ptr().cast(), n);
                }
                types[n].ids = ids;
                Self {
                    names: (Len(0), Len(n as u8)),
                    id_slices: (Len(n as u8), Len(n as u8 + 1)),
                    types,
                    ..Self::DEFAULT
                }
            }
            _ if 2 <= N => {
                types[0].ids = ids;
                types[1].names = names;
                Self {
                    id_slices: (Len(0), Len(1)),
                    name_slices: (Len(1), Len(1)),
                    types,
                    ..Self::DEFAULT
                }
            }
            _ => Self::ANY,
        }
    }

    const fn is_any(&self) -> bool {
        self.any
    }

    fn contains(&self, typ: impl Type) -> bool {
        if self.is_any() {
            return true;
        }

        // match self.flags {
        //     Flags::Any => return true,
        //     Flags::Empty => return false,
        //     _ => {}
        // }

        // let id_range = 0..self.ids.0 as usize;
        // let name_range = self.names.0 .0 as usize..self.names.1 .0 as usize;
        // let id_slice_range = self.id_slices.0 .0 as usize..self.id_slices.1 .0 as usize;
        // let name_slice_range = self.name_slices.0 .0 as usize..self.name_slices.1 .0 as usize;

        if self.ids_match(typ) {
            return true;
        }

        if self.names_match(typ) {
            return true;
        }

        if self.id_slices_match(typ) {
            return true;
        }

        self.name_slices_match(typ)
    }

    #[inline(always)]
    fn ids_match(&self, typ: impl Type) -> bool {
        let range = 0..self.ids.0 as usize;
        unsafe {
            self.types
                .get_unchecked(range)
                .iter()
                .map(|t| t.id)
                .any(|i| typ.eq(i))
        }
    }

    #[inline(always)]
    fn names_match(&self, typ: impl Type) -> bool {
        let range = self.names.0.0 as usize..self.names.1.0 as usize;
        unsafe {
            self.types
                .get_unchecked(range)
                .iter()
                .map(|t| t.name)
                .any(|n| typ.eq(n))
        }
    }

    #[inline(always)]
    fn id_slices_match(&self, typ: impl Type) -> bool {
        let range = self.id_slices.0.0 as usize..self.id_slices.1.0 as usize;
        unsafe {
            self.types
                .get_unchecked(range)
                .iter()
                .map(|t| t.ids)
                .any(|ids| ids.iter().any(|i| typ.eq(i)))
        }
    }

    #[inline(never)]
    fn name_slices_match(&self, typ: impl Type) -> bool {
        let range = self.name_slices.0.0 as usize..self.name_slices.1.0 as usize;
        unsafe {
            self.types
                .get_unchecked(range)
                .iter()
                .map(|t| t.names)
                .any(|names| names.iter().any(|n| typ.eq(n)))
        }
    }

    // fn contains_old(&self, typ: impl Type) -> bool {
    //     if self.is_any() {
    //         return true;
    //     }

    //     // debug_assert!(
    //     //     !self.any
    //     //         && (self.ids as usize) <= N
    //     //         && (self.id_slices.0 as usize) < N
    //     //         && (self.id_slices.1 as usize) <= N
    //     //         && (self.names.0 as usize) < N
    //     //         && (self.names.1 as usize) <= N
    //     //         && (self.name_slices.0 as usize) < N
    //     //         && (self.name_slices.1 as usize) <= N
    //     // );

    //     // let typ_id = typ.id();
    //     // let typ_name = typ.name();

    //     // dbg!(core::any::type_name_of_val(&typ));
    //     // dbg!(core::any::type_name_of_val(&typ_id));
    //     // dbg!(core::any::type_name_of_val(&typ_name));

    //     if self.check_ids(typ) {
    //         return true;
    //     }

    //     if self.check_id_slices(typ) {
    //         return true;
    //     }

    //     if self.check_names(typ) {
    //         return true;
    //     }

    //     if self.check_name_slices(typ) {
    //         return true;
    //     }

    //     false
    // }

    // RUSTFLAGS="-C codegen-units=1 -C opt-level=3 -C target-cpu=native -C link-arg=-fuse-ld=mold"
    // fn check_ids_match(&self, id: TypeId) -> bool {
    //     match self.ids {
    //         0 => false,
    //         1 => unsafe { self.types.get_unchecked(0).id == id },
    //         // 2 => unsafe {
    //         //     let mut ret = false;
    //         //     ret |= self.types.get_unchecked(0).id == id;
    //         //     ret |= self.types.get_unchecked(1).id == id;
    //         //     ret
    //         // },
    //         // 3 => unsafe {
    //         //     let mut ret = false;
    //         //     ret |= self.types.get_unchecked(0).id == id;
    //         //     ret |= self.types.get_unchecked(1).id == id;
    //         //     ret |= self.types.get_unchecked(2).id == id;
    //         //     ret
    //         // },
    //         // 4 => unsafe {
    //         //     let mut ret = false;
    //         //     ret |= self.types.get_unchecked(0).id == id;
    //         //     ret |= self.types.get_unchecked(1).id == id;
    //         //     ret |= self.types.get_unchecked(2).id == id;
    //         //     ret |= self.types.get_unchecked(3).id == id;
    //         //     ret
    //         // },
    //         n if n <= N as u8 => {
    //             for i in 0..n as usize {
    //                 if unsafe { self.types.get_unchecked(i).id } == id {
    //                     return true;
    //                 }
    //             }
    //             false
    //         }
    //         _ => unsafe { core::hint::unreachable_unchecked() },
    //     }
    // }

    // fn check_ids<T: Type>(&self, typ: T) -> bool {
    //     // dbg!(core::any::type_name_of_val(&typ));
    //     let range = 0..self.ids as usize;
    //     debug_assert!(self.types.get(range.clone()).is_some());

    //     // if let Some(id) = T::CONSTANT {
    //     //     return self.check_ids_match(id);
    //     // }
    //     // if self.check_ids_match(id) {
    //     //     return true;
    //     // }

    //     // for i in 0..N {
    //     //     if unsafe { self.types.get_unchecked(i).id } == id{
    //     //         return true;
    //     //     }
    //     //     if i == self.ids_end as usize {
    //     //         break;
    //     //     }
    //     // }
    //     // return false;

    //     // let slice = unsafe { core::mem::transmute::<_, &[u128]>(self.types.get_unchecked(range)) };
    //     // let bytes = unsafe { slice::from_raw_parts(slice.as_ptr().cast::<u8>(), size_of_val(slice))};
    //     // let id_arr = unsafe { core::mem::transmute::<_,u128>(id)}.to_ne_bytes();
    //     // // let id_int = unsafe { core::mem::transmute::<_, &u128>(&id) };
    //     // return bytes.contains(&id_arr[0])
    //     // }

    //     // let slice = unsafe { core::mem::transmute::<_,&[u128]>(self.types.get_unchecked(range)) };
    //     // slice.iter().any(|i| typ.eq(i))
    //     unsafe {
    //         self.types
    //             .get_unchecked(range)
    //             .iter()
    //             .map(|t| t.id)
    //             .any(|i| typ.eq(i))
    //     }
    // }

    // fn check_id_slices(&self, typ: impl Type) -> bool {
    //     let range = self.id_slices.0 as usize..self.id_slices.1 as usize;
    //     debug_assert!(self.types.get(range.clone()).is_some());
    //     unsafe {
    //         self.types
    //             .get_unchecked(range)
    //             .iter()
    //             .map(|t| t.ids)
    //             .any(|ids| ids.iter().copied().any(|i| typ.eq(i)))
    //     }
    // }

    // #[inline(never)]
    // fn check_names(&self, typ: impl Type) -> bool {
    //     let range = self.names.0 as usize..self.names.1 as usize;
    //     debug_assert!(self.types.get(range.clone()).is_some());
    //     unsafe {
    //         self.types
    //             .get_unchecked(range)
    //             .iter()
    //             .map(|t| t.name)
    //             .any(|n| typ.eq(n))
    //     }
    // }

    // #[inline(never)]
    // fn check_name_slices(&self, typ: impl Type) -> bool {
    //     let range = self.name_slices.0 as usize..self.name_slices.1 as usize;
    //     debug_assert!(self.types.get(range.clone()).is_some());
    //     unsafe {
    //         self.types
    //             .get_unchecked(range)
    //             .iter()
    //             .map(|t| t.names)
    //             .any(|names| names.iter().any(|n| typ.eq(n)))
    //     }
    // }
}

impl<T: Type, const N: usize> FromIterator<T> for Repr<'_, N> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        if N == 0 {
            let mut iter = iter.into_iter();
            return if iter.size_hint().0 > 0 || iter.next().is_some() {
                Self::ANY
            } else {
                Self::EMPTY
            };
        }
        // let mut this = Self::EMPTY;

        let mut types = [TypeRepr::UNUSED; N];

        // let mut ids = [TypeRepr::UNUSED; N];
        // let mut names = [TypeRepr::UNUSED; N];

        // let mut id_iter = ids.iter_mut();
        // let mut name_iter = names.iter_mut();

        let mut id_pos = 0u8;
        let mut name_pos = N as u8;

        for typ in iter {
            if let Ok(id) = typ.id().get() {
                if id_pos == name_pos {
                    return Self::ANY;
                }
                types[id_pos as usize].id = id;
                id_pos += 1;
            }

            if let Ok(name) = typ.name().get() {
                if name_pos <= id_pos {
                    return Self::ANY;
                }
                name_pos -= 1;
                types[name_pos as usize].name = name;
            }
        }

        Self {
            ids: Len(id_pos),
            names: (Len(name_pos), Len(N as u8)),
            types,
            ..Self::DEFAULT
        }
    }
}

// #[derive(Clone, Copy)]
// union InnerSet<'a> {
//     inline: [SimpleTyp; 7],
//     borrowed: (Option<&'a [TypId]>, Option<&'a [&'static str]>),
// }

// #[derive(Debug, Clone)]
// enum Inner<'a, const N: usize = 7> {
//     Any,
//     Empty,
//     Inline {
//         ids: NonZeroU8,
//         names: NonZeroU8,
//         types: [TypeRepr<'a>; N],
//     },
//     Borrowed {
//         ids: &'a [TypId],
//         names: &'a [&'static str],
//     },
//     // Some {
//     //     ids: InnerSet<'a, TypeId, N>,
//     //     names: InnerSet<'a, &'static str, N>,
//     // },
//     // Ids([TypeId; TYPESET_ARRAY_SIZE]),
//     // RefIds(&'a [TypeId]),

//     // Names([&'static str; TYPESET_ARRAY_SIZE / 2]),
//     // RefNames(&'a [&'static str]),

//     // Types {
//     //     ids: &'a [TypeId],
//     //     names: &'a [&'static str],
//     // },
//     // Ids(InnerSet<'a, TypeId, N>),
//     // Names(InnerSet<'a, &'static str, N>),
//     // Both {
//     //     ids: InnerSet<'a, TypeId, N>,
//     //     names: InnerSet<'a, &'static str, N>,
//     // },

//     // Array([SimpleType; N]),
//     // Borrowed(&'a [SimpleType]),
//     // #[cfg(feature = "std")]
//     // Owned(Box<[SimpleType]>),
// }

// #[cfg(not(feature = "std"))]
// impl<const N: usize> Copy for Inner<'_, N> {}

// impl<T: Type, const N: usize> FromIterator<T> for Inner<'_, N> {
//     #[inline]
//     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
//         let iter = iter.into_iter();
//         if iter.size_hint().1 == Some(0) {
//             return Self::Borrowed(&[]);
//         }

//         #[cfg(not(feature = "std"))]
//         if iter.size_hint().0 > N {
//             return Self::Any;
//         }

//         let mut iter = iter.flat_map(|t| {
//             let id = t.id().get().map(SimpleType::Id);
//             let name = t.name().get().map(SimpleType::Name);
//             id.into_iter().chain(name)
//         });

//         let mut arr = [SimpleType::Untyped; N];
//         for (slot, typ) in arr.iter_mut().zip(iter.by_ref()) {
//             *slot = typ;
//         }

//         #[cfg(not(feature = "std"))]
//         if iter.next().is_some() {
//             return Self::Any;
//         }

//         #[cfg(feature = "std")]
//         if let Some(t) = iter.next() {
//             let cap = N + 1 + iter.size_hint().0;
//             let mut vec = Vec::with_capacity(cap);
//             vec.extend(arr);
//             vec.push(t);
//             vec.extend(iter);
//             return Self::Owned(vec.into_boxed_slice());
//         }

//         Self::Array(arr)
//     }
// }

// impl<'a, const N: usize> IntoIterator for Inner<'a, N> {
//     type Item = SimpleType;
//     type IntoIter = Iter<'a, N>;

//     #[inline]
//     fn into_iter(self) -> Self::IntoIter {
//         match self {
//             Self::Any => Iter::Slice([].iter().copied()),
//             Self::Array(arr) => Iter::Array(arr.into_iter()),
//             Self::Borrowed(slice) => Iter::Slice(slice.iter().copied()),
//             #[cfg(feature = "std")]
//             Self::Owned(boxed) => Iter::Owned(boxed.into_iter()),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub enum Iter<'a, const N: usize> {
    Slice(core::iter::Copied<core::slice::Iter<'a, SimpleType>>),
    Array(core::array::IntoIter<SimpleType, N>),
    #[cfg(feature = "std")]
    Owned(std::vec::IntoIter<SimpleType>),
}

impl<const N: usize> Iterator for Iter<'_, N> {
    type Item = SimpleType;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Slice(iter) => iter.next(),
            Self::Array(iter) => iter.next(),
            #[cfg(feature = "std")]
            Self::Owned(iter) => iter.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Slice(iter) => iter.size_hint(),
            Self::Array(iter) => iter.size_hint(),
            #[cfg(feature = "std")]
            Self::Owned(iter) => iter.size_hint(),
        }
    }

    #[inline]
    fn count(self) -> usize {
        match self {
            Self::Slice(iter) => iter.count(),
            Self::Array(iter) => iter.count(),
            #[cfg(feature = "std")]
            Self::Owned(iter) => iter.count(),
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self {
            Self::Slice(iter) => iter.nth(n),
            Self::Array(iter) => iter.nth(n),
            #[cfg(feature = "std")]
            Self::Owned(iter) => iter.nth(n),
        }
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        match self {
            Self::Slice(iter) => iter.last(),
            Self::Array(iter) => iter.last(),
            #[cfg(feature = "std")]
            Self::Owned(iter) => iter.last(),
        }
    }

    #[inline]
    fn all<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        match self {
            Self::Slice(iter) => iter.all(f),
            Self::Array(iter) => iter.all(f),
            #[cfg(feature = "std")]
            Self::Owned(iter) => iter.all(f),
        }
    }

    #[inline]
    fn any<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        match self {
            Self::Slice(iter) => iter.any(f),
            Self::Array(iter) => iter.any(f),
            #[cfg(feature = "std")]
            Self::Owned(iter) => iter.any(f),
        }
    }
}

impl<const N: usize> ExactSizeIterator for Iter<'_, N> {
    #[inline]
    fn len(&self) -> usize {
        match self {
            Self::Slice(iter) => iter.len(),
            Self::Array(iter) => iter.len(),
            #[cfg(feature = "std")]
            Self::Owned(iter) => iter.len(),
        }
    }
}

impl<const N: usize> core::iter::FusedIterator for Iter<'_, N> {}
