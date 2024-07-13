#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::mvs_sdk::constant::*;
use serde::{Deserialize, Serialize};
use std::{ffi::*, mem::ManuallyDrop, ptr};

pub type Int8T = i8;
pub type Int16T = i16;
pub type Int32T = i32;
pub type Int64T = i64;
pub type UInt8T = u8;
pub type UInt16T = u16;
pub type UInt32T = u32;
pub type UInt64T = u64;
pub type IntLeast8T = i8;
pub type IntLeast16T = i16;
pub type IntLeast32T = i32;
pub type IntLeast64T = i64;
pub type UIntLeast8T = u8;
pub type UIntLeast16T = u16;
pub type UIntLeast32T = u32;
pub type UIntLeast64T = u64;
pub type IntFast8T = i8;
pub type IntFast16T = i64;
pub type IntFast32T = i64;
pub type IntFast64T = i64;
pub type UIntFast8T = u8;
pub type UIntFast16T = u64;
pub type UIntFast32T = u64;
pub type UIntFast64T = u64;
pub type IntptrT = isize;
pub type UIntptrT = usize;
pub type IntmaxT = i64;
pub type UIntmaxT = u64;

/// GigE设备信息
#[repr(C)]
#[derive(Debug)]
pub struct MvGigeDeviceInfo {
  /// IP配置选项
  pub nIpCfgOption: c_uint,
  /// 当前IP地址配置: bit31-static bit30-dhcp bit29-lla
  pub nIpCfgCurrent: c_uint,
  /// 当前主机IP地址
  pub nCurrentIp: c_uint,
  /// 当前子网掩码
  pub nCurrentSubNetMask: c_uint,
  /// 默认网关
  pub nDefaultGateWay: c_uint,
  /// 厂商名称
  pub chManufacturerName: [c_uchar; 32],
  /// 型号名称
  pub chModelName: [c_uchar; 32],
  /// 设备固件版本
  pub chDeviceVersion: [c_uchar; 32],
  /// 厂商特殊信息
  pub chManufacturerSpecificInfo: [c_uchar; 48],
  /// 序列号
  pub chSerialNumber: [c_uchar; 16],
  /// 用户定义名称
  pub chUserDefinedName: [c_uchar; 16],
  /// 网口Ip地址
  pub nNetExport: c_uint,
  /// 保留字节
  pub nReserved: [c_uint; 4],
}

/// USB设备信息
#[repr(C)]
#[derive(Debug)]
pub struct MvUsb3DeviceInfo {
  /// 控制输入端点
  pub CtrlInEndPoint: c_uchar,
  /// 控制输出端点
  pub CtrlOutEndPoint: c_uchar,
  /// 流端点
  pub StreamEndPoint: c_uchar,
  /// 事件端点
  pub EventEndPoint: c_uchar,
  /// 供应商ID号
  pub idVendor: c_ushort,
  /// 产品ID号
  pub idProduct: c_ushort,
  /// 设备序列号
  pub nDeviceNumber: c_uint,
  /// 设备GUID号
  pub chDeviceGUID: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 供应商名字
  pub chVendorName: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 型号名字
  pub chModelName: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 家族名字
  pub chFamilyName: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 设备版本号
  pub chDeviceVersion: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 制造商名字
  pub chManufacturerName: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 序列号
  pub chSerialNumber: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 用户自定义名字
  pub chUserDefinedName: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 支持的USB协议
  pub nbcdUSB: c_uint,
  /// 设备地址
  pub nDeviceAddress: c_uint,
  /// 保留字节
  pub nReserved: [c_uint; 2],
}

/// CameraLink设备信息
#[repr(C)]
#[derive(Debug)]
pub struct MvCamLDeviceInfo {
  /// 端口号
  pub chPortID: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 设备型号
  pub chModelName: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 家族名字
  pub chFamilyName: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 设备版本号
  pub chDeviceVersion: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 制造商名字
  pub chManufacturerName: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 序列号
  pub chSerialNumber: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 保留字节
  pub nReserved: [c_uint; 38],
}

