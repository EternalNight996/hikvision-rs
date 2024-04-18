#![allow(non_snake_case)]

use e_utils::{
  cstring_to_array, empty_to_array,
  parse::{AutoParse as _, ParseResult as _},
  res_c, CResult,
};
use serde::{Deserialize, Serialize};
use std::{ffi::*, ptr};

// 云台控制命令
///接通灯光电源
pub const LIGHT_PWRON: c_ulong = 2;
///接通雨刷开关
pub const WIPER_PWRON: c_ulong = 3;
///接通风扇开关
pub const FAN_PWRON: c_ulong = 4;
///接通加热器开关
pub const HEATER_PWRON: c_ulong = 5;
///接通辅助设备开关
pub const AUX_PWRON1: c_ulong = 6;
///接通辅助设备开关
pub const AUX_PWRON2: c_ulong = 7;
///焦距变大(倍率变大)
pub const ZOOM_IN: c_ulong = 11;
///焦距变小(倍率变小)
pub const ZOOM_OUT: c_ulong = 12;
///焦点前调
pub const FOCUS_NEAR: c_ulong = 13;
/// 焦点后调
pub const FOCUS_FAR: c_ulong = 14;
pub const IRIS_OPEN: c_ulong = 15; //光圈扩大
pub const IRIS_CLOSE: c_ulong = 16; //光圈缩小
pub const TILT_UP: c_ulong = 21; //云台上仰
pub const TILT_DOWN: c_ulong = 22; //云台下俯
pub const PAN_LEFT: c_ulong = 23; //云台左转
pub const PAN_RIGHT: c_ulong = 24; //云台右转
pub const UP_LEFT: c_ulong = 25; //云台上仰和左转
pub const UP_RIGHT: c_ulong = 26; //云台上仰和右转
pub const DOWN_LEFT: c_ulong = 27; //云台下俯和左转
pub const DOWN_RIGHT: c_ulong = 28; //云台下俯和右转
pub const PAN_AUTO: c_ulong = 29; //云台左右自动扫描
pub const TILT_DOWN_ZOOM_IN: c_ulong = 58; //云台下俯和焦距变大(倍率变大)
pub const TILT_DOWN_ZOOM_OUT: c_ulong = 59; //云台下俯和焦距变小(倍率变小)
pub const PAN_LEFT_ZOOM_IN: c_ulong = 60; //云台左转和焦距变大(倍率变大)
pub const PAN_LEFT_ZOOM_OUT: c_ulong = 61; //云台左转和焦距变小(倍率变小)
pub const PAN_RIGHT_ZOOM_IN: c_ulong = 62; //云台右转和焦距变大(倍率变大)
pub const PAN_RIGHT_ZOOM_OUT: c_ulong = 63; //云台右转和焦距变小(倍率变小)
pub const UP_LEFT_ZOOM_IN: c_ulong = 64; //云台上仰和左转和焦距变大(倍率变大)
pub const UP_LEFT_ZOOM_OUT: c_ulong = 65; //云台上仰和左转和焦距变小(倍率变小)
pub const UP_RIGHT_ZOOM_IN: c_ulong = 66; //云台上仰和右转和焦距变大(倍率变大)
pub const UP_RIGHT_ZOOM_OUT: c_ulong = 67; //云台上仰和右转和焦距变小(倍率变小)
pub const DOWN_LEFT_ZOOM_IN: c_ulong = 68; //云台下俯和左转和焦距变大(倍率变大)
pub const DOWN_LEFT_ZOOM_OUT: c_ulong = 69; //云台下俯和左转和焦距变小(倍率变小)
pub const DOWN_RIGHT_ZOOM_IN: c_ulong = 70; //云台下俯和右转和焦距变大(倍率变大)
pub const DOWN_RIGHT_ZOOM_OUT: c_ulong = 71; //云台下俯和右转和焦距变小(倍率变小)
pub const TILT_UP_ZOOM_IN: c_ulong = 72; //云台上仰和焦距变大(倍率变大)
pub const TILT_UP_ZOOM_OUT: c_ulong = 73; //云台上仰和焦距变小(倍率变小)

// 码流回调数据类型
pub const NET_DVR_SYSHEAD: c_ulong = 1;
pub const NET_DVR_STREAMDATA: c_ulong = 2;
pub const NET_DVR_AUDIOSTREAMDATA: c_ulong = 3;
pub const NET_DVR_PRIVATE_DATA: c_ulong = 112;

pub const NET_DVR_DEV_ADDRESS_MAX_LEN: usize = 129;
pub const NET_DVR_LOGIN_USERNAME_MAX_LEN: usize = 64;
pub const NET_DVR_LOGIN_PASSWD_MAX_LEN: usize = 64;
pub const NET_SDK_MAX_FILE_PATH: usize = 256;
pub const SERIALNO_LEN: usize = 48;
pub const STREAM_ID_LEN: usize = 32;

/// 错误结构
#[repr(C)]
#[derive(Deserialize, Serialize, Debug, e_utils::C, e_utils::Json)]
pub struct NetLastError {
  pub code: c_long,
  #[serde(with = "e_utils::parse::c_str_pointer_serializer")]
  pub msg: *const c_char,
}
impl Default for NetLastError {
  fn default() -> Self {
    Self {
      code: 0,
      msg: "".to_c_char(),
    }
  }
}
/// 日志类型
#[repr(u8)]
#[derive(Debug)]
pub enum HcnetLevel {
  Off = 0,
  OnlyError = 1,
  DebugE = 2,
  InfoDE = 3,
}

