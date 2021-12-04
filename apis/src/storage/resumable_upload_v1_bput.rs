// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!
//
#[derive(Debug, Clone, Default)]
#[doc = "调用 API 所用的路径参数"]
pub struct PathParams {
    r#ctx: Option<std::borrow::Cow<'static, str>>,
    r#chunk_offset: Option<std::borrow::Cow<'static, str>>,
    extended_segments: Vec<std::borrow::Cow<'static, str>>,
}
impl PathParams {
    #[inline]
    pub fn push_segment(mut self, segment: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        self.extended_segments.push(segment.into());
        self
    }
    fn build(self) -> Vec<std::borrow::Cow<'static, str>> {
        let mut all_segments: Vec<_> = Default::default();
        if let Some(segment) = self.r#ctx {
            all_segments.push(segment);
        }
        if let Some(segment) = self.r#chunk_offset {
            all_segments.push(segment);
        }
        all_segments.extend(self.extended_segments);
        all_segments
    }
}
impl PathParams {
    #[inline]
    #[doc = "前一次上传返回的块级上传控制信息"]
    pub fn set_ctx_as_str(mut self, value: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        self.r#ctx = Some(value.into());
        self
    }
    #[inline]
    #[doc = "当前片在整个块中的起始偏移"]
    pub fn set_chunk_offset_as_i8(mut self, value: i8) -> Self {
        self.r#chunk_offset = Some(value.to_string().into());
        self
    }
    #[inline]
    #[doc = "当前片在整个块中的起始偏移"]
    pub fn set_chunk_offset_as_i16(mut self, value: i16) -> Self {
        self.r#chunk_offset = Some(value.to_string().into());
        self
    }
    #[inline]
    #[doc = "当前片在整个块中的起始偏移"]
    pub fn set_chunk_offset_as_i32(mut self, value: i32) -> Self {
        self.r#chunk_offset = Some(value.to_string().into());
        self
    }
    #[inline]
    #[doc = "当前片在整个块中的起始偏移"]
    pub fn set_chunk_offset_as_i64(mut self, value: i64) -> Self {
        self.r#chunk_offset = Some(value.to_string().into());
        self
    }
    #[inline]
    #[doc = "当前片在整个块中的起始偏移"]
    pub fn set_chunk_offset_as_isize(mut self, value: isize) -> Self {
        self.r#chunk_offset = Some(value.to_string().into());
        self
    }
    #[inline]
    #[doc = "当前片在整个块中的起始偏移"]
    pub fn set_chunk_offset_as_u8(mut self, value: u8) -> Self {
        self.r#chunk_offset = Some(value.to_string().into());
        self
    }
    #[inline]
    #[doc = "当前片在整个块中的起始偏移"]
    pub fn set_chunk_offset_as_u16(mut self, value: u16) -> Self {
        self.r#chunk_offset = Some(value.to_string().into());
        self
    }
    #[inline]
    #[doc = "当前片在整个块中的起始偏移"]
    pub fn set_chunk_offset_as_u32(mut self, value: u32) -> Self {
        self.r#chunk_offset = Some(value.to_string().into());
        self
    }
    #[inline]
    #[doc = "当前片在整个块中的起始偏移"]
    pub fn set_chunk_offset_as_u64(mut self, value: u64) -> Self {
        self.r#chunk_offset = Some(value.to_string().into());
        self
    }
    #[inline]
    #[doc = "当前片在整个块中的起始偏移"]
    pub fn set_chunk_offset_as_usize(mut self, value: usize) -> Self {
        self.r#chunk_offset = Some(value.to_string().into());
        self
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "获取 API 所用的响应体参数"]
pub struct ResponseBody(serde_json::Value);
impl ResponseBody {
    #[allow(dead_code)]
    pub(crate) fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}
impl From<ResponseBody> for serde_json::Value {
    #[inline]
    fn from(val: ResponseBody) -> Self {
        val.0
    }
}
impl std::convert::AsRef<serde_json::Value> for ResponseBody {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl std::convert::AsMut<serde_json::Value> for ResponseBody {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        &mut self.0
    }
}
impl ResponseBody {
    #[doc = "获取 本次上传成功后的块级上传控制信息，用于后续上传片（bput）及创建文件（mkfile）"]
    pub fn get_ctx_as_str(&self) -> &str {
        self.0
            .as_object()
            .unwrap()
            .get("ctx")
            .unwrap()
            .as_str()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 本次上传成功后的块级上传控制信息，用于后续上传片（bput）及创建文件（mkfile）"]
    pub fn set_ctx_as_str(&mut self, new: String) -> Option<String> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("ctx".to_owned(), new.into())
            .and_then(|val| match val {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
    }
}
impl ResponseBody {
    #[doc = "获取 上传块 SHA1 值，使用 URL 安全的 Base64 编码"]
    pub fn get_checksum_as_str(&self) -> &str {
        self.0
            .as_object()
            .unwrap()
            .get("checksum")
            .unwrap()
            .as_str()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 上传块 SHA1 值，使用 URL 安全的 Base64 编码"]
    pub fn set_checksum_as_str(&mut self, new: String) -> Option<String> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("checksum".to_owned(), new.into())
            .and_then(|val| match val {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
    }
}
impl ResponseBody {
    #[doc = "获取 上传块 CRC32 值，客户可通过此字段对上传块的完整性进行校验"]
    pub fn get_crc_32_as_i64(&self) -> i64 {
        self.0
            .as_object()
            .unwrap()
            .get("crc32")
            .unwrap()
            .as_i64()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 上传块 CRC32 值，客户可通过此字段对上传块的完整性进行校验"]
    pub fn set_crc_32_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("crc32".to_owned(), new.into())
            .and_then(|val| val.as_i64())
    }
}
impl ResponseBody {
    #[doc = "获取 上传块 CRC32 值，客户可通过此字段对上传块的完整性进行校验"]
    pub fn get_crc_32_as_u64(&self) -> u64 {
        self.0
            .as_object()
            .unwrap()
            .get("crc32")
            .unwrap()
            .as_u64()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 上传块 CRC32 值，客户可通过此字段对上传块的完整性进行校验"]
    pub fn set_crc_32_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("crc32".to_owned(), new.into())
            .and_then(|val| val.as_u64())
    }
}
impl ResponseBody {
    #[doc = "获取 下一个上传块在切割块中的偏移"]
    pub fn get_offset_as_i64(&self) -> i64 {
        self.0
            .as_object()
            .unwrap()
            .get("offset")
            .unwrap()
            .as_i64()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 下一个上传块在切割块中的偏移"]
    pub fn set_offset_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("offset".to_owned(), new.into())
            .and_then(|val| val.as_i64())
    }
}
impl ResponseBody {
    #[doc = "获取 下一个上传块在切割块中的偏移"]
    pub fn get_offset_as_u64(&self) -> u64 {
        self.0
            .as_object()
            .unwrap()
            .get("offset")
            .unwrap()
            .as_u64()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 下一个上传块在切割块中的偏移"]
    pub fn set_offset_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("offset".to_owned(), new.into())
            .and_then(|val| val.as_u64())
    }
}
impl ResponseBody {
    #[doc = "获取 后续上传接收地址"]
    pub fn get_host_as_str(&self) -> &str {
        self.0
            .as_object()
            .unwrap()
            .get("host")
            .unwrap()
            .as_str()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 后续上传接收地址"]
    pub fn set_host_as_str(&mut self, new: String) -> Option<String> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("host".to_owned(), new.into())
            .and_then(|val| match val {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
    }
}
impl ResponseBody {
    #[doc = "获取 `ctx` 过期时间"]
    pub fn get_expired_at_as_str(&self) -> &str {
        self.0
            .as_object()
            .unwrap()
            .get("expired_at")
            .unwrap()
            .as_str()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 `ctx` 过期时间"]
    pub fn set_expired_at_as_str(&mut self, new: String) -> Option<String> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("expired_at".to_owned(), new.into())
            .and_then(|val| match val {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
    }
}
#[derive(Debug, Clone)]
pub struct Client<'client>(&'client qiniu_http_client::HttpClient);
impl<'client> Client<'client> {
    pub(super) fn new(http_client: &'client qiniu_http_client::HttpClient) -> Self {
        Self(http_client)
    }
}
impl<'client> Client<'client> {
    #[inline]
    pub fn new_request<E: qiniu_http_client::EndpointsProvider + 'client>(
        &self,
        endpoints_provider: E,
        path_params: PathParams,
        upload_token: impl qiniu_http_client::upload_token::UploadTokenProvider + 'static,
    ) -> SyncRequestBuilder<'client, E> {
        SyncRequestBuilder(
            self.0
                .post(&[qiniu_http_client::ServiceName::Up], endpoints_provider)
                .authorization(qiniu_http_client::Authorization::uptoken(upload_token))
                .idempotent(qiniu_http_client::Idempotent::Always)
                .path(crate::base_utils::join_path(
                    "/bput",
                    "",
                    path_params.build(),
                ))
                .accept_json(),
        )
    }
    #[inline]
    #[cfg(feature = "async")]
    pub fn new_async_request<E: qiniu_http_client::EndpointsProvider + 'client>(
        &self,
        endpoints_provider: E,
        path_params: PathParams,
        upload_token: impl qiniu_http_client::upload_token::UploadTokenProvider + 'static,
    ) -> AsyncRequestBuilder<'client, E> {
        AsyncRequestBuilder(
            self.0
                .async_post(&[qiniu_http_client::ServiceName::Up], endpoints_provider)
                .authorization(qiniu_http_client::Authorization::uptoken(upload_token))
                .idempotent(qiniu_http_client::Idempotent::Always)
                .path(crate::base_utils::join_path(
                    "/bput",
                    "",
                    path_params.build(),
                ))
                .accept_json(),
        )
    }
}
#[derive(Debug)]
pub struct SyncRequestBuilder<'req, E: 'req>(qiniu_http_client::SyncRequestBuilder<'req, E>);
#[derive(Debug)]
#[cfg(feature = "async")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
pub struct AsyncRequestBuilder<'req, E: 'req>(qiniu_http_client::AsyncRequestBuilder<'req, E>);
impl<'req, E: 'req> SyncRequestBuilder<'req, E> {
    #[inline]
    pub fn use_https(mut self, use_https: bool) -> Self {
        self.0 = self.0.use_https(use_https);
        self
    }
    #[inline]
    pub fn version(mut self, version: qiniu_http_client::http::Version) -> Self {
        self.0 = self.0.version(version);
        self
    }
    #[inline]
    pub fn headers(
        mut self,
        headers: impl Into<std::borrow::Cow<'req, qiniu_http_client::http::HeaderMap>>,
    ) -> Self {
        self.0 = self.0.headers(headers);
        self
    }
    #[inline]
    pub fn query_pairs(
        mut self,
        query_pairs: impl Into<qiniu_http_client::QueryPairs<'req>>,
    ) -> Self {
        self.0 = self.0.query_pairs(query_pairs);
        self
    }
    #[inline]
    pub fn extensions(mut self, extensions: qiniu_http_client::http::Extensions) -> Self {
        self.0 = self.0.extensions(extensions);
        self
    }
    #[inline]
    pub fn add_extension<T: Send + Sync + 'static>(mut self, val: T) -> Self {
        self.0 = self.0.add_extension(val);
        self
    }
    #[inline]
    pub fn on_uploading_progress(
        mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                &qiniu_http_client::http::TransferProgressInfo,
            ) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_uploading_progress(callback);
        self
    }
    #[inline]
    pub fn on_receive_response_status(
        mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                qiniu_http_client::http::StatusCode,
            ) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_receive_response_status(callback);
        self
    }
    #[inline]
    pub fn on_receive_response_header(
        mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                &qiniu_http_client::http::HeaderName,
                &qiniu_http_client::http::HeaderValue,
            ) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_receive_response_header(callback);
        self
    }
    #[inline]
    pub fn on_to_resolve_domain(
        mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::CallbackContext, &str) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_to_resolve_domain(callback);
        self
    }
    #[inline]
    pub fn on_domain_resolved(
        mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &str,
                &qiniu_http_client::ResolveAnswers,
            ) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_domain_resolved(callback);
        self
    }
    #[inline]
    pub fn on_to_choose_ips(
        mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &[qiniu_http_client::IpAddrWithPort],
            ) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_to_choose_ips(callback);
        self
    }
    #[inline]
    pub fn on_ips_chosen(
        mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &[qiniu_http_client::IpAddrWithPort],
                &[qiniu_http_client::IpAddrWithPort],
            ) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_ips_chosen(callback);
        self
    }
    #[inline]
    pub fn on_before_request_signed(
        mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_before_request_signed(callback);
        self
    }
    #[inline]
    pub fn on_after_request_signed(
        mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_after_request_signed(callback);
        self
    }
    #[inline]
    pub fn on_error(
        mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                &qiniu_http_client::ResponseError,
            ) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_error(callback);
        self
    }
    #[inline]
    pub fn on_before_backoff(
        mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext, std::time::Duration) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_before_backoff(callback);
        self
    }
    #[inline]
    pub fn on_after_backoff(
        mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext, std::time::Duration) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_after_backoff(callback);
        self
    }
}
impl<'req, E: qiniu_http_client::EndpointsProvider + 'req> SyncRequestBuilder<'req, E> {
    pub fn call(
        self,
        body: impl std::io::Read
            + qiniu_http_client::http::Reset
            + std::fmt::Debug
            + Send
            + Sync
            + 'static,
        content_length: u64,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = self.0.stream_as_body(body, content_length, None);
        let response = request.call()?;
        let parsed = response.parse_json()?;
        Ok(parsed)
    }
}
#[cfg(feature = "async")]
impl<'req, E: 'req> AsyncRequestBuilder<'req, E> {
    #[inline]
    pub fn use_https(mut self, use_https: bool) -> Self {
        self.0 = self.0.use_https(use_https);
        self
    }
    #[inline]
    pub fn version(mut self, version: qiniu_http_client::http::Version) -> Self {
        self.0 = self.0.version(version);
        self
    }
    #[inline]
    pub fn headers(
        mut self,
        headers: impl Into<std::borrow::Cow<'req, qiniu_http_client::http::HeaderMap>>,
    ) -> Self {
        self.0 = self.0.headers(headers);
        self
    }
    #[inline]
    pub fn query_pairs(
        mut self,
        query_pairs: impl Into<qiniu_http_client::QueryPairs<'req>>,
    ) -> Self {
        self.0 = self.0.query_pairs(query_pairs);
        self
    }
    #[inline]
    pub fn extensions(mut self, extensions: qiniu_http_client::http::Extensions) -> Self {
        self.0 = self.0.extensions(extensions);
        self
    }
    #[inline]
    pub fn add_extension<T: Send + Sync + 'static>(mut self, val: T) -> Self {
        self.0 = self.0.add_extension(val);
        self
    }
    #[inline]
    pub fn on_uploading_progress(
        mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                &qiniu_http_client::http::TransferProgressInfo,
            ) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_uploading_progress(callback);
        self
    }
    #[inline]
    pub fn on_receive_response_status(
        mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                qiniu_http_client::http::StatusCode,
            ) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_receive_response_status(callback);
        self
    }
    #[inline]
    pub fn on_receive_response_header(
        mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                &qiniu_http_client::http::HeaderName,
                &qiniu_http_client::http::HeaderValue,
            ) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_receive_response_header(callback);
        self
    }
    #[inline]
    pub fn on_to_resolve_domain(
        mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::CallbackContext, &str) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_to_resolve_domain(callback);
        self
    }
    #[inline]
    pub fn on_domain_resolved(
        mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &str,
                &qiniu_http_client::ResolveAnswers,
            ) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_domain_resolved(callback);
        self
    }
    #[inline]
    pub fn on_to_choose_ips(
        mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &[qiniu_http_client::IpAddrWithPort],
            ) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_to_choose_ips(callback);
        self
    }
    #[inline]
    pub fn on_ips_chosen(
        mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &[qiniu_http_client::IpAddrWithPort],
                &[qiniu_http_client::IpAddrWithPort],
            ) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_ips_chosen(callback);
        self
    }
    #[inline]
    pub fn on_before_request_signed(
        mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_before_request_signed(callback);
        self
    }
    #[inline]
    pub fn on_after_request_signed(
        mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_after_request_signed(callback);
        self
    }
    #[inline]
    pub fn on_error(
        mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                &qiniu_http_client::ResponseError,
            ) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_error(callback);
        self
    }
    #[inline]
    pub fn on_before_backoff(
        mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext, std::time::Duration) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_before_backoff(callback);
        self
    }
    #[inline]
    pub fn on_after_backoff(
        mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext, std::time::Duration) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_after_backoff(callback);
        self
    }
}
#[cfg(feature = "async")]
impl<'req, E: qiniu_http_client::EndpointsProvider + 'req> AsyncRequestBuilder<'req, E> {
    pub async fn call(
        self,
        body: impl futures::io::AsyncRead
            + qiniu_http_client::http::AsyncReset
            + Unpin
            + std::fmt::Debug
            + Send
            + Sync
            + 'static,
        content_length: u64,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = self.0.stream_as_body(body, content_length, None);
        let response = request.call().await?;
        let parsed = response.parse_json().await?;
        Ok(parsed)
    }
}