/// 不同设备特有信息
#[repr(C)]
pub union N19MvCCDeviceInfo3Dot0E {
  pub stGigEInfo: ManuallyDrop<MvGigeDeviceInfo>,
  pub stUsb3VInfo: ManuallyDrop<MvUsb3DeviceInfo>,
  pub stCamLInfo: ManuallyDrop<MvCamLDeviceInfo>,
}

/// 设备信息    \~english Device info
#[repr(C)]
pub struct MvCcDeviceInfo {
  /// 规范的主要版本
  pub nMajorVer: c_ushort,
  /// 规范的次要版本
  pub nMinorVer: c_ushort,
  /// MAC地址高位
  pub nMacAddrHigh: c_uint,
  /// MAC地址低位
  pub nMacAddrLow: c_uint,
  /// 设备传输层协议类型
  pub nTLayerType: c_uint,
  /// 保留字节
  pub nReserved: [c_uint; 4],
  /// 不同设备特有信息
  pub SpecialInfo: N19MvCCDeviceInfo3Dot0E,
}

/// # 设备的访问模式
/// ```text
/// 参数
/// pstDevInfo [IN] 设备信息结构体  
/// nAccessMode [IN] 访问权限
/// MV_ACCESS_Exclusive  1  独占权限，其他APP只允许读CCP寄存器  
/// MV_ACCESS_ExclusiveWithSwitch  2  可以从5模式下抢占权限，然后以独占权限打开  
/// MV_ACCESS_Control  3  控制权限，其他APP允许读所有寄存器  
/// MV_ACCESS_ControlWithSwitch  4  可以从5的模式下抢占权限，然后以控制权限打开  
/// MV_ACCESS_ControlSwitchEnable  5  以可被抢占的控制权限打开  
/// MV_ACCESS_ControlSwitchEnableWithKey  6  可以从5的模式下抢占权限，然后以可被抢占的控制权限打开  
/// MV_ACCESS_Monitor  7  读模式打开设备，适用于控制权限下  
/// ```
#[repr(u8)]
#[derive(Debug, Deserialize, Serialize)]
pub enum MvAccessMode {
  Exclusive = 1,
  ExclusiveWithSwitch = 2,
  Control = 3,
  ControlWithSwitch = 4,
  ControlSwitchEnable = 5,
  ControlSwitchEnableWithKey = 6,
  Monitor = 7,
}
/// 设备信息列表    \~english Device Information List
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MvCcDeviceInfoList {
  /// 在线设备数量
  pub nDeviceNum: c_uint,
  /// 支持最多256个设备
  pub pDeviceInfo: [*mut MvCcDeviceInfo; MV_MAX_DEVICE_NUM],
}
impl Default for MvCcDeviceInfoList {
  fn default() -> Self {
    MvCcDeviceInfoList {
      nDeviceNum: 0,
      pDeviceInfo: [ptr::null_mut(); MV_MAX_DEVICE_NUM],
    }
  }
}
unsafe impl Send for MvCcDeviceInfoList {}
unsafe impl Sync for MvCcDeviceInfoList {}
/// 排序方式
#[repr(C)]
#[derive(Debug, Deserialize, Serialize)]
pub enum MvSortMethod {
  /// 按序列号排序
  SortMethod_SerialNumber,
  /// 按用户自定义名字排序
  SortMethod_UserID,
  /// 按当前IP地址排序（升序）
  SortMethod_CurrentIP_ASC,
  /// 按当前IP地址排序（降序）
  SortMethod_CurrentIP_DESC,
}
/// # enumrateDevices_ex2 使用
/// ```text
/// MV_UNKNOW_DEVICE  0x00000000  未知设备类型
/// MV_GIGE_DEVICE  0x00000001  GigE设备
/// MV_1394_DEVICE  0x00000002  1394-a/b设备
/// MV_USB_DEVICE  0x00000004  USB3.0设备
/// MV_CAMERALINK_DEVICE  0x00000008  CameraLink设备
/// 例如：nTLayerType = MV_GIGE_DEVICE | MV_USB_DEVICE ，表示查找GigE和USB3.0设备
/// ```
#[repr(u8)]
#[derive(Debug, Deserialize, Serialize)]
pub enum MvEnumDeviceLayerType {
  ///
  UnknowDevice = 0,
  ///
  GigeDevice = 1,
  ///
  E1394Device = 2,
  ///
  UsbDevice = 4,
  ///
  CameralinkDevice = 8,
  ///
  All = 1 | 2 | 4 | 8,
}
/// 通过GenTL枚举到的Interface信息    \~english Interface Information with GenTL
#[repr(C)]
#[derive(Debug)]
pub struct MvGentlIfInfo {
  /// GenTL接口ID
  pub chInterfaceId: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 传输层类型
  pub chTlType: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 设备显示名称
  pub chDisplayName: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// GenTL的cti文件索引
  pub nCtiIndex: c_uint,
  /// 保留字节
  pub nReserved: [c_uint; 8],
}

