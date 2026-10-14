/// 成功，无错误
pub const MV_OK: u32 = 0x00000000;

// 通用错误码定义:范围0x80000000-0x800000FF
/// 错误或无效的句柄
pub const MV_E_HANDLE: u32 = 0x80000000;
/// 不支持的功能
pub const MV_E_SUPPORT: u32 = 0x80000001;
/// 缓存已满
pub const MV_E_BUFOVER: u32 = 0x80000002;
/// 函数调用顺序错误
pub const MV_E_CALLORDER: u32 = 0x80000003;
/// 错误的参数
pub const MV_E_PARAMETER: u32 = 0x80000004;
/// 资源申请失败
pub const MV_E_RESOURCE: u32 = 0x80000006;
/// 无数据
pub const MV_E_NODATA: u32 = 0x80000007;
/// 前置条件有误，或运行环境已发生变化
pub const MV_E_PRECONDITION: u32 = 0x80000008;
/// 版本不匹配
pub const MV_E_VERSION: u32 = 0x80000009;
/// 传入的内存空间不足
pub const MV_E_NOENOUGH_BUF: u32 = 0x8000000A;
/// 异常图像，可能是丢包导致图像不完整
pub const MV_E_ABNORMAL_IMAGE: u32 = 0x8000000B;
/// 动态导入DLL失败
pub const MV_E_LOAD_LIBRARY: u32 = 0x8000000C;
/// 没有可输出的缓存
pub const MV_E_NOOUTBUF: u32 = 0x8000000D;
/// 未知的错误
pub const MV_E_UNKNOW: u32 = 0x800000FF;

// GenICam系列错误:范围0x80000100-0x800001FF
/// 通用错误
pub const MV_E_GC_GENERIC: u32 = 0x80000100;
/// 参数非法
pub const MV_E_GC_ARGUMENT: u32 = 0x80000101;
/// 值超出范围
pub const MV_E_GC_RANGE: u32 = 0x80000102;
/// 属性
pub const MV_E_GC_PROPERTY: u32 = 0x80000103;
/// 运行环境有问题
pub const MV_E_GC_RUNTIME: u32 = 0x80000104;
/// 逻辑错误
pub const MV_E_GC_LOGICAL: u32 = 0x80000105;
/// 节点访问条件有误
pub const MV_E_GC_ACCESS: u32 = 0x80000106;
/// 超时
pub const MV_E_GC_TIMEOUT: u32 = 0x80000107;
/// 转换异常
pub const MV_E_GC_DYNAMICCAST: u32 = 0x80000108;
/// GenICam未知错误
pub const MV_E_GC_UNKNOW: u32 = 0x800001FF;

// GigE_STATUS对应的错误码:范围0x80000200-0x800002FF
/// 命令不被设备支持
pub const MV_E_NOT_IMPLEMENTED: u32 = 0x80000200;
/// 访问的目标地址不存在
pub const MV_E_INVALID_ADDRESS: u32 = 0x80000201;
/// 目标地址不可写
pub const MV_E_WRITE_PROTECT: u32 = 0x80000202;
/// 设备无访问权限
pub const MV_E_ACCESS_DENIED: u32 = 0x80000203;
/// 设备忙，或网络断开
pub const MV_E_BUSY: u32 = 0x80000204;
/// 网络包数据错误
pub const MV_E_PACKET: u32 = 0x80000205;
/// 网络相关错误
pub const MV_E_NETER: u32 = 0x80000206;
/// 设备IP冲突
pub const MV_E_IP_CONFLICT: u32 = 0x80000221;

// USB_STATUS对应的错误码:范围0x80000300-0x800003FF
/// 读usb出错
pub const MV_E_USB_READ: u32 = 0x80000300;
/// 写usb出错
pub const MV_E_USB_WRITE: u32 = 0x80000301;
/// 设备异常
pub const MV_E_USB_DEVICE: u32 = 0x80000302;
/// GenICam相关错误
pub const MV_E_USB_GENICAM: u32 = 0x80000303;
/// 带宽不足  该错误码新增
pub const MV_E_USB_BANDWIDTH: u32 = 0x80000304;
/// 驱动不匹配或者未装驱动
pub const MV_E_USB_DRIVER: u32 = 0x80000305;
/// USB未知的错误
pub const MV_E_USB_UNKNOW: u32 = 0x800003FF;

// 升级时对应的错误码:范围0x80000400-0x800004FF
/// 升级固件不匹配
pub const MV_E_UPG_FILE_MISMATCH: u32 = 0x80000400;
/// 升级固件语言不匹配
pub const MV_E_UPG_LANGUSGE_MISMATCH: u32 = 0x80000401;
/// 升级冲突（设备已经在升级了再次请求升级即返回此错误）
pub const MV_E_UPG_CONFLICT: u32 = 0x80000402;
/// 升级时设备内部出现错误
pub const MV_E_UPG_INNER_ERR: u32 = 0x80000403;
/// 升级时未知错误
pub const MV_E_UPG_UNKNOW: u32 = 0x800004FF;

