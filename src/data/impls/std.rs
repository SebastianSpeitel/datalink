use ::std::collections::HashMap;

use crate::data::Data;
use crate::links::{LinkError, Links, LinksExt};
use crate::rr::{Req, Request};

impl<R: Req> Data<R> for String {
    #[inline]
    fn provide_value<'d>(&self, mut request: Request<'d, R>) {
        request.provide_ref(self);
    }
}
mod path {
    use super::*;
    #[cfg(target_os = "linux")]
    use ::std::os::unix::ffi::OsStrExt;
    use ::std::path::{Path, PathBuf};

    impl<R: Req> Data<R> for PathBuf {
        #[inline]
        fn provide_value<'d>(&self, mut request: Request<'d, R>) {
            request.provide_ref(self);

            if R::requests::<str>() {
                request.provide_str(self.to_string_lossy().as_ref());
            }

            #[cfg(target_os = "linux")]
            request.provide_bytes(OsStrExt::as_bytes(self.as_os_str()));
        }
    }

    impl<R: Req> Data<R> for Path {
        #[inline]
        fn provide_value<'d>(&self, mut request: Request<'d, R>) {
            if R::requests::<str>() {
                request.provide_str(self.to_string_lossy().as_ref());
            }

            #[cfg(target_os = "linux")]
            request.provide_bytes(OsStrExt::as_bytes(self.as_os_str()));
        }
    }
}

mod ffi {
    use super::*;
    use ::std::ffi::{OsStr, OsString};
    #[cfg(target_os = "linux")]
    use ::std::os::unix::ffi::OsStrExt;

    impl<R: Req> Data<R> for OsString {
        #[inline]
        fn provide_value<'d>(&self, mut request: Request<'d, R>) {
            request.provide_ref(self);

            if R::requests::<str>() {
                request.provide_str(self.to_string_lossy().as_ref());
            }

            #[cfg(target_os = "linux")]
            request.provide_bytes(OsStrExt::as_bytes(self.as_os_str()));
        }
    }

    impl<R: Req> Data<R> for OsStr {
        #[inline]
        fn provide_value<'d>(&self, mut request: Request<'d, R>) {
            if R::requests::<str>() {
                request.provide_str(self.to_string_lossy().as_ref());
            }

            #[cfg(target_os = "linux")]
            request.provide_bytes(OsStrExt::as_bytes(self));
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

    impl<R: Req> Data<R> for net::Ipv4Addr {
        #[inline]
        fn provide_value<'d>(&self, mut request: Request<'d, R>) {
            request.provide_ref(self);
            if R::requests::<String>() {
                request.provide_str_owned(self.to_string());
            }
        }
    }

    impl<R: Req> Data<R> for net::Ipv6Addr {
        #[inline]
        fn provide_value<'d>(&self, mut request: Request<'d, R>) {
            request.provide_ref(self);
            if R::requests::<String>() {
                request.provide_str_owned(self.to_string());
            }
        }
    }

    impl<R: Req> Data<R> for net::IpAddr {
        #[inline]
        fn provide_value<'d>(&self, request: Request<'d, R>) {
            match self {
                net::IpAddr::V4(ip) => ip.provide_value(request),
                net::IpAddr::V6(ip) => ip.provide_value(request),
            }
        }

        #[inline]
        fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
            match self {
                net::IpAddr::V4(ip) => Data::<R>::provide_links(ip, links),
                net::IpAddr::V6(ip) => Data::<R>::provide_links(ip, links),
            }
        }
    }

    impl<R: Req> Data<R> for net::SocketAddrV4 {
        #[inline]
        fn provide_value<'d>(&self, mut request: Request<'d, R>) {
            request.provide_ref(self);

            if R::requests::<String>() {
                request.provide_str_owned(self.to_string());
            }
        }

        #[inline]
        fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
            links.push_keyed(Box::new(self.ip().to_owned()), Box::new(IP))?;
            links.push_keyed(Box::new(self.port()), Box::new(PORT))?;

            Ok(())
        }
    }

    impl<R: Req> Data<R> for net::SocketAddrV6 {
        #[inline]
        fn provide_value<'d>(&self, mut request: Request<'d, R>) {
            request.provide_ref(self);

            if R::requests::<String>() {
                request.provide_str_owned(self.to_string());
            }
        }

        #[inline]
        fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
            links.push_keyed(Box::new(self.ip().to_owned()), Box::new(IP))?;
            links.push_keyed(Box::new(self.port()), Box::new(PORT))?;

            Ok(())
        }
    }

    impl<R: Req> Data<R> for net::SocketAddr {
        #[inline]
        fn provide_value<'d>(&self, request: Request<'d, R>) {
            match self {
                net::SocketAddr::V4(addr) => addr.provide_value(request),
                net::SocketAddr::V6(addr) => addr.provide_value(request),
            }
        }

        #[inline]
        fn provide_links(&self, links: &mut dyn Links) -> Result<(), LinkError> {
            match self {
                net::SocketAddr::V4(addr) => Data::<R>::provide_links(addr, links),
                net::SocketAddr::V6(addr) => Data::<R>::provide_links(addr, links),
            }
        }
    }
}

impl<R, K, V, S: ::std::hash::BuildHasher> Data<R> for HashMap<K, V, S>
where
    R: Req,
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
            if query.matches_owned((k, v)) {
                Some((k.to_owned(), v.to_owned()))
            } else {
                None
            }
        }))?;
        Ok(())
    }
}

impl<R: Req, T> Data<R> for Vec<T>
where
    T: Data<R> + Data + ToOwned + 'static,
    T::Owned: Data<R> + Data,
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