/// 通过GenTL枚举到的设备信息列表    \~english Device Information List with GenTL
#[repr(C)]
#[derive(Debug)]
pub struct MvGentlIfInfoList {
  /// 在线设备数量
  pub nInterfaceNum: c_uint,
  /// 支持最多256个设备
  pub pIfInfo: [*mut MvGentlIfInfo; MV_MAX_GENTL_IF_NUM as usize],
}

/// 通过GenTL枚举到的设备信息    \~english Device Information with GenTL
#[repr(C)]
#[derive(Debug)]
pub struct MvGentlDeviceInfo {
  /// GenTL接口ID
  pub chInterfaceId: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 设备ID
  pub chDeviceId: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 供应商名字
  pub chVendorName: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 型号名字
  pub chModelName: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 传输层类型
  pub chTlType: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 显示名称
  pub chDisplayName: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 用户自定义名字
  pub chUserDefinedName: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 序列号
  pub chSerialNumber: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// 设备版本号
  pub chDeviceVersion: [c_uchar; INFO_MAX_BUFFER_SIZE],
  /// cti索引
  pub nCtiIndex: c_uint,
  /// 保留字节
  pub nReserved: [c_uint; 8],
}

/// 设备信息列表    \~english Device Information List with GenTL
#[repr(C)]
#[derive(Debug)]
pub struct MvGentlDeviceInfoList {
  /// 在线设备数量
  pub nDeviceNum: c_uint,
  /// GenTL设备信息
  pub pDeviceInfo: [*mut MvGentlDeviceInfo; MV_MAX_GENTL_DEV_NUM as usize],
}

/// Chunk内容    \~english The content of ChunkData
#[repr(C)]
#[derive(Debug)]
pub struct MvChunkDataContent {
  /// 块数据
  pub pChunkData: *mut c_uchar,
  /// 块数据ID
  pub nChunkId: c_uint,
  /// 块数据长度
  pub nChunkLen: c_uint,
  /// 保留字节
  pub nReserved: [c_uint; 8],
}

/// values for enumeration 'MvGvspPixelType'
pub type MvGvspPixelType = c_int;

