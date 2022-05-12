// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!
//
#[derive(Debug, Clone, Default)]
#[doc = "调用 API 所用的路径参数"]
pub struct PathParams {
    r#bucket_name: Option<std::borrow::Cow<'static, str>>,
    r#object_name: Option<std::borrow::Cow<'static, str>>,
    r#upload_id: Option<std::borrow::Cow<'static, str>>,
    r#part_number: Option<std::borrow::Cow<'static, str>>,
    extended_segments: Vec<std::borrow::Cow<'static, str>>,
}
impl PathParams {
    #[inline]
    #[must_use]
    #[doc = "追加新的路径段"]
    pub fn push_segment(mut self, segment: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        self.extended_segments.push(segment.into());
        self
    }
    fn build(self) -> Vec<std::borrow::Cow<'static, str>> {
        let mut all_segments: Vec<_> = Default::default();
        if let Some(segment) = self.r#bucket_name {
            all_segments.push(segment);
        }
        all_segments.push(std::borrow::Cow::Borrowed("objects"));
        all_segments.push(self.r#object_name.unwrap_or(std::borrow::Cow::Borrowed("~")));
        if let Some(segment) = self.r#upload_id {
            all_segments.push(std::borrow::Cow::Borrowed("uploads"));
            all_segments.push(segment);
        }
        if let Some(segment) = self.r#part_number {
            all_segments.push(segment);
        }
        all_segments.extend(self.extended_segments);
        all_segments
    }
}
impl PathParams {
    #[inline]
    #[must_use]
    #[doc = "存储空间名称"]
    pub fn set_bucket_name_as_str(mut self, value: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        self.r#bucket_name = Some(value.into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "对象名称"]
    pub fn set_object_name_as_str(mut self, value: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        self.r#object_name = Some(qiniu_utils::base64::urlsafe(value.into().as_bytes()).into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "在服务端申请的 Multipart Upload 任务 id"]
    pub fn set_upload_id_as_str(mut self, value: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        self.r#upload_id = Some(value.into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "每一个上传的分片都有一个标识它的号码"]
    pub fn set_part_number_as_i8(mut self, value: i8) -> Self {
        self.r#part_number = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "每一个上传的分片都有一个标识它的号码"]
    pub fn set_part_number_as_i16(mut self, value: i16) -> Self {
        self.r#part_number = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "每一个上传的分片都有一个标识它的号码"]
    pub fn set_part_number_as_i32(mut self, value: i32) -> Self {
        self.r#part_number = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "每一个上传的分片都有一个标识它的号码"]
    pub fn set_part_number_as_i64(mut self, value: i64) -> Self {
        self.r#part_number = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "每一个上传的分片都有一个标识它的号码"]
    pub fn set_part_number_as_isize(mut self, value: isize) -> Self {
        self.r#part_number = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "每一个上传的分片都有一个标识它的号码"]
    pub fn set_part_number_as_u8(mut self, value: u8) -> Self {
        self.r#part_number = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "每一个上传的分片都有一个标识它的号码"]
    pub fn set_part_number_as_u16(mut self, value: u16) -> Self {
        self.r#part_number = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "每一个上传的分片都有一个标识它的号码"]
    pub fn set_part_number_as_u32(mut self, value: u32) -> Self {
        self.r#part_number = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "每一个上传的分片都有一个标识它的号码"]
    pub fn set_part_number_as_u64(mut self, value: u64) -> Self {
        self.r#part_number = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "每一个上传的分片都有一个标识它的号码"]
    pub fn set_part_number_as_usize(mut self, value: usize) -> Self {
        self.r#part_number = Some(value.to_string().into());
        self
    }
}
#[derive(Debug, Clone, Default)]
#[doc = "调用 API 所用的 HTTP 头参数"]
pub struct RequestHeaders {
    map: qiniu_http_client::http::header::HeaderMap,
}
impl RequestHeaders {
    #[inline]
    #[must_use]
    #[doc = "插入 HTTP 头参数"]
    pub fn insert(
        mut self,
        header_name: qiniu_http_client::http::header::HeaderName,
        header_value: qiniu_http_client::http::header::HeaderValue,
    ) -> Self {
        self.map.insert(header_name, header_value);
        self
    }
}
impl<'a> From<RequestHeaders> for std::borrow::Cow<'a, qiniu_http_client::http::header::HeaderMap> {
    #[inline]
    fn from(m: RequestHeaders) -> Self {
        std::borrow::Cow::Owned(m.map)
    }
}
impl<'a> From<&'a RequestHeaders> for std::borrow::Cow<'a, qiniu_http_client::http::header::HeaderMap> {
    #[inline]
    fn from(m: &'a RequestHeaders) -> Self {
        std::borrow::Cow::Borrowed(&m.map)
    }
}
impl RequestHeaders {
    #[inline]
    #[must_use]
    #[doc = "上传块内容的 md5 值，如果指定服务端会进行校验，不指定不校验"]
    pub fn set_md_5(self, value: impl Into<qiniu_http_client::http::header::HeaderValue>) -> Self {
        self.insert(
            qiniu_http_client::http::header::HeaderName::from_bytes("Content-MD5".as_bytes()).unwrap(),
            value.into(),
        )
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
impl Default for ResponseBody {
    #[inline]
    fn default() -> Self {
        Self(serde_json::Value::Object(Default::default()))
    }
}
impl From<ResponseBody> for serde_json::Value {
    #[inline]
    fn from(val: ResponseBody) -> Self {
        val.0
    }
}
impl AsRef<serde_json::Value> for ResponseBody {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl AsMut<serde_json::Value> for ResponseBody {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        &mut self.0
    }
}
impl ResponseBody {
    #[doc = "获取 上传块内容的 etag，用来标识块，completeMultipartUpload API 调用的时候作为参数进行文件合成"]
    pub fn get_etag_as_str(&self) -> &str {
        self.0.as_object().unwrap().get("etag").unwrap().as_str().unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 上传块内容的 etag，用来标识块，completeMultipartUpload API 调用的时候作为参数进行文件合成"]
    pub fn set_etag_as_str(&mut self, new: String) -> Option<String> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("etag".to_owned(), new.into())
            .and_then(|val| match val {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
    }
}
impl ResponseBody {
    #[doc = "获取 上传块内容的 MD5 值"]
    pub fn get_md_5_as_str(&self) -> &str {
        self.0.as_object().unwrap().get("md5").unwrap().as_str().unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 上传块内容的 MD5 值"]
    pub fn set_md_5_as_str(&mut self, new: String) -> Option<String> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("md5".to_owned(), new.into())
            .and_then(|val| match val {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
    }
}
#[doc = "API 调用客户端"]
#[derive(Debug, Clone)]
pub struct Client<'client>(&'client qiniu_http_client::HttpClient);
impl<'client> Client<'client> {
    pub(super) fn new(http_client: &'client qiniu_http_client::HttpClient) -> Self {
        Self(http_client)
    }
}
impl<'client> Client<'client> {
    #[inline]
    #[doc = "创建一个新的阻塞请求，该方法的异步版本为 [`Self::new_async_request`]"]
    pub fn new_request<E: qiniu_http_client::EndpointsProvider + 'client>(
        &self,
        endpoints_provider: E,
        path_params: PathParams,
        upload_token: impl qiniu_http_client::upload_token::UploadTokenProvider + Clone + 'client,
    ) -> SyncRequestBuilder<'client, E> {
        RequestBuilder({
            let mut builder = self.0.put(&[qiniu_http_client::ServiceName::Up], endpoints_provider);
            builder.authorization(qiniu_http_client::Authorization::uptoken(upload_token));
            builder.idempotent(qiniu_http_client::Idempotent::Always);
            builder.path(crate::base_utils::join_path("/buckets", "", path_params.build()));
            builder.accept_json();
            builder
        })
    }
    #[inline]
    #[cfg(feature = "async")]
    #[doc = "创建一个新的异步请求"]
    pub fn new_async_request<E: qiniu_http_client::EndpointsProvider + 'client>(
        &self,
        endpoints_provider: E,
        path_params: PathParams,
        upload_token: impl qiniu_http_client::upload_token::UploadTokenProvider + Clone + 'client,
    ) -> AsyncRequestBuilder<'client, E> {
        RequestBuilder({
            let mut builder = self
                .0
                .async_put(&[qiniu_http_client::ServiceName::Up], endpoints_provider);
            builder.authorization(qiniu_http_client::Authorization::uptoken(upload_token));
            builder.idempotent(qiniu_http_client::Idempotent::Always);
            builder.path(crate::base_utils::join_path("/buckets", "", path_params.build()));
            builder.accept_json();
            builder
        })
    }
}
#[derive(Debug)]
#[doc = "API 请求构造器"]
pub struct RequestBuilder<'req, B, E>(qiniu_http_client::RequestBuilder<'req, B, E>);
#[doc = "API 阻塞请求构造器"]
pub type SyncRequestBuilder<'req, E> = RequestBuilder<'req, qiniu_http_client::SyncRequestBody<'req>, E>;
#[cfg(feature = "async")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
#[doc = "API 异步请求构造器"]
pub type AsyncRequestBuilder<'req, E> = RequestBuilder<'req, qiniu_http_client::AsyncRequestBody<'req>, E>;
impl<'req, B, E> RequestBuilder<'req, B, E> {
    #[inline]
    #[doc = "设置是否使用 HTTPS"]
    pub fn use_https(&mut self, use_https: bool) -> &mut Self {
        self.0.use_https(use_https);
        self
    }
    #[inline]
    #[doc = "设置 HTTP 协议版本"]
    pub fn version(&mut self, version: qiniu_http_client::http::Version) -> &mut Self {
        self.0.version(version);
        self
    }
    #[inline]
    #[doc = "设置 HTTP 请求头"]
    pub fn headers(
        &mut self,
        headers: impl Into<std::borrow::Cow<'req, qiniu_http_client::http::HeaderMap>>,
    ) -> &mut Self {
        self.0.headers(headers);
        self
    }
    #[inline]
    #[doc = "添加 HTTP 请求头"]
    pub fn set_header(
        &mut self,
        header_name: impl qiniu_http_client::http::header::IntoHeaderName,
        header_value: impl Into<qiniu_http_client::http::HeaderValue>,
    ) -> &mut Self {
        self.0.set_header(header_name, header_value);
        self
    }
    #[inline]
    #[doc = "设置查询参数"]
    pub fn query(&mut self, query: impl Into<std::borrow::Cow<'req, str>>) -> &mut Self {
        self.0.query(query);
        self
    }
    #[inline]
    #[doc = "设置查询参数"]
    pub fn query_pairs(&mut self, query_pairs: impl Into<Vec<qiniu_http_client::QueryPair<'req>>>) -> &mut Self {
        self.0.query_pairs(query_pairs);
        self
    }
    #[inline]
    #[doc = "追加查询参数"]
    pub fn append_query_pair(
        &mut self,
        query_pair_key: impl Into<qiniu_http_client::QueryPairKey<'req>>,
        query_pair_value: impl Into<qiniu_http_client::QueryPairValue<'req>>,
    ) -> &mut Self {
        self.0.append_query_pair(query_pair_key, query_pair_value);
        self
    }
    #[inline]
    #[doc = "设置扩展信息"]
    pub fn extensions(&mut self, extensions: qiniu_http_client::http::Extensions) -> &mut Self {
        self.0.extensions(extensions);
        self
    }
    #[doc = "添加扩展信息"]
    #[inline]
    pub fn add_extension<T: Send + Sync + 'static>(&mut self, val: T) -> &mut Self {
        self.0.add_extension(val);
        self
    }
    #[inline]
    #[doc = "上传进度回调函数"]
    pub fn on_uploading_progress(
        &mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                qiniu_http_client::http::TransferProgressInfo,
            ) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_uploading_progress(callback);
        self
    }
    #[inline]
    #[doc = "设置响应状态码回调函数"]
    pub fn on_receive_response_status(
        &mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                qiniu_http_client::http::StatusCode,
            ) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_receive_response_status(callback);
        self
    }
    #[inline]
    #[doc = "设置响应 HTTP 头回调函数"]
    pub fn on_receive_response_header(
        &mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                &qiniu_http_client::http::HeaderName,
                &qiniu_http_client::http::HeaderValue,
            ) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_receive_response_header(callback);
        self
    }
    #[inline]
    #[doc = "设置域名解析前回调函数"]
    pub fn on_to_resolve_domain(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::CallbackContext, &str) -> anyhow::Result<()> + Send + Sync + 'req,
    ) -> &mut Self {
        self.0.on_to_resolve_domain(callback);
        self
    }
    #[inline]
    #[doc = "设置域名解析成功回调函数"]
    pub fn on_domain_resolved(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &str,
                &qiniu_http_client::ResolveAnswers,
            ) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_domain_resolved(callback);
        self
    }
    #[inline]
    #[doc = "设置 IP 地址选择前回调函数"]
    pub fn on_to_choose_ips(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::CallbackContext, &[qiniu_http_client::IpAddrWithPort]) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_to_choose_ips(callback);
        self
    }
    #[inline]
    #[doc = "设置 IP 地址选择成功回调函数"]
    pub fn on_ips_chosen(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &[qiniu_http_client::IpAddrWithPort],
                &[qiniu_http_client::IpAddrWithPort],
            ) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_ips_chosen(callback);
        self
    }
    #[inline]
    #[doc = "设置 HTTP 请求签名前回调函数"]
    pub fn on_before_request_signed(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext) -> anyhow::Result<()> + Send + Sync + 'req,
    ) -> &mut Self {
        self.0.on_before_request_signed(callback);
        self
    }
    #[inline]
    #[doc = "设置 HTTP 请求前回调函数"]
    pub fn on_after_request_signed(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext) -> anyhow::Result<()> + Send + Sync + 'req,
    ) -> &mut Self {
        self.0.on_after_request_signed(callback);
        self
    }
    #[inline]
    #[doc = "设置响应成功回调函数"]
    pub fn on_response(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                &qiniu_http_client::http::ResponseParts,
            ) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_response(callback);
        self
    }
    #[inline]
    #[doc = "设置响应错误回调函数"]
    pub fn on_error(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                &qiniu_http_client::ResponseError,
            ) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_error(callback);
        self
    }
    #[inline]
    #[doc = "设置退避前回调函数"]
    pub fn on_before_backoff(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext, std::time::Duration) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_before_backoff(callback);
        self
    }
    #[inline]
    #[doc = "设置退避后回调函数"]
    pub fn on_after_backoff(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext, std::time::Duration) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_after_backoff(callback);
        self
    }
    #[inline]
    #[doc = "获取 HTTP 请求构建器部分参数"]
    pub fn parts(&self) -> &qiniu_http_client::RequestBuilderParts<'req> {
        self.0.parts()
    }
    #[inline]
    #[doc = "获取 HTTP 请求构建器部分参数的可变引用"]
    pub fn parts_mut(&mut self) -> &mut qiniu_http_client::RequestBuilderParts<'req> {
        self.0.parts_mut()
    }
}
impl<'req, E: qiniu_http_client::EndpointsProvider + Clone + 'req> SyncRequestBuilder<'req, E> {
    #[doc = "阻塞发起 HTTP 请求"]
    pub fn call(
        &mut self,
        body: impl std::io::Read + qiniu_http_client::http::Reset + std::fmt::Debug + Send + Sync + 'static,
        content_length: u64,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = self.0.stream_as_body(body, content_length, None);
        let response = request.call()?;
        let parsed = response.parse_json()?;
        Ok(parsed)
    }
}
#[cfg(feature = "async")]
impl<'req, E: qiniu_http_client::EndpointsProvider + Clone + 'req> AsyncRequestBuilder<'req, E> {
    #[doc = "异步发起 HTTP 请求"]
    pub async fn call(
        &mut self,
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
