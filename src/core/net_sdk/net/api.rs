use std::{ffi::*, ptr};

use crate::Lib;

use super::{
  types::{HcnetLevel, NetDvrPreviewInfo, RealDataCallback, TagPdcParamKey},
  LpNetDvrJpegpara, LpnetDvrDeviceInfoV40, NetDvrUserLoginInfo, NetLastError, NetSdkInitCfgType,
};
use e_utils::{
  auto_any_res_c,
  parse::{AutoParse, CAutoParse},
  CResult,
};
use libloading::{Library, Symbol};

#[repr(C)]
#[derive(Default)]
pub struct HcNetCoreSdk {
  handle: c_long,
  lib: Lib,
  uid: c_long,
}
unsafe impl Send for HcNetCoreSdk {}
unsafe impl Sync for HcNetCoreSdk {}
impl HcNetCoreSdk {
  //// 设置LIB
  pub fn set_lib(&mut self, lib: Lib) {
    self.lib = lib;
  }
  /// 获取LIB
  pub fn lib(&self) -> &Library {
    self.lib.get()
  }
  /// 释放
  pub fn free(&mut self) -> Option<()> {
    self.lib.free()
  }
}

impl HcNetCoreSdk {
  /// 用户注册设备（支持异步登录）。。
  pub unsafe extern "C" fn login_v40(
    &mut self,
    login_info: &mut NetDvrUserLoginInfo,
    dev_info: &mut LpnetDvrDeviceInfoV40,
  ) -> CResult<c_long> {
    let func: Symbol<
      '_,
      unsafe extern "C" fn(
        pLoginInfo: *mut NetDvrUserLoginInfo,
        lpDeviceInfo: *mut LpnetDvrDeviceInfoV40,
      ) -> c_long,
    > = auto_any_res_c!(self.lib().get(b"NET_DVR_Login_V40\0"));
    let uid = func(login_info, dev_info);
    if uid != -1 {
      self.uid = uid;
    }
    CResult::Ok(uid)
  }
  /// 设置NET SDK初始化参数。
  pub unsafe extern "C" fn init_cfg(&self, etype: NetSdkInitCfgType) -> CResult<bool> {
    // 设置SDK初始化参数。
    let func = auto_any_res_c!(self
      .lib()
      .get::<Symbol<'_, unsafe extern "C" fn(enumType: c_uint, lpInBuff: *const c_void) -> bool>>(
        b"NET_DVR_SetSDKInitCfg\0",
      ));
    let et: c_uint = (&etype).into();
    // let ibuff = etype.to_c_ptr();
    let ibuff = match &etype {
      NetSdkInitCfgType::NetSdkInitCfgAbility(x) => x as *const _ as *const c_void,
      NetSdkInitCfgType::NetSdkInitCfgSdkPath(x) => x as *const _ as *const c_void,
      NetSdkInitCfgType::NetSdkInitCfgLibeayPath(x) => *x as *const c_void,
      NetSdkInitCfgType::NetSdkInitCfgSsleayPath(x) => *x as *const c_void,
    };
    let res = func(et, ibuff);
    CResult::Ok(res)
  }

  /// 设置抓图模式
  /// dwCaptureMode
  /// [in] 抓图模式
  pub unsafe extern "C" fn set_capture_mode(&self, mode: TagPdcParamKey) -> CResult<bool> {
    unsafe {
      // 设置抓图模式
      let func: Symbol<'_, unsafe extern "C" fn(dwCaptureMode: c_uint) -> bool> =
        auto_any_res_c!(self.lib().get(b"NET_DVR_SetCapturePictureMode\0"));
      let res = func(mode as c_uint);
      CResult::Ok(res)
    }
  }
  /// 预览时抓图并保存成图片文件。
  /// ```
  /// lRealHandle
  /// [in] 预览句柄，NET_DVR_RealPlay_V40的返回值
  /// sPicFileName
  /// [in] 保存图片的文件路径，包含文件名，比如："D:/test.bmp"或"D:/test.jpg"
  /// dwTimeOut
  /// [in] 超时时间，目前无效
  /// ```
  /// 该接口为预览阻塞模式抓图，预览接口必须传入有效的窗口句柄，正常解码显示的时候才能调用该接口进行抓图。图片数据格式支持BMP和JPEG两种，通过NET_DVR_SetCapturePictureMode可以设置数据格式，不同的格式保存文件名使用不同的后缀（.bmp或者.jpg）。
  pub unsafe extern "C" fn capture_picture_block(
    &self,
    fname: *const c_char,
    timeout: c_uint,
  ) -> CResult<bool> {
    // 设置抓图模式
    let func: Symbol<
      '_,
      unsafe extern "C" fn(
        lRealHandle: c_long,
        sPicFileName: *const c_char,
        dwTimeOut: c_uint,
      ) -> bool,
    > = auto_any_res_c!(self.lib().get(b"NET_DVR_CapturePictureBlock\0"));
    let res = func(self.handle, fname, timeout);
    CResult::Ok(res)
  }
  /// # 预览时抓图并保存在指定内存中。
  /// ```
  /// lRealHandle
  /// [in] 预览句柄，NET_DVR_RealPlay_V40的返回值
  /// pPicBuf
  /// [in] 保存图片数据的缓冲区
  /// dwPicSize
  /// [in] 缓冲区大小，分配的缓冲区内存必须大于等于图片数据的大小
  /// lpSizeReturned
  /// [out] 返回图片数据的实际大小
  /// ```
  pub unsafe extern "C" fn capture_picture_block_new(
    &self,
    pic_buf: *mut c_char,
    pic_size: c_uint,
    size_returned: *mut c_uint,
  ) -> CResult<bool> {
    // 设置抓图模式
    let func: Symbol<
      '_,
      unsafe extern "C" fn(
        lRealHandle: c_long,
        pPicBuf: *mut c_char,
        dwPicSize: c_uint,
        lpSizeReturned: *mut c_uint,
      ) -> bool,
    > = auto_any_res_c!(self.lib().get(b"NET_DVR_CapturePictureBlock_New\0"));
    let res = func(self.handle, pic_buf, pic_size, size_returned);
    CResult::Ok(res)
  }
  /// 预览时，单帧数据捕获并保存成图片。
  /// lRealHandle
  /// [in] 预览句柄，NET_DVR_RealPlay_V40的返回值
  /// 调用NET_DVR_CapturePicture进行抓图，要求在调用NET_DVR_RealPlay_V40等接口时传入非空的播放句柄（播放库解码显示），否则时接口会返回失败，调用次序错误。
  pub unsafe extern "C" fn capture_picture(&self, fname: *const c_char) -> CResult<bool> {
    // 设置抓图模式
    let func: Symbol<
      '_,
      unsafe extern "C" fn(lRealHandle: c_long, sPicFileName: *const c_char) -> bool,
    > = auto_any_res_c!(self.lib().get(b"NET_DVR_CapturePicture\0"));
    let res = func(self.handle, fname);
    CResult::Ok(res)
  }

  /// 用户注销。
  /// lUserID
  /// [in] 用户ID号，NET_DVR_Login_V40等登录接口的返回值
  pub unsafe extern "C" fn logout_sdk(&mut self) -> CResult<bool> {
    let func: Symbol<'_, unsafe extern "C" fn(lUserID: c_long) -> bool> =
      auto_any_res_c!(self.lib().get(b"NET_DVR_Logout\0"));
    let res = func(self.uid);
    if res {
      self.uid = -1
    }
    CResult::Ok(res)
  }
  /// 关闭预览
  /// lRealHandle
  /// [in] 预览句柄，NET_DVR_RealPlay或者NET_DVR_RealPlay_V30的返回值
  pub unsafe extern "C" fn stop_real_play_sdk(&mut self) -> CResult<bool> {
    let func: Symbol<'_, unsafe extern "C" fn(lRealHandle: c_long) -> bool> =
      auto_any_res_c!(self.lib().get(b"NET_DVR_StopRealPlay\0"));
    let res = func(self.handle);
    if res {
      self.handle = -1;
    }
    CResult::Ok(res)
  }

  /// 打开预览
  /// type
  /// 标记回调出了哪些私有信息类型
  /// stTarget
  /// 目标
  /// stTarget_EX
  /// 深眸目标
  pub unsafe extern "C" fn open_preview_sdk(
    &mut self,
    preview_info: NetDvrPreviewInfo,
    callback: Option<RealDataCallback>,
  ) -> CResult<bool> {
    println!("1  uid {}", self.uid);
    let func: Symbol<
      '_,
      unsafe extern "C" fn(
        lUserID: c_long,
        lpPreviewInfo: NetDvrPreviewInfo,
        fRealDataCallBack_V30: *const c_void,
        pUser: *mut c_void,
      ) -> c_long,
    > = auto_any_res_c!(self.lib().get(b"NET_DVR_RealPlay_V40\0"));
    let cback = callback
      .and_then(|x| Some(x as *const RealDataCallback as *const c_void))
      .unwrap_or(ptr::null());
    let handle = func(self.uid, preview_info, cback, ptr::null_mut());
    self.handle = handle;
    CResult::Ok(handle != -1)
  }
  /// 释放SDK资源，在程序结束之前调用。
  pub unsafe extern "C" fn cleanup_sdk(&self) -> CResult<bool> {
    let func: Symbol<'_, unsafe extern "C" fn() -> bool> =
      auto_any_res_c!(self.lib().get(b"NET_DVR_Cleanup\0"));
    let res = func();
    CResult::Ok(res)
  }

  /// 获取最后错误代码
  pub unsafe extern "C" fn get_last_error_sdk(&self, msg: *const c_char) -> CResult<NetLastError> {
    let func: Symbol<'_, unsafe extern "C" fn() -> c_long> =
      auto_any_res_c!(self.lib().get(b"NET_DVR_GetLastError\0"));
    let err = func();
    CResult::Ok(NetLastError {
      code: err,
      msg: format!("NetLastError<{}<{}>>", msg.c_to_string(), err).to_c_char(),
    })
  }
  /// SDK本地日志初始化
  /// nLogLevel
  /// [in] 日志的等级（默认为0）：0-表示关闭日志，1-表示只输出ERROR错误日志，2-输出ERROR错误信息和DEBUG调试信息，3-输出ERROR错误信息、DEBUG调试信息和debug普通信息等所有信息
  /// strLogDir
  /// [in] 日志文件的路径，windows默认值为"C:\\SdkLog\\"；linux默认值"/home/sdklog/"
  /// bAutoDel
  /// [in] 是否删除超出的文件数，默认值为TRUE
  pub unsafe extern "C" fn set_log_to_file_sdk(
    &self,
    log_path: *const c_char,
    level: HcnetLevel,
    auto_del: bool,
  ) -> CResult<bool> {
    let func: Symbol<
      '_,
      unsafe extern "C" fn(nLogLevel: c_uint, strLogDir: *const c_char, bAutoDel: bool) -> bool,
    > = auto_any_res_c!(self.lib().get(b"NET_DVR_SetLogToFile\0"));
    let res = func(level as c_uint, log_path, auto_del);
    CResult::Ok(res)
  }

  /// 初始化SDK，调用其他SDK函数的前提。
  pub unsafe extern "C" fn init_sdk(&self) -> CResult<bool> {
    // 初始化DLL SDK
    let func: Symbol<'_, unsafe extern "C" fn() -> bool> =
      auto_any_res_c!(self.lib().get(b"NET_DVR_Init\0"));
    let res = func();
    CResult::Ok(res)
  }

  /// 单帧数据捕获并保存成JPEG图。
  /// ```
  /// Parameters
  /// lUserID
  /// [in] NET_DVR_Login_V40等登录接口的返回值
  /// lChannel
  /// [in] 通道号
  /// lpJpegPara
  /// [in] JPEG图像参数
  /// sPicFileName
  /// [in] 保存JPEG图的文件路径（包括文件名）
  /// ```
  /// 该接口用于设备的单帧数据捕获，并保存成JPEG图片文件。JPEG抓图功能或者抓图分辨率需要设备支持，如果不支持接口返回失败，错误号23或者29。
  pub unsafe extern "C" fn capture_jpeg(
    &self,
    uid: c_long,
    channel: c_long,
    jpeg_conf: LpNetDvrJpegpara,
    fname: *const c_char,
  ) -> CResult<bool> {
    // 初始化DLL SDK
    let func: Symbol<
      '_,
      unsafe extern "C" fn(
        lUserID: c_long,
        lChannel: c_long,
        lpJpegPara: LpNetDvrJpegpara,
        sPicFileName: *const c_char,
      ) -> bool,
    > = auto_any_res_c!(self.lib().get(b"NET_DVR_CaptureJPEGPicture\0"));
    let res = func(uid, channel, jpeg_conf, fname);
    CResult::Ok(res)
  }

  /// # 获取预览时用来解码和显示的播放器句柄。
  ///
  /// ```
  /// Parameters
  /// lRealHandle
  /// [in] 预览句柄，NET_DVR_RealPlay或者NET_DVR_RealPlay_V30的返回值
  /// Return Values
  /// －1表示失败，其他值表示播放句柄，接口返回失败请调用NET_DVR_GetLastError获取错误码，通过错误码判断出错原因。
  /// Remarks
  /// 用户可以通过返回的句柄自行实现播放库SDK提供的其他功能，详见播放库SDK开发包里面的《播放器SDK编程指南》。
  /// 例如，使用PlayM4_GetBMP(LONG nPort,……)、PlayM4_GetJPEG(LONG nPort,……)这两个接口，即可实现将当前预览图像以BMP或JPEG格式抓图保存到内存中： PlayM4_GetBMP(NET_DVR_GetRealPlayerIndex(),……)
  /// PlayM4_GetJPEG(NET_DVR_GetRealPlayerIndex(),……)
  /// 例如，调用PlayM4_RenderPrivateData(NET_DVR_GetRealPlayerIndex(), ……)可以显示或者关闭预览画面上的智能叠加信息。
  /// ```
  /// 该接口用于设备的单帧数据捕获，并保存成JPEG图片文件。JPEG抓图功能或者抓图分辨率需要设备支持，如果不支持接口返回失败，错误号23或者29。
  pub unsafe extern "C" fn get_real_player_index(&self, preview_handle: c_long) -> CResult<c_long> {
    // 初始化DLL SDK
    let func: Symbol<'_, unsafe extern "C" fn(lRealHandle: c_long) -> c_long> =
      auto_any_res_c!(self.lib().get(b"NET_DVR_GetRealPlayerIndex\0"));
    let res = func(preview_handle);
    CResult::Ok(res)
  }

  /// # 获取所有IP，用于支持多网卡接口。
  /// ```text
  /// Parameters
  /// strIP
  /// [out] 存放IP的缓冲区，不能为空
  /// pValidNum
  /// [out] 所有有效 IP 的数量
  /// pEnableBind
  /// [out] 是否绑定
  /// ```
  pub unsafe extern "C" fn get_local_ip(
    &self,
    str_ip: *mut *mut c_char,
    valid_num: *mut c_ulong,
    enable_bind: *mut bool,
  ) -> CResult<bool> {
    let func: Symbol<
      '_,
      unsafe extern "C" fn(
        strIP: *mut *mut c_char,
        pValidNum: *mut c_ulong,
        pEnableBind: *mut bool,
      ) -> bool,
    > = auto_any_res_c!(self.lib().get(b"NET_DVR_GetLocalIP\0"));
    let res = func(str_ip, valid_num, enable_bind);
    CResult::Ok(res)
  }

  /// # 通过解析服务器，获取设备的动态IP地址和端口号。
  /// ```text
  /// Parameters
  /// sServerIP
  /// [in] 解析服务器(IPServer或者hiDDNS)的IP地址或者域名
  /// wServerPort
  /// [in] 解析服务器的端口号。IP Server端口号为7071，hiDDNS服务器的端口号为80。
  /// sDVRName
  /// [in] 设备名称
  /// wDVRNameLen
  /// [in] 设备名称的长度
  /// sDVRSerialNumber
  /// [in] 设备的序列号
  /// wDVRSerialLen
  /// [in] 设备序列号的长度
  /// sGetIP
  /// [out] 获取到的设备IP地址指针
  /// dwPort
  /// [out] 获取到的设备端口号指针
  /// ```
  pub unsafe extern "C" fn ip_by_resolve_svr_ex(
    &self,
    server_ip: *const c_uchar,
    server_port: c_ushort,
    dvr_name: *const c_uchar,
    dvr_name_len: c_ushort,
    dvr_serial_number: *const c_uchar,
    dvr_serial_len: c_ushort,
    get_ip: *mut c_char,
    port: *mut c_uint,
  ) -> CResult<bool> {
    let func: Symbol<
      '_,
      unsafe extern "C" fn(
        sServerIP: *const c_uchar,
        wServerPort: c_ushort,
        sDVRName: *const c_uchar,
        wDVRNameLen: c_ushort,
        sDVRSerialNumber: *const c_uchar,
        wDVRSerialLen: c_ushort,
        sGetIP: *mut c_char,
        dwPort: *mut c_uint,
      ) -> bool,
    > = auto_any_res_c!(self.lib().get(b"NET_DVR_GetDVRIPByResolveSvr_EX\0"));
    let res = func(
      server_ip,
      server_port,
      dvr_name,
      dvr_name_len,
      dvr_serial_number,
      dvr_serial_len,
      get_ip,
      port,
    );
    CResult::Ok(res)
  }
}