use std::fmt;

/// MVS 返回码。
///
/// # ⚠️ 语义提醒（最容易踩的坑）
/// 本 crate 的 `mvs_sdk` 把所有 `MV_CC_*` 的返回码放在 **`CResult::Ok(code)`** 里
/// （`CResult::Err` 只用于 libloading 取符号失败）。也就是说 **`Ok` 不等于成功**：
/// `0 = MV_OK`，非 0 是 `MV_E_*` 错误码。调用方必须过一遍 [`check`]（或自判 `code != 0`），
/// 否则「设置曝光失败」「开流失败」会与成功表现完全一致。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MvError(pub u32);

impl MvError {
  /// 由原始返回码构造：`0`（MV_OK）返回 `None`
  pub fn from_code(code: i32) -> Option<Self> {
    if code == 0 { None } else { Some(Self(code as u32)) }
  }
  /// 原始有符号码（与 SDK 日志/调试输出一致）
  pub fn raw(self) -> i32 {
    self.0 as i32
  }
  /// 官方常量名（未收录的码归为 `MV_E_UNKNOW`）
  pub fn name(self) -> &'static str {
    describe(self.0).0
  }
  /// 官方中文说明
  pub fn message(self) -> &'static str {
    describe(self.0).1
  }
}

impl fmt::Display for MvError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}(0x{:08X}): {}", self.name(), self.0, self.message())
  }
}

impl std::error::Error for MvError {}

/// 检查 MVS 返回码：`0 → Ok(code)`，非 0 → `Err(MvError)`。
///
/// 典型用法（替代 `CResult` 只看 Ok/Err 的写法）：
/// ```ignore
/// let code = unsafe { sdk.start_grabbing() };
/// match code {
///   e_utils::CResult::Ok(c) => { check(c).expect("开始取流失败"); }
///   e_utils::CResult::Err(e) => panic!("SDK 符号缺失: {e:?}"),
/// }
/// ```
pub fn check(code: i32) -> Result<i32, MvError> {
  match MvError::from_code(code) {
    None => Ok(code),
    Some(e) => Err(e),
  }
}