// 设备参数结构体 V30
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LpnetDvrDeviceInfoV30 {
  #[serde(with = "crate::utils::c_uchar_48_ser")]
  pub sSerialNumber: [c_uchar; SERIALNO_LEN], // 序列号
  pub byAlarmInPortNum: c_uchar,     // 模拟报警输入个数
  pub byAlarmOutPortNum: c_uchar,    // 模拟报警输出个数
  pub byDiskNum: c_uchar,            // 硬盘个数
  pub byDVRType: c_uchar,            // 设备类型
  pub byChanNum: c_uchar, // 设备模拟通道个数，数字（IP）通道最大个数为byIPChanNum + by_high_dChanNum*256
  pub byStartChan: c_uchar, // 模拟通道的起始通道号，从1开始。数字通道的起始通道号见下面参数byStart_dChan
  pub byAudioChanNum: c_uchar, // 设备语音对讲通道数
  pub byIPChanNum: c_uchar, // 设备最大数字通道个数，低8位，高8位见by_high_dChanNum
  pub byZeroChanNum: c_uchar, // 零通道编码个数
  pub byMainProto: c_uchar, // 主码流传输协议类型：0- private，1- rtsp，2- 同时支持私有协议和rtsp协议取流（默认采用私有协议取流）
  pub bySubProto: c_uchar, // 子码流传输协议类型：0- private，1- rtsp，2- 同时支持私有协议和rtsp协议取流（默认采用私有协议取流）
  pub bySupport: c_uchar,  // 能力，位与结果为0表示不支持，1表示支持
  pub bySupport1: c_uchar, // 能力集扩充，位与结果为0表示不支持，1表示支持
  pub bySupport2: c_uchar, // 能力集扩充，位与结果为0表示不支持，1表示支持
  pub wDevType: c_ushort,  // 设备型号，详见下文列表
  pub bySupport3: c_uchar, // 能力集扩展，位与结果：0- 不支持，1- 支持
  pub byMultiStreamProto: c_uchar, // 是否支持多码流，按位表示，位与结果：0-不支持，1-支持
  pub byStartDChan: c_uchar, // 起始数字通道号，0表示无数字通道，比如DVR或IPC
  pub byStartDTalkChan: c_uchar, // 起始数字对讲通道号，区别于模拟对讲通道号，0表示无数字对讲通道
  pub byHighDChanNum: c_uchar, // 数字通道个数，高8位
  pub bySupport4: c_uchar, // 能力集扩展，按位表示，位与结果：0- 不支持，1- 支持
  pub byLanguageType: c_uchar, // 支持语种能力，按位表示，位与结果：0- 不支持，1- 支持
  pub byVoiceInChanNum: c_uchar, // 音频输入通道数
  pub byStartVoiceInChanNo: c_uchar, // 音频输入起始通道号，0表示无效
  // pub bySupport5: c_uchar, // 按位表示,0-不支持,1-支持,bit0-支持多码流
  // pub bySupport6: c_uchar, // 按位表示,0-不支持,1-支持
  // #[serde(with = "crate::utils::c_uchar_2_ser")]
  pub byRes3: [c_uchar; 2],
  pub byMirrorChanNum: c_uchar, // 镜像通道个数，录播主机中用于表示导播通
  pub wStartMirrorChanNo: c_ushort, // 起始镜像通道号
  // pub bySupport7: c_uchar,      // 能力,按位表示,0-不支持,1-支持
  // bySupport7 & 0x1  表示设备是否支持NET_VCA_RULECFG_V42扩展
  // bySupport7 & 0x2  表示设备是否支持IPC HVT 模式扩展
  // bySupport7 & 0x04 表示设备是否支持返回锁定时间
  // bySupport7 & 0x08 表示设置云台PTZ位置时，是否支持带通道号
  // bySupport7 & 0x10 表示设备是否支持双系统升级备份
  // bySupport7 & 0x20 表示设备是否支持OSD字符叠加V50
  // bySupport7 & 0x40 表示设备是否支持主从（从摄像机）
  // bySupport7 & 0x80 表示设备是否支持报文加密
  // #[serde(with = "crate::utils::c_uchar_2_ser")]
  pub byRes2: [c_uchar; 2], // 保留，置为0
}
impl Default for LpnetDvrDeviceInfoV30 {
  fn default() -> Self {
    Self {
      sSerialNumber: [0; SERIALNO_LEN],
      byAlarmInPortNum: 0,
      byAlarmOutPortNum: 0,
      byDiskNum: 0,
      byDVRType: 0,
      byChanNum: 0,
      byStartChan: 0,
      byAudioChanNum: 0,
      byIPChanNum: 0,
      byZeroChanNum: 0,
      byMainProto: 0,
      bySubProto: 0,
      bySupport: 0,
      bySupport1: 0,
      bySupport2: 0,
      wDevType: 0,
      bySupport3: 0,
      byMultiStreamProto: 0,
      byStartDChan: 0,
      byStartDTalkChan: 0,
      byHighDChanNum: 0,
      bySupport4: 0,
      byLanguageType: 0,
      byVoiceInChanNum: 0,
      byStartVoiceInChanNo: 0,
      byRes3: [0; 2],
      byMirrorChanNum: 0,
      byRes2: [0; 2],
      wStartMirrorChanNo: 0,
    }
  }
}
// 设备参数结构体 V40
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LpnetDvrDeviceInfoV40 {
  pub struDeviceV30: LpnetDvrDeviceInfoV30, // 设备信息
  pub bySupportLock: c_uchar, // 设备支持锁定功能，该字段由SDK根据设备返回值来赋值的。bySupportLock为1时，dwSurplusLockTime和byRetryLoginTime有效
  pub byRetryLoginTime: c_uchar, // 剩余可尝试登陆的次数，用户名，密码错误时，此参数有效
  pub byPasswordLevel: c_uchar, // admin密码安全等级
  pub byProxyType: c_uchar,   // 代理类型，0-不使用代理, 1-使用socks5代理, 2-使用EHome代理
  pub dwSurplusLockTime: c_uint, // 剩余时间，单位秒，用户锁定时，此参数有效
  pub byCharEncodeType: c_uchar, // 字符编码类型
  pub bySupportDev5: c_uchar, // 支持v50版本的设备参数获取，设备名称和设备类型名称长度扩展为64字节
  pub bySupport: c_uchar,     // 能力集扩展，位与结果：0- 不支持，1- 支持
  pub byLoginMode: c_uchar,   // 登录模式:0- Private登录，1- ISAPI登录
  pub byRes3: c_uint,         // 保留，置为0
  // dwOEMCode: c_ulong,     // OEM Code
  pub iResidualValidity: c_int, // 该用户密码剩余有效天数，单位：天，返回负值，表示密码已经超期使用，例如“-3表示密码已经超期使用3天”
  pub byResidualValidity: c_uchar, // iResidualValidity字段是否有效，0-无效，1-有效
  pub bySingleStartDTalkChan: c_uchar, // 独立音轨接入的设备，起始接入通道号，0-为保留字节，无实际含义，音轨通道号不能从0开始
  pub bySingleDTalkChanNums: c_uchar,  // 独立音轨接入的设备的通道总数，0-表示不支持
  // 1- 管理员创建一个非管理员用户为其设置密码，该非管理员用户正确登录设备后要提示“请修改初始登录密码”，未修改的情况下，用户每次登入都会进行提醒；
  // 2- 当非管理员用户的密码被管理员修改，该非管理员用户再次正确登录设备后，需要提示“请重新设置登录密码”，未修改的情况下，用户每次登入都会进行提醒。
  pub byPassWordResetLevel: c_uchar, // 0-无效，
  // bySupportStreamEncrypt & 0x1 表示是否支持RTP/TLS取流
  // bySupportStreamEncrypt & 0x2 表示是否支持SRTP/UDP取流
  // bySupportStreamEncrypt & 0x4 表示是否支持SRTP/MULTICAST取流
  pub bySupportStreamEncrypt: c_uchar, // 能力集扩展，位与结果：0- 不支持，1- 支持
  pub byMarketType: c_uchar,           // 0-无效（未知类型）,1-经销型，2-行业型
  #[serde(with = "crate::utils::c_uchar_238_ser")]
  pub byRes2: [c_uchar; 238], // 保留，置为0
}
impl Default for LpnetDvrDeviceInfoV40 {
  fn default() -> Self {
    LpnetDvrDeviceInfoV40 {
      struDeviceV30: LpnetDvrDeviceInfoV30::default(),
      bySupportLock: 0,
      byRetryLoginTime: 0,
      byPasswordLevel: 0,
      byProxyType: 0,
      dwSurplusLockTime: 0,
      byCharEncodeType: 0,
      bySupportDev5: 0,
      bySupport: 0,
      byLoginMode: 0,
      byRes3: 0,
      iResidualValidity: 0,
      byResidualValidity: 0,
      bySingleStartDTalkChan: 0,
      bySingleDTalkChanNums: 0,
      byPassWordResetLevel: 0,
      bySupportStreamEncrypt: 0,
      byMarketType: 0,
      byRes2: [0; 238],
    }
  }
}

