#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    non_ascii_idents,
    indirect_structural_match,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications
)]

mod error;
mod request;
mod response;

pub use error::{
    Error as ResponseError, ErrorBuilder as ResponseErrorBuilder, ErrorKind as ResponseErrorKind,
    MapError,
};
pub use http::{
    header::{self, HeaderMap, HeaderName, HeaderValue, InvalidHeaderName, InvalidHeaderValue},
    method::Method,
    request::Request as HTTPRequest,
    status::{InvalidStatusCode, StatusCode},
    uri::{self, Uri},
    Extensions, Version,
};
pub use request::{
    Request, RequestBody as SyncRequestBody, RequestBuilder, TransferProgressInfo, UserAgent,
};
pub use response::{
    Metrics, Response, ResponseBody as SyncResponseBody, ResponseBuilder, Result as ResponseResult,
};
use std::{
    fmt::Debug,
    io::{Result as IOResult, Seek, SeekFrom},
};

/// 同步 HTTP 响应
pub type SyncRequest<'r> = Request<'r, SyncRequestBody<'r>>;
/// 同步 HTTP 响应构建器
pub type SyncRequestBuilder<'r> = RequestBuilder<'r, SyncRequestBody<'r>>;

/// 同步 HTTP 响应
pub type SyncResponse = Response<SyncResponseBody>;
/// 同步 HTTP 响应构建器
pub type SyncResponseBuilder = ResponseBuilder<SyncResponseBody>;
/// 同步 HTTP 响应结果
pub type SyncResponseResult = ResponseResult<SyncResponseBody>;

#[cfg(feature = "async")]
mod async_req_resp {
    pub use super::{request::AsyncRequestBody, response::AsyncResponseBody};
    use super::{
        request::{Request, RequestBuilder},
        response::{Response, ResponseBuilder, Result as ResponseResult},
    };

    /// 异步 HTTP 响应
    pub type AsyncRequest<'r> = Request<'r, AsyncRequestBody<'r>>;
    /// 异步 HTTP 响应构建器
    pub type AsyncRequestBuilder<'r> = RequestBuilder<'r, AsyncRequestBody<'r>>;

    /// 异步 HTTP 响应
    #[cfg_attr(feature = "docs", doc(cfg(r#async)))]
    pub type AsyncResponse = Response<AsyncResponseBody>;

    /// 异步 HTTP 响应构建器
    #[cfg_attr(feature = "docs", doc(cfg(r#async)))]
    pub type AsyncResponseBuilder = ResponseBuilder<AsyncResponseBody>;

    /// 异步 HTTP 响应结果
    #[cfg_attr(feature = "docs", doc(cfg(r#async)))]
    pub type AsyncResponseResult = ResponseResult<AsyncResponseBody>;
}

#[cfg(feature = "async")]
pub use {
    async_req_resp::{
        AsyncRequest, AsyncRequestBody, AsyncRequestBuilder, AsyncResponse, AsyncResponseBody,
        AsyncResponseBuilder, AsyncResponseResult,
    },
    futures_lite::{AsyncRead, AsyncSeek},
};

#[cfg(feature = "async")]
use std::{future::Future, pin::Pin};

#[cfg(feature = "async")]
type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a + Send>>;

/// HTTP 请求处理函数
///
/// 实现该接口，即可处理所有七牛 SDK 发送的 HTTP 请求
pub trait HTTPCaller: Debug + Send + Sync {
    /// 同步发送 HTTP 请求
    fn call<'a>(&self, request: &'a mut SyncRequest<'_>) -> SyncResponseResult;

    /// 异步发送 HTTP 请求
    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(r#async)))]
    fn async_call<'a>(
        &'a self,
        request: &'a mut AsyncRequest<'_>,
    ) -> BoxFuture<'a, AsyncResponseResult>;

    #[inline]
    fn is_resolved_ip_addrs_supported(&self) -> bool {
        false
    }

    #[inline]
    fn is_response_metrics_supported(&self) -> bool {
        false
    }
}

pub trait Reset {
    fn reset(&mut self) -> IOResult<()>;
}

impl<T: Seek> Reset for T {
    #[inline]
    fn reset(&mut self) -> IOResult<()> {
        self.seek(SeekFrom::Start(0))?;
        Ok(())
    }
}

#[cfg(feature = "async")]
#[cfg_attr(feature = "docs", doc(cfg(r#async)))]
pub trait AsyncReset {
    fn reset(&mut self) -> BoxFuture<IOResult<()>>;
}

#[cfg(feature = "async")]
impl<T: AsyncSeek + Unpin + Send + Sync> AsyncReset for T {
    #[inline]
    fn reset(&mut self) -> BoxFuture<IOResult<()>> {
        use futures_lite::io::AsyncSeekExt;

        Box::pin(async move {
            self.seek(SeekFrom::Start(0)).await?;
            Ok(())
        })
    }
}

pub mod preclude {
    pub use super::{HTTPCaller, Metrics, Reset};

    #[cfg(feature = "async")]
    pub use super::AsyncReset;
}
