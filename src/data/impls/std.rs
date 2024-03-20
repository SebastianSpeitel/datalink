use ::std::{borrow::Cow, collections::HashMap};

use crate::data::Data;
use crate::links::{LinkError, Links, LinksExt};
use crate::value::ValueBuiler;

impl Data for String {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.str(Cow::Borrowed(self));
    }
}

impl Data for char {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.u32(*self as u32);
        value.str(Cow::Owned(self.to_string()));
    }
}

impl Data for [u8] {
    #[inline]
    fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
        value.bytes(Cow::Borrowed(self));
    }
}

mod path {
    use super::*;
    #[cfg(target_os = "linux")]
    use ::std::os::unix::ffi::OsStrExt;
    use ::std::path::{Path, PathBuf};

    impl Data for PathBuf {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.str(self.to_string_lossy());

            #[cfg(target_os = "linux")]
            value.bytes(Cow::Borrowed(OsStrExt::as_bytes(self.as_os_str())));
        }
    }

    impl Data for Path {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.str(self.to_string_lossy());

            #[cfg(target_os = "linux")]
            value.bytes(Cow::Borrowed(OsStrExt::as_bytes(self.as_os_str())));
        }
    }
}

mod ffi {
    use super::*;
    use ::std::ffi::{OsStr, OsString};
    #[cfg(target_os = "linux")]
    use ::std::os::unix::ffi::OsStrExt;

    impl Data for OsString {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.str(self.to_string_lossy());

            #[cfg(target_os = "linux")]
            value.bytes(Cow::Borrowed(OsStrExt::as_bytes(self.as_os_str())));
        }
    }

    impl Data for OsStr {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.str(self.to_string_lossy());

            #[cfg(target_os = "linux")]
            value.bytes(Cow::Borrowed(OsStrExt::as_bytes(self)));
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
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.str(self.to_string().into());
        }
    }

    impl Data for net::Ipv6Addr {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.str(self.to_string().into());
        }
    }

    impl Data for net::IpAddr {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            match self {
                net::IpAddr::V4(ip) => ip.provide_value(value),
                net::IpAddr::V6(ip) => ip.provide_value(value),
            }
        }

        #[inline]
        fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
            match self {
                net::IpAddr::V4(ip) => ip.provide_links(links),
                net::IpAddr::V6(ip) => ip.provide_links(links),
            }
        }
    }

    impl Data for net::SocketAddrV4 {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.str(self.to_string().into());
        }

        #[inline]
        fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
            links.push_keyed(Box::new(self.ip().to_owned()), Box::new(IP))?;
            links.push_keyed(Box::new(self.port()), Box::new(PORT))?;

            Ok(())
        }
    }

    impl Data for net::SocketAddrV6 {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            value.str(self.to_string().into());
        }

        #[inline]
        fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
            links.push_keyed(Box::new(self.ip().to_owned()), Box::new(IP))?;
            links.push_keyed(Box::new(self.port()), Box::new(PORT))?;

            Ok(())
        }
    }

    impl Data for net::SocketAddr {
        #[inline]
        fn provide_value<'d>(&'d self, value: &mut dyn ValueBuiler<'d>) {
            match self {
                net::SocketAddr::V4(addr) => addr.provide_value(value),
                net::SocketAddr::V6(addr) => addr.provide_value(value),
            }
        }

        #[inline]
        fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
            match self {
                net::SocketAddr::V4(addr) => addr.provide_links(links),
                net::SocketAddr::V6(addr) => addr.provide_links(links),
            }
        }
    }
}

impl<K, V, S: ::std::hash::BuildHasher> Data for HashMap<K, V, S>
where
    K: Data + ToOwned + 'static,
    K::Owned: Data,
    V: Data + ToOwned + 'static,
    V::Owned: Data,
{
    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        links.extend(self.iter().map(|(k, t)| (k.to_owned(), t.to_owned())))?;
        Ok(())
    }

    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        use crate::query::Filter;
        links.extend(self.iter().filter_map(|(k, v)| {
            if query.matches((k, v)) {
                Some((k.to_owned(), v.to_owned()))
            } else {
                None
            }
        }))?;
        Ok(())
    }
}

impl<T> Data for Vec<T>
where
    T: Data + ToOwned + 'static,
    T::Owned: Data,
{
    #[inline]
    fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
        links.extend(self.iter().map(ToOwned::to_owned))?;
        Ok(())
    }

    #[inline]
    fn query_links(
        &self,
        links: &mut dyn Links,
        query: &crate::query::Query,
    ) -> Result<(), LinkError> {
        use crate::query::Filter;
        links.extend(self.iter().filter_map(|v| {
            if Filter::<T>::matches(query, v) {
                Some(v.to_owned())
            } else {
                None
            }
        }))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::data::DataExt;

    #[test]
    fn string() {
        let s = String::from("Hello, world!");

        assert_eq!(DataExt::as_str(&s), Some("Hello, world!".into()));
    }

    #[test]
    fn str() {
        let s = "Hello, world!";

        assert_eq!(DataExt::as_str(&s), Some("Hello, world!".into()));
    }

    #[test]
    fn bool() {
        let b = true;

        assert_eq!(DataExt::as_bool(&b), Some(true));
    }
}