// 异步登录回调函数
pub type FLoginResultCallBack = unsafe extern "C" fn(
  lUserID: c_long,
  dwResult: c_uint,
  lpDeviceInfo: LpnetDvrDeviceInfoV30,
  pUser: *mut c_void,
) -> *const c_void;

/// NET_DVR_Login_V40()参数
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetDvrUserLoginInfo {
  /// 设备地址，IP 或者普通域名
  #[serde(with = "crate::utils::c_uchar_129_ser")]
  pub sDeviceAddress: [c_uchar; NET_DVR_DEV_ADDRESS_MAX_LEN],
  /// 是否启用能力集透传：0- 不启用透传，默认；1- 启用透传
  pub byUseTransport: c_uchar,
  /// 设备端口号，例如：8000
  pub wPort: c_ushort,
  /// 登录用户名，例如：admin
  #[serde(with = "crate::utils::c_uchar_64_ser")]
  pub sUserName: [c_uchar; NET_DVR_LOGIN_USERNAME_MAX_LEN],
  /// 登录密码，例如：12345
  #[serde(with = "crate::utils::c_uchar_64_ser")]
  pub sPassword: [c_uchar; NET_DVR_LOGIN_PASSWD_MAX_LEN],
  /// 登录状态回调函数，bUseAsynLogin 为1时有效
  #[serde(with = "crate::core::net_sdk::function_login_callback_pointer_serializer")]
  pub cbLoginResult: FLoginResultCallBack,
  /// 用户数据
  #[serde(with = "e_utils::parse::raw_pointer_serializer")]
  pub pUser: *mut c_void,
  /// 是否异步登录：0- 否，1- 是
  pub bUseAsynLogin: bool,
  /// 0:不使用代理，1：使用标准代理，2：使用EHome代理
  pub byProxyType: c_uchar,
  /// 0-不进行转换，默认,1-接口上输入输出全部使用UTC时间,SDK完成UTC时间与设备时区的转换,
  /// 2-接口上输入输出全部使用平台本地时间，SDK完成平台本地时间与设备时区的转换
  pub byUseUTCTime: c_uchar,
  /// 0-Private 1-ISAPI 2-自适应
  pub byLoginMode: c_uchar,
  /// 0-不适用tls，1-使用tls 2-自适应
  pub byHttps: c_uchar,
  /// 代理服务器序号，添加代理服务器信息时，相对应的服务器数组下表值
  pub iProxyID: c_long,
  /// 认证方式，0-不认证，1-双向认证，2-单向认证；认证仅在使用TLS的时候生效;
  pub byVerifyMode: c_char,
  #[serde(with = "crate::utils::c_uchar_119_ser")]
  pub byRes3: [c_uchar; 119],
}
impl Default for NetDvrUserLoginInfo {
  fn default() -> Self {
    extern "C" fn loginResult_callback(
      _lUserID: c_long,
      _dwResult: c_uint,
      _lpDeviceInfo: LpnetDvrDeviceInfoV30,
      _pUser: *mut c_void,
    ) -> *const c_void {
      ptr::null()
    }
    NetDvrUserLoginInfo {
      sDeviceAddress: [0; NET_DVR_DEV_ADDRESS_MAX_LEN],
      byUseTransport: 0,
      wPort: 0,
      sUserName: [0; NET_DVR_LOGIN_USERNAME_MAX_LEN],
      sPassword: [0; NET_DVR_LOGIN_PASSWD_MAX_LEN],
      cbLoginResult: loginResult_callback,
      pUser: ptr::null_mut(),
      bUseAsynLogin: false,
      byProxyType: 0,
      byUseUTCTime: 0,
      byLoginMode: 0,
      byHttps: 0,
      iProxyID: 0,
      byVerifyMode: 0,
      byRes3: [0; 119],
    }
  }
}