/// 输出帧的信息    \~english Output Frame Information
#[repr(C)]
pub struct MvFrameOutInfoEx {
  /// 图像宽
  pub nWidth: c_ushort,
  /// 图像高
  pub nHeight: c_ushort,
  /// 像素格式
  pub enPixelType: MvGvspPixelType,
  /// 帧号
  pub nFrameNum: c_uint,
  /// 时间戳高32位
  pub nDevTimeStampHigh: c_uint,
  /// 时间戳低32位
  pub nDevTimeStampLow: c_uint,
  /// 保留，8字节对齐
  pub nReserved0: c_uint,
  /// 主机生成的时间戳
  pub nHostTimeStamp: c_longlong,
  /// 帧的长度
  pub nFrameLen: c_uint,
  // 以下为chunk新增水印信息
  /// 设备水印时标
  pub nSecondCount: c_uint,
  pub nCycleCount: c_uint,
  pub nCycleOffset: c_uint,
  /// 增益
  pub fGain: c_float,
  /// 曝光时间
  pub fExposureTime: c_float,
  /// 平均亮度
  pub nAverageBrightness: c_uint,
  /// 白平衡相关
  pub nRed: c_uint,
  pub nGreen: c_uint,
  pub nBlue: c_uint,
  /// 帧计数
  pub nFrameCounter: c_uint,
  /// 触发计数
  pub nTriggerIndex: c_uint,
  /// 输入/输出
  pub nInput: c_uint,
  pub nOutput: c_uint,
  /// ROI区域
  pub nOffsetX: c_ushort,
  pub nOffsetY: c_ushort,
  pub nChunkWidth: c_ushort,
  pub nChunkHeight: c_ushort,
  /// 本帧丢包数
  pub nLostPacket: c_uint,
  /// 未解析的Chunkdata个数
  pub nUnparsedChunkNum: c_uint,
  /// 数据库链表
  pub UnparsedChunkList: MvFrameOutInfoExUnion,
  /// 保留字节
  pub nReserved: [c_uint; 36],
}
impl Default for MvFrameOutInfoEx {
  fn default() -> Self {
    Self {
      nWidth: 0,
      nHeight: 0,
      enPixelType: 0,
      nFrameNum: 0,
      nDevTimeStampHigh: 0,
      nDevTimeStampLow: 0,
      nReserved0: 0,
      nHostTimeStamp: 0,
      nFrameLen: 0,
      nSecondCount: 0,
      nCycleCount: 0,
      nCycleOffset: 0,
      fGain: 0.0,
      fExposureTime: 0.0,
      nAverageBrightness: 0,
      nRed: 0,
      nGreen: 0,
      nBlue: 0,
      nFrameCounter: 0,
      nTriggerIndex: 0,
      nInput: 0,
      nOutput: 0,
      nOffsetX: 0,
      nOffsetY: 0,
      nChunkWidth: 0,
      nChunkHeight: 0,
      nLostPacket: 0,
      nUnparsedChunkNum: 0,
      UnparsedChunkList: MvFrameOutInfoExUnion { nAligning: 0 },
      nReserved: [0; 36],
    }
  }
}

#[repr(C)]
pub union MvFrameOutInfoExUnion {
  /// Chunk内容
  pub pUnparsedChunkContent: *mut MvChunkDataContent,
  /// 校准字段
  pub nAligning: c_longlong,
}

/// 图像结构体，输出图像指针地址及图像信息    \~english Image Struct, output the pointer of Image and the information of the specific image
#[repr(C)]
pub struct MvFrameOut {
  /// 图像指针地址
  pub pBufAddr: *mut c_uchar,
  /// 图像信息
  pub stFrameInfo: MvFrameOutInfoEx,
  /// 保留字节
  pub nRes: [c_uint; 16],
}

/// values for enumeration '_MV_GRAB_STRATEGY_'
#[derive(Debug)]
pub enum MvGrabStrategy {
  /// 从旧到新一帧一帧的获取图像
  OneByOne = 0,
  /// 获取列表中最新的一帧图像（同时清除列表中的其余图像）
  LatestImagesOnly = 1,
  /// 获取列表中最新的图像
  LatestImages = 2,
  /// 等待下一帧图像
  UpcomingImage = 3,
}

/// 网络传输的相关信息    \~english Network transmission information
#[repr(C)]
#[derive(Debug)]
pub struct MvNetTransInfo {
  /// 已接收数据大小  [统计StartGrabbing和StopGrabbing之间的数据量]
  pub nReceiveDataSize: c_longlong,
  /// 丢帧数量
  pub nThrowFrameCount: c_int,
  /// 收到帧计数
  pub nNetRecvFrameCount: c_uint,
  /// 请求重发包数
  pub nRequestResendPacketCount: c_longlong,
  /// 重发包数
  pub nResendPacketCount: c_longlong,
}

/// values for enumeration 'MvSavePointCloudFileType'
pub type MvSavePointCloudFileType = c_int;

