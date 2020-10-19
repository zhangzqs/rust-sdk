use chashmap::CHashMap;
use serde::{Deserialize, Serialize};
use serde_json::Error as JSONError;
use std::{
    any::Any,
    env::temp_dir,
    fmt::Debug,
    fs::{create_dir_all, OpenOptions},
    io::Error as IOError,
    iter::FromIterator,
    net::IpAddr,
    path::{Path, PathBuf},
    result::Result,
    time::{Duration, SystemTime},
};
use thiserror::Error;

#[cfg(feature = "async")]
use futures::future::BoxFuture;

pub trait Resolver: Any + Debug + Sync + Send {
    fn resolve(&self, domain: &str, port: u16) -> ResolveResult;

    #[inline]
    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(r#async)))]
    fn async_retry<'a>(&'a self, domain: &'a str, port: u16) -> BoxFuture<'a, ResolveResult> {
        Box::pin(async move { self.resolve(domain, port) })
    }

    fn as_any(&self) -> &dyn Any;
    fn as_resolver(&self) -> &dyn Resolver;
}

#[derive(Error, Debug)]
pub enum ResolveError {
    #[error("Resolve domain name error: {0}")]
    IOError(#[from] IOError),
}
pub type ResolveResult = Result<Box<[IpAddr]>, ResolveError>;

#[derive(Debug)]
pub struct SimpleResolver;

impl Resolver for SimpleResolver {
    #[inline]
    fn resolve(&self, domain: &str, port: u16) -> ResolveResult {
        use std::net::ToSocketAddrs;
        Ok((domain, port)
            .to_socket_addrs()?
            .map(|socket_addr| socket_addr.ip())
            .collect())
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn as_resolver(&self) -> &dyn Resolver {
        self
    }
}

#[derive(Debug)]
pub struct CachedResolver<R: Resolver> {
    backend: R,
    cache: CHashMap<CachedResolverKey, CachedResolverValue>,
    lifetime: Duration,
    persistent: Option<PersistentFile>,
}

#[derive(Debug)]
struct PersistentFile {
    path: PathBuf,
    auto_persistent: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
struct CachedResolverKey {
    domain: Box<str>,
    port: u16,
}

#[derive(Debug, Clone)]
struct CachedResolverValue {
    ip_addrs: Box<[IpAddr]>,
    deadline: SystemTime,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct PersistentCache {
    lifetime: Duration,
    cache_entries: Vec<PersistentCacheEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistentCacheEntry {
    key: CachedResolverKey,
    ip_addrs: Box<[IpAddr]>,
    deadline: SystemTime,
}

impl<R: Resolver> CachedResolver<R> {
    #[inline]
    pub fn new(backend: R, lifetime: Duration) -> Self {
        Self {
            backend,
            lifetime,
            cache: Default::default(),
            persistent: None,
        }
    }

    pub fn load_from(
        path: impl Into<PathBuf>,
        auto_persistent: bool,
        backend: R,
    ) -> PersistentResult<Self> {
        let path = path.into();
        let (cache, lifetime) = Self::load_cache_from_persistent_file(&path)
            .map(|cache| cache.into_cache_and_lifetime())?;
        Ok(Self {
            backend,
            lifetime,
            cache,
            persistent: Some(PersistentFile {
                path,
                auto_persistent,
            }),
        })
    }

    pub fn load_or_create_from(
        path: impl Into<PathBuf>,
        auto_persistent: bool,
        backend: R,
    ) -> Self {
        let path = path.into();
        let (cache, lifetime) = Self::load_cache_from_persistent_file(&path)
            .map(|cache| cache.into_cache_and_lifetime())
            .unwrap_or_else(|_| (Default::default(), Duration::from_secs(120)));
        Self {
            backend,
            lifetime,
            cache,
            persistent: Some(PersistentFile {
                path,
                auto_persistent,
            }),
        }
    }

    pub fn persistent(&self) -> PersistentResult<()> {
        self.save_cache_into_persistent_file()
    }

    pub fn set_auto_persistent(&mut self, auto_persistent: bool) {
        if let Some(persistent) = &mut self.persistent {
            persistent.auto_persistent = auto_persistent;
        }
    }

    pub fn as_backend(&self) -> &R {
        &self.backend
    }

    pub fn into_backend(self) -> R {
        self.backend
    }

    fn load_cache_from_persistent_file(path: &Path) -> PersistentResult<PersistentCache> {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let cache = serde_json::from_reader(&mut file)?;
        Ok(cache)
    }

    fn save_cache_into_persistent_file(&self) -> PersistentResult<()> {
        if let Some(persistent) = &self.persistent {
            if let Some(parent_dir) = persistent.path.parent() {
                create_dir_all(parent_dir)?;
            }
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&persistent.path)?;
            serde_json::to_writer(
                &mut file,
                &PersistentCache::from_cache_and_lifetime(self.cache.to_owned(), self.lifetime),
            )?;
        }
        Ok(())
    }

    pub fn persistent_path(&self) -> Option<&Path> {
        self.persistent
            .as_ref()
            .map(|persistent| persistent.path.as_path())
    }

    pub fn default_persistent_path() -> PathBuf {
        let mut path = dirs::cache_dir().unwrap_or_else(temp_dir);
        path.push(".qiniu-rust-sdk");
        path.push("resolver-cache.json");
        path
    }
}

impl Default for CachedResolver<SimpleResolver> {
    #[inline]
    fn default() -> Self {
        Self::load_or_create_from(Self::default_persistent_path(), true, SimpleResolver)
    }
}

impl<R: Resolver> Resolver for CachedResolver<R> {
    #[inline]
    fn resolve(&self, domain: &str, port: u16) -> ResolveResult {
        let cache_key = CachedResolverKey {
            domain: domain.to_owned().into(),
            port,
        };
        if let Some(cache_entry) = self.cache.get(&cache_key) {
            if cache_entry.deadline > SystemTime::now() {
                return Ok(cache_entry.ip_addrs.to_owned());
            }
        }
        let mut resolve_result: Option<ResolveResult> = None;
        let mut need_to_persistent = false;
        self.cache.alter(cache_key, |may_be_cache| {
            if let Some(cache) = &may_be_cache {
                if cache.deadline > SystemTime::now() {
                    resolve_result = Some(Ok(cache.ip_addrs.to_owned()));
                    return may_be_cache;
                }
            }
            match self.backend.resolve(domain, port) {
                Ok(ip_addrs) => {
                    resolve_result = Some(Ok(ip_addrs.to_owned()));
                    if let Some(persistent) = &self.persistent {
                        if persistent.auto_persistent {
                            need_to_persistent = true;
                        }
                    }
                    Some(CachedResolverValue {
                        ip_addrs,
                        deadline: SystemTime::now() + self.lifetime,
                    })
                }
                Err(err) => {
                    resolve_result = Some(Err(err));
                    None
                }
            }
        });

        if need_to_persistent {
            self.save_cache_into_persistent_file().ok();
        }

        resolve_result.unwrap()
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn as_resolver(&self) -> &dyn Resolver {
        self
    }
}

#[derive(Error, Debug)]
pub enum PersistentError {
    #[error("i/o error: {0}")]
    IOError(#[from] IOError),

    #[error("JSON serialize/deserialize error: {0}")]
    JSONError(#[from] JSONError),
}
pub type PersistentResult<T> = Result<T, PersistentError>;

impl PersistentCache {
    fn into_cache_and_lifetime(
        self,
    ) -> (CHashMap<CachedResolverKey, CachedResolverValue>, Duration) {
        let cache = CHashMap::from_iter(self.cache_entries.into_iter().map(|entry| {
            (
                entry.key,
                CachedResolverValue {
                    ip_addrs: entry.ip_addrs,
                    deadline: entry.deadline,
                },
            )
        }));
        (cache, self.lifetime)
    }

    fn from_cache_and_lifetime(
        cache: CHashMap<CachedResolverKey, CachedResolverValue>,
        lifetime: Duration,
    ) -> Self {
        PersistentCache {
            cache_entries: cache
                .into_iter()
                .map(|(key, value)| PersistentCacheEntry {
                    key,
                    ip_addrs: value.ip_addrs,
                    deadline: value.deadline,
                })
                .collect(),
            lifetime,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::HashMap,
        error::Error,
        fs::File,
        io::ErrorKind as IOErrorKind,
        net::Ipv4Addr,
        sync::Arc,
        thread::{sleep, spawn},
    };
    use tap::tap::TapOptional;
    use tempfile::tempdir;

    #[derive(Debug, Clone, Default)]
    struct ResolverFromTable {
        table: HashMap<CachedResolverKey, Box<[IpAddr]>>,
        resolved: CHashMap<CachedResolverKey, usize>,
    }

    impl ResolverFromTable {
        fn add(&mut self, domain: impl Into<String>, port: u16, ip_addrs: Vec<IpAddr>) {
            self.table.insert(
                CachedResolverKey {
                    domain: domain.into().into_boxed_str(),
                    port,
                },
                ip_addrs.into_boxed_slice(),
            );
        }

        fn resolved(&self, domain: impl Into<String>, port: u16) -> Option<usize> {
            self.resolved
                .get(&CachedResolverKey {
                    domain: domain.into().into_boxed_str(),
                    port,
                })
                .map(|v| *v)
        }
    }

    impl Resolver for ResolverFromTable {
        #[inline]
        fn resolve(&self, domain: &str, port: u16) -> ResolveResult {
            let key = CachedResolverKey {
                domain: domain.to_owned().into_boxed_str(),
                port,
            };
            Ok(self
                .table
                .get(&key)
                .tap_some(|_| {
                    self.resolved.alter(key, |resolved| {
                        if let Some(resolved) = resolved {
                            Some(resolved + 1)
                        } else {
                            Some(1)
                        }
                    });
                })
                .cloned()
                .unwrap_or(vec![].into_boxed_slice()))
        }

        #[inline]
        fn as_any(&self) -> &dyn Any {
            self
        }

        #[inline]
        fn as_resolver(&self) -> &dyn Resolver {
            self
        }
    }

    #[test]
    fn test_thread_safe_cached_resolver() -> Result<(), Box<dyn Error>> {
        let mut backend = ResolverFromTable::default();
        backend.add(
            "test_domain_1.com",
            80,
            vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1))],
        );
        backend.add(
            "test_domain_2.com",
            80,
            vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2))],
        );
        backend.add(
            "test_domain_3.com",
            80,
            vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 3))],
        );
        let resolver = Arc::new(CachedResolver::new(backend, Duration::from_secs(5)));
        let threads_1: Vec<_> = (0..3)
            .map(|_| {
                let resolver = resolver.to_owned();
                spawn(move || {
                    let result = resolver.resolve("test_domain_1.com", 80).unwrap();
                    assert_eq!(
                        result,
                        vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1))].into_boxed_slice()
                    );
                })
            })
            .collect();
        let threads_2: Vec<_> = (0..5)
            .map(|_| {
                let resolver = resolver.to_owned();
                spawn(move || {
                    let result = resolver.resolve("test_domain_2.com", 80).unwrap();
                    assert_eq!(
                        result,
                        vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2))].into_boxed_slice()
                    );
                })
            })
            .collect();
        let threads_3: Vec<_> = (0..7)
            .map(|_| {
                let resolver = resolver.to_owned();
                spawn(move || {
                    let result = resolver.resolve("test_domain_3.com", 80).unwrap();
                    assert_eq!(
                        result,
                        vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 3))].into_boxed_slice()
                    );
                })
            })
            .collect();
        threads_1
            .into_iter()
            .chain(threads_2.into_iter())
            .chain(threads_3.into_iter())
            .try_for_each(|thread| thread.join())
            .unwrap();
        let backend = Arc::try_unwrap(resolver).unwrap().into_backend();
        assert_eq!(backend.resolved("test_domain_1.com", 80), Some(1));
        assert_eq!(backend.resolved("test_domain_2.com", 80), Some(1));
        assert_eq!(backend.resolved("test_domain_3.com", 80), Some(1));
        Ok(())
    }

    #[test]
    fn test_resolver_cache() -> Result<(), Box<dyn Error>> {
        let mut backend = ResolverFromTable::default();
        backend.add(
            "test_domain_1.com",
            80,
            vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1))],
        );
        let resolver = CachedResolver::new(backend, Duration::from_secs(1));

        for _ in 0..5 {
            let result = resolver.resolve("test_domain_1.com", 80).unwrap();
            assert_eq!(
                result,
                vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1))].into_boxed_slice()
            );
        }

        sleep(Duration::from_secs(1));

        for _ in 0..5 {
            let result = resolver.resolve("test_domain_1.com", 80).unwrap();
            assert_eq!(
                result,
                vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1))].into_boxed_slice()
            );
        }

        let backend = resolver.into_backend();
        assert_eq!(backend.resolved("test_domain_1.com", 80), Some(2));

        Ok(())
    }

    #[test]
    fn test_persistent_resolver() -> Result<(), Box<dyn Error>> {
        let mut backend = ResolverFromTable::default();
        backend.add(
            "test_domain_1.com",
            80,
            vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1))],
        );
        backend.add(
            "test_domain_2.com",
            80,
            vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2))],
        );
        backend.add(
            "test_domain_3.com",
            80,
            vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 3))],
        );

        let tempdir = tempdir()?;
        let tempfile_path = {
            let mut path = tempdir.path().to_owned();
            path.push("resolve_result");
            path
        };
        {
            let resolver =
                CachedResolver::load_or_create_from(&tempfile_path, false, backend.to_owned());
            {
                let err = File::open(resolver.persistent_path().unwrap()).unwrap_err();
                assert_eq!(err.kind(), IOErrorKind::NotFound);
            }
            {
                let result = resolver.resolve("test_domain_1.com", 80).unwrap();
                assert_eq!(
                    result,
                    vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1))].into_boxed_slice()
                );
            }
            {
                let result = resolver.resolve("test_domain_2.com", 80).unwrap();
                assert_eq!(
                    result,
                    vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2))].into_boxed_slice()
                );
            }
            resolver.persistent()?;
            File::open(resolver.persistent_path().unwrap())?;
            {
                let result = resolver.resolve("test_domain_3.com", 80).unwrap();
                assert_eq!(
                    result,
                    vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 3))].into_boxed_slice()
                );
            }
        }
        {
            let resolver =
                CachedResolver::load_or_create_from(&tempfile_path, true, backend.to_owned());
            {
                let result = resolver.resolve("test_domain_1.com", 80).unwrap();
                assert_eq!(
                    result,
                    vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1))].into_boxed_slice()
                );
            }
            {
                let result = resolver.resolve("test_domain_2.com", 80).unwrap();
                assert_eq!(
                    result,
                    vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2))].into_boxed_slice()
                );
            }
            {
                let result = resolver.resolve("test_domain_3.com", 80).unwrap();
                assert_eq!(
                    result,
                    vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 3))].into_boxed_slice()
                );
            }
            let backend = resolver.into_backend();
            assert_eq!(backend.resolved("test_domain_1.com", 80), None);
            assert_eq!(backend.resolved("test_domain_2.com", 80), None);
            assert_eq!(backend.resolved("test_domain_3.com", 80), Some(1));
        }
        {
            let resolver = CachedResolver::load_or_create_from(&tempfile_path, true, backend);
            {
                let result = resolver.resolve("test_domain_1.com", 80).unwrap();
                assert_eq!(
                    result,
                    vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1))].into_boxed_slice()
                );
            }
            {
                let result = resolver.resolve("test_domain_2.com", 80).unwrap();
                assert_eq!(
                    result,
                    vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2))].into_boxed_slice()
                );
            }
            {
                let result = resolver.resolve("test_domain_3.com", 80).unwrap();
                assert_eq!(
                    result,
                    vec![IpAddr::V4(Ipv4Addr::new(192, 168, 0, 3))].into_boxed_slice()
                );
            }
            let backend = resolver.into_backend();
            assert_eq!(backend.resolved("test_domain_1.com", 80), None);
            assert_eq!(backend.resolved("test_domain_2.com", 80), None);
            assert_eq!(backend.resolved("test_domain_3.com", 80), None);
        }

        Ok(())
    }
}
