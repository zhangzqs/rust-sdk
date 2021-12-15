// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!
//
#[derive(Debug, Clone, Default)]
#[doc = "调用 API 所用的路径参数"]
pub struct PathParams {
    r#entry: Option<std::borrow::Cow<'static, str>>,
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
}
#[derive(Debug, Clone, Default)]
#[doc = "调用 API 所用的 URL 查询参数"]
pub struct QueryParams<'a> {
    map: indexmap::IndexMap<
        qiniu_http_client::QueryPairKey<'a>,
        qiniu_http_client::QueryPairValue<'a>,
    >,
}
impl<'a> QueryParams<'a> {
    #[inline]
    pub fn insert(
        mut self,
        query_pair_key: qiniu_http_client::QueryPairKey<'a>,
        query_pair_value: qiniu_http_client::QueryPairValue<'a>,
    ) -> Self {
        self.map.insert(query_pair_key, query_pair_value);
        self
    }
    fn build(self) -> qiniu_http_client::QueryPairs<'a> {
        qiniu_http_client::QueryPairs::from_iter(self.map)
    }
}
impl<'a> From<QueryParams<'a>> for qiniu_http_client::QueryPairs<'a> {
    #[inline]
    fn from(map: QueryParams<'a>) -> Self {
        map.build()
    }
}
impl<'a> QueryParams<'a> {
    #[inline]
    #[doc = "如果文件是通过分片上传的，是否返回对应的分片信息"]
    pub fn set_need_parts_as_bool(self, value: bool) -> Self {
        self.insert("needparts".into(), value.to_string().into())
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
    #[doc = "获取 对象大小，单位为字节"]
    pub fn get_size_as_i64(&self) -> i64 {
        self.0
            .as_object()
            .unwrap()
            .get("fsize")
            .unwrap()
            .as_i64()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 对象大小，单位为字节"]
    pub fn set_size_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("fsize".to_owned(), new.into())
            .and_then(|val| val.as_i64())
    }
}
impl ResponseBody {
    #[doc = "获取 对象大小，单位为字节"]
    pub fn get_size_as_u64(&self) -> u64 {
        self.0
            .as_object()
            .unwrap()
            .get("fsize")
            .unwrap()
            .as_u64()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 对象大小，单位为字节"]
    pub fn set_size_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("fsize".to_owned(), new.into())
            .and_then(|val| val.as_u64())
    }
}
impl ResponseBody {
    #[doc = "获取 对象哈希值"]
    pub fn get_hash_as_str(&self) -> &str {
        self.0
            .as_object()
            .unwrap()
            .get("hash")
            .unwrap()
            .as_str()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 对象哈希值"]
    pub fn set_hash_as_str(&mut self, new: String) -> Option<String> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("hash".to_owned(), new.into())
            .and_then(|val| match val {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
    }
}
impl ResponseBody {
    #[doc = "获取 对象 MIME 类型"]
    pub fn get_mime_type_as_str(&self) -> &str {
        self.0
            .as_object()
            .unwrap()
            .get("mimeType")
            .unwrap()
            .as_str()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 对象 MIME 类型"]
    pub fn set_mime_type_as_str(&mut self, new: String) -> Option<String> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("mimeType".to_owned(), new.into())
            .and_then(|val| match val {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
    }
}
impl ResponseBody {
    #[doc = "获取 对象存储类型，`0` 表示普通存储，`1` 表示低频存储，`2` 表示归档存储"]
    pub fn get_type_as_i64(&self) -> i64 {
        self.0
            .as_object()
            .unwrap()
            .get("type")
            .unwrap()
            .as_i64()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 对象存储类型，`0` 表示普通存储，`1` 表示低频存储，`2` 表示归档存储"]
    pub fn set_type_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("type".to_owned(), new.into())
            .and_then(|val| val.as_i64())
    }
}
impl ResponseBody {
    #[doc = "获取 对象存储类型，`0` 表示普通存储，`1` 表示低频存储，`2` 表示归档存储"]
    pub fn get_type_as_u64(&self) -> u64 {
        self.0
            .as_object()
            .unwrap()
            .get("type")
            .unwrap()
            .as_u64()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 对象存储类型，`0` 表示普通存储，`1` 表示低频存储，`2` 表示归档存储"]
    pub fn set_type_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("type".to_owned(), new.into())
            .and_then(|val| val.as_u64())
    }
}
impl ResponseBody {
    #[doc = "获取 文件上传时间，UNIX 时间戳格式，单位为 100 纳秒"]
    pub fn get_put_time_as_i64(&self) -> i64 {
        self.0
            .as_object()
            .unwrap()
            .get("putTime")
            .unwrap()
            .as_i64()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 文件上传时间，UNIX 时间戳格式，单位为 100 纳秒"]
    pub fn set_put_time_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("putTime".to_owned(), new.into())
            .and_then(|val| val.as_i64())
    }
}
impl ResponseBody {
    #[doc = "获取 文件上传时间，UNIX 时间戳格式，单位为 100 纳秒"]
    pub fn get_put_time_as_u64(&self) -> u64 {
        self.0
            .as_object()
            .unwrap()
            .get("putTime")
            .unwrap()
            .as_u64()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 文件上传时间，UNIX 时间戳格式，单位为 100 纳秒"]
    pub fn set_put_time_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("putTime".to_owned(), new.into())
            .and_then(|val| val.as_u64())
    }
}
impl ResponseBody {
    #[doc = "获取 归档存储文件的解冻状态，`2` 表示解冻完成，`1` 表示解冻中；归档文件冻结时，不返回该字段"]
    pub fn get_unfreezing_status_as_i64(&self) -> Option<i64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("restoreStatus"))
            .and_then(|val| val.as_i64())
    }
}
impl ResponseBody {
    #[doc = "设置 归档存储文件的解冻状态，`2` 表示解冻完成，`1` 表示解冻中；归档文件冻结时，不返回该字段"]
    pub fn set_unfreezing_status_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("restoreStatus".to_owned(), new.into())
                .and_then(|val| val.as_i64())
        })
    }
}
impl ResponseBody {
    #[doc = "获取 归档存储文件的解冻状态，`2` 表示解冻完成，`1` 表示解冻中；归档文件冻结时，不返回该字段"]
    pub fn get_unfreezing_status_as_u64(&self) -> Option<u64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("restoreStatus"))
            .and_then(|val| val.as_u64())
    }
}
impl ResponseBody {
    #[doc = "设置 归档存储文件的解冻状态，`2` 表示解冻完成，`1` 表示解冻中；归档文件冻结时，不返回该字段"]
    pub fn set_unfreezing_status_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("restoreStatus".to_owned(), new.into())
                .and_then(|val| val.as_u64())
        })
    }
}
impl ResponseBody {
    #[doc = "获取 文件状态。`1` 表示禁用；只有禁用状态的文件才会返回该字段"]
    pub fn get_status_as_i64(&self) -> Option<i64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("status"))
            .and_then(|val| val.as_i64())
    }
}
impl ResponseBody {
    #[doc = "设置 文件状态。`1` 表示禁用；只有禁用状态的文件才会返回该字段"]
    pub fn set_status_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("status".to_owned(), new.into())
                .and_then(|val| val.as_i64())
        })
    }
}
impl ResponseBody {
    #[doc = "获取 文件状态。`1` 表示禁用；只有禁用状态的文件才会返回该字段"]
    pub fn get_status_as_u64(&self) -> Option<u64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("status"))
            .and_then(|val| val.as_u64())
    }
}
impl ResponseBody {
    #[doc = "设置 文件状态。`1` 表示禁用；只有禁用状态的文件才会返回该字段"]
    pub fn set_status_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("status".to_owned(), new.into())
                .and_then(|val| val.as_u64())
        })
    }
}
impl ResponseBody {
    #[doc = "获取 对象 MD5 值，只有通过直传文件和追加文件 API 上传的文件，服务端确保有该字段返回"]
    pub fn get_md_5_as_str(&self) -> Option<&str> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("md5"))
            .and_then(|val| val.as_str())
    }
}
impl ResponseBody {
    #[doc = "设置 对象 MD5 值，只有通过直传文件和追加文件 API 上传的文件，服务端确保有该字段返回"]
    pub fn set_md_5_as_str(&mut self, new: String) -> Option<String> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("md5".to_owned(), new.into())
                .and_then(|val| match val {
                    serde_json::Value::String(s) => Some(s),
                    _ => None,
                })
        })
    }
}
impl ResponseBody {
    #[doc = "获取 文件过期删除日期，UNIX 时间戳格式，文件在设置过期时间后才会返回该字段"]
    pub fn get_expiration_time_as_i64(&self) -> Option<i64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("expiration"))
            .and_then(|val| val.as_i64())
    }
}
impl ResponseBody {
    #[doc = "设置 文件过期删除日期，UNIX 时间戳格式，文件在设置过期时间后才会返回该字段"]
    pub fn set_expiration_time_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("expiration".to_owned(), new.into())
                .and_then(|val| val.as_i64())
        })
    }
}
impl ResponseBody {
    #[doc = "获取 文件过期删除日期，UNIX 时间戳格式，文件在设置过期时间后才会返回该字段"]
    pub fn get_expiration_time_as_u64(&self) -> Option<u64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("expiration"))
            .and_then(|val| val.as_u64())
    }
}
impl ResponseBody {
    #[doc = "设置 文件过期删除日期，UNIX 时间戳格式，文件在设置过期时间后才会返回该字段"]
    pub fn set_expiration_time_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("expiration".to_owned(), new.into())
                .and_then(|val| val.as_u64())
        })
    }
}
impl ResponseBody {
    #[doc = "获取 文件生命周期中转为低频存储的日期，UNIX 时间戳格式，文件在设置转低频后才会返回该字段"]
    pub fn get_transition_to_ia_time_as_i64(&self) -> Option<i64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("transitionToIA"))
            .and_then(|val| val.as_i64())
    }
}
impl ResponseBody {
    #[doc = "设置 文件生命周期中转为低频存储的日期，UNIX 时间戳格式，文件在设置转低频后才会返回该字段"]
    pub fn set_transition_to_ia_time_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("transitionToIA".to_owned(), new.into())
                .and_then(|val| val.as_i64())
        })
    }
}
impl ResponseBody {
    #[doc = "获取 文件生命周期中转为低频存储的日期，UNIX 时间戳格式，文件在设置转低频后才会返回该字段"]
    pub fn get_transition_to_ia_time_as_u64(&self) -> Option<u64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("transitionToIA"))
            .and_then(|val| val.as_u64())
    }
}
impl ResponseBody {
    #[doc = "设置 文件生命周期中转为低频存储的日期，UNIX 时间戳格式，文件在设置转低频后才会返回该字段"]
    pub fn set_transition_to_ia_time_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("transitionToIA".to_owned(), new.into())
                .and_then(|val| val.as_u64())
        })
    }
}
impl ResponseBody {
    #[doc = "获取 文件生命周期中转为归档存储的日期，UNIX 时间戳格式，文件在设置转归档后才会返回该字段"]
    pub fn get_transition_to_archive_time_as_i64(&self) -> Option<i64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("transitionToARCHIVE"))
            .and_then(|val| val.as_i64())
    }
}
impl ResponseBody {
    #[doc = "设置 文件生命周期中转为归档存储的日期，UNIX 时间戳格式，文件在设置转归档后才会返回该字段"]
    pub fn set_transition_to_archive_time_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("transitionToARCHIVE".to_owned(), new.into())
                .and_then(|val| val.as_i64())
        })
    }
}
impl ResponseBody {
    #[doc = "获取 文件生命周期中转为归档存储的日期，UNIX 时间戳格式，文件在设置转归档后才会返回该字段"]
    pub fn get_transition_to_archive_time_as_u64(&self) -> Option<u64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("transitionToARCHIVE"))
            .and_then(|val| val.as_u64())
    }
}
impl ResponseBody {
    #[doc = "设置 文件生命周期中转为归档存储的日期，UNIX 时间戳格式，文件在设置转归档后才会返回该字段"]
    pub fn set_transition_to_archive_time_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("transitionToARCHIVE".to_owned(), new.into())
                .and_then(|val| val.as_u64())
        })
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "每个分片的大小"]
pub struct PartSizes(serde_json::Value);
impl PartSizes {
    #[allow(dead_code)]
    pub(crate) fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}
