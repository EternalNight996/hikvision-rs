use e_utils::{
    auto_any_res_c,
    parse::{AutoParse as _, CAutoParse},
    ui::{HWND, RECT},
    CResult,
};
use libloading::{Library, Symbol};
use std::ffi::*;

use super::{PlayM4LastError, PlayStreamOpenMode};
use crate::net_sdk::play::DecCBFunWin;
use crate::Lib;

#[repr(C)]
#[derive(Default)]
pub struct HcNetPlayCoreSdk {
    handle: c_long,
    lib: Lib,
}
unsafe impl Send for HcNetPlayCoreSdk {}
unsafe impl Sync for HcNetPlayCoreSdk {}

impl HcNetPlayCoreSdk {
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
impl HcNetPlayCoreSdk {
    /// 获取错误号。
    /// nPort
    /// [in] 播放通道号
    pub unsafe extern "C" fn get_last_error(&self, msg: *const c_char) -> CResult<PlayM4LastError> {
        let func: Symbol<'_, unsafe extern "C" fn(nPort: c_long) -> c_ushort> =
            auto_any_res_c!(self.lib().get(b"PlayM4_GetLastError\0"));
        let res = func(self.handle);
        CResult::Ok(PlayM4LastError {
            code: res,
            msg: format!("PlayM4LastError<{}<{}>>", msg.c_to_string(), res).to_c_char(),
        })
    }

