// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!
//
#[derive(Debug, Default)]
#[doc = "调用 API 所用的请求体参数"]
pub struct RequestBody {
    r#operations: Vec<std::borrow::Cow<'static, str>>,
    extended_pairs: Vec<(
        std::borrow::Cow<'static, str>,
        Option<std::borrow::Cow<'static, str>>,
    )>,
}
impl RequestBody {
    #[inline]
    pub fn append_pair(
        mut self,
        key: impl Into<std::borrow::Cow<'static, str>>,
        value: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        self.extended_pairs.push((key.into(), Some(value.into())));
        self
    }
    fn build(
        self,
    ) -> Vec<(
        std::borrow::Cow<'static, str>,
        Option<std::borrow::Cow<'static, str>>,
    )> {
        let mut all_pairs: Vec<_> = Default::default();
        for value in self.r#operations.into_iter() {
            all_pairs.push(("op".into(), Some(value)));
        }
        all_pairs.extend(self.extended_pairs);
        all_pairs
    }
}
impl IntoIterator for RequestBody {
    type Item = (
        std::borrow::Cow<'static, str>,
        Option<std::borrow::Cow<'static, str>>,
    );
    type IntoIter = std::vec::IntoIter<Self::Item>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.build().into_iter()
    }
}
impl RequestBody {
    #[inline]
    #[doc = "单一对象管理指令"]
    pub fn append_operations_as_str(
        mut self,
        value: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        self.r#operations.push(value.into());
        self
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "获取 API 所用的响应体参数"]
pub struct ResponseBody<'a>(std::borrow::Cow<'a, serde_json::Value>);
impl<'a> ResponseBody<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(value: std::borrow::Cow<'a, serde_json::Value>) -> Self {
        Self(value)
    }
}
impl<'a> From<ResponseBody<'a>> for serde_json::Value {
    #[inline]
    fn from(val: ResponseBody<'a>) -> Self {
        val.0.into_owned()
    }
}
impl<'a> std::convert::AsRef<serde_json::Value> for ResponseBody<'a> {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        self.0.as_ref()
    }
}
impl<'a> std::convert::AsMut<serde_json::Value> for ResponseBody<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        self.0.to_mut()
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "每个管理指令的响应信息"]
pub struct OperationResponse<'a>(std::borrow::Cow<'a, serde_json::Value>);
impl<'a> OperationResponse<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(value: std::borrow::Cow<'a, serde_json::Value>) -> Self {
        Self(value)
    }
}
impl<'a> From<OperationResponse<'a>> for serde_json::Value {
    #[inline]
    fn from(val: OperationResponse<'a>) -> Self {
        val.0.into_owned()
    }
}
impl<'a> std::convert::AsRef<serde_json::Value> for OperationResponse<'a> {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        self.0.as_ref()
    }
}
impl<'a> std::convert::AsMut<serde_json::Value> for OperationResponse<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        self.0.to_mut()
    }
}
impl<'a> OperationResponse<'a> {
    #[doc = "获取 响应状态码"]
    pub fn get_code_as_i64(&self) -> i64 {
        self.0
            .as_object()
            .unwrap()
            .get("code")
            .unwrap()
            .as_i64()
            .unwrap()
    }
}
impl<'a> OperationResponse<'a> {
    #[doc = "设置 响应状态码"]
    pub fn set_code_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0
            .to_mut()
            .as_object_mut()
            .unwrap()
            .insert("code".to_owned(), new.into())
            .and_then(|val| val.as_i64())
    }
}
impl<'a> OperationResponse<'a> {
    #[doc = "获取 响应状态码"]
    pub fn get_code_as_u64(&self) -> u64 {
        self.0
            .as_object()
            .unwrap()
            .get("code")
            .unwrap()
            .as_u64()
            .unwrap()
    }
}
impl<'a> OperationResponse<'a> {
    #[doc = "设置 响应状态码"]
    pub fn set_code_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0
            .to_mut()
            .as_object_mut()
            .unwrap()
            .insert("code".to_owned(), new.into())
            .and_then(|val| val.as_u64())
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "管理指令的响应数据"]
pub struct OperationResponseData<'a>(std::borrow::Cow<'a, serde_json::Value>);
impl<'a> OperationResponseData<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(value: std::borrow::Cow<'a, serde_json::Value>) -> Self {
        Self(value)
    }
}
impl<'a> From<OperationResponseData<'a>> for serde_json::Value {
    #[inline]
    fn from(val: OperationResponseData<'a>) -> Self {
        val.0.into_owned()
    }
}
impl<'a> std::convert::AsRef<serde_json::Value> for OperationResponseData<'a> {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        self.0.as_ref()
    }
}
impl<'a> std::convert::AsMut<serde_json::Value> for OperationResponseData<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        self.0.to_mut()
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 管理指令的错误信息，仅在发生错误时才返回"]
    pub fn get_error_as_str(&self) -> Option<&str> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("error"))
            .and_then(|val| val.as_str())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 管理指令的错误信息，仅在发生错误时才返回"]
    pub fn set_error_as_str(&mut self, new: String) -> Option<String> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("error".to_owned(), new.into())
                .and_then(|val| match val {
                    serde_json::Value::String(s) => Some(s),
                    _ => None,
                })
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 对象大小，单位为字节，仅对 stat 指令才有效"]
    pub fn get_size_as_i64(&self) -> Option<i64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("fsize"))
            .and_then(|val| val.as_i64())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 对象大小，单位为字节，仅对 stat 指令才有效"]
    pub fn set_size_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("fsize".to_owned(), new.into())
                .and_then(|val| val.as_i64())
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 对象大小，单位为字节，仅对 stat 指令才有效"]
    pub fn get_size_as_u64(&self) -> Option<u64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("fsize"))
            .and_then(|val| val.as_u64())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 对象大小，单位为字节，仅对 stat 指令才有效"]
    pub fn set_size_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("fsize".to_owned(), new.into())
                .and_then(|val| val.as_u64())
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 对象哈希值，仅对 stat 指令才有效"]
    pub fn get_hash_as_str(&self) -> Option<&str> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("hash"))
            .and_then(|val| val.as_str())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 对象哈希值，仅对 stat 指令才有效"]
    pub fn set_hash_as_str(&mut self, new: String) -> Option<String> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("hash".to_owned(), new.into())
                .and_then(|val| match val {
                    serde_json::Value::String(s) => Some(s),
                    _ => None,
                })
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 对象 MIME 类型，仅对 stat 指令才有效"]
    pub fn get_mime_type_as_str(&self) -> Option<&str> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("mimeType"))
            .and_then(|val| val.as_str())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 对象 MIME 类型，仅对 stat 指令才有效"]
    pub fn set_mime_type_as_str(&mut self, new: String) -> Option<String> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("mimeType".to_owned(), new.into())
                .and_then(|val| match val {
                    serde_json::Value::String(s) => Some(s),
                    _ => None,
                })
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 对象存储类型，`0` 表示普通存储，`1` 表示低频存储，`2` 表示归档存储，仅对 stat 指令才有效"]
    pub fn get_type_as_i64(&self) -> Option<i64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("type"))
            .and_then(|val| val.as_i64())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 对象存储类型，`0` 表示普通存储，`1` 表示低频存储，`2` 表示归档存储，仅对 stat 指令才有效"]
    pub fn set_type_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("type".to_owned(), new.into())
                .and_then(|val| val.as_i64())
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 对象存储类型，`0` 表示普通存储，`1` 表示低频存储，`2` 表示归档存储，仅对 stat 指令才有效"]
    pub fn get_type_as_u64(&self) -> Option<u64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("type"))
            .and_then(|val| val.as_u64())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 对象存储类型，`0` 表示普通存储，`1` 表示低频存储，`2` 表示归档存储，仅对 stat 指令才有效"]
    pub fn set_type_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("type".to_owned(), new.into())
                .and_then(|val| val.as_u64())
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 文件上传时间，UNIX 时间戳格式，单位为 100 纳秒，仅对 stat 指令才有效"]
    pub fn get_put_time_as_i64(&self) -> Option<i64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("putTime"))
            .and_then(|val| val.as_i64())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 文件上传时间，UNIX 时间戳格式，单位为 100 纳秒，仅对 stat 指令才有效"]
    pub fn set_put_time_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("putTime".to_owned(), new.into())
                .and_then(|val| val.as_i64())
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 文件上传时间，UNIX 时间戳格式，单位为 100 纳秒，仅对 stat 指令才有效"]
    pub fn get_put_time_as_u64(&self) -> Option<u64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("putTime"))
            .and_then(|val| val.as_u64())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 文件上传时间，UNIX 时间戳格式，单位为 100 纳秒，仅对 stat 指令才有效"]
    pub fn set_put_time_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("putTime".to_owned(), new.into())
                .and_then(|val| val.as_u64())
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 归档存储文件的解冻状态，`2` 表示解冻完成，`1` 表示解冻中；归档文件冻结时，不返回该字段，仅对 stat 指令才有效"]
    pub fn get_unfreezing_status_as_i64(&self) -> Option<i64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("restoreStatus"))
            .and_then(|val| val.as_i64())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 归档存储文件的解冻状态，`2` 表示解冻完成，`1` 表示解冻中；归档文件冻结时，不返回该字段，仅对 stat 指令才有效"]
    pub fn set_unfreezing_status_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("restoreStatus".to_owned(), new.into())
                .and_then(|val| val.as_i64())
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 归档存储文件的解冻状态，`2` 表示解冻完成，`1` 表示解冻中；归档文件冻结时，不返回该字段，仅对 stat 指令才有效"]
    pub fn get_unfreezing_status_as_u64(&self) -> Option<u64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("restoreStatus"))
            .and_then(|val| val.as_u64())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 归档存储文件的解冻状态，`2` 表示解冻完成，`1` 表示解冻中；归档文件冻结时，不返回该字段，仅对 stat 指令才有效"]
    pub fn set_unfreezing_status_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("restoreStatus".to_owned(), new.into())
                .and_then(|val| val.as_u64())
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 文件状态。`1` 表示禁用；只有禁用状态的文件才会返回该字段，仅对 stat 指令才有效"]
    pub fn get_status_as_i64(&self) -> Option<i64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("status"))
            .and_then(|val| val.as_i64())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 文件状态。`1` 表示禁用；只有禁用状态的文件才会返回该字段，仅对 stat 指令才有效"]
    pub fn set_status_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("status".to_owned(), new.into())
                .and_then(|val| val.as_i64())
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 文件状态。`1` 表示禁用；只有禁用状态的文件才会返回该字段，仅对 stat 指令才有效"]
    pub fn get_status_as_u64(&self) -> Option<u64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("status"))
            .and_then(|val| val.as_u64())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 文件状态。`1` 表示禁用；只有禁用状态的文件才会返回该字段，仅对 stat 指令才有效"]
    pub fn set_status_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("status".to_owned(), new.into())
                .and_then(|val| val.as_u64())
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 对象 MD5 值，只有通过直传文件和追加文件 API 上传的文件，服务端确保有该字段返回，仅对 stat 指令才有效"]
    pub fn get_md_5_as_str(&self) -> Option<&str> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("md5"))
            .and_then(|val| val.as_str())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 对象 MD5 值，只有通过直传文件和追加文件 API 上传的文件，服务端确保有该字段返回，仅对 stat 指令才有效"]
    pub fn set_md_5_as_str(&mut self, new: String) -> Option<String> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("md5".to_owned(), new.into())
                .and_then(|val| match val {
                    serde_json::Value::String(s) => Some(s),
                    _ => None,
                })
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 文件过期删除日期，UNIX 时间戳格式，文件在设置过期时间后才会返回该字段，仅对 stat 指令才有效"]
    pub fn get_expiration_time_as_i64(&self) -> Option<i64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("expiration"))
            .and_then(|val| val.as_i64())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 文件过期删除日期，UNIX 时间戳格式，文件在设置过期时间后才会返回该字段，仅对 stat 指令才有效"]
    pub fn set_expiration_time_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("expiration".to_owned(), new.into())
                .and_then(|val| val.as_i64())
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 文件过期删除日期，UNIX 时间戳格式，文件在设置过期时间后才会返回该字段，仅对 stat 指令才有效"]
    pub fn get_expiration_time_as_u64(&self) -> Option<u64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("expiration"))
            .and_then(|val| val.as_u64())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 文件过期删除日期，UNIX 时间戳格式，文件在设置过期时间后才会返回该字段，仅对 stat 指令才有效"]
    pub fn set_expiration_time_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("expiration".to_owned(), new.into())
                .and_then(|val| val.as_u64())
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 文件生命周期中转为低频存储的日期，UNIX 时间戳格式，文件在设置转低频后才会返回该字段，仅对 stat 指令才有效"]
    pub fn get_transition_to_ia_time_as_i64(&self) -> Option<i64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("transitionToIA"))
            .and_then(|val| val.as_i64())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 文件生命周期中转为低频存储的日期，UNIX 时间戳格式，文件在设置转低频后才会返回该字段，仅对 stat 指令才有效"]
    pub fn set_transition_to_ia_time_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("transitionToIA".to_owned(), new.into())
                .and_then(|val| val.as_i64())
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 文件生命周期中转为低频存储的日期，UNIX 时间戳格式，文件在设置转低频后才会返回该字段，仅对 stat 指令才有效"]
    pub fn get_transition_to_ia_time_as_u64(&self) -> Option<u64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("transitionToIA"))
            .and_then(|val| val.as_u64())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 文件生命周期中转为低频存储的日期，UNIX 时间戳格式，文件在设置转低频后才会返回该字段，仅对 stat 指令才有效"]
    pub fn set_transition_to_ia_time_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("transitionToIA".to_owned(), new.into())
                .and_then(|val| val.as_u64())
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 文件生命周期中转为归档存储的日期，UNIX 时间戳格式，文件在设置转归档后才会返回该字段，仅对 stat 指令才有效"]
    pub fn get_transition_to_archive_time_as_i64(&self) -> Option<i64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("transitionToARCHIVE"))
            .and_then(|val| val.as_i64())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 文件生命周期中转为归档存储的日期，UNIX 时间戳格式，文件在设置转归档后才会返回该字段，仅对 stat 指令才有效"]
    pub fn set_transition_to_archive_time_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("transitionToARCHIVE".to_owned(), new.into())
                .and_then(|val| val.as_i64())
        })
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "获取 文件生命周期中转为归档存储的日期，UNIX 时间戳格式，文件在设置转归档后才会返回该字段，仅对 stat 指令才有效"]
    pub fn get_transition_to_archive_time_as_u64(&self) -> Option<u64> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("transitionToARCHIVE"))
            .and_then(|val| val.as_u64())
    }
}
impl<'a> OperationResponseData<'a> {
    #[doc = "设置 文件生命周期中转为归档存储的日期，UNIX 时间戳格式，文件在设置转归档后才会返回该字段，仅对 stat 指令才有效"]
    pub fn set_transition_to_archive_time_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("transitionToARCHIVE".to_owned(), new.into())
                .and_then(|val| val.as_u64())
        })
    }
}
impl<'a> OperationResponse<'a> {
    #[doc = "获取 响应数据"]
    pub fn get_data(&self) -> OperationResponseData {
        OperationResponseData::new(std::borrow::Cow::Borrowed(
            self.0.as_object().unwrap().get("data").unwrap(),
        ))
    }
}
impl<'a> OperationResponse<'a> {
    #[doc = "设置 响应数据"]
    pub fn set_data(&mut self, new: OperationResponseData) -> Option<OperationResponseData> {
        self.0
            .to_mut()
            .as_object_mut()
            .unwrap()
            .insert("data".to_owned(), new.into())
            .map(std::borrow::Cow::Owned)
            .map(OperationResponseData::new)
    }
}
impl<'a> ResponseBody<'a> {
    #[doc = "解析 JSON 得到 OperationResponse 列表"]
    pub fn to_operation_response_vec(&self) -> Vec<OperationResponse> {
        self.0
            .as_array()
            .unwrap()
            .iter()
            .map(std::borrow::Cow::Borrowed)
            .map(OperationResponse::new)
            .collect()
    }
}
impl<'a> From<Vec<OperationResponse<'a>>> for ResponseBody<'a> {
    #[inline]
    fn from(val: Vec<OperationResponse<'a>>) -> Self {
        Self(std::borrow::Cow::Owned(serde_json::Value::from(val)))
    }
}
impl<'a, 'b> From<&'a [OperationResponse<'a>]> for ResponseBody<'b> {
    #[inline]
    fn from(val: &'a [OperationResponse<'a>]) -> Self {
        Self(std::borrow::Cow::Owned(serde_json::Value::from(val)))
    }
}
impl<'a> ResponseBody<'a> {
    pub fn len(&self) -> usize {
        self.0.as_array().unwrap().len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.as_array().unwrap().is_empty()
    }
}
impl<'a> ResponseBody<'a> {
    #[doc = "在列表的指定位置插入 JSON OperationResponse"]
    pub fn insert_operation_response(&mut self, index: usize, val: OperationResponse<'a>) {
        self.0
            .to_mut()
            .as_array_mut()
            .unwrap()
            .insert(index, val.into());
    }
}
impl<'a> ResponseBody<'a> {
    #[doc = "在列表的指定位置移出 JSON OperationResponse"]
    pub fn remove_as_operation_response(&mut self, index: usize) -> OperationResponse {
        OperationResponse::new(std::borrow::Cow::Owned(
            self.0.to_mut().as_array_mut().unwrap().remove(index),
        ))
    }
}
impl<'a> ResponseBody<'a> {
    #[doc = "在列表尾部追加 JSON OperationResponse"]
    pub fn push_operation_response(&mut self, val: OperationResponse<'a>) {
        self.0.to_mut().as_array_mut().unwrap().push(val.into());
    }
}
impl<'a> ResponseBody<'a> {
    #[doc = "在列表尾部取出 JSON OperationResponse"]
    pub fn pop_operation_response(&mut self) -> Option<OperationResponse> {
        self.0
            .to_mut()
            .as_array_mut()
            .unwrap()
            .pop()
            .map(std::borrow::Cow::Owned)
            .map(OperationResponse::new)
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
        credential: Box<dyn qiniu_http_client::credential::CredentialProvider>,
    ) -> SyncRequestBuilder<'client, E> {
        SyncRequestBuilder(
            self.0
                .post(&[qiniu_http_client::ServiceName::Rs], endpoints_provider)
                .authorization(qiniu_http_client::Authorization::v2(credential))
                .idempotent(qiniu_http_client::Idempotent::Default)
                .path("batch")
                .accept_json(),
        )
    }
    #[inline]
    #[cfg(feature = "async")]
    pub fn new_async_request<E: qiniu_http_client::EndpointsProvider + 'client>(
        &self,
        endpoints_provider: E,
        credential: Box<dyn qiniu_http_client::credential::CredentialProvider>,
    ) -> AsyncRequestBuilder<'client, E> {
        AsyncRequestBuilder(
            self.0
                .async_post(&[qiniu_http_client::ServiceName::Rs], endpoints_provider)
                .authorization(qiniu_http_client::Authorization::v2(credential))
                .idempotent(qiniu_http_client::Idempotent::Default)
                .path("batch")
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
    pub fn on_success(
        mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                &qiniu_http_client::ResponseInfo,
            ) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_success(callback);
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
        body: RequestBody,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody<'static>>> {
        let request = self.0.post_form(body);
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
    pub fn on_success(
        mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                &qiniu_http_client::ResponseInfo,
            ) -> bool
            + Send
            + Sync
            + 'req,
    ) -> Self {
        self.0 = self.0.on_success(callback);
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
        body: RequestBody,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody<'static>>> {
        let request = self.0.post_form(body);
        let response = request.call().await?;
        let parsed = response.parse_json().await?;
        Ok(parsed)
    }
}