impl From<PartSizes> for serde_json::Value {
    #[inline]
    fn from(val: PartSizes) -> Self {
        val.0
    }
}
impl std::convert::AsRef<serde_json::Value> for PartSizes {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl std::convert::AsMut<serde_json::Value> for PartSizes {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        &mut self.0
    }
}
impl PartSizes {
    pub fn len(&self) -> usize {
        self.0.as_array().unwrap().len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.as_array().unwrap().is_empty()
    }
}
impl PartSizes {
    #[doc = "解析 JSON 得到整型列表"]
    pub fn to_i64_vec(&self) -> Vec<i64> {
        self.0
            .as_array()
            .unwrap()
            .iter()
            .map(|ele| ele.as_i64().unwrap())
            .collect()
    }
}
impl PartSizes {
    #[doc = "解析 JSON 得到无符号整型列表"]
    pub fn to_u64_vec(&self) -> Vec<u64> {
        self.0
            .as_array()
            .unwrap()
            .iter()
            .map(|ele| ele.as_u64().unwrap())
            .collect()
    }
}
impl PartSizes {
    #[doc = "在列表的指定位置移出 JSON i64 整型"]
    pub fn remove_as_i64(&mut self, index: usize) -> Option<i64> {
        match self.0.as_array_mut().unwrap().remove(index) {
            serde_json::Value::Number(s) => s.as_i64(),
            _ => None,
        }
    }
}
impl PartSizes {
    #[doc = "在列表尾部取出 JSON i64 整型"]
    pub fn pop_as_i64(&mut self) -> Option<i64> {
        self.0
            .as_array_mut()
            .unwrap()
            .pop()
            .and_then(|val| match val {
                serde_json::Value::Number(s) => s.as_i64(),
                _ => None,
            })
    }
}
impl PartSizes {
    #[doc = "在列表的指定位置移出 JSON u64 整型"]
    pub fn remove_as_u64(&mut self, index: usize) -> Option<u64> {
        match self.0.as_array_mut().unwrap().remove(index) {
            serde_json::Value::Number(s) => s.as_u64(),
            _ => None,
        }
    }
}
impl PartSizes {
    #[doc = "在列表尾部取出 JSON u64 整型"]
    pub fn pop_as_u64(&mut self) -> Option<u64> {
        self.0
            .as_array_mut()
            .unwrap()
            .pop()
            .and_then(|val| match val {
                serde_json::Value::Number(s) => s.as_u64(),
                _ => None,
            })
    }
}
impl From<Vec<i8>> for PartSizes {
    #[inline]
    fn from(val: Vec<i8>) -> Self {
        Self(serde_json::Value::from(val))
    }
}
impl PartSizes {
    #[doc = "在列表的指定位置插入 JSON i8 整型"]
    pub fn insert_i8(&mut self, index: usize, val: i8) {
        self.0.as_array_mut().unwrap().insert(index, val.into());
    }
}
impl PartSizes {
    #[doc = "在列表尾部追加 JSON i8 整型"]
    pub fn push_i8(&mut self, val: i8) {
        self.0.as_array_mut().unwrap().push(val.into());
    }
}
impl From<Vec<i16>> for PartSizes {
    #[inline]
    fn from(val: Vec<i16>) -> Self {
        Self(serde_json::Value::from(val))
    }
}
impl PartSizes {
    #[doc = "在列表的指定位置插入 JSON i16 整型"]
    pub fn insert_i16(&mut self, index: usize, val: i16) {
        self.0.as_array_mut().unwrap().insert(index, val.into());
    }
}
impl PartSizes {
    #[doc = "在列表尾部追加 JSON i16 整型"]
    pub fn push_i16(&mut self, val: i16) {
        self.0.as_array_mut().unwrap().push(val.into());
    }
}
impl From<Vec<i32>> for PartSizes {
    #[inline]
    fn from(val: Vec<i32>) -> Self {
        Self(serde_json::Value::from(val))
    }
}
impl PartSizes {
    #[doc = "在列表的指定位置插入 JSON i32 整型"]
    pub fn insert_i32(&mut self, index: usize, val: i32) {
        self.0.as_array_mut().unwrap().insert(index, val.into());
    }
}
impl PartSizes {
    #[doc = "在列表尾部追加 JSON i32 整型"]
    pub fn push_i32(&mut self, val: i32) {
        self.0.as_array_mut().unwrap().push(val.into());
    }
}
impl From<Vec<i64>> for PartSizes {
    #[inline]
    fn from(val: Vec<i64>) -> Self {
        Self(serde_json::Value::from(val))
    }
}
impl PartSizes {
    #[doc = "在列表的指定位置插入 JSON i64 整型"]
    pub fn insert_i64(&mut self, index: usize, val: i64) {
        self.0.as_array_mut().unwrap().insert(index, val.into());
    }
}
impl PartSizes {
    #[doc = "在列表尾部追加 JSON i64 整型"]
    pub fn push_i64(&mut self, val: i64) {
        self.0.as_array_mut().unwrap().push(val.into());
    }
}
impl From<Vec<isize>> for PartSizes {
    #[inline]
    fn from(val: Vec<isize>) -> Self {
        Self(serde_json::Value::from(val))
    }
}
impl PartSizes {
    #[doc = "在列表的指定位置插入 JSON isize 整型"]
    pub fn insert_isize(&mut self, index: usize, val: isize) {
        self.0.as_array_mut().unwrap().insert(index, val.into());
    }
}
impl PartSizes {
    #[doc = "在列表尾部追加 JSON isize 整型"]
    pub fn push_isize(&mut self, val: isize) {
        self.0.as_array_mut().unwrap().push(val.into());
    }
}
impl From<Vec<u8>> for PartSizes {
    #[inline]
    fn from(val: Vec<u8>) -> Self {
        Self(serde_json::Value::from(val))
    }
}
impl PartSizes {
    #[doc = "在列表的指定位置插入 JSON u8 整型"]
    pub fn insert_u8(&mut self, index: usize, val: u8) {
        self.0.as_array_mut().unwrap().insert(index, val.into());
    }
}
impl PartSizes {
    #[doc = "在列表尾部追加 JSON u8 整型"]
    pub fn push_u8(&mut self, val: u8) {
        self.0.as_array_mut().unwrap().push(val.into());
    }
}
impl From<Vec<u16>> for PartSizes {
    #[inline]
    fn from(val: Vec<u16>) -> Self {
        Self(serde_json::Value::from(val))
    }
}
impl PartSizes {
    #[doc = "在列表的指定位置插入 JSON u16 整型"]
    pub fn insert_u16(&mut self, index: usize, val: u16) {
        self.0.as_array_mut().unwrap().insert(index, val.into());
    }
}
impl PartSizes {
    #[doc = "在列表尾部追加 JSON u16 整型"]
    pub fn push_u16(&mut self, val: u16) {
        self.0.as_array_mut().unwrap().push(val.into());
    }
}
impl From<Vec<u32>> for PartSizes {
    #[inline]
    fn from(val: Vec<u32>) -> Self {
        Self(serde_json::Value::from(val))
    }
}
impl PartSizes {
    #[doc = "在列表的指定位置插入 JSON u32 整型"]
    pub fn insert_u32(&mut self, index: usize, val: u32) {
        self.0.as_array_mut().unwrap().insert(index, val.into());
    }
}
impl PartSizes {
    #[doc = "在列表尾部追加 JSON u32 整型"]
    pub fn push_u32(&mut self, val: u32) {
        self.0.as_array_mut().unwrap().push(val.into());
    }
}
impl From<Vec<u64>> for PartSizes {
    #[inline]
    fn from(val: Vec<u64>) -> Self {
        Self(serde_json::Value::from(val))
    }
}
impl PartSizes {
    #[doc = "在列表的指定位置插入 JSON u64 整型"]
    pub fn insert_u64(&mut self, index: usize, val: u64) {
        self.0.as_array_mut().unwrap().insert(index, val.into());
    }
}
impl PartSizes {
    #[doc = "在列表尾部追加 JSON u64 整型"]
    pub fn push_u64(&mut self, val: u64) {
        self.0.as_array_mut().unwrap().push(val.into());
    }
}
impl From<Vec<usize>> for PartSizes {
    #[inline]
    fn from(val: Vec<usize>) -> Self {
        Self(serde_json::Value::from(val))
    }
}
impl PartSizes {
    #[doc = "在列表的指定位置插入 JSON usize 整型"]
    pub fn insert_usize(&mut self, index: usize, val: usize) {
        self.0.as_array_mut().unwrap().insert(index, val.into());
    }
}
impl PartSizes {
    #[doc = "在列表尾部追加 JSON usize 整型"]
    pub fn push_usize(&mut self, val: usize) {
        self.0.as_array_mut().unwrap().push(val.into());
    }
}
impl ResponseBody {
    #[doc = "获取 每个分片的大小，如没有指定 need_parts 参数则不返回"]
    pub fn get_parts(&self) -> Option<PartSizes> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("parts"))
            .cloned()
            .map(PartSizes::new)
    }
}
impl ResponseBody {
    #[doc = "设置 每个分片的大小，如没有指定 need_parts 参数则不返回"]
    pub fn set_parts(&mut self, new: PartSizes) -> Option<PartSizes> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("parts".to_owned(), new.into())
                .map(PartSizes::new)
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
        credential: impl qiniu_http_client::credential::CredentialProvider + 'static,
    ) -> SyncRequestBuilder<'client, E> {
        RequestBuilder({
            let mut builder = self
                .0
                .get(&[qiniu_http_client::ServiceName::Rs], endpoints_provider);
            builder.authorization(qiniu_http_client::Authorization::v2(credential));
            builder.idempotent(qiniu_http_client::Idempotent::Default);
            builder.path(crate::base_utils::join_path(
                "/stat",
                "",
                path_params.build(),
            ));
            builder.accept_json();
            builder
        })
    }
    #[inline]
    #[cfg(feature = "async")]
    pub fn new_async_request<E: qiniu_http_client::EndpointsProvider + 'client>(
        &self,
        endpoints_provider: E,
        path_params: PathParams,
        credential: impl qiniu_http_client::credential::CredentialProvider + 'static,
    ) -> AsyncRequestBuilder<'client, E> {
        RequestBuilder({
            let mut builder = self
                .0
                .async_get(&[qiniu_http_client::ServiceName::Rs], endpoints_provider);
            builder.authorization(qiniu_http_client::Authorization::v2(credential));
            builder.idempotent(qiniu_http_client::Idempotent::Default);
            builder.path(crate::base_utils::join_path(
                "/stat",
                "",
                path_params.build(),
            ));
            builder.accept_json();
            builder
        })
    }
}
#[derive(Debug)]
pub struct RequestBuilder<'req, B: 'req, E: 'req>(qiniu_http_client::RequestBuilder<'req, B, E>);
pub type SyncRequestBuilder<'req, E> =
    RequestBuilder<'req, qiniu_http_client::SyncRequestBody<'req>, E>;
