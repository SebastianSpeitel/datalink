use std::collections::HashMap;
#[cfg(target_os = "linux")]
use std::os::unix::prelude::*;

use crate::{Data, LinkBuilder, Request, RequestExt};

impl Data for String {
    #[inline]
    fn query(&self, mut request: impl Request) {
        request.provide_str(self);
    }
    #[inline]
    fn query_owned(self, mut request: impl Request) {
        request.provide_value(self);
    }
}

/// # Example
/// ```
/// use datalink::{Data, DataExt};
///
/// let s = Box::from("Hello, world!");
/// assert_eq!(Box::<str>::as_string(&s).unwrap(), "Hello, world!");
/// ```
impl Data for Box<str> {
    #[inline]
    fn query(&self, mut request: impl Request) {
        request.provide_str(self);
    }

    #[inline]
    fn query_owned(self, mut request: impl Request) {
        request.provide_value(self);
    }
}

impl Data for Box<[u8]> {
    #[inline]
    fn query(&self, mut request: impl Request) {
        request.provide_bytes(self);
    }

    #[inline]
    fn query_owned(self, mut request: impl Request) {
        request.provide_value(self);
    }
}

mod path {
    use super::*;
    use std::path::{Path, PathBuf};

    impl Data for PathBuf {
        #[inline]
        fn query(&self, mut request: impl Request) {
            use std::borrow::Cow;
            if request.would_accept_value_of::<&str>() || request.would_accept_value_of::<String>()
            {
                match self.to_string_lossy() {
                    Cow::Borrowed(s) => request.provide_str(s),
                    Cow::Owned(s) => request.provide_value(s),
                }
            }

            #[cfg(target_os = "linux")]
            self.as_os_str().as_bytes().query(request);
        }
    }

    impl Data for Path {
        #[inline]
        fn query(&self, mut request: impl Request) {
            use std::borrow::Cow;
            if request.would_accept_value_of::<&str>() || request.would_accept_value_of::<String>()
            {
                match self.to_string_lossy() {
                    Cow::Borrowed(s) => request.provide_str(s),
                    Cow::Owned(s) => request.provide_value(s),
                }
            }

            #[cfg(target_os = "linux")]
            self.as_os_str().as_bytes().query(request);
        }
    }

    impl Data for &Path {
        #[inline]
        fn query(&self, request: impl Request) {
            (*self).query(request);
        }
    }
}

mod ffi {
    use super::*;
    use std::ffi::{OsStr, OsString};

    impl Data for OsString {
        #[inline]
        fn query(&self, mut request: impl Request) {
            use std::borrow::Cow;
            if request.would_accept_value_of::<&str>() || request.would_accept_value_of::<String>()
            {
                match self.to_string_lossy() {
                    Cow::Borrowed(s) => request.provide_str(s),
                    Cow::Owned(s) => request.provide_value(s),
                }
            }

            #[cfg(target_os = "linux")]
            self.as_bytes().query(request);
        }
    }

    impl Data for OsStr {
        #[inline]
        fn query(&self, mut request: impl Request) {
            use std::borrow::Cow;
            if request.would_accept_value_of::<&str>() || request.would_accept_value_of::<String>()
            {
                match self.to_string_lossy() {
                    Cow::Borrowed(s) => request.provide_str(s),
                    Cow::Owned(s) => request.provide_value(s),
                }
            }

            #[cfg(target_os = "linux")]
            self.as_bytes().query(request);
        }
    }
}

mod net {
    use super::*;
    use std::net;

    #[cfg(not(feature = "well_known"))]
    const IP: &str = "ip";
    #[cfg(not(feature = "well_known"))]
    const PORT: &str = "port";

    #[cfg(feature = "well_known")]
    use crate::well_known::net::{IP, PORT};

    impl Data for net::Ipv4Addr {
        #[inline]
        fn query(&self, mut request: impl Request) {
            request.provide_value(self.to_bits());
            request.provide_value(self.octets());
            request.provide_value_with(|| self.to_string());
        }
    }

    impl Data for net::Ipv6Addr {
        #[inline]
        fn query(&self, mut request: impl Request) {
            request.provide_value(self.to_bits());
            request.provide_value(self.octets());
            request.provide_value(self.segments());
            request.provide_value_with(|| self.to_string());
        }
    }

    impl Data for net::IpAddr {
        #[inline]
        fn query(&self, mut request: impl Request) {
            request.provide_discriminant(self);

            match self {
                Self::V4(ip) => ip.query(request),
                Self::V6(ip) => ip.query(request),
            }
        }

        #[inline]
        fn query_owned(self, mut request: impl Request) {
            request.provide_discriminant(&self);
            match self {
                Self::V4(ip) => ip.query_owned(request),
                Self::V6(ip) => ip.query_owned(request),
            }
        }
    }

    impl Data for net::SocketAddrV4 {
        #[inline]
        fn query(&self, mut request: impl Request) {
            request.provide_value_with(|| self.to_string());
            request.provide_link((IP, *self.ip()));
            request.provide_link((PORT, self.port()));
        }
    }

    impl Data for net::SocketAddrV6 {
        #[inline]
        fn query(&self, mut request: impl Request) {
            request.provide_value_with(|| self.to_string());
            request.provide_link((IP, *self.ip()));
            request.provide_link((PORT, self.port()));
            request.provide_link(("flowinfo", self.flowinfo()));
            request.provide_link(("scope_id", self.scope_id()));
        }
    }

    impl Data for net::SocketAddr {
        #[inline]
        fn query(&self, mut request: impl Request) {
            request.provide_discriminant(self);

            match self {
                Self::V4(ip) => ip.query(request),
                Self::V6(ip) => ip.query(request),
            }
        }
    }
}

impl<K, V, S: ::core::hash::BuildHasher> Data for HashMap<K, V, S>
where
    K: Data + ToOwned<Owned: Data + 'static> + 'static,
    V: Data + ToOwned<Owned: Data + 'static> + 'static,
{
    #[inline]
    fn query(&self, mut request: impl Request) {
        for (k, v) in self {
            request.provide_link_with(|| LinkBuilder::new_ownable(v).key_ownable(k));
        }
    }

    #[inline]
    fn query_owned(self, mut request: impl Request) {
        for e in self {
            request.provide_link(e);
        }
    }
}

impl<T> Data for Vec<T>
where
    T: Data + ToOwned<Owned: Data + 'static> + 'static,
{
    #[inline]
    fn query(&self, mut request: impl Request) {
        for e in self {
            let link = LinkBuilder::new_ownable(e);
            request.provide_link_with(|| link);
        }
    }

    #[inline]
    fn query_owned(self, mut request: impl Request) {
        for d in self {
            request.provide_link((d,));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DataExt;

    #[test]
    fn string() {
        let s = String::from("Hello, world!");

        assert_eq!(DataExt::as_string(&s), Some("Hello, world!".into()));
    }

    #[test]
    fn str() {
        let s = "Hello, world!";

        assert_eq!(DataExt::as_string(&s), Some("Hello, world!".into()));
    }

    #[test]
    fn bool() {
        let b = true;

        assert_eq!(DataExt::as_bool(&b), Some(true));
    }
}
