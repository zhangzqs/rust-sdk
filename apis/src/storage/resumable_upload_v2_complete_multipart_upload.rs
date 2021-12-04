// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!
//
#[derive(Debug, Clone, Default)]
#[doc = "调用 API 所用的路径参数"]
pub struct PathParams {
    r#bucket_name: Option<std::borrow::Cow<'static, str>>,
    r#object_name: Option<std::borrow::Cow<'static, str>>,
    r#upload_id: Option<std::borrow::Cow<'static, str>>,
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
        if let Some(segment) = self.r#bucket_name {
            all_segments.push(segment);
        }
        all_segments.push(std::borrow::Cow::Borrowed("objects"));
        all_segments.push(
            self.r#object_name
                .unwrap_or(std::borrow::Cow::Borrowed("~")),
        );
        if let Some(segment) = self.r#upload_id {
            all_segments.push(std::borrow::Cow::Borrowed("uploads"));
            all_segments.push(segment);
        }
        all_segments.extend(self.extended_segments);
        all_segments
    }
}
impl PathParams {
    #[inline]
    #[doc = "存储空间名称"]
    pub fn set_bucket_name_as_str(
        mut self,
        value: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        self.r#bucket_name = Some(value.into());
        self
    }
    #[inline]
    #[doc = "对象名称"]
    pub fn set_object_name_as_str(
        mut self,
        value: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        self.r#object_name = Some(qiniu_utils::base64::urlsafe(value.into().as_bytes()).into());
        self
    }
    #[inline]
    #[doc = "在服务端申请的 Multipart Upload 任务 id"]
    pub fn set_upload_id_as_str(
        mut self,
        value: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        self.r#upload_id = Some(value.into());
        self
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "调用 API 所用的请求体参数"]
pub struct RequestBody(serde_json::Value);
impl RequestBody {
    #[allow(dead_code)]
    pub(crate) fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}
impl From<RequestBody> for serde_json::Value {
    #[inline]
    fn from(val: RequestBody) -> Self {
        val.0
    }
}
impl std::convert::AsRef<serde_json::Value> for RequestBody {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl std::convert::AsMut<serde_json::Value> for RequestBody {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        &mut self.0
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "单个分片信息"]
pub struct PartInfo(serde_json::Value);
impl PartInfo {
    #[allow(dead_code)]
    pub(crate) fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}
impl From<PartInfo> for serde_json::Value {
    #[inline]
    fn from(val: PartInfo) -> Self {
        val.0
    }
}
impl std::convert::AsRef<serde_json::Value> for PartInfo {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl std::convert::AsMut<serde_json::Value> for PartInfo {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        &mut self.0
    }
}
impl PartInfo {
    #[doc = "获取 每一个上传的分片都有一个标识它的号码"]
    pub fn get_part_number_as_i64(&self) -> i64 {
        self.0
            .as_object()
            .unwrap()
            .get("partNumber")
            .unwrap()
            .as_i64()
            .unwrap()
    }
}
impl PartInfo {
    #[doc = "设置 每一个上传的分片都有一个标识它的号码"]
    pub fn set_part_number_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("partNumber".to_owned(), new.into())
            .and_then(|val| val.as_i64())
    }
}
impl PartInfo {
    #[doc = "获取 每一个上传的分片都有一个标识它的号码"]
    pub fn get_part_number_as_u64(&self) -> u64 {
        self.0
            .as_object()
            .unwrap()
            .get("partNumber")
            .unwrap()
            .as_u64()
            .unwrap()
    }
}
impl PartInfo {
    #[doc = "设置 每一个上传的分片都有一个标识它的号码"]
    pub fn set_part_number_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("partNumber".to_owned(), new.into())
            .and_then(|val| val.as_u64())
    }
}
impl PartInfo {
    #[doc = "获取 上传块的 etag"]
    pub fn get_etag_as_str(&self) -> &str {
        self.0
            .as_object()
            .unwrap()
            .get("etag")
            .unwrap()
            .as_str()
            .unwrap()
    }
}
impl PartInfo {
    #[doc = "设置 上传块的 etag"]
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
impl RequestBody {
    #[doc = "获取 已经上传的分片列表"]
    pub fn get_parts(&self) -> PartInfo {
        PartInfo::new(self.0.as_object().unwrap().get("parts").cloned().unwrap())
    }
}
impl RequestBody {
    #[doc = "设置 已经上传的分片列表"]
    pub fn set_parts(&mut self, new: PartInfo) -> Option<PartInfo> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("parts".to_owned(), new.into())
            .map(PartInfo::new)
    }
}
impl RequestBody {
    #[doc = "获取 上传的原始文件名，若未指定，则魔法变量中无法使用 fname，ext，suffix"]
    pub fn get_file_name_as_str(&self) -> Option<&str> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("fname"))
            .and_then(|val| val.as_str())
    }
}
impl RequestBody {
    #[doc = "设置 上传的原始文件名，若未指定，则魔法变量中无法使用 fname，ext，suffix"]
    pub fn set_file_name_as_str(&mut self, new: String) -> Option<String> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("fname".to_owned(), new.into())
                .and_then(|val| match val {
                    serde_json::Value::String(s) => Some(s),
                    _ => None,
                })
        })
    }
}
impl RequestBody {
    #[doc = "获取 若指定了则设置上传文件的 MIME 类型，若未指定，则根据文件内容自动检测 MIME 类型"]
    pub fn get_mime_type_as_str(&self) -> Option<&str> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("mime_type"))
            .and_then(|val| val.as_str())
    }
}
impl RequestBody {
    #[doc = "设置 若指定了则设置上传文件的 MIME 类型，若未指定，则根据文件内容自动检测 MIME 类型"]
    pub fn set_mime_type_as_str(&mut self, new: String) -> Option<String> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("mime_type".to_owned(), new.into())
                .and_then(|val| match val {
                    serde_json::Value::String(s) => Some(s),
                    _ => None,
                })
        })
    }
}
impl RequestBody {
    #[doc = "获取 用户自定义文件 metadata 信息的键值对，可以设置多个，MetaKey 和 MetaValue 都是 string，，其中 可以由字母、数字、下划线、减号组成，且长度小于等于 50，单个文件 MetaKey 和 MetaValue 总和大小不能超过 1024 字节，MetaKey 必须以 `x-qn-meta-` 作为前缀"]
    pub fn get_metadata(&self) -> Option<crate::base_types::StringMap> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("metadata"))
            .cloned()
            .map(crate::base_types::StringMap::new)
    }
    #[doc = "设置 用户自定义文件 metadata 信息的键值对，可以设置多个，MetaKey 和 MetaValue 都是 string，，其中 可以由字母、数字、下划线、减号组成，且长度小于等于 50，单个文件 MetaKey 和 MetaValue 总和大小不能超过 1024 字节，MetaKey 必须以 `x-qn-meta-` 作为前缀"]
    pub fn set_metadata(
        &mut self,
        new: crate::base_types::StringMap,
    ) -> Option<crate::base_types::StringMap> {
        self.0
            .as_object_mut()
            .and_then(|object| object.insert("metadata".to_owned(), new.into()))
            .map(crate::base_types::StringMap::new)
    }
}
impl RequestBody {
    #[doc = "获取 用户自定义变量"]
    pub fn get_custom_vars(&self) -> Option<crate::base_types::StringMap> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("customVars"))
            .cloned()
            .map(crate::base_types::StringMap::new)
    }
    #[doc = "设置 用户自定义变量"]
    pub fn set_custom_vars(
        &mut self,
        new: crate::base_types::StringMap,
    ) -> Option<crate::base_types::StringMap> {
        self.0
            .as_object_mut()
            .and_then(|object| object.insert("customVars".to_owned(), new.into()))
            .map(crate::base_types::StringMap::new)
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
        upload_token: impl qiniu_http_client::upload_token::UploadTokenProvider + 'static,
    ) -> SyncRequestBuilder<'client, E> {
        SyncRequestBuilder(
            self.0
                .post(&[qiniu_http_client::ServiceName::Up], endpoints_provider)
                .authorization(qiniu_http_client::Authorization::uptoken(upload_token))
                .idempotent(qiniu_http_client::Idempotent::Default)
                .path(crate::base_utils::join_path(
                    "/buckets",
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
                .idempotent(qiniu_http_client::Idempotent::Default)
                .path(crate::base_utils::join_path(
                    "/buckets",
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
        body: &RequestBody,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = self.0.json(body)?;
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
        body: &RequestBody,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = self.0.json(body)?;
        let response = request.call().await?;
        let parsed = response.parse_json().await?;
        Ok(parsed)
    }
}
