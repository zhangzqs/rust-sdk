// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!
//
#[derive(Debug, Clone, Default)]
#[doc = "调用 API 所用的 URL 查询参数"]
pub struct QueryParams<'a> {
    map: indexmap::IndexMap<qiniu_http_client::QueryPairKey<'a>, qiniu_http_client::QueryPairValue<'a>>,
}
impl<'a> QueryParams<'a> {
    #[inline]
    #[must_use]
    #[doc = "插入一个新的查询参数对"]
    pub fn insert(
        mut self,
        query_pair_key: qiniu_http_client::QueryPairKey<'a>,
        query_pair_value: qiniu_http_client::QueryPairValue<'a>,
    ) -> Self {
        self.map.insert(query_pair_key, query_pair_value);
        self
    }
    fn build(self) -> Vec<qiniu_http_client::QueryPair<'a>> {
        Vec::from_iter(self.map)
    }
}
impl<'a> From<QueryParams<'a>> for Vec<qiniu_http_client::QueryPair<'a>> {
    #[inline]
    fn from(map: QueryParams<'a>) -> Self {
        map.build()
    }
}
impl<'a> QueryParams<'a> {
    #[inline]
    #[must_use]
    #[doc = "指定存储空间"]
    pub fn set_bucket_as_str(self, value: impl Into<qiniu_http_client::QueryPairValue<'a>>) -> Self {
        self.insert("bucket".into(), value.into())
    }
    #[inline]
    #[must_use]
    #[doc = "上一次列举返回的位置标记，作为本次列举的起点信息"]
    pub fn set_marker_as_str(self, value: impl Into<qiniu_http_client::QueryPairValue<'a>>) -> Self {
        self.insert("marker".into(), value.into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_i8(self, value: i8) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_i16(self, value: i16) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_i32(self, value: i32) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_i64(self, value: i64) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_isize(self, value: isize) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_u8(self, value: u8) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_u16(self, value: u16) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_u32(self, value: u32) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_u64(self, value: u64) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_usize(self, value: usize) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "指定前缀，只有资源名匹配该前缀的资源会被列出"]
    pub fn set_prefix_as_str(self, value: impl Into<qiniu_http_client::QueryPairValue<'a>>) -> Self {
        self.insert("prefix".into(), value.into())
    }
    #[inline]
    #[must_use]
    #[doc = "指定目录分隔符，列出所有公共前缀（模拟列出目录效果）"]
    pub fn set_delimiter_as_str(self, value: impl Into<qiniu_http_client::QueryPairValue<'a>>) -> Self {
        self.insert("delimiter".into(), value.into())
    }
    #[inline]
    #[must_use]
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
    #[doc = "获取 有剩余条目则返回非空字符串，作为下一次列举的参数传入，如果没有剩余条目则返回空字符串"]
    pub fn get_marker_as_str(&self) -> Option<&str> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("marker"))
            .and_then(|val| val.as_str())
    }
}
impl ResponseBody {
    #[doc = "设置 有剩余条目则返回非空字符串，作为下一次列举的参数传入，如果没有剩余条目则返回空字符串"]
    pub fn set_marker_as_str(&mut self, new: String) -> Option<String> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("marker".to_owned(), new.into())
                .and_then(|val| match val {
                    serde_json::Value::String(s) => Some(s),
                    _ => None,
                })
        })
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "公共前缀的数组"]
pub struct CommonPrefixes(serde_json::Value);
impl CommonPrefixes {
    #[allow(dead_code)]
    pub(crate) fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}