#[cfg(feature = "async")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
pub type AsyncRequestBuilder<'req, E> =
    RequestBuilder<'req, qiniu_http_client::AsyncRequestBody<'req>, E>;
impl<'req, B: 'req, E: 'req> RequestBuilder<'req, B, E> {
    #[inline]
    pub fn use_https(&mut self, use_https: bool) -> &mut Self {
        self.0.use_https(use_https);
        self
    }
    #[inline]
    pub fn version(&mut self, version: qiniu_http_client::http::Version) -> &mut Self {
        self.0.version(version);
        self
    }
    #[inline]
    pub fn headers(
        &mut self,
        headers: impl Into<std::borrow::Cow<'req, qiniu_http_client::http::HeaderMap>>,
    ) -> &mut Self {
        self.0.headers(headers);
        self
    }
    #[inline]
    pub fn query_pairs(
        &mut self,
        query_pairs: impl Into<qiniu_http_client::QueryPairs<'req>>,
    ) -> &mut Self {
        self.0.query_pairs(query_pairs);
        self
    }
    #[inline]
    pub fn extensions(&mut self, extensions: qiniu_http_client::http::Extensions) -> &mut Self {
        self.0.extensions(extensions);
        self
    }
    #[inline]
    pub fn add_extension<T: Send + Sync + 'static>(&mut self, val: T) -> &mut Self {
        self.0.add_extension(val);
        self
    }
    #[inline]
    pub fn on_uploading_progress(
        &mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                &qiniu_http_client::http::TransferProgressInfo,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_uploading_progress(callback);
        self
    }
    #[inline]
    pub fn on_receive_response_status(
        &mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                qiniu_http_client::http::StatusCode,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_receive_response_status(callback);
        self
    }
    #[inline]
    pub fn on_receive_response_header(
        &mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                &qiniu_http_client::http::HeaderName,
                &qiniu_http_client::http::HeaderValue,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_receive_response_header(callback);
        self
    }
    #[inline]
    pub fn on_to_resolve_domain(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &str,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_to_resolve_domain(callback);
        self
    }
    #[inline]
    pub fn on_domain_resolved(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &str,
                &qiniu_http_client::ResolveAnswers,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_domain_resolved(callback);
        self
    }
    #[inline]
    pub fn on_to_choose_ips(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &[qiniu_http_client::IpAddrWithPort],
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_to_choose_ips(callback);
        self
    }
    #[inline]
    pub fn on_ips_chosen(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &[qiniu_http_client::IpAddrWithPort],
                &[qiniu_http_client::IpAddrWithPort],
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_ips_chosen(callback);
        self
    }
    #[inline]
    pub fn on_before_request_signed(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_before_request_signed(callback);
        self
    }
    #[inline]
    pub fn on_after_request_signed(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_after_request_signed(callback);
        self
    }
    #[inline]
    pub fn on_error(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                &qiniu_http_client::ResponseError,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_error(callback);
        self
    }
    #[inline]
    pub fn on_before_backoff(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                std::time::Duration,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_before_backoff(callback);
        self
    }
    #[inline]
    pub fn on_after_backoff(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                std::time::Duration,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_after_backoff(callback);
        self
    }
    #[inline]
    pub fn parts(&self) -> &qiniu_http_client::RequestBuilderParts<'req> {
        self.0.parts()
    }
    #[inline]
    pub fn parts_mut(&mut self) -> &mut qiniu_http_client::RequestBuilderParts<'req> {
        self.0.parts_mut()
    }
}
impl<'req, E: qiniu_http_client::EndpointsProvider + Clone + 'req> SyncRequestBuilder<'req, E> {
    pub fn call(
        &mut self,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = &mut self.0;
        let response = request.call()?;
        let parsed = response.parse_json()?;
        Ok(parsed)
    }
}
#[cfg(feature = "async")]
impl<'req, E: qiniu_http_client::EndpointsProvider + Clone + 'req> AsyncRequestBuilder<'req, E> {
    pub async fn call(
        &mut self,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = &mut self.0;
        let response = request.call().await?;
        let parsed = response.parse_json().await?;
        Ok(parsed)
    }
}
