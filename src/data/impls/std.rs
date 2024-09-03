use ::std::collections::HashMap;
#[cfg(target_os = "linux")]
use std::os::unix::prelude::*;

use crate::{Data, Request};

impl Data for String {
    #[inline]
    fn query(&self, request: &mut impl Request) {
        request.provide_str(self);
    }
    #[inline]
    fn query_owned(self, request: &mut impl Request) {
        request.provide_string(self);
    }
}

mod path {
    use super::*;
    use ::std::path::{Path, PathBuf};

    impl Data for PathBuf {
        #[inline]
        fn query(&self, request: &mut impl Request) {
            request.provide_ref_unchecked(self);

            if true {
                // check if query request &str
                self.to_string_lossy().query(request);
            }

            #[cfg(target_os = "linux")]
            self.as_os_str().as_bytes().query(request);
        }
    }

    impl Data for Path {
        #[inline]
        fn query(&self, request: &mut impl Request) {
            // query.provide_other_ref(self);

            if true {
                // check if query request &str
                self.to_string_lossy().query(request);
            }

            #[cfg(target_os = "linux")]
            self.as_os_str().as_bytes().query(request);
        }
    }
}

mod ffi {
    use super::*;
    use ::std::ffi::{OsStr, OsString};

    impl Data for OsString {
        #[inline]
        fn query(&self, request: &mut impl Request) {
            request.provide_ref_unchecked(self);

            if true {
                // check if query request &str
                self.to_string_lossy().query(request);
            }

            #[cfg(target_os = "linux")]
            self.as_bytes().query(request);
        }
    }

    impl Data for OsStr {
        #[inline]
        fn query(&self, request: &mut impl Request) {
            // query.provide_other_ref(self);

            if true {
                // check if query request &str
                self.to_string_lossy().query(request);
            }

            #[cfg(target_os = "linux")]
            self.as_bytes().query(request);
        }
    }
}

mod net {
    use super::*;
    use ::std::net;

    #[cfg(not(feature = "well_known"))]
    const IP: &str = "ip";
    #[cfg(not(feature = "well_known"))]
    const PORT: &str = "port";

    #[cfg(feature = "well_known")]
    use crate::well_known::net::{IP, PORT};

    impl Data for net::Ipv4Addr {
        #[inline]
        fn query(&self, request: &mut impl Request) {
            request.provide_ref_unchecked(self);
            if true {
                // check if query requests u32
                request.provide_u32(self.to_bits());
            }

            if true {
                // check if query requests String
                request.provide_string(self.to_string());
            }
        }
    }

    impl Data for net::Ipv6Addr {
        #[inline]
        fn query(&self, request: &mut impl Request) {
            request.provide_ref_unchecked(self);

            if true {
                request.provide_u128(self.to_bits());
            }

            if true {
                // check if query requests bytes
                // todo: provide owned array when possible
                request.provide_bytes(&self.octets());
            }

            if true {
                request.provide_string(self.to_string());
            }
        }
    }

    impl Data for net::IpAddr {
        #[inline]
        fn query(&self, request: &mut impl Request) {
            request.provide_ref_unchecked(self);
            request.provide_discriminant(self);

            match self {
                Self::V4(ip) => ip.query(request),
                Self::V6(ip) => ip.query(request),
            }
        }
    }

    impl Data for net::SocketAddrV4 {
        #[inline]
        fn query(&self, request: &mut impl Request) {
            request.provide_ref_unchecked(self);
            request.provide_ref_unchecked(self.ip());

            if true {
                request.provide_string(self.to_string());
            }

            request.push_link((IP, self.ip().to_owned()));
            request.push_link((PORT, self.port()));
        }
    }

    impl Data for net::SocketAddrV6 {
        #[inline]
        fn query(&self, request: &mut impl Request) {
            request.provide_ref_unchecked(self);
            request.provide_ref_unchecked(self.ip());

            if true {
                request.provide_string(self.to_string());
            }

            request.push_link((IP, self.ip().to_owned()));
            request.push_link((PORT, self.port()));
        }
    }

    impl Data for net::SocketAddr {
        #[inline]
        fn query(&self, request: &mut impl Request) {
            request.provide_ref_unchecked(self);
            request.provide_discriminant(self);

            match self {
                Self::V4(ip) => ip.query(request),
                Self::V6(ip) => ip.query(request),
            }
        }
    }
}

impl<K, V, S: ::std::hash::BuildHasher> Data for HashMap<K, V, S>
where
    K: Data + Clone + ToOwned<Owned: Data + 'static>,
    V: Data + Clone + ToOwned<Owned: Data + 'static>,
{
    #[inline]
    fn query(&self, request: &mut impl Request) {
        for l in self.iter().map(|(k, v)| (k.clone(), v.clone())) {
            request.push_link(l);
        }
    }
}

impl<T> Data for Vec<T>
where
    T: Data + Clone + 'static,
{
    #[inline]
    fn query(&self, request: &mut impl Request) {
        for d in self.iter().cloned() {
            request.push_link(crate::link::Unkeyed(d));
        }
    }

    #[inline]
    fn query_owned(self, request: &mut impl Request) {
        for d in self {
            // todo use Unkeyed
            request.push_link(crate::link::Unkeyed(d));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::DataExt;

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