impl Default for CommonPrefixes {
    #[inline]
    fn default() -> Self {
        Self(serde_json::Value::Array(Default::default()))
    }
}
impl From<CommonPrefixes> for serde_json::Value {
    #[inline]
    fn from(val: CommonPrefixes) -> Self {
        val.0
    }
}
impl AsRef<serde_json::Value> for CommonPrefixes {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl AsMut<serde_json::Value> for CommonPrefixes {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        &mut self.0
    }
}
impl CommonPrefixes {
    #[doc = "解析 JSON 得到 String 列表"]
    pub fn to_str_vec(&self) -> Vec<&str> {
        self.0
            .as_array()
            .unwrap()
            .iter()
            .map(|ele| ele.as_str().unwrap())
            .collect()
    }
}
impl From<Vec<String>> for CommonPrefixes {
    #[inline]
    fn from(val: Vec<String>) -> Self {
        Self(serde_json::Value::from(val))
    }
}
impl CommonPrefixes {
    #[doc = "获取数组的长度"]
    pub fn len(&self) -> usize {
        self.0.as_array().unwrap().len()
    }
    #[doc = "数组是否为空"]
    pub fn is_empty(&self) -> bool {
        self.0.as_array().unwrap().is_empty()
    }
}
impl CommonPrefixes {
    #[doc = "在列表的指定位置插入 JSON String"]
    pub fn insert_str(&mut self, index: usize, val: String) {
        self.0.as_array_mut().unwrap().insert(index, val.into());
    }
}
impl CommonPrefixes {
    #[doc = "在列表的指定位置移出 JSON String"]
    pub fn remove_as_str(&mut self, index: usize) -> Option<String> {
        match self.0.as_array_mut().unwrap().remove(index) {
            serde_json::Value::String(s) => Some(s),
            _ => None,
        }
    }
}
impl CommonPrefixes {
    #[doc = "在列表尾部追加 JSON String"]
    pub fn push_str(&mut self, val: String) {
        self.0.as_array_mut().unwrap().push(val.into());
    }
}
impl CommonPrefixes {
    #[doc = "在列表尾部取出 JSON String"]
    pub fn pop_as_str(&mut self) -> Option<String> {
        self.0.as_array_mut().unwrap().pop().and_then(|val| match val {
            serde_json::Value::String(s) => Some(s),
            _ => None,
        })
    }
}
impl ResponseBody {
    #[doc = "获取 公共前缀的数组，如没有指定 delimiter 参数则不返回"]
    pub fn get_common_prefixes(&self) -> Option<CommonPrefixes> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("common_prefixes"))
            .cloned()
            .map(CommonPrefixes::new)
    }
}
impl ResponseBody {
    #[doc = "设置 公共前缀的数组，如没有指定 delimiter 参数则不返回"]
    pub fn set_common_prefixes(&mut self, new: CommonPrefixes) -> Option<CommonPrefixes> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("common_prefixes".to_owned(), new.into())
                .map(CommonPrefixes::new)
        })
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "条目的数组，不能用来判断是否还有剩余条目"]
pub struct ListedObjects(serde_json::Value);
impl ListedObjects {
    #[allow(dead_code)]
    pub(crate) fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}