/// 保存3D数据到缓存    \~english Save 3D data to buffer
#[repr(C)]
#[derive(Debug)]
pub struct MvSavePointCloudParam {
  /// 每一行点的数量，即图像宽
  pub nLinePntNum: c_uint,
  /// 行数，即图像高
  pub nLineNum: c_uint,
  /// 输入数据的像素格式
  pub enSrcPixelType: MvGvspPixelType,
  /// 输入数据缓存
  pub pSrcData: *mut c_uchar,
  /// 输入数据大小
  pub nSrcDataLen: c_uint,
  /// 输出像素数据缓存
  pub pDstBuf: *mut c_uchar,
  /// 提供的输出缓冲区大小(nLinePntNum * nLineNum * (16*3 + 4) + 2048)
  pub nDstBufSize: c_uint,
  /// 输出像素数据缓存长度
  pub nDstBufLen: c_uint,
  /// 提供输出的点云文件类型
  pub enPointCloudFileType: MvSavePointCloudFileType,
  /// 保留字节
  pub nReserved: [c_uint; 8],
}

/// \~chinese 全匹配的一种信息结构体    \~english A fully matched information structure
#[repr(C)]
#[derive(Debug)]
pub struct MvAllMatchInfo {
  /// 需要输出的信息类型
  pub nType: c_uint,
  /// 输出的信息缓存，由调用者分配
  pub pInfo: *mut c_void,
  /// 信息缓存的大小
  pub nInfoSize: c_uint,
}

/// 网络流量和丢包信息反馈结构体，对应类型为 MV_MATCH_TYPE_NET_DETECT    \~english Network traffic and packet loss feedback structure, the corresponding type is MV_MATCH_TYPE_NET_DETECT
#[repr(C)]
#[derive(Debug)]
pub struct MvMatchInfoNetDetect {
  /// 已接收数据大小
  pub nReceiveDataSize: c_longlong,
  /// 丢帧数量
  pub nLostPacketCount: c_longlong,
  /// 丢失的包数量
  pub nLostFrameCount: c_uint,
  /// 收到帧计数
  pub nNetRecvFrameCount: c_uint,
  /// 请求重发包数
  pub nRequestResendPacketCount: c_longlong,
  /// 重发包数
  pub nResendPacketCount: c_longlong,
}

/// \~chinese host收到从u3v设备端的总字节数，对应类型为 MV_MATCH_TYPE_USB_DETECT    \~english The total number of bytes host received from the u3v device side, the corresponding type is MV_MATCH_TYPE_USB_DETECT
#[repr(C)]
#[derive(Debug)]
pub struct MvMatchInfoUsbDetect {
  /// 已接收数据大小
  pub nReceiveDataSize: c_longlong,
  /// 已收到的帧数
  pub nReceivedFrameCount: c_uint,
  /// 错误帧数
  pub nErrorFrameCount: c_uint,
  /// 保留字节
  pub nReserved: [c_uint; 2],
}

/// \~chinese 显示帧信息   \~english Display frame information
#[repr(C)]
#[derive(Debug)]
pub struct MvDisplayFrameInfo {
  /// 窗口句柄
  pub hWnd: *mut c_void,
  /// 显示的数据
  pub pData: *mut c_uchar,
  /// 数据长度
  pub nDataLen: c_uint,
  /// 图像宽
  pub nWidth: c_ushort,
  /// 图像高
  pub nHeight: c_ushort,
  /// 像素格式
  pub enPixelType: MvGvspPixelType,
  /// 保留字节
  pub nRes: [c_uint; 4],
}

/// values for enumeration 'MvSaveIamgeType'
pub type MvSaveIamgeType = c_uint;

/// [IN] 插值方法 0-快速 1-均衡 2-最优（其它值默认为最优）
pub type MvSaveIamgeMethodValue = c_uint;