    /// 关闭播放。
    /// nPort
    /// [in] 播放通道号
    pub unsafe extern "C" fn stop(&mut self) -> CResult<bool> {
        let func: Symbol<'_, unsafe extern "C" fn(lRealHandle: c_long) -> bool> =
            auto_any_res_c!(self.lib().get(b"PlayM4_Stop\0"));
        let res = func(self.handle);
        if res {
            self.handle = -1
        }
        CResult::Ok(res)
    }
    /// 关闭流。
    /// nPort
    /// [in] 播放通道号
    pub unsafe extern "C" fn close_stream(&self) -> CResult<bool> {
        let func: Symbol<'_, unsafe extern "C" fn(lRealHandle: c_long) -> bool> =
            auto_any_res_c!(self.lib().get(b"PlayM4_CloseStream\0"));
        let res = func(self.handle);
        CResult::Ok(res)
    }
    /// 释放端口。
    /// nPort
    /// [in] 播放通道号
    pub unsafe extern "C" fn free_port(&self) -> CResult<bool> {
        // 释放已使用的通道号。
        let func: Symbol<'_, unsafe extern "C" fn(lRealHandle: c_long) -> bool> =
            auto_any_res_c!(self.lib().get(b"PlayM4_FreePort\0"));
        let res = func(self.handle);
        CResult::Ok(res)
    }
    /// 获取未使用的通道号。
    /// nPort
    /// [out] 播放通道号，指向用于获取端口号的LONG型变量指针
    pub unsafe extern "C" fn get_port_sdk(&mut self) -> CResult<bool> {
        let func: Symbol<'_, unsafe extern "C" fn(lRealHandle: *mut c_long) -> bool> =
            auto_any_res_c!(self.lib().get(b"PlayM4_GetPort\0"));
        let res = func(&mut self.handle);
        CResult::Ok(res)
    }
    /// 输入流数据。
    /// nPort
    /// [in] 播放通道号
    /// pBuf
    /// [in] 流数据缓冲区地址
    /// nSize
    /// [in] 流数据缓冲区大小
    pub unsafe extern "C" fn input_data(
        &self,
        p_buf: *const c_uchar,
        n_size: c_uint,
    ) -> CResult<bool> {
        // PlayM4_InputData
        let func: Symbol<
            '_,
            unsafe extern "C" fn(nPort: c_long, pBuf: *const c_uchar, nSize: c_uint) -> bool,
        > = auto_any_res_c!(self.lib().get(b"PlayM4_InputData\0"));
        let res = func(self.handle, p_buf, n_size);
        CResult::Ok(res)
    }
    /// 输入流数据。
    /// nPort
    /// [in] 播放通道号
    /// pBuf
    /// [in] 流数据缓冲区地址
    /// nSize
    /// [in] 流数据缓冲区大小
    pub unsafe extern "C" fn start(&self, hwnd: HWND) -> CResult<bool> {
        // # 开始解码播放
        let func: Symbol<'_, unsafe extern "C" fn(nPort: c_long, hWnd: HWND) -> bool> =
            auto_any_res_c!(self.lib().get(b"PlayM4_Play\0"));
        let res = func(self.handle, hwnd);
        CResult::Ok(res)
    }
    /// 设置或增加显示区域。
    /// nPort
    /// [in] 播放通道号
    /// nRegionNum
    /// [in] 显示区域序号，0~(MAX_DISPLAY_WND-1)，设定哪个窗口进行区域显示。如果nRegionNum为0，表示对主要显示窗口（PlayM4_Play中设置的窗口）进行设置，将忽略hDestWnd和bEnable的设置
    /// pSrcRect
    /// [in] 设置在要显示的原始图像上的区域，如：如果原始图像是352*288，那么pSrcRect可设置的范围只能在（0，0，352，288）之中。如果pSrcRect=NULL,将显示整个图像
    /// hDestWnd
    /// [in] 设置显示窗口。如果该区域的窗口已经设置过（打开过），那么该参数被忽略
    /// bEnable
    /// [in] 打开（设置）或关闭显示区域
    pub unsafe extern "C" fn set_display_region(
        &self,
        region_num: c_uint,
        src_rect: *const RECT,
        hwnd: HWND,
        enable: bool,
    ) -> CResult<bool> {
        // # 开始解码播放
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                nPort: c_long,
                nRegionNum: c_uint,
                pSrcRect: *const RECT,
                hDestWnd: HWND,
                bEnable: bool,
            ) -> bool,
        > = auto_any_res_c!(self.lib().get(b"PlayM4_SetDisplayRegion\0"));
        let res = func(self.handle, region_num, src_rect, hwnd, enable);
        CResult::Ok(res)
    }
    /// 打开流。
    /// nPort
    /// [in] 播放通道号
    /// pFileHeadBuf
    /// [in] 文件头数据
    /// nSize
    /// [in] 文件头长度
    /// nBufPoolSize
    /// [in] 设置播放器中存放数据流的缓冲区大小。范围是SOURCE_BUF_MIN~ SOURCE_BUF_MAX。该值过小会导致无法解码。
    /// 宏定义 宏定义值 含义
    /// SOURCE_BUF_MIN 1024*50 缓冲数据流缓冲最小值  
    /// SOURCE_BUF_MAX 1024*100000 缓冲数据流缓冲最大值  
    pub unsafe extern "C" fn open_stream(
        &self,
        file_head_buf: *const c_uchar,
        size: c_uint,
        buf_pool_size: c_uint,
    ) -> CResult<bool> {
        // # 开始解码播放
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                nPort: c_long,
                pFileHeadBuf: *const c_uchar,
                nSize: c_uint,
                nBufPoolSize: c_uint,
            ) -> bool,
        > = auto_any_res_c!(self.lib().get(b"PlayM4_OpenStream\0"));
        let res = func(self.handle, file_head_buf, size, buf_pool_size);
        CResult::Ok(res)
    }

    /// 设置流播放模式。
    /// nPort
    /// [in] 播放通道号
    /// nMode
    /// [in] 流播放模式，如下所示：
    /// 宏定义 宏定义值 含义
    /// STREAME_REALTIME 0 此模式（默认）下, 会尽量保正实时性, 防止数据阻塞; 而且数据检查严格  
    /// STREAME_FILE 1 此模式下按时间戳播放  
    pub unsafe extern "C" fn set_stream_open_mode(
        &self,
        mode: PlayStreamOpenMode,
    ) -> CResult<bool> {
        let func: Symbol<'_, unsafe extern "C" fn(nPort: c_long, nMode: c_uint) -> bool> =
            auto_any_res_c!(self.lib().get(b"PlayM4_SetStreamOpenMode\0"));
        let res = func(self.handle, mode as c_uint);
        CResult::Ok(res)
    }

    ///解码回调（带目标数据和数据大小和用户指针）。
    /// nPort
    /// [out] 播放器通道号
    /// pBuf
    /// [out] 解码后的音视频数据指针
    /// nSize
    /// [out] 解码后的音视频数据pBuf的长度
    /// pFrameInfo
    /// [out] 图像和声音信息结构体指针
    /// nUser
    /// [out] 用户数据
    /// nReserved2
    /// [out] 保留参数
    /// pDest
    /// [in] 目标数据，暂时设置为NULL
    /// nDestSize
    /// [in] 目标数据大小，暂时设置为0
    /// nUser
    /// [in] 用户指针
    pub unsafe extern "C" fn set_dec_callback_exmend(
        &self,
        dec_cb_fun: DecCBFunWin,
        dest: *const c_void,
        dest_size: c_long,
        user: *const c_void,
    ) -> CResult<bool> {
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                nPort: c_long,
                fDecCBFun: DecCBFunWin,
                pDest: *const c_void,
                nDestSize: c_long,
                nUser: *const c_void,
            ) -> bool,
        > = auto_any_res_c!(self.lib().get(b"PlayM4_SetDecCallBackExMend\0"));
        let res = func(self.handle, dec_cb_fun, dest, dest_size, user);
        CResult::Ok(res)
    }

    ///图像数据转为JPEG 格式。
    /// pBuf
    /// [in] 抓图回调函数中图像缓冲区
    /// nSize
    /// [in] 抓图回调函数中图像的大小
    /// nWidth
    /// [in] 抓图回调函数中图像宽度
    /// nHeight
    /// [in] 抓图回调函数中图像高度
    /// nType
    /// [in] 抓图回调函数中图像类型，（当前的播放库获取的类型是yv12）
    /// sFileName
    /// [in] 要保存的文件名，最好以JPG作为文件扩展名
    pub unsafe extern "C" fn convert_to_jpeg_file(
        &self,
        buf: *const c_char,
        size: c_long,
        width: c_long,
        height: c_long,
        ntype: c_long,
        fname: *const c_char,
    ) -> CResult<bool> {
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                pBuf: *const c_char,
                nSize: c_long,
                nWidth: c_long,
                nHeight: c_long,
                nType: c_long,
                sFileName: *const c_char,
            ) -> bool,
        > = auto_any_res_c!(self.lib().get(b"PlayM4_ConvertToJpegFile\0"));
        let res = func(buf, size, width, height, ntype, fname);
        CResult::Ok(res)
    }

    /// # 直接抓取JPEG图像。
    ///```
    /// Parameters
    /// nPort
    /// [in] 播放通道号
    /// pJpeg
    /// [in] 存放JEPG图像数据地址，由用户分配，不得小于JPEG图像大小，建议大小w * h * 3/2， 其中w和h分别为图像宽高
    /// nBufSize
    /// [in] 申请的缓冲区大小
    /// pJpegSize
    /// [out] 获取到的实际JPEG图像数据大小
    /// Return Values
    /// 成功返回TRUE；失败返回FALSE。获取错误码调用PlayM4_GetLastError。
    /// Remarks
    /// 获取的数据为一帧JPEG数据，写成文件即可用图片浏览工具查看。
    /// pbuf所指向的内容在函数里面不能被修改。
    ///```
    pub unsafe extern "C" fn get_jepg(
        &self,
        play_handle: c_long,
        jpeg: *mut c_uchar,
        buf_size: c_uint,
        jpeg_size: *mut c_uint,
    ) -> CResult<bool> {
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                nPort: c_long,
                pJpeg: *mut c_uchar,
                nBufSize: c_uint,
                pJpegSize: *mut c_uint,
            ) -> bool,
        > = auto_any_res_c!(self.lib().get(b"PlayM4_GetJPEG\0"));
        let res = func(play_handle, jpeg, buf_size, jpeg_size);
        CResult::Ok(res)
    }
}