/// 定义NET_SDK_INIT_CFG_TYPE的枚举
#[repr(C)]
#[derive(Debug, e_utils::C)]
pub enum NetSdkInitCfgType {
  /// SDK支持的业务能力
  NetSdkInitCfgAbility(NetDvrInitCfgAbility),
  /// 设置组件库加载路径，即HCNetSDK.dll或HCNetSDK.so的路径。sPath可以为绝对路基或相对路径。若为相对路径，相对路径+应用程序可执行路径不超过256。
  NetSdkInitCfgSdkPath(NetDvrLocalSdkPath),
  /// 带有OpenSSL加密库库名的路径，支持1.0.2和1.1.1版本，SDK内部加载库后，自动获取openssl版本
  NetSdkInitCfgLibeayPath(*const c_char),
  /// 带有OpenSSL通信库库名的路径，支持1.0.2和1.1.1版本，SDK内部加载库后，自动获取openssl版本
  NetSdkInitCfgSsleayPath(*const c_char),
}
impl From<NetSdkInitCfgType> for c_uint {
  fn from(value: NetSdkInitCfgType) -> Self {
    (&value).into()
  }
}
impl From<&NetSdkInitCfgType> for c_uint {
  fn from(value: &NetSdkInitCfgType) -> Self {
    match value {
      NetSdkInitCfgType::NetSdkInitCfgAbility(_) => 1,
      NetSdkInitCfgType::NetSdkInitCfgSdkPath(_) => 2,
      NetSdkInitCfgType::NetSdkInitCfgLibeayPath(_) => 3,
      NetSdkInitCfgType::NetSdkInitCfgSsleayPath(_) => 4,
    }
  }
}

/// 最大限制
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InitCfgMaxNum {
  InitCfgNum2048 = 2048,
  InitCfgNum5120 = 5120,
  InitCfgNum10240 = 10240,
  InitCfgNum15360 = 15360,
  InitCfgNum20480 = 20480,
}

// 组件库加载路径信息
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetDvrInitCfgAbility {
  // 最大注册用户个数，具体定义如下所示：
  pub enumMaxLoginUsersNum: InitCfgMaxNum,
  /// 最大报警布防路数，具体定义如下所示：
  pub enumMaxAlarmNum: InitCfgMaxNum,
}

// 组件库加载路径信息
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetDvrLocalSdkPath {
  /// 日志路径 // 组件库地址
  #[serde(with = "crate::utils::c_uchar_256_ser")]
  pub sPath: [c_uchar; NET_SDK_MAX_FILE_PATH],
  /// 保留字段
  #[serde(with = "crate::utils::c_uchar_128_ser")]
  pub byRes: [c_uchar; 128],
}
impl NetDvrLocalSdkPath {
  ///
  pub unsafe extern "C" fn new<S: AsRef<str>>(fpath: S) -> CResult<Self> {
    let cstr: CString = res_c!(fpath.as_ref().to_cstring().res_c());
    let sPath = cstring_to_array!(c_uchar, 256, cstr);
    // Initialize byRes with zeros
    let byRes = empty_to_array!(c_uchar, 128);

    CResult::Ok(NetDvrLocalSdkPath { sPath, byRes })
  }
}

// 定义预览参数结构体
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetDvrPreviewInfo {
  /// 通道号
  pub lCannel: c_long,
  /// 码流类型，0-主码流，1-子码流，2-码流3，3-码流4, 4-码流5,5-码流6,7-码流7,8-码流8,9-码流9,10-码流10
  pub dwStreamType: c_ulong,
  /// 0：TCP方式,1：UDP方式,2：多播方式,3 - RTP方式，4-RTP/RTSP,5-RSTP/HTTP ,6- HRUDP（可靠传输） ,7-RTSP/HTTPS
  pub dwLinkMode: c_ulong,
  /// 播放窗口的句柄,为NULL表示不播放图象
  // #[serde(with = "e_utils::parse::hwnd_serializer")]
  // pub hPlayWnd: e_utils::ui::HWND,
  #[serde(with = "e_utils::parse::raw_pointer_serializer")]
  pub hPlayWnd: *mut c_void,
  /// 0-非阻塞取流, 1-阻塞取流, 如果阻塞SDK内部connect失败将会有5s的超时才能够返回,不适合于轮询取流操作
  pub bBlocked: bool,
  /// 0-不启用录像回传,1启用录像回传
  pub bPassbackRecord: bool,
  /// 预览模式，0-正常预览，1-延迟预览
  pub byPreviewMode: c_uchar,
  /// 流ID，lChannel为0xffffffff时启用此参数
  pub byStreamID: [c_uchar; STREAM_ID_LEN],
  /// 应用层取流协议，0-私有协议，1-RTSP协议,
  pub byProtoType: c_uchar,
  /// 2-SRTP码流加密（对应此结构体中dwLinkMode 字段，支持如下方式,
  /// 为1，表示udp传输方式，信令走TLS加密，码流走SRTP加密，为2，表示多播传输方式，信令走TLS加密，码流走SRTP加密）
  pub byRes1: c_uchar,
  /// 码流数据编解码类型 0-通用编码数据 1-热成像探测器产生的原始数据
  pub byVideoCodingType: c_uchar,
  /// 播放库播放缓冲区最大缓冲帧数，范围1-50，置0时默认为1
  pub dwDisplayBufNum: c_ulong,
  /// NPQ是直连模式，还是过流媒体：0-直连 1-过流媒体
  pub byNPQMode: c_uchar,
  /// 是否接收metadata数据
  pub byRecvMetaData: c_uchar,
  /// 设备是否支持该功能通过GET /ISAPI/System/capabilities 中DeviceCap.SysCap.isSupportMetadata是否存在且为true
  /// 数据类型，0-码流数据，1-音频数据
  pub byDataType: c_uchar,
  /// 结果
  #[serde(with = "crate::utils::c_uchar_213_ser")]
  #[serde(rename = "byRes")]
  pub byRes: [c_uchar; 213],
}
impl Default for NetDvrPreviewInfo {
  fn default() -> Self {
    NetDvrPreviewInfo {
      lCannel: 1,
      dwStreamType: 0,
      dwLinkMode: 0,
      hPlayWnd: std::ptr::null_mut(),
      bBlocked: true,
      bPassbackRecord: false,
      byPreviewMode: 0,
      byStreamID: [0; STREAM_ID_LEN],
      byProtoType: 0,
      byRes1: 0,
      byVideoCodingType: 0,
      dwDisplayBufNum: 0,
      byNPQMode: 0,
      byRecvMetaData: 0,
      byDataType: 0,
      byRes: [0; 213],
    }
  }
}
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
// 定义JPEG图像信息结构体
pub struct NetDvrJpegPara {
  pub wPicSize: c_ushort,
  pub wPicQuality: c_ushort,
}
// 叠加字符
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetDvrShowStringInfo {
  pub w_show_string: c_ushort,
  pub w_stringSize: c_ushort,
  pub w_show_string_top_left_x: c_ushort,
  pub w_show_string_top_left_y: c_ushort,
  #[serde(with = "crate::utils::c_uchar_44_ser")]
  pub s_string: [c_uchar; 44],
}

