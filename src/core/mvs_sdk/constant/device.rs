pub use std::ffi::*;
/// MVS SDK路径环境变量
pub const HCMVS_LIB_ENV: &'static str = "HCMVS_LIB";
/// Windows 控制动态链接库
#[cfg(target_os = "windows")]
pub const HCMVS_CAMERA_CONTROL_LIB: &'static str = "MvCameraControl.dll";
#[cfg(target_os = "linux")]
pub const HCMVS_CAMERA_CONTROL_LIB: &'static str = "";

/// 设备类型定义
/// 未知设备类型，保留意义
pub const MV_UNKNOWN_DEVICE: usize = 0x00000000;
/// GigE设备
pub const MV_GIGE_DEVICE: usize = 0x00000001;
/// 1394-a/b 设备
pub const MV_1394_DEVICE: usize = 0x00000002;
/// USB 设备
pub const MV_USB_DEVICE: usize = 0x00000004;
/// CameraLink设备
pub const MV_CAMERALINK_DEVICE: usize = 0x00000008;

/// 最大的数据信息大小
pub const INFO_MAX_BUFFER_SIZE: usize = 64;

/// 最多支持的传输层实例个数
pub const MV_MAX_TLS_NUM: usize = 8;
/// 最大支持的设备个数
pub const MV_MAX_DEVICE_NUM: usize = 256;

/// 最大支持的GenTL数量
pub const MV_MAX_GENTL_IF_NUM: usize = 256;
/// 最大支持的GenTL设备数量
pub const MV_MAX_GENTL_DEV_NUM: usize = 256;

/// 网络流量和丢包信息
pub const MV_MATCH_TYPE_NET_DETECT: usize = 0x00000001;
pub const MV_MATCH_TYPE_USB_DETECT: usize = 0x00000002;

/// GigEVision IP配置
pub const MV_IP_CFG_STATIC: usize = 0x05000000; // 静态
pub const MV_IP_CFG_DHCP: usize = 0x06000000; // DHCP
pub const MV_IP_CFG_LLA: usize = 0x04000000; // LLA

/// GigEVision网络传输模式
pub const MV_NET_TRANS_DRIVER: usize = 0x00000001; // 驱动
pub const MV_NET_TRANS_SOCKET: usize = 0x00000002; // Socket

/// CameraLink波特率
pub const MV_CAML_BAUDRATE_9600: usize = 0x00000001;
pub const MV_CAML_BAUDRATE_19200: usize = 0x00000002;
pub const MV_CAML_BAUDRATE_38400: usize = 0x00000004;
pub const MV_CAML_BAUDRATE_57600: usize = 0x00000008;
pub const MV_CAML_BAUDRATE_115200: usize = 0x00000010;
pub const MV_CAML_BAUDRATE_230400: usize = 0x00000020;
pub const MV_CAML_BAUDRATE_460800: usize = 0x00000040;
pub const MV_CAML_BAUDRATE_921600: usize = 0x00000080;
pub const MV_CAML_BAUDRATE_AUTOMAX: usize = 0x40000000; // 最大值

/// 异常消息类型
/// 设备断开连接
pub const MV_EXCEPTION_DEV_DISCONNECT: usize = 0x00008001;
/// SDK与驱动版本不匹配
pub const MV_EXCEPTION_VERSION_CHECK: usize = 0x00008002;

/// 设备Event事件名称最大长度
pub const MAX_EVENT_NAME_SIZE: usize = 128;
/// 最大XML符号数
pub const MV_MAX_XML_SYMBOLIC_NUM: usize = 64;