/// 图片保存参数    \~english Save Image Parameters
#[repr(C)]
#[derive(Debug)]
pub struct MvSaveImageParamEx {
  /// 输入数据缓存
  pub pData: *mut c_uchar,
  /// 输入数据大小
  pub nDataLen: c_uint,
  /// 输入数据的像素格式
  pub enPixelType: MvGvspPixelType,
  /// 图像宽
  pub nWidth: c_ushort,
  /// 图像高
  pub nHeight: c_ushort,
  /// 输出图片缓存
  pub pImageBuffer: *mut c_uchar,
  /// 输出图片大小
  pub nImageLen: c_uint,
  /// 提供的输出缓冲区大小
  pub nBufferSize: c_uint,
  /// 输出图片格式
  pub enImageType: MvSaveIamgeType,
  /// 编码质量, (50-99]
  pub nJpgQuality: c_uint,
  /// Bayer格式转为RGB24的插值方法  0-最近邻 1-双线性 2-Hamilton
  pub iMethodValue: MvSaveIamgeMethodValue,
  /// 保留字节
  pub nReserved: [c_uint; 3],
}

/// 保存BMP、JPEG、PNG、TIFF图片文件的参数    \~english Save BMP、JPEG、PNG、TIFF image file parameters
#[repr(C)]
#[derive(Debug)]
pub struct MvSaveImgToFileParam {
  /// 输入数据的像素格式
  pub enPixelType: MvGvspPixelType,
  /// 输入数据缓存
  pub pData: *mut c_uchar,
  /// 输入数据大小
  pub nDataLen: c_uint,
  /// 图像宽
  pub nWidth: c_ushort,
  /// 图像高
  pub nHeight: c_ushort,
  /// 输入图片格式
  pub enImageType: MvSaveIamgeType,
  /// JPG编码质量(50-99],PNG编码质量[0-9]
  pub nQuality: c_uint,
  /// 输入文件路径
  pub pImagePath: [c_char; 256],
  /// Bayer格式转为RGB24的插值方法 0-最近邻 1-双线性 2-Hamilton
  pub iMethodValue: c_int,
  /// 保留字节
  pub nReserved: [c_uint; 8],
}

/// 图像转换结构体    \~english Pixel convert structure
#[repr(C)]
#[derive(Debug)]
pub struct MvPixelConvertParam {
  /// 图像宽
  pub nWidth: c_ushort,
  /// 图像高
  pub nHeight: c_ushort,
  /// 源像素格式
  pub enSrcPixelType: MvGvspPixelType,
  /// 输入数据缓存
  pub pSrcData: *mut c_uchar,
  /// 输入数据大小
  pub nSrcDataLen: c_uint,
  /// 目标像素格式
  pub enDstPixelType: MvGvspPixelType,
  /// 输出数据缓存
  pub pDstBuffer: *mut c_uchar,
  /// 输出数据大小
  pub nDstLen: c_uint,
  /// 提供的输出缓冲区大小
  pub nDstBufferSize: c_uint,
  /// 保留字节
  pub nRes: [c_uint; 4],
}
///
pub type MvRecordFormatType = c_int;
/// 录像参数    \~english Record Parameters
#[repr(C)]
#[derive(Debug)]
pub struct MvCcRecordParam {
  /// 输入数据的像素格式
  pub enPixelType: MvGvspPixelType,
  /// 图像宽(指定目标参数时需为2的倍数)
  pub nWidth: c_ushort,
  /// 图像高(指定目标参数时需为2的倍数)
  pub nHeight: c_ushort,
  /// 帧率fps(1/16-120)
  pub fFrameRate: c_float,
  /// 码率kbps(128kbps-16Mbps)
  pub nBitRate: c_uint,
  /// 录像格式
  pub enRecordFmtType: MvRecordFormatType,
  /// 录像文件存放路径(如果路径中存在中文，需转成utf-8)
  pub strFilePath: [c_char; 256],
  /// 保留字节
  pub nRes: [c_uint; 8],
}

const MAX_EVENT_NAME_SIZE: usize = 256;
type STRING = [c_char; 256];