// 叠加字符
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetDvrShowStringV30 {
  pub dwSize: c_ulong,
  pub stru_stringInfo: [NetDvrShowStringInfo; 8],
}

// 透传接口输出参数结构体
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetDvrXmlConfigOutput {
  pub dwSize: c_ulong,
  #[serde(with = "e_utils::parse::raw_pointer_serializer")]
  pub lpOutBuffer: *mut c_void,
  pub dwOutBufferSize: c_ulong,
  pub dwReturned_xmlSize: c_ulong,
  #[serde(with = "e_utils::parse::raw_pointer_serializer")]
  pub lpStatusBuffer: *mut c_void,
  pub dwStatusSize: c_ulong,
  #[serde(with = "crate::utils::c_uchar_32_ser")]
  pub byRes: [c_uchar; 32],
}

// 透传接口输入参数结构体
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetDvrXmlConfigInput {
  pub dwSize: c_ulong,
  #[serde(with = "e_utils::parse::raw_pointer_serializer")]
  pub lpRequestURL: *mut c_void,
  pub dwRequestURLLen: c_ulong,
  #[serde(with = "e_utils::parse::raw_pointer_serializer")]
  pub lpInBuffer: *mut c_void,
  pub dwInBufferSize: c_ulong,
  pub dwRecvTimeOut: c_ulong,
  pub byForceEncrpt: c_uchar,
  pub byNumOfMultiPart: c_uchar,
  #[serde(with = "crate::utils::c_uchar_30_ser")]
  pub byRes: [c_uchar; 30],
}

// 报警设备信息结构体
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetDvrAlarmer {
  pub byUserIDValid: c_schar,     // UserID是否有效 0-无效，1-有效
  pub bySerialValid: c_schar,     // 序列号是否有效 0-无效，1-有效
  pub byVersionValid: c_schar,    // 版本号是否有效 0-无效，1-有效
  pub byDeviceNameValid: c_schar, // 设备名字是否有效 0-无效，1-有效
  pub byMacAddrValid: c_schar,    // MAC地址是否有效 0-无效，1-有效
  pub byLinkPortValid: c_schar,   // login端口是否有效 0-无效，1-有效
  pub byDeviceIPValid: c_schar,   // 设备IP是否有效 0-无效，1-有效
  pub bySocketIPValid: c_schar,   // socket ip是否有效 0-无效，1-有效
  pub lUserID: c_ulong,           // NET_DVR_Login()返回值, 布防时有效
  #[serde(with = "crate::utils::c_uchar_48_ser")]
  pub sSerialNumber: [c_uchar; 48], // 序列号
  pub dwDeviceVersion: c_ulong,   // 版本信息 高16位表示主版本，低16位表示次版本
  #[serde(with = "crate::utils::c_uchar_32_ser")]
  pub sDeviceName: [c_uchar; 32], // 设备名字
  #[serde(with = "crate::utils::c_uchar_6_ser")]
  pub byMacAddr: [c_uchar; 6], // MAC地址
  pub wLinkPort: c_ushort,        // link port
  #[serde(with = "crate::utils::c_uchar_128_ser")]
  pub sDeviceIP: [c_uchar; 128], // IP地址
  #[serde(with = "crate::utils::c_uchar_128_ser")]
  pub sSocketIP: [c_uchar; 128], // 报警主动上传时的socket IP地址
  pub byIPProtocol: c_schar,      // Ip协议 0-IPV4, 1-IPV6
  #[serde(with = "crate::utils::c_uchar_11_ser")]
  pub byRes2: [c_uchar; 11],
}

// 报警布防参数结构体
#[repr(C)]
pub struct NetDvrSetupAlarmParam {
  pub dwSize: c_ulong,            // 结构体大小
  pub byLevel: c_schar,           // 布防优先级：0- 一等级（高），1- 二等级（中），2- 三等级（低）
  pub byAlarmInfoType: c_schar, // 上传报警信息类型（抓拍机支持），0-老报警信息（NET_DVR_PLATE_RESULT），1-新报警信息(NET_ITS_PLATE_RESULT)2012-9-28
  pub byRetAlarmTypeV40: c_schar, // 0- 返回NET_DVR_ALARMINFO_V30或NET_DVR_ALARMINFO,
  // 1- 设备支持NET_DVR_ALARMINFO_V40则返回NET_DVR_ALARMINFO_V40，不支持则返回NET_DVR_ALARMINFO_V30或NET_DVR_ALARMINFO
  pub byRetDevInfoVersion: c_schar, // CVR上传报警信息回调结构体版本号 0-COMM_ALARM_DEVICE， 1-COMM_ALARM_DEVICE_V40
  pub byRet_vqdAlarmType: c_schar, // VQD报警上传类型，0-上传报报警NET_DVR_VQD_DIAGNOSE_INFO，1-上传报警NET_DVR_VQD_ALARM
  pub byFaceAlarmDetection: c_schar,
  pub bySupport: c_schar,
  pub byBrokenNetHttp: c_schar,
  pub wTaskNo: c_ushort, // 任务处理号 和 (上传数据NET_DVR_VEHICLE_RECOG_RESULT中的字段dwTaskNo对应 同时 下发任务结构 NET_DVR_VEHICLE_RECOG_COND中的字段dwTaskNo对应)
  pub byDeployType: c_schar, // 布防类型：0-客户端布防，1-实时布防
  pub byRes1: [c_uchar; 3],
  pub byAlarmTypeURL: c_schar, // bit0-表示人脸抓拍报警上传
  // 0- 表示二进制传输，1- 表示URL传输（设备支持的情况下，设备支持能力根据具体报警能力集判断,同时设备需要支持URL的相关服务，当前是”云存储“）
  pub byCustomCtrl: c_schar, // Bit0- 表示支持副驾驶人脸子图上传: 0-不上传,1-上传 (注：只在公司内部8600/8200等平台开放)
}

