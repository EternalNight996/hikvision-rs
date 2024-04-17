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
