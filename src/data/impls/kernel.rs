use core::{marker::PhantomData, mem::MaybeUninit, num::NonZero, ptr::NonNull};

use crate::{Data, Request};

pub struct Config;

trait AsSlice {
    fn ptr(&self) -> NonNull<u8>;
    fn len(&self) -> usize;
    #[inline]
    fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.ptr().as_ptr(), self.len()) }
    }
}

trait Backend<'a> {
    type Repr;

    fn empty() -> Self::Repr;
    fn from_str(value: &'a str) -> Self::Repr;
    fn from_boxed(value: Box<str>) -> Self::Repr;

    fn as_non_null(repr: &Self::Repr) -> NonNull<u8>;
    fn len(repr: &Self::Repr) -> usize;

    #[inline]
    fn from_static(value: &'static str) -> Self::Repr {
        Self::from_str(value)
    }

    #[inline]
    fn from_string(value: String) -> Self::Repr {
        Self::from_boxed(value.into_boxed_str())
    }

    #[inline]
    fn as_bytes(repr: &Self::Repr) -> &[u8] {
        unsafe { core::slice::from_raw_parts(Self::as_non_null(repr).as_ptr(), Self::len(repr)) }
    }

    #[inline]
    fn as_str(repr: &Self::Repr) -> &str {
        unsafe { core::str::from_utf8_unchecked(Self::as_bytes(repr)) }
    }

    #[inline]
    fn is_empty(repr: &Self::Repr) -> bool {
        Self::len(repr) == 0
    }
}

#[derive(Debug)]
struct CustomStr<'a, B: Backend<'a>>(B::Repr);

impl<'a, B: Backend<'a>> AsRef<str> for CustomStr<'a, B> {
    #[inline]
    fn as_ref(&self) -> &str {
        B::as_str(&self.0)
    }
}

struct Repr {
    
}

// #[derive(Debug)]
// enum CustomStr<'a, B: Backend<'a>> {
//     Empty(B::Empty),
//     Inline(B::Inline),
//     Static(B::Static),
//     Borrowed(B::Borrowed),
//     Owned(B::Owned),
// }

// impl<'a, B: Backend<'a>> CustomStr<'a, B> {
//     #[inline]
//     pub fn new_static(value: &'static str) -> Self {
//         if B::TRY_INLINE
//             && let Ok(inline) = B::Inline::try_from(value)
//         {
//             return Self::Inline(inline);
//         }
//         Self::Static(value.into())
//     }

//     pub fn new_borrowed(value: &'a str) -> Self {
//         if B::TRY_INLINE
//             && let Ok(inline) = B::Inline::try_from(value)
//         {
//             return Self::Inline(inline);
//         }
//         Self::Borrowed(value.into())
//     }

//     #[inline]
//     pub fn new_owned(value: impl Into<Box<str>>) -> Self {
//         let value = value.into();
//         if B::TRY_INLINE
//             && let Ok(inline) = B::Inline::try_from(value.as_ref())
//         {
//             return Self::Inline(inline);
//         }
//         Self::Owned(value.into())
//     }

//     #[inline]
//     pub fn as_bytes(&self) -> &[u8] {
//         match *self {
//             Self::Empty(..) => &[],
//             Self::Inline(ref value) => value.as_slice(),
//             Self::Static(ref value) => value.as_slice(),
//             Self::Borrowed(ref value) => value.as_slice(),
//             Self::Owned(ref value) => value.as_slice(),
//         }
//     }
//     #[inline]
//     pub fn as_str(&self) -> &str {
//         unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
//     }
// }

#[derive(Debug)]
pub enum NicheStr<'a> {
    Empty,
    Inline {
        data: [MaybeUninit<u8>; 14],
        len: NonZero<u8>,
    },
    Static {
        data: NonNull<u8>,
        len: NonZero<u32>,
    },
    Borrowed {
        data: NonNull<u8>,
        len: NonZero<u32>,
        _lifetime: PhantomData<&'a ()>,
    },
    Owned {
        data: NonNull<u8>,
        len: NonZero<u32>,
    },
}

impl Drop for NicheStr<'_> {
    #[inline]
    fn drop(&mut self) {
        use core::ptr::drop_in_place;
        use core::slice::from_raw_parts_mut;
        let Self::Owned { data, len } = self else {
            return;
        };
        let slice = unsafe { from_raw_parts_mut(data.as_ptr(), len.get() as usize) };
        unsafe {
            drop_in_place(slice);
        }
    }
}

impl<'a> NicheStr<'a> {
    #[inline]
    pub const fn new_static(value: &'static str) -> Self {
        let Some(len) = NonZero::new(value.len() as u32) else {
            return Self::Empty;
        };
        debug_assert!(value.len() <= u32::MAX as usize);
        if len.get() <= 14 {
            let mut data = [MaybeUninit::uninit(); 14];
            unsafe {
                core::ptr::copy_nonoverlapping(
                    value.as_ptr(),
                    data.as_mut_ptr().cast(),
                    value.len(),
                );
            }
            return Self::Inline {
                data,
                len: unsafe { NonZero::new_unchecked(len.get() as u8) },
            };
        }
        Self::Static {
            data: NonNull::from_ref(value).cast(),
            len,
        }
    }