// 上传的报警信息结构体。
#[repr(C)]
pub struct NetDvrAlarmInfoV30 {
  pub dwAlarmType: c_ulong,                 // 报警类型
  pub dwAlarmInputNumber: c_ulong,          // 报警输入端口，当报警类型为0、23时有效
  pub byAlarmOutputNumber: [c_uchar; 96], // 触发的报警输出端口，值为1表示该报警端口输出，如byAlarmOutputNumber[0]=1表示触发第1个报警输出口输出，依次类推
  pub byAlarm_relateChannel: [c_uchar; 64], // 触发的录像通道，值为1表示该通道录像，如byAlarm_relateChannel[0]=1表示触发第1个通道录像
  pub byChannel: [c_uchar; 64], // 发生报警的通道。当报警类型为2、3、6、9、10、11、13、15、16时有效，如byChannel[0]=1表示第1个通道报警
  pub byDiskNumber: [c_uchar; 33], // 发生报警的硬盘。当报警类型为1，4，5时有效，by_diskNumber[0]=1表示1号硬盘异常
}

// 时间参数结构体
#[repr(C)]
pub struct NetDvrTime {
  pub dwYear: c_ulong,   // 年
  pub dwMonth: c_ulong,  // 月
  pub dwDay: c_ulong,    // 日
  pub dwHour: c_ulong,   // 时
  pub dwMinute: c_ulong, // 分
  pub dwSecond: c_ulong, // 秒
}

// IP地址结构体
#[repr(C)]
pub struct NetDvrIpAddr {
  pub sIPV4: [c_uchar; 16],  // 设备IPv4地址
  pub sIPV6: [c_uchar; 128], // 设备IPv6地址
}

// 门禁主机事件信息
#[repr(C)]
pub struct NetDvrAcsEventInfo {
  pub dwSize: c_ulong,              // 结构体大小
  pub byCardNo: [c_uchar; 32],      // 卡号
  pub byCardType: c_schar, // 卡类型：1- 普通卡，3- 非授权名单卡，4- 巡更卡，5- 胁迫卡，6- 超级卡，7- 来宾卡，8- 解除卡，为0表示无效
  pub byAllowListNo: c_schar, // 授权名单单号，取值范围：1~8，0表示无效
  pub byReportChannel: c_schar, // 报告上传通道：1- 布防上传，2- 中心组1上传，3- 中心组2上传，0表示无效
  pub byCardReaderKind: c_schar, // 读卡器类型：0- 无效，1- IC读卡器，2- 身份证读卡器，3- 二维码读卡器，4- 指纹头
  pub dwCardReaderNo: c_ulong,   // 读卡器编号，为0表示无效
  pub dwDoorNo: c_ulong, // 门编号（或者梯控的楼层编号），为0表示无效（当接的设备为人员通道设备时，门1为进方向，门2为出方向）
  pub dwVerifyNo: c_ulong, // 多重卡认证序号，为0表示无效
  pub dwAlarmInNo: c_ulong, // 报警输入号，为0表示无效
  pub dwAlarmOutNo: c_ulong, // 报警输出号，为0表示无效
  pub dwCaseSensorNo: c_ulong, // 事件触发器编号
  pub dwRs485No: c_ulong, // RS485通道号，为0表示无效
  pub dwMultiCardGroupNo: c_ulong, // 群组编号
  pub wAccessChannel: c_ushort, // 人员通道号
  pub byDeviceNo: c_schar, // 设备编号，为0表示无效
  pub byDistractControlNo: c_schar, // 分控器编号，为0表示无效
  pub dwEmployeeNo: c_ulong, // 工号，为0无效
  pub wLocalControllerID: c_ushort, // 就地控制器编号，0-门禁主机，1-255代表就地控制器
  pub byInternetAccess: c_schar, // 网口ID：（1-上行网口1,2-上行网口2,3-下行网口1）
  pub byType: c_schar, // 防区类型，0:即时防区,1-24小时防区,2-延时防区,3-内部防区,4-钥匙防区,5-火警防区,6-周界防区,7-24小时无声防区,
  // 8-24小时辅助防区,9-24小时震动防区,10-门禁紧急开门防区,11-门禁紧急关门防区，0xff-无
  pub byMacAddr: [c_uchar; 6],                 // 物理地址，为0无效
  pub bySwipeCardType: c_schar,                // 刷卡类型，0-无效，1-二维码
  pub byMask: c_schar,                         // 是否带口罩：0-保留，1-未知，2-不戴口罩，3-戴口罩
  pub dwSerialNo: c_ulong,                     // 事件流水号，为0无效
  pub byChannelControllerID: c_schar, // 通道控制器ID，为0无效，1-主通道控制器，2-从通道控制器
  pub byChannelControllerLampID: c_schar, // 通道控制器灯板ID，为0无效（有效范围1-255）
  pub byChannelControllerIRAdaptorID: c_schar, // 通道控制器红外转接板ID，为0无效（有效范围1-255）
  pub byChannelControllerIREmitterID: c_schar, // 通道控制器红外对射ID，为0无效（有效范围1-255）
  pub byHelmet: c_schar,              // 可选，是否戴安全帽：0-保留，1-未知，2-不戴安全, 3-戴安全帽
  pub byRes: [c_uchar; 3],            // 保留，置为0
}

