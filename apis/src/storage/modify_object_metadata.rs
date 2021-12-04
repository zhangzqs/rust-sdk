// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!
//
#[derive(Debug, Clone, Default)]
#[doc = "调用 API 所用的路径参数"]
pub struct PathParams {
    r#entry: Option<std::borrow::Cow<'static, str>>,
    r#mime_type: Option<std::borrow::Cow<'static, str>>,
    r#condition: Option<std::borrow::Cow<'static, str>>,
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
        if let Some(segment) = self.r#entry {
            all_segments.push(segment);
        }
        if let Some(segment) = self.r#mime_type {
            all_segments.push(std::borrow::Cow::Borrowed("mime"));
            all_segments.push(segment);
        }
        if let Some(segment) = self.r#condition {
            all_segments.push(std::borrow::Cow::Borrowed("cond"));
            all_segments.push(segment);
        }
        all_segments.extend(self.extended_segments);
        all_segments
    }
}
impl PathParams {
    #[inline]
    #[doc = "指定目标对象空间与目标对象名称"]
    pub fn set_entry_as_str(mut self, value: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        self.r#entry = Some(qiniu_utils::base64::urlsafe(value.into().as_bytes()).into());
        self
    }
    #[inline]
    #[doc = "新的 MIME 类型"]
    pub fn set_mime_type_as_str(
        mut self,
        value: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        self.r#mime_type = Some(qiniu_utils::base64::urlsafe(value.into().as_bytes()).into());
        self
    }
    #[inline]
    #[doc = "条件匹配，当前支持设置 hash、mime、fsize、putTime 条件，只有条件匹配才会执行修改操作，格式为 condKey1=condVal1&condKey2=condVal2"]
    pub fn set_condition_as_str(
        mut self,
        value: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        self.r#condition = Some(qiniu_utils::base64::urlsafe(value.into().as_bytes()).into());
        self
    }
    #[inline]
    #[doc = "对象存储元信息，键可以自定义，它可以由字母、数字、下划线、减号组成，必须以 x-qn-meta- 为前缀，且长度小于等于 50，单个文件键和值总和大小不能超过 1024 字节，可以同时修改多个键"]
    pub fn append_meta_data_as_str(
        mut self,
        key: impl Into<std::borrow::Cow<'static, str>>,
        value: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        self.extended_segments.push(key.into());
        self.extended_segments
            .push(qiniu_utils::base64::urlsafe(value.into().as_bytes()).into());
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
        credential: impl qiniu_http_client::credential::CredentialProvider + 'static,
    ) -> SyncRequestBuilder<'client, E> {
        SyncRequestBuilder(
            self.0
                .post(&[qiniu_http_client::ServiceName::Rs], endpoints_provider)
                .authorization(qiniu_http_client::Authorization::v2(credential))
                .idempotent(qiniu_http_client::Idempotent::Always)
                .path(crate::base_utils::join_path(
                    "/chgm",
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
        credential: impl qiniu_http_client::credential::CredentialProvider + 'static,
    ) -> AsyncRequestBuilder<'client, E> {
        AsyncRequestBuilder(
            self.0
                .async_post(&[qiniu_http_client::ServiceName::Rs], endpoints_provider)
                .authorization(qiniu_http_client::Authorization::v2(credential))
                .idempotent(qiniu_http_client::Idempotent::Always)
                .path(crate::base_utils::join_path(
                    "/chgm",
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
    pub fn call(self) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = self.0;
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
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = self.0;
        let response = request.call().await?;
        let parsed = response.parse_json().await?;
        Ok(parsed)
    }
}