/// 码 → (常量名, 中文说明)
fn describe(code: u32) -> (&'static str, &'static str) {
  match code {
    MV_OK => ("MV_OK", "成功，无错误"),
    MV_E_HANDLE => ("MV_E_HANDLE", "错误或无效的句柄"),
    MV_E_SUPPORT => ("MV_E_SUPPORT", "不支持的功能"),
    MV_E_BUFOVER => ("MV_E_BUFOVER", "缓存已满"),
    MV_E_CALLORDER => ("MV_E_CALLORDER", "函数调用顺序错误"),
    MV_E_PARAMETER => ("MV_E_PARAMETER", "错误的参数"),
    MV_E_RESOURCE => ("MV_E_RESOURCE", "资源申请失败"),
    MV_E_NODATA => ("MV_E_NODATA", "无数据"),
    MV_E_PRECONDITION => ("MV_E_PRECONDITION", "前置条件有误，或运行环境已发生变化"),
    MV_E_VERSION => ("MV_E_VERSION", "版本不匹配"),
    MV_E_NOENOUGH_BUF => ("MV_E_NOENOUGH_BUF", "传入的内存空间不足"),
    MV_E_ABNORMAL_IMAGE => ("MV_E_ABNORMAL_IMAGE", "异常图像，可能是丢包导致图像不完整"),
    MV_E_LOAD_LIBRARY => ("MV_E_LOAD_LIBRARY", "动态导入DLL失败"),
    MV_E_NOOUTBUF => ("MV_E_NOOUTBUF", "没有可输出的缓存"),
    MV_E_GC_GENERIC => ("MV_E_GC_GENERIC", "GenICam 通用错误"),
    MV_E_GC_ARGUMENT => ("MV_E_GC_ARGUMENT", "GenICam 参数非法"),
    MV_E_GC_RANGE => ("MV_E_GC_RANGE", "GenICam 值超出范围"),
    MV_E_GC_PROPERTY => ("MV_E_GC_PROPERTY", "GenICam 属性错误"),
    MV_E_GC_RUNTIME => ("MV_E_GC_RUNTIME", "GenICam 运行环境有问题"),
    MV_E_GC_LOGICAL => ("MV_E_GC_LOGICAL", "GenICam 逻辑错误"),
    MV_E_GC_ACCESS => ("MV_E_GC_ACCESS", "节点访问条件有误"),
    MV_E_GC_TIMEOUT => ("MV_E_GC_TIMEOUT", "GenICam 超时"),
    MV_E_GC_DYNAMICCAST => ("MV_E_GC_DYNAMICCAST", "GenICam 转换异常"),
    MV_E_NOT_IMPLEMENTED => ("MV_E_NOT_IMPLEMENTED", "命令不被设备支持"),
    MV_E_INVALID_ADDRESS => ("MV_E_INVALID_ADDRESS", "访问的目标地址不存在"),
    MV_E_WRITE_PROTECT => ("MV_E_WRITE_PROTECT", "目标地址不可写"),
    MV_E_ACCESS_DENIED => ("MV_E_ACCESS_DENIED", "设备无访问权限（多为被其它程序独占）"),
    MV_E_BUSY => ("MV_E_BUSY", "设备忙，或网络断开"),
    MV_E_PACKET => ("MV_E_PACKET", "网络包数据错误"),
    MV_E_NETER => ("MV_E_NETER", "网络相关错误"),
    MV_E_IP_CONFLICT => ("MV_E_IP_CONFLICT", "设备IP冲突"),
    MV_E_USB_READ => ("MV_E_USB_READ", "读 USB 出错"),
    MV_E_USB_WRITE => ("MV_E_USB_WRITE", "写 USB 出错"),
    MV_E_USB_DEVICE => ("MV_E_USB_DEVICE", "USB 设备异常"),
    MV_E_USB_GENICAM => ("MV_E_USB_GENICAM", "USB GenICam 相关错误"),
    MV_E_USB_BANDWIDTH => ("MV_E_USB_BANDWIDTH", "USB 带宽不足"),
    MV_E_USB_DRIVER => ("MV_E_USB_DRIVER", "USB 驱动不匹配或未安装"),
    MV_E_UPG_FILE_MISMATCH => ("MV_E_UPG_FILE_MISMATCH", "升级固件不匹配"),
    MV_E_UPG_LANGUSGE_MISMATCH => ("MV_E_UPG_LANGUSGE_MISMATCH", "升级固件语言不匹配"),
    MV_E_UPG_CONFLICT => ("MV_E_UPG_CONFLICT", "升级冲突（设备已在升级）"),
    MV_E_UPG_INNER_ERR => ("MV_E_UPG_INNER_ERR", "升级时设备内部出现错误"),
    MV_E_GC_UNKNOW => ("MV_E_GC_UNKNOW", "GenICam 未知错误"),
    MV_E_USB_UNKNOW => ("MV_E_USB_UNKNOW", "USB 未知错误"),
    MV_E_UPG_UNKNOW => ("MV_E_UPG_UNKNOW", "升级时未知错误"),
    _ => ("MV_E_UNKNOW", "未知的错误"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// 0 是成功——这是本 crate 最容易搞错的一点
  #[test]
  fn zero_code_is_success() {
    assert!(MvError::from_code(0).is_none());
    assert_eq!(check(0), Ok(0));
  }

  /// 非 0（含负数形式的 MV_E_*）必须变成 Err，且带上常量名与十六进制码
  #[test]
  fn non_zero_code_becomes_error() {
    let err = check(MV_E_HANDLE as i32).expect_err("MV_E_HANDLE 必须报错");
    assert_eq!(err.name(), "MV_E_HANDLE");
    assert_eq!(err.0, 0x80000000);
    let msg = err.to_string();
    assert!(msg.contains("0x80000000"), "文案应含十六进制码: {msg}");
    assert!(msg.contains("句柄"), "文案应含中文说明: {msg}");
  }

  /// 未收录的码要落到 MV_E_UNKNOW，而不是 panic 或空字符串
  #[test]
  fn unknown_code_falls_back() {
    let err = MvError(0x12345678);
    assert_eq!(err.name(), "MV_E_UNKNOW");
    assert_eq!(err.message(), "未知的错误");
    assert_eq!(err.raw(), 0x12345678);
  }

  /// 几个现场高频码的映射抽查（丢包/带宽/占用/超时）
  #[test]
  fn common_field_codes_are_mapped() {
    assert_eq!(MvError(MV_E_ABNORMAL_IMAGE).name(), "MV_E_ABNORMAL_IMAGE");
    assert_eq!(MvError(MV_E_USB_BANDWIDTH).name(), "MV_E_USB_BANDWIDTH");
    assert_eq!(MvError(MV_E_ACCESS_DENIED).name(), "MV_E_ACCESS_DENIED");
    assert_eq!(MvError(MV_E_GC_TIMEOUT).name(), "MV_E_GC_TIMEOUT");
  }
}