// 门禁主机报警信息结构体
#[repr(C)]
pub struct NetDvrAcsAlarmInfo {
  pub dwSize: c_ulong,                      // 结构体大小
  pub dwMajor: c_ulong,                     // 报警主类型，具体定义见“Remarks”说明
  pub dwMinor: c_ulong, // 报警次类型，次类型含义根据主类型不同而不同，具体定义见“Remarks”说明
  pub struTime: NetDvrTime, // 报警时间
  pub sNetUser: [c_uchar; 16], // 网络操作的用户名
  pub struRemoteHostAddr: NetDvrIpAddr, // 远程主机地址
  pub struAcsEventInfo: NetDvrAcsEventInfo, // 报警信息详细参数
  pub dwPicDataLen: c_ulong, // 图片数据大小，不为0是表示后面带数据
  pub pPicData: *mut c_void, // 图片数据缓冲区
  pub wInductiveEventType: c_ushort, // 归纳事件类型，0-无效，客户端判断该值为非0值后，报警类型通过归纳事件类型区分，否则通过原有报警主次类型（dwMajor、dwMinor）区分
  pub byPicTransType: c_schar,       // 图片数据传输方式: 0-二进制；1-url
  pub byRes1: c_schar,               // 保留，置为0
  pub dwIOTChannelNo: c_ulong,       // IOT通道号
  pub pAcsEventInfoExtend: *mut c_void, // byAcsEventInfoExtend为1时，表示指向一个NET_DVR_ACS_EVENT_INFO_EXTEND结构体
  pub byAcsEventInfoExtend: c_schar,    // pAcsEventInfoExtend是否有效：0-无效，1-有效
  pub byTimeType: c_schar,              // 时间类型：0-设备本地时间，1-UTC时间（struTime的时间）
  pub byRes2: c_schar,                  // 保留，置为0
  pub byAcsEventInfoExtendV20: c_schar, // pAcsEventInfoExtendV20是否有效：0-无效，1-有效
  pub pAcsEventInfoExtendV20: *mut c_void, // byAcsEventInfoExtendV20为1时，表示指向一个NET_DVR_ACS_EVENT_INFO_EXTEND_V20结构体
  pub byRes: [c_uchar; 4],                 // 保留，置为0
}
pub type LPNetDvrAcsAlarmInfo = *mut NetDvrAcsAlarmInfo;

// 点坐标参数结构体
#[repr(C)]
pub struct NetVcaPoint {
  pub fX: c_float,
  pub fY: c_float,
}

// 身份证刷卡信息扩展参数
#[repr(C)]
pub struct NetDvrIdCardInfoExtend {
  pub byRemoteCheck: c_uchar,
  pub byThermometryUnit: c_uchar,
  pub byIsAbnomalTemperature: c_uchar,
  pub byRes2: c_uchar,
  pub fCurrTemperature: c_float,
  pub struRegionCoordinates: NetVcaPoint,
  pub dwQRCodeInfoLen: c_ulong,
  pub dwVisibleLightDataLen: c_ulong,
  pub dwThermalDataLen: c_ulong,
  pub pQRCodeInfo: *mut c_schar,
  pub pVisibleLightData: *mut c_schar,
  pub pThermalData: *mut c_schar,
  pub wXCoordinate: c_ushort,
  pub wYCoordinate: c_ushort,
  pub wWidth: c_ushort,
  pub wHeight: c_ushort,
  pub byHealthCode: c_uchar,
  pub byNADCode: c_uchar,
  pub byTravelCode: c_uchar,
  pub byVaccineStatus: c_uchar,
  pub byRes: [c_uchar; 1024],
}

// 日期信息结构体
#[repr(C)]
pub struct NetDvrDate {
  pub wYear: c_ushort,  // 年
  pub byMonth: c_uchar, // 月
  pub byDay: c_uchar,   // 日
}

// 身份证信息结构体
#[repr(C)]
pub struct NetDvrIdCardInfo {
  pub dwSize: c_ulong,                    // 结构体大小
  pub byName: [c_uchar; 128],             // 姓名
  pub struBirth: NetDvrDate,              // 出生日期
  pub byAddr: [c_uchar; 280],             // 住址
  pub byIDNum: [c_uchar; 32],             // 身份证号码
  pub byIssuingAuthority: [c_uchar; 128], // 签发机关
  pub struStartDate: NetDvrDate,          // 有效开始日期
  pub struEndDate: NetDvrDate,            // 有效结束日期
  pub byTermOfValidity: c_uchar,          // 有效期限
  pub bySex: c_uchar,                     // 性别
  pub byNation: c_uchar,                  // 民族
  pub byRes: [c_uchar; 101],              // 保留，置为0
}

// 时间参数结构体
#[repr(C)]
pub struct NetDvrTimeV30 {
  pub wYear: c_ushort,           // 年
  pub byMonth: c_uchar,          // 月
  pub byDay: c_uchar,            // 日
  pub byHour: c_uchar,           // 时
  pub byMinute: c_uchar,         // 分
  pub bySecond: c_uchar,         // 秒
  pub byISO8601: c_uchar,        // ISO8601标准
  pub wMilliSec: c_ushort,       // 毫秒
  pub cTimeDifferenceH: c_uchar, // 时区差，小时
  pub cTimeDifferenceM: c_uchar, // 时区差，分钟
}

// 身份证刷卡信息上传结构体
#[repr(C)]
pub struct NetDvrIdCardInfoAlarm {
  pub dwSize: c_ulong,                                // 结构长度
  pub struIDCardCfg: NetDvrIdCardInfo,                // 身份证信息
  pub dwMajor: c_ulong,                               // 报警主类型
  pub dwMinor: c_ulong,                               // 报警次类型
  pub struSwipeTime: NetDvrTimeV30,                   // 刷卡时间
  pub byNetUser: [c_uchar; 16],                       // 网络操作的用户名
  pub struRemoteHostAddr: NetDvrIpAddr,               // 远程主机地址
  pub dwCardReaderNo: c_ulong,                        // 读卡器编号
  pub dwDoorNo: c_ulong,                              // 门编号
  pub dwPicDataLen: c_ulong,                          // 图片数据大小
  pub pPicData: *mut c_void,                          // 身份证图片数据缓冲区
  pub byCardType: c_schar,                            // 卡类型
  pub byDeviceNo: c_schar,                            // 设备编号
  pub byMask: c_schar,                                // 是否带口罩
  pub byRes2: c_schar,                                // 保留，置为0
  pub dwFingerPrintDataLen: c_ulong,                  // 指纹数据大小
  pub pFingerPrintData: *mut c_void,                  // 指纹数据缓冲区
  pub dwCapturePicDataLen: c_ulong,                   // 抓拍图片数据大小
  pub pCapturePicData: *mut c_void,                   // 抓拍图片数据缓冲区
  pub dwCertificatePicDataLen: c_ulong,               // 证件抓拍图片数据大小
  pub pCertificatePicData: *mut c_void,               // 证件抓拍图片数据缓冲区
  pub byCardReaderKind: c_schar,                      // 读卡器属于哪一类
  pub byRes3: [c_uchar; 2],                           // 保留，置为0
  pub byIDCardInfoExtend: c_schar,                    // 身份证刷卡扩展事件信息是否有效
  pub pIDCardInfoExtend: *mut NetDvrIdCardInfoExtend, // 身份证刷卡扩展事件信息
  pub byRes: [c_uchar; 172],                          // 保留，置为0
}