    #[inline]
    pub fn new_owned(value: impl Into<Box<str>>) -> Self {
        let value = value.into();
        let Some(len) = NonZero::new(value.len() as u32) else {
            return Self::Empty;
        };
        debug_assert!(value.len() <= u32::MAX as usize);
        if len.get() <= 14 {
            let mut data = [MaybeUninit::uninit(); 14];
            unsafe {
                core::ptr::copy_nonoverlapping(
                    value.as_ptr(),
                    data.as_mut_ptr().cast(),
                    value.len(),
                );
            }
            return Self::Inline {
                data,
                len: unsafe { NonZero::new_unchecked(len.get() as u8) },
            };
        }
        let data = NonNull::from_ref(Box::leak(value)).cast();
        Self::Owned { data, len }
    }
}

impl AsRef<str> for NicheStr<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        use core::slice::from_raw_parts;
        use core::str::from_utf8_unchecked;
        match *self {
            Self::Empty => "",
            Self::Inline { data, len } => unsafe {
                from_utf8_unchecked(from_raw_parts(data.as_ptr().cast(), len.get() as _))
            },
            Self::Static { data, len }
            | Self::Borrowed { data, len, .. }
            | Self::Owned { data, len } => unsafe {
                from_utf8_unchecked(from_raw_parts(data.as_ptr(), len.get() as _))
            },
        }
    }
}

impl Clone for NicheStr<'_> {
    #[inline]
    fn clone(&self) -> Self {
        match *self {
            Self::Empty => Self::Empty,
            NicheStr::Static { data, len } => Self::Static { data, len },
            Self::Inline { data, len } => Self::Inline { data, len },
            Self::Borrowed { data, len, .. } => Self::Borrowed {
                data,
                len,
                _lifetime: PhantomData,
            },
            Self::Owned { data, len, .. } => {
                let slice =
                    unsafe { core::slice::from_raw_parts(data.as_ptr(), len.get() as usize) };
                let vec = slice.to_owned().into_boxed_slice();
                let data = NonNull::from_ref(vec.as_ref()).cast();
                Self::Owned {
                    data: data,
                    len: len,
                }
            }
        }
    }
}

impl core::fmt::Display for NicheStr<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use core::slice::from_raw_parts;
        use core::str::from_utf8_unchecked;
        match *self {
            Self::Empty => Ok(()),
            Self::Inline { data, len } => {
                let slice = unsafe {
                    from_utf8_unchecked(from_raw_parts(data.as_ptr().cast(), len.get() as _))
                };
                f.write_str(slice)
            }
            Self::Static { data, len }
            | Self::Borrowed { data, len, .. }
            | Self::Owned { data, len } => {
                let slice =
                    unsafe { from_utf8_unchecked(from_raw_parts(data.as_ptr(), len.get() as _)) };
                f.write_str(slice)
            }
        }
    }
}

impl<'a> From<&'a str> for NicheStr<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        // maybe create inline if possible
        debug_assert!(value.len() <= u32::MAX as usize);
        match NonZero::new(value.len() as u32) {
            None => Self::Empty,
            Some(len) => Self::Borrowed {
                data: NonNull::from_ref(value).cast(),
                len,
                _lifetime: PhantomData,
            },
        }
    }
}

impl From<Box<str>> for NicheStr<'_> {
    #[inline]
    fn from(value: Box<str>) -> Self {
        Self::new_owned(value)
    }
}

impl From<String> for NicheStr<'_> {
    #[inline]
    fn from(value: String) -> Self {
        Self::new_owned(value)
    }
}

#[derive(Debug, Clone)]
pub struct TextValue<'a>(NicheStr<'a>);

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Disabled,
    BuiltIn,
    Module,
    String(TextValue<'a>),
    Int(i64),
    Hex(u64),
}

impl core::fmt::Display for Value<'_> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            Self::Disabled => f.write_str("n"),
            Self::BuiltIn => f.write_str("y"),
            Self::Module => f.write_str("m"),
            Self::String(ref value) => value.0.fmt(f),
            Self::Int(value) => write!(f, "{value}"),
            Self::Hex(value) => write!(f, "0x{value:x}"),
        }
    }
}

impl<'a> From<&'a str> for Value<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        Self::String(TextValue(value.into()))
    }
}

impl From<String> for Value<'_> {
    #[inline]
    fn from(value: String) -> Self {
        Self::String(TextValue(value.into()))
    }
}

mod kconfig {

    #[derive(Debug, Clone, Copy)]
    enum ValueRef<'a> {
        Disabled,
        BuiltIn,
        Module,
        String(&'a str),
        Int(i64),
        Hex(u64),
    }

    #[derive(Debug, Clone)]
    enum Value {
        Disabled,
        BuiltIn,
        Module,
        String(Box<str>),
        Int(i64),
        Hex(u64),
    }

    #[derive(Debug, Clone, Copy)]
    struct OptionRef<'a> {
        symbol: &'a str,
        value: ValueRef<'a>,
    }

    #[derive(Debug, Clone)]
    struct Option {
        symbol: Box<str>,
        value: Value,
    }
}