/// 未定义的点云格式
pub const MV_POINT_CLOUD_FILE_UNDEFINED: c_int = 0;
/// 持续采集模式
pub const MV_ACQ_MODE_CONTINUOUS: c_int = 2;
/// 多帧模式
pub const MV_ACQ_MODE_MULTI: c_int = 1;
/// 打开
pub const MV_TRIGGER_MODE_ON: c_int = 1;
/// 内部用于 AccessMode 循环检测
pub const AM_CYCLE_DETECT: c_int = 6;
/// 对象未被初始化
pub const AM_UNDEFINED: c_int = 5;
/// 读和写
pub const AM_RW: c_int = 4;
/// 只读
pub const AM_RO: c_int = 3;
/// 只写
pub const AM_WO: c_int = 2;
/// 不可用
pub const AM_NA: c_int = 1;
/// 没有实现
pub const AM_NI: c_int = 0;
/// Tif格式
pub const MV_IMAGE_TIF: c_int = 4;
/// Png格式
pub const MV_IMAGE_PNG: c_int = 3;
/// Jpeg格式
pub const MV_IMAGE_JPEG: c_int = 2;
/// Bmp格式
pub const MV_IMAGE_BMP: c_int = 1;
/// 连续
pub const MV_GAIN_MODE_CONTINUOUS: c_int = 2;
/// 单次
pub const MV_GAIN_MODE_ONCE: c_int = 1;
/// 等待下一帧图像
pub const MV_GRAB_STRATEGY_UPCOMING_IMAGE: c_int = 3;
/// 获取列表中最新的图像
pub const MV_GRAB_STRATEGY_LATEST_IMAGES: c_int = 2;
/// 获取列表中最新的一帧图像（同时清除列表中的其余图像）
pub const MV_GRAB_STRATEGY_LATEST_IMAGES_ONLY: c_int = 1;
/// OBJ点云格式
pub const MV_POINT_CLOUD_FILE_OBJ: c_int = 3;
/// CSV点云格式
pub const MV_POINT_CLOUD_FILE_CSV: c_int = 2;
/// PLY点云格式
pub const MV_POINT_CLOUD_FILE_PLY: c_int = 1;
/// 单帧模式
pub const MV_ACQ_MODE_SINGLE: c_int = 0;
/// 关闭
pub const MV_TRIGGER_MODE_OFF: c_int = 0;
/// 软触发
pub const MV_TRIGGER_SOURCE_SOFTWARE: c_int = 7;
/// 曝光超时模式
pub const MV_EXPOSURE_MODE_TIMED: c_int = 0;
/// 未定义的格式类型
pub const MV_FORMAT_TYPE_UNDEFINED: c_int = 0;
/// 关闭增益模式
pub const MV_GAIN_MODE_OFF: c_int = 0;
/// 自动连续曝光模式
pub const MV_EXPOSURE_AUTO_MODE_CONTINUOUS: c_int = 2;
/// 单次自动曝光模式
pub const MV_EXPOSURE_AUTO_MODE_ONCE: c_int = 1;
/// 关闭自动曝光模式
pub const MV_EXPOSURE_AUTO_MODE_OFF: c_int = 0;
/// IValue接口类型
pub const IFT_IVALUE: c_int = 0;
/// 从旧到新一帧一帧的获取图像
pub const MV_GRAB_STRATEGY_ONE_BY_ONE: c_int = 0;
/// AVI视频格式
pub const MV_FORMAT_TYPE_AVI: c_int = 1;
/// gamma选择项User
pub const MV_GAMMA_SELECTOR_USER: c_int = 1;
/// IString接口类型
pub const IFT_ISTRING: c_int = 6;
/// 白平衡自动关闭
pub const MV_BALANCE_WHITE_AUTO_OFF: c_int = 0;
/// gamma选择项SRGB
pub const MV_GAMMA_SELECTOR_SRGB: c_int = 2;
/// IPort接口类型
pub const IFT_IPORT: c_int = 11;
/// 白平衡自动连续
pub const MV_BALANCE_WHITE_AUTO_CONTINUOUS: c_int = 1;
/// IEnumEntry接口类型
pub const IFT_IENUM_ENTRY: c_int = 10;
/// ICategory接口类型
pub const IFT_ICATEGORY: c_int = 8;
/// IRegister接口类型
pub const IFT_IREGISTER: c_int = 7;
/// 未定义的图像类型
pub const MV_IMAGE_UNDEFINED: c_int = 0;
/// IFloat接口类型
pub const IFT_IFLOAT: c_int = 5;
/// IEnumeration接口类型
pub const IFT_IENUMERATION: c_int = 9;
/// ICommand接口类型
pub const IFT_ICOMMAND: c_int = 4;
/// IBoolean接口类型
pub const IFT_IBOOLEAN: c_int = 3;
/// IInteger接口类型
pub const IFT_IINTEGER: c_int = 2;
/// 表示组播模式，但本实例不接收图像数据
pub const MV_GIGE_TRANSTYPE_MULTICAST_WITHOUT_RECV: c_int = 65537;
/// IBase接口类型
pub const IFT_IBASE: c_int = 1;
/// 表示设置了单播，但本例不接收图像数据
pub const MV_GIGE_TRANSTYPE_UNICAST_WITHOUT_RECV: c_int = 65536;
/// 单次自动白平衡
pub const MV_BALANCE_WHITE_AUTO_ONCE: c_int = 2;
/// 表示局域网内广播，暂不支持
pub const MV_GIGE_TRANSTYPE_LIMITED_BROADCAST: c_int = 2;
/// 表示组播
pub const MV_GIGE_TRANSTYPE_MULTICAST: c_int = 1;
/// 表示单播(默认)
pub const MV_GIGE_TRANSTYPE_UNICAST: c_int = 0;
/// 表示从相机获取，暂不支持
pub const MV_GIGE_TRANSTYPE_CAMERA_DEFINED: c_int = 4;
/// 表示子网内广播，暂不支持
pub const MV_GIGE_TRANSTYPE_SUBNET_BROADCAST: c_int = 3;
/// 曝光模式触发宽
pub const MV_EXPOSURE_MODE_TRIGGER_WIDTH: c_int = 1;
/// 表示用户自定义应用端接收图像数据Port号
pub const MV_GIGE_TRANSTYPE_UNICAST_DEFINED_PORT: c_int = 5;
/// 触发源变频器
pub const MV_TRIGGER_SOURCE_FREQUENCY_CONVERTER: c_int = 8;
/// 触发源计数器
pub const MV_TRIGGER_SOURCE_COUNTER_0: c_int = 4;
/// LINE3 触发源
pub const MV_TRIGGER_SOURCE_LINE_3: c_int = 3;
/// LINE2 触发源
pub const MV_TRIGGER_SOURCE_LINE_2: c_int = 2;
/// LINE1 触发源
pub const MV_TRIGGER_SOURCE_LINE_1: c_int = 1;
/// LINE0 触发源
pub const MV_TRIGGER_SOURCE_LINE_0: c_int = 0;