impl Default for ListedObjects {
    #[inline]
    fn default() -> Self {
        Self(serde_json::Value::Array(Default::default()))
    }
}
impl From<ListedObjects> for serde_json::Value {
    #[inline]
    fn from(val: ListedObjects) -> Self {
        val.0
    }
}
impl AsRef<serde_json::Value> for ListedObjects {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl AsMut<serde_json::Value> for ListedObjects {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        &mut self.0
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "对象条目，包含对象的元信息"]
pub struct ListedObjectEntry(serde_json::Value);
impl ListedObjectEntry {
    #[allow(dead_code)]
    pub(crate) fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}
impl Default for ListedObjectEntry {
    #[inline]
    fn default() -> Self {
        Self(serde_json::Value::Object(Default::default()))
    }
}
impl From<ListedObjectEntry> for serde_json::Value {
    #[inline]
    fn from(val: ListedObjectEntry) -> Self {
        val.0
    }
}
impl AsRef<serde_json::Value> for ListedObjectEntry {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl AsMut<serde_json::Value> for ListedObjectEntry {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        &mut self.0
    }
}
impl ListedObjectEntry {
    #[doc = "获取 对象名称"]
    pub fn get_key_as_str(&self) -> &str {
        self.0.as_object().unwrap().get("key").unwrap().as_str().unwrap()
    }
}
impl ListedObjectEntry {
    #[doc = "设置 对象名称"]
    pub fn set_key_as_str(&mut self, new: String) -> Option<String> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("key".to_owned(), new.into())
            .and_then(|val| match val {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
    }
}
impl ListedObjectEntry {
    #[doc = "获取 文件上传时间，UNIX 时间戳格式，单位为 100 纳秒"]
    pub fn get_put_time_as_i64(&self) -> i64 {
        self.0.as_object().unwrap().get("putTime").unwrap().as_i64().unwrap()
    }
}
impl ListedObjectEntry {
    #[doc = "设置 文件上传时间，UNIX 时间戳格式，单位为 100 纳秒"]
    pub fn set_put_time_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("putTime".to_owned(), new.into())
            .and_then(|val| val.as_i64())
    }
}
impl ListedObjectEntry {
    #[doc = "获取 文件上传时间，UNIX 时间戳格式，单位为 100 纳秒"]
    pub fn get_put_time_as_u64(&self) -> u64 {
        self.0.as_object().unwrap().get("putTime").unwrap().as_u64().unwrap()
    }
}
impl ListedObjectEntry {
    #[doc = "设置 文件上传时间，UNIX 时间戳格式，单位为 100 纳秒"]
    pub fn set_put_time_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("putTime".to_owned(), new.into())
            .and_then(|val| val.as_u64())
    }
}
impl ListedObjectEntry {
    #[doc = "获取 文件的哈希值"]
    pub fn get_hash_as_str(&self) -> &str {
        self.0.as_object().unwrap().get("hash").unwrap().as_str().unwrap()
    }
}
impl ListedObjectEntry {
    #[doc = "设置 文件的哈希值"]
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
impl ListedObjectEntry {
    #[doc = "获取 对象大小，单位为字节"]
    pub fn get_size_as_i64(&self) -> i64 {
        self.0.as_object().unwrap().get("fsize").unwrap().as_i64().unwrap()
    }
}
impl ListedObjectEntry {
    #[doc = "设置 对象大小，单位为字节"]
    pub fn set_size_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("fsize".to_owned(), new.into())
            .and_then(|val| val.as_i64())
    }
}
impl ListedObjectEntry {
    #[doc = "获取 对象大小，单位为字节"]
    pub fn get_size_as_u64(&self) -> u64 {
        self.0.as_object().unwrap().get("fsize").unwrap().as_u64().unwrap()
    }
}
impl ListedObjectEntry {
    #[doc = "设置 对象大小，单位为字节"]
    pub fn set_size_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("fsize".to_owned(), new.into())
            .and_then(|val| val.as_u64())
    }
}
impl ListedObjectEntry {
    #[doc = "获取 对象 MIME 类型"]
    pub fn get_mime_type_as_str(&self) -> &str {
        self.0.as_object().unwrap().get("mimeType").unwrap().as_str().unwrap()
    }
}
impl ListedObjectEntry {
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
impl ListedObjectEntry {
    #[doc = "获取 资源内容的唯一属主标识"]
    pub fn get_end_user_as_str(&self) -> Option<&str> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("endUser"))
            .and_then(|val| val.as_str())
    }
}
impl ListedObjectEntry {
    #[doc = "设置 资源内容的唯一属主标识"]
    pub fn set_end_user_as_str(&mut self, new: String) -> Option<String> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("endUser".to_owned(), new.into())
                .and_then(|val| match val {
                    serde_json::Value::String(s) => Some(s),
                    _ => None,
                })
        })
    }
}
impl ListedObjectEntry {
    #[doc = "获取 对象存储类型，`0` 表示普通存储，`1` 表示低频存储，`2` 表示归档存储"]
    pub fn get_type_as_i64(&self) -> Option<i64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("type"))
            .and_then(|val| val.as_i64())
    }
}
impl ListedObjectEntry {
    #[doc = "设置 对象存储类型，`0` 表示普通存储，`1` 表示低频存储，`2` 表示归档存储"]
    pub fn set_type_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("type".to_owned(), new.into())
                .and_then(|val| val.as_i64())
        })
    }
}
impl ListedObjectEntry {
    #[doc = "获取 对象存储类型，`0` 表示普通存储，`1` 表示低频存储，`2` 表示归档存储"]
    pub fn get_type_as_u64(&self) -> Option<u64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("type"))
            .and_then(|val| val.as_u64())
    }
}
impl ListedObjectEntry {
    #[doc = "设置 对象存储类型，`0` 表示普通存储，`1` 表示低频存储，`2` 表示归档存储"]
    pub fn set_type_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0.as_object_mut().and_then(|object| {
            object
                .insert("type".to_owned(), new.into())
                .and_then(|val| val.as_u64())
        })
    }
}
impl ListedObjectEntry {
    #[doc = "获取 文件的存储状态，即禁用状态和启用状态间的的互相转换，`0` 表示启用，`1`表示禁用"]
    pub fn get_unfreezing_status_as_i64(&self) -> i64 {
        self.0.as_object().unwrap().get("status").unwrap().as_i64().unwrap()
    }
}
impl ListedObjectEntry {
    #[doc = "设置 文件的存储状态，即禁用状态和启用状态间的的互相转换，`0` 表示启用，`1`表示禁用"]
    pub fn set_unfreezing_status_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("status".to_owned(), new.into())
            .and_then(|val| val.as_i64())
    }
}
impl ListedObjectEntry {
    #[doc = "获取 文件的存储状态，即禁用状态和启用状态间的的互相转换，`0` 表示启用，`1`表示禁用"]
    pub fn get_unfreezing_status_as_u64(&self) -> u64 {
        self.0.as_object().unwrap().get("status").unwrap().as_u64().unwrap()
    }
}
impl ListedObjectEntry {
    #[doc = "设置 文件的存储状态，即禁用状态和启用状态间的的互相转换，`0` 表示启用，`1`表示禁用"]
    pub fn set_unfreezing_status_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("status".to_owned(), new.into())
            .and_then(|val| val.as_u64())
    }
}
impl ListedObjectEntry {
    #[doc = "获取 对象 MD5 值，只有通过直传文件和追加文件 API 上传的文件，服务端确保有该字段返回"]
    pub fn get_md_5_as_str(&self) -> Option<&str> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("md5"))
            .and_then(|val| val.as_str())
    }
}
impl ListedObjectEntry {
    #[doc = "设置 对象 MD5 值，只有通过直传文件和追加文件 API 上传的文件，服务端确保有该字段返回"]
    pub fn set_md_5_as_str(&mut self, new: String) -> Option<String> {
        self.0.as_object_mut().and_then(|object| {
            object.insert("md5".to_owned(), new.into()).and_then(|val| match val {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
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
impl Default for PartSizes {
    #[inline]
    fn default() -> Self {
        Self(serde_json::Value::Array(Default::default()))
    }
}
impl From<PartSizes> for serde_json::Value {
    #[inline]
    fn from(val: PartSizes) -> Self {
        val.0
    }
}
impl AsRef<serde_json::Value> for PartSizes {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl AsMut<serde_json::Value> for PartSizes {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        &mut self.0
    }
}
impl PartSizes {
    #[doc = "获取数组的长度"]
    pub fn len(&self) -> usize {
        self.0.as_array().unwrap().len()
    }
    #[doc = "数组是否为空"]
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
        self.0.as_array_mut().unwrap().pop().and_then(|val| match val {
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
        self.0.as_array_mut().unwrap().pop().and_then(|val| match val {
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
impl ListedObjectEntry {
    #[doc = "获取 每个分片的大小，如没有指定 need_parts 参数则不返回"]
    pub fn get_parts(&self) -> Option<PartSizes> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("parts"))
            .cloned()
            .map(PartSizes::new)
    }
}
impl ListedObjectEntry {
    #[doc = "设置 每个分片的大小，如没有指定 need_parts 参数则不返回"]
    pub fn set_parts(&mut self, new: PartSizes) -> Option<PartSizes> {
        self.0
            .as_object_mut()
            .and_then(|object| object.insert("parts".to_owned(), new.into()).map(PartSizes::new))
    }
}
impl ListedObjects {
    #[doc = "解析 JSON 得到 ListedObjectEntry 列表"]
    pub fn to_listed_object_entry_vec(&self) -> Vec<ListedObjectEntry> {
        self.0
            .as_array()
            .unwrap()
            .iter()
            .cloned()
            .map(ListedObjectEntry::new)
            .collect()
    }
}
impl From<Vec<ListedObjectEntry>> for ListedObjects {
    #[inline]
    fn from(val: Vec<ListedObjectEntry>) -> Self {
        Self(serde_json::Value::from(val))
    }
}
impl ListedObjects {
    #[doc = "获取数组的长度"]
    pub fn len(&self) -> usize {
        self.0.as_array().unwrap().len()
    }
    #[doc = "数组是否为空"]
    pub fn is_empty(&self) -> bool {
        self.0.as_array().unwrap().is_empty()
    }
}
impl ListedObjects {
    #[doc = "在列表的指定位置插入 JSON ListedObjectEntry"]
    pub fn insert_listed_object_entry(&mut self, index: usize, val: ListedObjectEntry) {
        self.0.as_array_mut().unwrap().insert(index, val.into());
    }
}
impl ListedObjects {
    #[doc = "在列表的指定位置移出 JSON ListedObjectEntry"]
    pub fn remove_as_listed_object_entry(&mut self, index: usize) -> ListedObjectEntry {
        ListedObjectEntry::new(self.0.as_array_mut().unwrap().remove(index))
    }
}
impl ListedObjects {
    #[doc = "在列表尾部追加 JSON ListedObjectEntry"]
    pub fn push_listed_object_entry(&mut self, val: ListedObjectEntry) {
        self.0.as_array_mut().unwrap().push(val.into());
    }
}
impl ListedObjects {
    #[doc = "在列表尾部取出 JSON ListedObjectEntry"]
    pub fn pop_listed_object_entry(&mut self) -> Option<ListedObjectEntry> {
        self.0.as_array_mut().unwrap().pop().map(ListedObjectEntry::new)
    }
}
impl ResponseBody {
    #[doc = "获取 条目的数组，不能用来判断是否还有剩余条目"]
    pub fn get_items(&self) -> ListedObjects {
        ListedObjects::new(self.0.as_object().unwrap().get("items").cloned().unwrap())
    }
}
impl ResponseBody {
    #[doc = "设置 条目的数组，不能用来判断是否还有剩余条目"]
    pub fn set_items(&mut self, new: ListedObjects) -> Option<ListedObjects> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("items".to_owned(), new.into())
            .map(ListedObjects::new)
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
        credential: impl qiniu_http_client::credential::CredentialProvider + Clone + 'client,
    ) -> SyncRequestBuilder<'client, E> {
        RequestBuilder({
            let mut builder = self.0.get(&[qiniu_http_client::ServiceName::Rsf], endpoints_provider);
            builder.authorization(qiniu_http_client::Authorization::v2(credential));
            builder.idempotent(qiniu_http_client::Idempotent::Default);
            builder.path("list");
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
        credential: impl qiniu_http_client::credential::CredentialProvider + Clone + 'client,
    ) -> AsyncRequestBuilder<'client, E> {
        RequestBuilder({
            let mut builder = self
                .0
                .async_get(&[qiniu_http_client::ServiceName::Rsf], endpoints_provider);
            builder.authorization(qiniu_http_client::Authorization::v2(credential));
            builder.idempotent(qiniu_http_client::Idempotent::Default);
            builder.path("list");
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
        header_name: impl Into<qiniu_http_client::http::HeaderName>,
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
    #[doc = "设置响应状态码回调函数"]
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
    #[doc = "设置响应 HTTP 头回调函数"]
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
    #[doc = "设置域名解析前回调函数"]
    pub fn on_to_resolve_domain(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::CallbackContext, &str) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
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
            ) -> qiniu_http_client::CallbackResult
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
    #[doc = "设置 IP 地址选择成功回调函数"]
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
    #[doc = "设置 HTTP 请求签名前回调函数"]
    pub fn on_before_request_signed(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_before_request_signed(callback);
        self
    }
    #[inline]
    #[doc = "设置 HTTP 请求前回调函数"]
    pub fn on_after_request_signed(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
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
            ) -> qiniu_http_client::CallbackResult
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
            ) -> qiniu_http_client::CallbackResult
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
    #[doc = "设置退避后回调函数"]
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
    pub fn call(&mut self) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = &mut self.0;
        let response = request.call()?;
        let parsed = response.parse_json()?;
        Ok(parsed)
    }
}
#[cfg(feature = "async")]
impl<'req, E: qiniu_http_client::EndpointsProvider + Clone + 'req> AsyncRequestBuilder<'req, E> {
    #[doc = "异步发起 HTTP 请求"]
    pub async fn call(&mut self) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = &mut self.0;
        let response = request.call().await?;
        let parsed = response.parse_json().await?;
        Ok(parsed)
    }
}