/// 录像数据    \~english Record Data
#[repr(C)]
#[derive(Debug)]
pub struct MvCcInputFrameInfo {
  /// 图像数据指针
  pub pData: *mut c_uchar,
  /// 输入数据大小
  pub nDataLen: c_uint,
  /// 保留字节
  pub nRes: [c_uint; 8],
}
/// MvCamExposureMode values for enumeration '_MV_CAM_EXPOSURE_MODE_'
#[repr(i32)]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub enum MvCamExposureMode {
  ///
  Off = MV_EXPOSURE_AUTO_MODE_OFF,
  ///
  Once = MV_EXPOSURE_AUTO_MODE_ONCE,
  ///
  #[default]
  Continuous = MV_EXPOSURE_AUTO_MODE_CONTINUOUS,
}
/// MV_CAM_GAIN_MODE values for enumeration '_MV_CAM_GAIN_MODE_'
#[repr(i32)]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub enum MvCamGainMode {
  ///
  Off = MV_GAIN_MODE_OFF,
  ///
  Once = MV_GAIN_MODE_ONCE,
  ///
  #[default]
  Continuous = MV_GAIN_MODE_CONTINUOUS,
}
/// values for enumeration '_MV_CAM_ACQUISITION_MODE_'
pub type MvCamAcquisitionMode = c_int;

// values for enumeration '_MV_CAM_GAIN_MODE_'
// pub type MvCamGainMode = c_int;

// values for enumeration '_MV_CAM_EXPOSURE_MODE_'
// pub type MvCamExposureMode = c_int;

/// values for enumeration '_MV_CAM_EXPOSURE_AUTO_MODE_'
pub type MvCamExposureAutoMode = c_int;

/// values for enumeration '_MV_CAM_TRIGGER_MODE_'
pub type MvCamTriggerMode = c_int;

/// values for enumeration '_MV_CAM_GAMMA_SELECTOR_'
pub type MvCamGammaSelector = c_int;

/// values for enumeration '_MV_CAM_BALANCEWHITE_AUTO_'
pub type MvCamBalancewhiteAuto = c_int;

/// values for enumeration '_MV_CAM_TRIGGER_SOURCE_'
pub type MvCamTriggerSource = c_int;

/// Event事件回调信息\    \~english Event callback infomation
#[repr(C)]
#[derive(Debug)]
pub struct MvEventOutInfo {
  /// Event名称
  pub eventName: [c_char; MAX_EVENT_NAME_SIZE],
  /// Event号
  pub nEventId: c_ushort,
  /// 流通道序号
  pub nStreamChannel: c_ushort,
  /// 帧号高位
  pub nBlockIdHigh: c_uint,
  /// 帧号低位
  pub nBlockIdLow: c_uint,
  /// 时间戳高位
  pub nTimestampHigh: c_uint,
  /// 时间戳低位
  pub nTimestampLow: c_uint,
  /// Event数据
  pub pEventData: *mut c_void,
  /// Event数据长度
  pub nEventDataSize: c_uint,
  /// 保留字节
  pub nReserved: [c_uint; 16],
}

/// 文件存取    \~english File Access
#[repr(C)]
#[derive(Debug)]
pub struct MvCcFileAccess {
  /// 用户文件名
  pub pUserFileName: STRING,
  /// 设备文件名
  pub pDevFileName: STRING,
  /// 保留字节
  pub nReserved: [c_uint; 32],
}

/// 文件存取进度    \~english File Access Progress
#[repr(C)]
#[derive(Debug)]
pub struct MvCcFileAccessProgress {
  /// 已完成的长度
  pub nCompleted: Int64T,
  /// 总长度
  pub nTotal: Int64T,
  /// 保留字节
  pub nReserved: [c_uint; 8],
}

/// values for enumeration '_MV_GIGE_TRANSMISSION_TYPE_'
pub type MvGigeTransmissionType = c_int;

/// 传输模式，可以为单播模式、组播模式等    \~english Transmission type
#[repr(C)]
pub struct MvTransmissionType {
  /// 传输模式
  pub enTransmissionType: MvGigeTransmissionType,
  /// 目标IP，组播模式下有意义
  pub nDestIp: c_uint,
  /// 目标Port，组播模式下有意义
  pub nDestPort: c_ushort,
  /// 保留字节
  pub nReserved: [c_uint; 32],
}

const MV_MAX_XML_SYMBOLIC_NUM: usize = 32;