// 图片数据结构体
#[repr(C)]
pub struct NetDvrAlarmIsapiPicdata {
  pub dwPicLen: c_ulong,          // 图片数据长度
  pub byPicType: c_schar,         // 图片格式: 1- jpg
  pub byRes: [c_uchar; 3],        // 保留字节
  pub szFilename: [c_uchar; 256], // 图片名称
  pub pPicData: *mut c_void,      // 图片数据
}

pub type LpnetDvrAlarmIsapiPicdata = *mut NetDvrAlarmIsapiPicdata;

// 报警信息结构体
#[repr(C)]
pub struct NetDvrAlarmIsapiInfo {
  pub pAlarmData: *mut c_void,   // 报警数据
  pub dwAlarmDataLen: c_ulong,   // 报警数据长度
  pub byDataType: c_schar,       // 0-invalid,1-xml,2-json
  pub byPicturesNumber: c_schar, // 图片数量
  pub byRes: [c_uchar; 2],       // 保留字节
  pub pPicPackData: *mut c_void, // 图片变长部分
  pub byRes1: [c_uchar; 32],     // 保留字节
}

pub type LpnetDvrAlarmIsapiInfo = *mut NetDvrAlarmIsapiInfo;

// 本地通用配置结构体
#[repr(C)]
pub struct NetDvrLocalGeneralCfg {
  pub byExceptionCbDirectly: c_uchar, // 0-通过线程池异常回调，1-直接异常回调给上层
  pub byNotSplitRecordFile: c_uchar,  // 回放和预览中保存到本地录像文件不切片 0-默认切片，1-不切片
  pub byResumeUpgradeEnable: c_uchar, // 断网续传升级使能，0-关闭（默认），1-开启
  pub byAlarmJsonPictureSeparate: c_uchar, // 控制 JSON 透传报警数据和图片是否分离，0-不分离，1-分离（分离后走 COMM_ISAPI_ALARM 回调返回）
  pub byRes: [c_uchar; 4],                 // 保留
  pub i64FileSize: c_ulonglong,            // 单位：Byte
  pub dwResumeUpgradeTimeout: c_ulong,     // 断网续传重连超时时间，单位毫秒
  pub byRes1: [c_ushort; 236],             // 保留
}

/// 码流回调函数
pub type RealDataCallback = extern "C" fn(
  lPlayHandle: c_long,
  dwDataType: c_uint,
  pBuffer: *const c_uchar,
  dwBufSize: c_uint,
  pUser: *mut c_void,
) -> *const c_void;

/// 抓图模式
#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TagPdcParamKey {
  /// BMP模式
  BmpMode = 0,
  /// JPEG模式
  JpegMode = 1,
}

/// JPEG图像信息结构体。
/// ```
///Members
/// wPicSize
/// 图片尺寸：0-CIF(352*288/352*240)，1-QCIF(176*144/176*120)，2-4CIF(704*576/704*480)或D1(720*576/720*486)，3-UXGA(1600*1200)， 4-SVGA(800*600)，5-HD720P(1280*720)，6-VGA(640*480)，7-XVGA(1280*960)，8-HD900P(1600*900)，9-HD1080P(1920*1080)，10-2560*1920， 11-1600*304，12-2048*1536，13-2448*2048，14-2448*1200，15-2448*800，16-XGA(1024*768)，17-SXGA(1280*1024)，18-WD1(960*576/960*480), 19-1080I(1920*1080)，20-576*576，21-1536*1536，22-1920*1920，23-320*240，24-720*720，25-1024*768，26-1280*1280，27-1600*600， 28-2048*768，29-160*120，75-336*256，78-384*256，79-384*216，80-320*256，82-320*192，83-512*384，127-480*272，128-512*272， 161-288*320，162-144*176，163-480*640，164-240*320，165-120*160，166-576*720，167-720*1280，168-576*960，180-180*240, 181-360*480, 182-540*720, 183-720*960, 184-960*1280, 185-1080*1440, 500-384*288, 0xff-Auto(使用当前码流分辨率)
/// wPicQuality
/// 图片质量系数：0-最好，1-较好，2-一般
/// ```
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LpNetDvrJpegpara {
  #[serde(rename = "wPicSize")]
  /// 图片尺寸：0-CIF(352*288/352*240)，1-QCIF(176*144/176*120)，2-4CIF(704*576/704*480)或D1(720*576/720*486)，3-UXGA(1600*1200)， 4-SVGA(800*600)，5-HD720P(1280*720)，6-VGA(640*480)，7-XVGA(1280*960)，8-HD900P(1600*900)，9-HD1080P(1920*1080)，10-2560*1920， 11-1600*304，12-2048*1536，13-2448*2048，14-2448*1200，15-2448*800，16-XGA(1024*768)，17-SXGA(1280*1024)，18-WD1(960*576/960*480), 19-1080I(1920*1080)，20-576*576，21-1536*1536，22-1920*1920，23-320*240，24-720*720，25-1024*768，26-1280*1280，27-1600*600， 28-2048*768，29-160*120，75-336*256，78-384*256，79-384*216，80-320*256，82-320*192，83-512*384，127-480*272，128-512*272， 161-288*320，162-144*176，163-480*640，164-240*320，165-120*160，166-576*720，167-720*1280，168-576*960，180-180*240, 181-360*480, 182-540*720, 183-720*960, 184-960*1280, 185-1080*1440, 500-384*288, 0xff-Auto(使用当前码流分辨率)
  pub wPicSize: c_ushort,
  /// 图片质量系数：0-最好，1-较好，2-一般
  #[serde(rename = "wPicQuality")]
  pub wPicQuality: c_ushort,
}