/// 动作命令信息    \~english Action Command
#[repr(C)]
#[derive(Debug)]
pub struct MvActionCmdInfo {
  /// 设备密钥
  pub nDeviceey: c_uint,
  /// 组键
  pub nGroupKey: c_uint,
  /// 组掩码
  pub nGroupMask: c_uint,
  /// 只有设置成1时Action Time才有效，非1时无效
  pub bActionTimeEnable: c_uint,
  /// 预定的时间，和主频有关
  pub nActionTime: Int64T,
  /// 广播包地址
  pub pBroadcastAddress: [c_char; 256],
  /// 等待ACK的超时时间，如果为0表示不需要ACK
  pub nTimeOut: c_uint,
  /// 预留
  pub nReserved: [c_uint; 16],
}

/// 动作命令返回信息    \~english Action Command Result
#[repr(C)]
#[derive(Debug)]
pub struct MvActionCmdResult {
  /// IP配置选项
  pub strDeviceAddress: [c_uchar; 16],
  /// 状态码
  pub nStatus: c_int,
  /// 预留
  pub nReserved: [c_uint; 4],
}

/// 动作命令返回信息列表    \~english Action Command Result List
#[repr(C)]
pub struct MvActionCmdResultList {
  /// 返回值个数
  pub nNumResults: c_uint,
  /// 动作命令返回信息
  pub pResults: *mut MvActionCmdResult,
}

/// values for enumeration 'MV_XML_InterfaceType'
pub type MvXmlInterfaceType = c_int;

/// values for enumeration 'MV_XML_AccessMode'
pub type MvXmlAccessMode = c_int;

/// 枚举类型值    \~english Enumeration Value
#[repr(C)]
#[derive(Debug)]
pub struct MvCcEnumValue {
  /// 当前值
  pub nCurValue: c_uint,
  /// 数据的有效数据个数
  pub nSupportedNum: c_uint,
  /// 支持值列表
  pub nSupportValue: [c_uint; MV_MAX_XML_SYMBOLIC_NUM],
  /// 预留
  pub nReserved: [c_uint; 4],
}

/// Int类型值    \~english Int Value
#[repr(C)]
#[derive(Debug)]
pub struct MvCcIntValue {
  /// 当前值
  pub nCurValue: c_uint,
  /// 最大值
  pub nMax: c_uint,
  /// 最小值
  pub nMin: c_uint,
  /// 步径
  pub nInc: c_uint,
  /// 预留
  pub nReserved: [c_uint; 4],
}

/// Int类型值Ex    \~english Int Value Ex
#[repr(C)]
#[derive(Deserialize, Serialize, Debug)]
pub struct MvCcIntValueEx {
  /// 当前值
  pub nCurValue: Int64T,
  /// 最大值
  pub nMax: Int64T,
  /// 最小值
  pub nMin: Int64T,
  /// 步径
  pub nInc: Int64T,
  /// 预留
  pub nReserved: [c_uint; 16],
}
impl Default for MvCcIntValueEx {
  fn default() -> Self {
    Self {
      nCurValue: 0,
      nMax: 0,
      nMin: 0,
      nInc: 0,
      nReserved: [0; 16],
    }
  }
}
/// Float类型值    \~english Float Value
#[repr(C)]
#[derive(Debug)]
pub struct MvCcFloatValue {
  /// 当前值
  pub fCurValue: c_float,
  /// 最大值
  pub fMax: c_float,
  /// 最小值
  pub fMin: c_float,
  /// 预留
  pub nReserved: [c_uint; 4],
}

/// String类型值    \~english String Value
#[repr(C)]
#[derive(Debug)]
pub struct MvCcStringValue {
  /// 当前值
  pub chCurValue: [c_char; 256],
  /// 最大长度
  pub nMaxLength: Int64T,
  /// 预留
  pub nReserved: [c_uint; 2],
}

/// # cbOutput [IN] 回调函数指针  
pub type CbOutputCallback = extern "C" fn(
  pData: *const c_uchar,
  pstFrameInfo: *const MvFrameOutInfoEx,
  pUser: *const c_void,
) -> c_void;
