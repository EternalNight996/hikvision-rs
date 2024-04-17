use super::types::*;
use crate::Lib;
use e_utils::{auto_any_res_c, CResult};
use libloading::{Library, Symbol};
use std::{ffi::*, ptr};

#[repr(C)]
pub struct HcMvsCoreSdk {
    handle: *mut c_void,
    lib: Lib,
    devlist: MvCcDeviceInfoList,
}
impl HcMvsCoreSdk {
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
impl Default for HcMvsCoreSdk {
    fn default() -> Self {
        Self {
            handle: ptr::null_mut(),
            lib: Lib::default(),
            devlist: MvCcDeviceInfoList::default(),
        }
    }
}

// 基础配置
impl HcMvsCoreSdk {
    /// 设置设备列表
    pub fn set_devlist(&mut self, devlist: MvCcDeviceInfoList) {
        self.devlist = devlist;
    }
    /// 获取设备列表
    pub fn get_devlist(&self) -> &MvCcDeviceInfoList {
        &self.devlist
    }
}
unsafe impl Send for HcMvsCoreSdk {}
unsafe impl Sync for HcMvsCoreSdk {}
// 设备的基本指令和操作
impl HcMvsCoreSdk {
    /// # ch:获取Integer属性值     | en:
    /// ```
    /// 参数
    /// handle [IN] 设备句柄  
    /// strKey [IN] 属性键值，如获取宽度信息则为"Width"  
    /// pstIntValue [IN][OUT] 返回给调用者有关设备属性结构体指针  
    ///
    /// 备注
    /// 连接设备之后调用该接口可以获取int类型的指定节点的值，具体可以查看客户端属性描述，如下图所示：
    /// ```
    pub unsafe extern "C" fn get_int_value_ex(
        &self,
        key: *const c_char,
        intvalue: *mut MvCcIntValueEx,
    ) -> CResult<c_int> {
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                handle: *const c_void,
                strKey: *const c_char,
                pstIntValue: *mut MvCcIntValueEx,
            ) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_GetIntValueEx\0"));
        CResult::Ok(func(self.handle, key, intvalue))
    }
    /// # ch:使用内部缓存获取一帧图片（与 MV_CC_Display() 不能同时使用）    | en:
    /// ```
    /// 参数
    /// handle [IN] 设备句柄  
    /// pstFrame [OUT] 图像数据和图像信息  
    /// nMsec [IN] 等待超时时间，输入INFINITE时表示无限等待，直到收到一帧数据或者停止取流
    ///
    /// 备注
    ///  • 调用该接口获取图像数据帧之前需要先调用 MV_CC_StartGrabbing() 启动图像采集。该接口为主动式获取帧数据，上层应用程序需要根据帧率，控制好调用该接口的频率。该接口支持设置超时时间，SDK内部等待直到有数据时返回，可以增加取流平稳性，适合用于对平稳性要求较高的场合。
    ///  • 该接口与 MV_CC_FreeImageBuffer() 配套使用，当处理完取到的数据后，需要用 MV_CC_FreeImageBuffer() 接口将pstFrame内的数据指针权限进行释放。
    ///  • 该接口与 MV_CC_GetOneFrameTimeout() 相比，有着更高的效率。且其取流缓存的分配是由sdk内部自动分配的，而 MV_CC_GetOneFrameTimeout() 接口是需要客户自行分配。
    ///  • 该接口在调用 MV_CC_Display() 后无法取流。
    ///  • 该接口对于U3V、GIGE设备均可支持。
    ///  • 该接口不支持CameraLink设备。
    /// ```
    pub unsafe extern "C" fn get_image_buffer(
        &self,
        frame: *mut MvFrameOut,
        msec: c_uint,
    ) -> CResult<c_int> {
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                handle: *const c_void,
                pstFrame: *mut MvFrameOut,
                nMsec: c_uint,
            ) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_GetImageBuffer\0"));
        CResult::Ok(func(self.handle, frame, msec))
    }
    /// #释放图像缓存(此接口用于释放不再使用的图像缓存，与 MV_CC_GetImageBuffer() 配套使用)
    /// ```
    /// 参数
    /// handle [IN] 设备句柄  
    /// pstFrame [IN] 图像数据和图像数据  
    ///
    /// 备注
    /// • 该接口与 MV_CC_GetImageBuffer() 配套使用，使用 MV_CC_GetImageBuffer() 接口取到的图像数据pstFrame，需要用 MV_CC_FreeImageBuffer() 接口进行权限释放。
    ///  • 该接口对于取流效率高于GetOneFrameTimeout接口，且GetImageBuffer在不进行Free的情况下，最大支持输出的节点数与SetImageNode接口所设置的节点数相等，默认节点数是1。
    ///  • 该接口对于U3V、GIGE设备均可支持。
    ///  • 该接口不支持CameraLink设备。
    /// ```
    pub unsafe extern "C" fn free_image_buffer(&self, frame: *mut MvFrameOut) -> CResult<c_int> {
        let func: Symbol<
            '_,
            unsafe extern "C" fn(handle: *const c_void, pstFrame: *mut MvFrameOut) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_FreeImageBuffer\0"));
        CResult::Ok(func(self.handle, frame))
    }
    /// # ch:打开获取或设置相机参数的GUI界面   | en:
    pub unsafe extern "C" fn open_params_gui(&self) -> CResult<c_int> {
        // MV_CAMCTRL_API int __stdcall MV_CC_OpenParamsGUI  ( IN void *  handle )
        let func: Symbol<'_, unsafe extern "C" fn(handle: *const c_void) -> c_int> =
            auto_any_res_c!(self.lib().get(b"MV_CC_OpenParamsGUI\0"));
        CResult::Ok(func(self.handle))
    }
    /// # ch:获取SDK版本号 | en:Get SDK Version
    pub unsafe extern "C" fn get_sdk_version(&self) -> CResult<c_uint> {
        // # C原型：unsigned int __stdcall MV_CC_GetSDKVersion();
        let func: Symbol<'_, unsafe extern "C" fn() -> c_uint> =
            auto_any_res_c!(self.lib().get(b"MV_CC_GetSDKVersion\0"));
        CResult::Ok(func())
    }
    /// # ch:获取支持的传输层 | en:Get supported Transport Layer
    pub unsafe extern "C" fn enumrate_tls(&self) -> CResult<c_int> {
        // # C原型：int __stdcall MV_CC_EnumerateTls();
        let func: Symbol<'_, unsafe extern "C" fn() -> c_int> =
            auto_any_res_c!(self.lib().get(b"MV_CC_EnumerateTls\0"));
        CResult::Ok(func())
    }
    /// # ch:枚举设备 | en:Enumerate Device
    /// ```text
    /// 参数
    /// nTLayerType [IN] 枚举传输层
    /// 按位表示，支持复选，可选协议类型如下所示：
    /// 宏定义 宏定义值 含义
    /// MV_UNKNOW_DEVICE  0x00000000  未知设备类型
    /// MV_GIGE_DEVICE  0x00000001  GigE设备
    /// MV_1394_DEVICE  0x00000002  1394-a/b设备
    /// MV_USB_DEVICE  0x00000004  USB3.0设备
    /// MV_CAMERALINK_DEVICE  0x00000008  CameraLink设备
    /// 例如：nTLayerType = MV_GIGE_DEVICE | MV_USB_DEVICE ，表示查找GigE和USB3.0设备
    /// pstDevList [OUT] 设备列表
    /// strManufacturerName [IN] 厂商名字
    /// ```
    pub unsafe extern "C" fn enumrate_devices(
        &self,
        layer_type: MvEnumDeviceLayerType,
        out_dev_list: *mut MvCcDeviceInfoList,
    ) -> CResult<c_int> {
        // # C原型:int __stdcall MV_CC_EnumDevicesEx2(unsigned int nTLayerType, MV_CC_DEVICE_INFO_LIST* pstDevList)
        let func: Symbol<
            '_,
            unsafe extern "C" fn(nTLayerType: c_uint, pstDevList: *mut MvCcDeviceInfoList) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_EnumDevices\0"));
        CResult::Ok(func(layer_type as c_uint, out_dev_list))
    }
    /// # ch:枚举设备 | en:Enumerate Device
    /// ```text
    /// 参数
    /// nTLayerType [IN] 枚举传输层
    /// 按位表示，支持复选，可选协议类型如下所示：
    /// 宏定义 宏定义值 含义
    /// MV_UNKNOW_DEVICE  0x00000000  未知设备类型
    /// MV_GIGE_DEVICE  0x00000001  GigE设备
    /// MV_1394_DEVICE  0x00000002  1394-a/b设备
    /// MV_USB_DEVICE  0x00000004  USB3.0设备
    /// MV_CAMERALINK_DEVICE  0x00000008  CameraLink设备
    /// 例如：nTLayerType = MV_GIGE_DEVICE | MV_USB_DEVICE ，表示查找GigE和USB3.0设备
    /// pstDevList [OUT] 设备列表
    /// strManufacturerName [IN] 厂商名字
    /// ```
    pub unsafe extern "C" fn enumrate_devices_ex2(
        &self,
        layer_type: MvEnumDeviceLayerType,
        out_dev_list: *mut MvCcDeviceInfoList,
        manufacturer_mame: *const c_char,
        sort_method: MvSortMethod,
    ) -> CResult<c_int> {
        // # C原型:int __stdcall MV_CC_EnumDevicesEx2(unsigned int nTLayerType, MV_CC_DEVICE_INFO_LIST* pstDevList)
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                nTLayerType: c_uint,
                pstDevList: *mut MvCcDeviceInfoList,
                strManufacturerName: *const c_char,
                enSortMethod: MvSortMethod,
            ) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_EnumDevicesEx2\0"));
        CResult::Ok(func(
            layer_type as c_uint,
            out_dev_list,
            manufacturer_mame,
            sort_method,
        ))
    }
    /// # ch:设备是否可达 | en:Is the device accessible
    /// ```
    /// 备注 • 读取设备CCP寄存器的值，判断当前状态是否具有某种访问权限。
    ///  • 如果设备不支持MV_ACCESS_ExclusiveWithSwitch、MV_ACCESS_ControlWithSwitch、MV_ACCESS_ControlSwitchEnableWithKey这三种模式，接口返回false。目前设备不支持这3种抢占模式，国际上主流的厂商的设备也都暂不支持这3种模式。
    /// ```
    pub unsafe extern "C" fn is_device_accessible(
        &self,
        dev_info: &MvCcDeviceInfo,
        access_mode: MvAccessMode,
    ) -> CResult<bool> {
        // # C原型：bool __stdcall MV_CC_IsDeviceAccessible(IN MV_CC_DEVICE_INFO* pstDevInfo, IN unsigned int nAccessMode);
        let func: Symbol<
            '_,
            unsafe extern "C" fn(pstDevInfo: *const MvCcDeviceInfo, nAccessMode: c_uint) -> bool,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_IsDeviceAccessible\0"));
        CResult::Ok(func(dev_info, access_mode as c_uint))
    }
    /// # ch:创建设备句柄 | en:Create Device Handle
    /// ```
    /// 参数
    /// handle [OUT] 设备句柄  
    /// pstDevInfo [IN] 设备信息结构体
    ///
    /// 根据输入的设备信息，创建库内部必须的资源和初始化内部模块。通过该接口创建句柄，调用SDK接口，会默认生成SDK日志文件，如果不需要生成日志文件，可以通过 MV_CC_CreateHandleWithoutLog() 创建句柄。
    /// ```
    pub unsafe extern "C" fn create_handle(&mut self, dev_info: &MvCcDeviceInfo) -> CResult<c_int> {
        // # C原型:int MV_CC_CreateHandle(void ** handle, MV_CC_DEVICE_INFO* pstDevInfo)
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                handle: *mut *mut c_void,
                pstDevInfo: *const MvCcDeviceInfo,
            ) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_CreateHandle\0"));
        let res = func(&mut self.handle, dev_info);
        CResult::Ok(res)
    }
    /// # ch:创建设备句柄，不生成日志  | en:Create Device Handle
    /// ```
    /// 参数
    /// handle [OUT] 设备句柄  
    /// pstDevInfo [IN] 设备信息结构体
    ///
    /// 根据输入的设备信息，创建库内部必须的资源和初始化内部模块。通过该接口创建句柄，调用SDK接口，会默认生成SDK日志文件，如果不需要生成日志文件，可以通过 MV_CC_CreateHandleWithoutLog() 创建句柄。
    /// ```
    pub unsafe extern "C" fn create_handle_without_log(
        &mut self,
        dev_info: &MvCcDeviceInfo,
    ) -> CResult<c_int> {
        // # C原型:int MV_CC_CreateHandle(void ** handle, MV_CC_DEVICE_INFO* pstDevInfo)
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                handle: *mut *mut c_void,
                pstDevInfo: *const MvCcDeviceInfo,
            ) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_CreateHandleWithoutLog\0"));
        let res = func(&mut self.handle, dev_info);
        CResult::Ok(res)
    }

    /// # ch:销毁设备句柄 | en:Destroy Device Handle
    /// ```
    /// 参数
    /// handle [OUT] 设备句柄  
    /// pstDevInfo [IN] 设备信息结构体
    ///
    /// 根据输入的设备信息，创建库内部必须的资源和初始化内部模块。通过该接口创建句柄，调用SDK接口，会默认生成SDK日志文件，如果不需要生成日志文件，可以通过 MV_CC_CreateHandleWithoutLog() 创建句柄。
    /// ```
    pub unsafe extern "C" fn destroy_handle(&self) -> CResult<c_int> {
        // # C原型:int MV_CC_CreateHandleWithoutLog(void ** handle, MV_CC_DEVICE_INFO* pstDevInfo)
        let func: Symbol<'_, unsafe extern "C" fn(handle: *const c_void) -> c_int> =
            auto_any_res_c!(self.lib().get(b"MV_CC_DestroyHandle\0"));
        CResult::Ok(func(self.handle))
    }
    /// # ch:打开设备 | en:Open Device
    /// ```
    ///参数
    /// handle [IN] 设备句柄  
    ///
    /// nAccessMode [IN] 访问权限
    /// MV_ACCESS_Exclusive  1  独占权限，其他APP只允许读CCP寄存器  
    /// MV_ACCESS_ExclusiveWithSwitch  2  可以从5模式下抢占权限，然后以独占权限打开  
    /// MV_ACCESS_Control  3  控制权限，其他APP允许读所有寄存器  
    /// MV_ACCESS_ControlWithSwitch  4  可以从5的模式下抢占权限，然后以控制权限打开  
    /// MV_ACCESS_ControlSwitchEnable  5  以可被抢占的控制权限打开  
    /// MV_ACCESS_ControlSwitchEnableWithKey  6  可以从5的模式下抢占权限，然后以可被抢占的控制权限打开  
    /// MV_ACCESS_Monitor  7  读模式打开设备，适用于控制权限下
    ///
    /// nSwitchoverKey [IN] 切换访问权限时的密钥
    /// ```
    pub unsafe extern "C" fn open_device(
        &self,
        access_mode: MvAccessMode,
        switchover_key: c_ushort,
    ) -> CResult<c_int> {
        //# C原型:int MV_CC_OpenDevice(void* handle, unsigned int nAccessMode, unsigned short nSwitchoverKey)
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                handle: *const c_void,
                nAccessMode: c_uint,
                nSwitchoverKey: c_ushort,
            ) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_OpenDevice\0"));
        CResult::Ok(func(self.handle, access_mode as c_uint, switchover_key))
    }
    /// # ch:关闭设备 | en:Close Device
    /// ```
    ///参数
    /// handle [IN] 设备句柄  
    /// ```
    pub unsafe extern "C" fn close_device(&self) -> CResult<c_int> {
        let func: Symbol<'_, unsafe extern "C" fn(handle: *const c_void) -> c_int> =
            auto_any_res_c!(self.lib().get(b"MV_CC_CloseDevice\0"));
        CResult::Ok(func(self.handle))
    }
    /// # ch:判断设备是否处于连接状态 | en: Is The Device Connected
    /// ```
    ///参数
    /// handle [IN] 设备句柄  
    /// ```
    pub unsafe extern "C" fn is_device_connected(&self) -> CResult<bool> {
        let func: Symbol<'_, unsafe extern "C" fn(handle: *const c_void) -> bool> =
            auto_any_res_c!(self.lib().get(b"MV_CC_IsDeviceConnected\0"));
        CResult::Ok(func(self.handle))
    }
    /// # ch:注册图像数据回调 | en:Register the image callback function
    /// ```
    /// 参数
    /// handle [IN] 设备句柄  
    /// cbOutput [IN] 回调函数指针  
    /// pUser [IN] 用户自定义变量  
    /// 返回成功，返回MV_OK；失败，返回错误码
    /// 备注 • 通过该接口可以设置图像数据回调函数，在 MV_CC_CreateHandle() 之后即可调用。
    ///  • 图像数据采集有两种方式，两种方式不能复用：
    /// 1. 调用 MV_CC_RegisterImageCallBackEx() 设置图像数据回调函数，然后调用 MV_CC_StartGrabbing() 开始采集，采集的图像数据在设置的回调函数中返回。
    /// 2. 调用 MV_CC_StartGrabbing() 开始采集，然后在应用层循环调用 MV_CC_GetOneFrameTimeout() 获取指定像素格式的帧数据，获取帧数据时上层应用程序需要根据帧率控制好调用该接口的频率。
    ///  • 该接口不支持CameraLink设备。
    /// 示例ChunkData.cpp, Grab_Callback.cpp , 以及 LensShadingCorrection.cpp.
    /// ```
    pub unsafe extern "C" fn register_image_callback_ex(
        &self,
        cb_output_callback: CbOutputCallback,
        user: *const c_void,
    ) -> CResult<c_int> {
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                handle: *const c_void,
                cbOutput: CbOutputCallback,
                pUser: *const c_void,
            ) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_RegisterImageCallBackEx\0"));
        CResult::Ok(func(self.handle, cb_output_callback, user))
    }
    /// # ch:注册取流回调 | en:Register the image callback function
    /// ```
    /// 参数
    /// handle [IN] 设备句柄  
    /// cbOutput [IN] 回调函数指针  
    /// pUser [IN] 用户自定义变量  
    /// 返回成功，返回MV_OK；失败，返回错误码
    /// 备注 • 通过该接口可以设置图像数据回调函数，在 MV_CC_CreateHandle() 之后即可调用。
    ///  • 图像数据采集有两种方式，两种方式不能复用：
    /// 1. 调用 MV_CC_RegisterImageCallBackEx() 设置图像数据回调函数，然后调用 MV_CC_StartGrabbing() 开始采集，采集的图像数据在设置的回调函数中返回。
    /// 2. 调用 MV_CC_StartGrabbing() 开始采集，然后在应用层循环调用 MV_CC_GetOneFrameTimeout() 获取指定像素格式的帧数据，获取帧数据时上层应用程序需要根据帧率控制好调用该接口的频率。
    ///  • 该接口不支持CameraLink设备。
    /// 示例ChunkData.cpp, Grab_Callback.cpp , 以及 LensShadingCorrection.cpp.
    /// ```
    pub unsafe extern "C" fn register_image_callback_for_rgb(
        &self,
        cb_output_callback: CbOutputCallback,
        user: *const c_void,
    ) -> CResult<c_int> {
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                handle: *const c_void,
                cbOutput: CbOutputCallback,
                pUser: *const c_void,
            ) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_RegisterImageCallBackForRGB\0"));
        CResult::Ok(func(self.handle, cb_output_callback, user))
    }
    /// # ch:注册取流回调 | en:Register the image callback function
    /// ```
    /// 参数
    /// handle [IN] 设备句柄  
    /// cbOutput [IN] 回调函数指针  
    /// pUser [IN] 用户自定义变量  
    /// 返回成功，返回MV_OK；失败，返回错误码
    /// 备注 • 通过该接口可以设置图像数据回调函数，在 MV_CC_CreateHandle() 之后即可调用。
    ///  • 图像数据采集有两种方式，两种方式不能复用：
    /// 1. 调用 MV_CC_RegisterImageCallBackEx() 设置图像数据回调函数，然后调用 MV_CC_StartGrabbing() 开始采集，采集的图像数据在设置的回调函数中返回。
    /// 2. 调用 MV_CC_StartGrabbing() 开始采集，然后在应用层循环调用 MV_CC_GetOneFrameTimeout() 获取指定像素格式的帧数据，获取帧数据时上层应用程序需要根据帧率控制好调用该接口的频率。
    ///  • 该接口不支持CameraLink设备。
    /// 示例ChunkData.cpp, Grab_Callback.cpp , 以及 LensShadingCorrection.cpp.
    /// ```
    pub unsafe extern "C" fn register_image_callback_for_bgr(
        &self,
        cb_output_callback: CbOutputCallback,
        user: *const c_void,
    ) -> CResult<c_int> {
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                handle: *const c_void,
                cbOutput: CbOutputCallback,
                pUser: *const c_void,
            ) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_RegisterImageCallBackForBGR\0"));
        CResult::Ok(func(self.handle, cb_output_callback, user))
    }

    /// # ch:开始取流 | en:Start Grabbing
    /// ```
    /// 参数
    /// handle [IN] 设备句柄
    ///
    /// 示例ChunkData.cpp, ConnectSpecCamera.cpp, ConvertPixelType.cpp, Events.cpp, Grab_ActionCommand.cpp, Grab_Callback.cpp, GrabImage.cpp, GrabImage_Display.cpp, GrabStrategies.cpp, HighBandwidthDecode.cpp, LensShadingCorrection.cpp, MultiCast.cpp, Recording.cpp , 以及 SavePointCloudData_3D.cpp.
    /// ```
    pub unsafe extern "C" fn start_grabbing(&self) -> CResult<c_int> {
        let func: Symbol<'_, unsafe extern "C" fn(handle: *const c_void) -> c_int> =
            auto_any_res_c!(self.lib().get(b"MV_CC_StartGrabbing\0"));
        CResult::Ok(func(self.handle))
    }
    /// # ch:停止取流 | en:Stop Grabbing
    /// ```
    /// 参数
    /// handle [IN] 设备句柄
    ///
    /// 示例ChunkData.cpp, ConnectSpecCamera.cpp, ConvertPixelType.cpp, Events.cpp, Grab_ActionCommand.cpp, Grab_Callback.cpp, GrabImage.cpp, GrabImage_Display.cpp, GrabStrategies.cpp, HighBandwidthDecode.cpp, LensShadingCorrection.cpp, MultiCast.cpp, Recording.cpp , 以及 SavePointCloudData_3D.cpp.
    /// ```
    pub unsafe extern "C" fn stop_grabbing(&self) -> CResult<c_int> {
        let func: Symbol<'_, unsafe extern "C" fn(handle: *const c_void) -> c_int> =
            auto_any_res_c!(self.lib().get(b"MV_CC_StopGrabbing\0"));
        CResult::Ok(func(self.handle))
    }
    /// # ch:探测网络最佳包大小(只对GigE相机有效) | en:Detection network optimal package size(It only works for the GigE camera)
    /// ```
    /// 参数
    /// handle [IN] 设备句柄
    ///
    /// 备注
    ///  • 获取最佳的packet size，对应GigEVision设备是SCPS，对应U3V设备是每次从驱动读取的包大小，该大小即网络上传输一个包的大小。该接口需要在 MV_CC_OpenDevice() 之后、 MV_CC_StartGrabbing() 之前调用。
    ///  • 该接口不支持CameraLink设备
    /// ```
    pub unsafe extern "C" fn get_optimal_packet_size(&self) -> CResult<c_int> {
        let func: Symbol<'_, unsafe extern "C" fn(handle: *const c_void) -> c_int> =
            auto_any_res_c!(self.lib().get(b"MV_CC_GetOptimalPacketSize\0"));
        CResult::Ok(func(self.handle))
    }
    /// # ch:设置Integer型属性值  | en:
    /// ```
    /// 参数
    /// handle [IN] 设备句柄  
    /// strKey [IN] 属性键值，如获取宽度信息则为"Width"  
    /// nValue [IN] 想要设置的相机的属性值  
    /// ```
    pub unsafe extern "C" fn set_int_value(
        &self,
        key: *const c_char,
        value: c_uint,
    ) -> CResult<c_int> {
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                handle: *const c_void,
                strKey: *const c_char,
                nValue: c_uint,
            ) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_SetIntValue\0"));
        CResult::Ok(func(self.handle, key, value))
    }
    /// # ch:获取Boolean属性值   | en:
    /// ```
    /// 参数
    /// handle [IN] 设备句柄  
    /// strKey [IN] 属性键值  
    /// pbValue [OUT] 返回给调用者有关设备属性值  
    /// ```
    pub unsafe extern "C" fn get_bool_value(
        &self,
        key: *const c_char,
        value: *mut bool,
    ) -> CResult<c_int> {
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                handle: *const c_void,
                strKey: *const c_char,
                pbValue: *mut bool,
            ) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_GetBoolValue\0"));
        CResult::Ok(func(self.handle, key, value))
    }
    /// # ch:设置Enum型属性值    | en:
    /// ```
    /// 参数
    /// handle [IN] 设备句柄  
    /// strKey [IN] 属性键值，如设置像素格式信息则为"PixelFormat"  
    /// nValue [IN] 想要设置的设备的属性值
    ///
    /// 备注
    /// 连接设备之后调用该接口可以设置Enum类型的指定节点的值。
    /// ```
    pub unsafe extern "C" fn set_enum_value(
        &self,
        key: *const c_char,
        value: c_uint,
    ) -> CResult<c_int> {
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                handle: *const c_void,
                strKey: *const c_char,
                pbValue: c_uint,
            ) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_SetEnumValue\0"));
        CResult::Ok(func(self.handle, key, value))
    }
    /// # 设置SDK日志路径
    /// ```
    /// 参数
    /// strSDKLogPath [IN] SDK日志路径  
    /// 返回成功，返回MV_OK；失败，返回错误码 。 备注 • 设置路径之后，可以指定路径存放日志。
    ///  • v2.4.1版本新增日志服务，开启服务之后该接口无效，默认日志服务为开启状态。
    /// ```
    pub unsafe extern "C" fn set_sdk_log_path(&self, path: *const c_char) -> CResult<c_int> {
        let func: Symbol<'_, unsafe extern "C" fn(strSDKLogPath: *const c_char) -> c_int> =
            auto_any_res_c!(self.lib().get(b"MV_CC_SetSDKLogPath\0"));
        CResult::Ok(func(path))
    }
    /// # 显示图像，注册显示窗口，内部自动显示（与 MV_CC_GetImageBuffer() 不能同时使用）
    /// ```
    /// 参数
    /// handle [IN] 句柄  
    /// hWnd [IN] 显示窗口句柄  
    /// ```
    pub unsafe extern "C" fn display(&self, hwnd: *mut c_void) -> CResult<c_int> {
        // MV_CAMCTRL_API int __stdcall MV_CC_Display  ( IN void *  handle,  void *  hWnd  )
        let func: Symbol<
            '_,
            unsafe extern "C" fn(handle: *const c_void, hWnd: *mut c_void) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_Display\0"));
        CResult::Ok(func(self.handle, hwnd))
    }
    /// # 显示一帧图像
    /// ```
    /// 参数
    /// handle [IN] 设备句柄  
    /// pstDisplayInfo [IN] 图像信息  
    /// ```
    pub unsafe extern "C" fn display_one_frame(
        &self,
        display_info: &MvDisplayFrameInfo,
    ) -> CResult<c_int> {
        // MV_CAMCTRL_API int __stdcall MV_CC_DisplayOneFrame  ( IN void *  handle,  IN MV_DISPLAY_FRAME_INFO *  pstDisplayInfo  )
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                handle: *const c_void,
                pSaveParam: *const MvDisplayFrameInfo,
            ) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_DisplayOneFrame\0"));
        CResult::Ok(func(self.handle, display_info))
    }
    /// # 保存图像到文件
    /// ```
    /// 参数
    /// handle [IN] 设备句柄  
    /// pstSaveParam [IN][OUT] 保存图片参数结构体
    /// 备注
    ///  • 通过将接口可以将从设备采集到的原始图像数据转换成JPEG或者BMP等格式并存放在指定内存中，然后用户可以将转换之后的数据直接保存成图片文件。该接口调用无接口顺序要求，有图像源数据就可以进行转换，可以先调用图像采集 接口，获取一帧图像数据，然后再通过该接口转换格式。
    ///  • MV_CC_SaveImageEx2() 比 MV_CC_SaveImageEx() 增加参数handle，为了保证与其他接口的统一。
    /// ```
    pub unsafe extern "C" fn save_image_to_file(
        &self,
        file_param: *mut MvSaveImgToFileParam,
    ) -> CResult<c_int> {
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                handle: *const c_void,
                pstSaveFileParam: *mut MvSaveImgToFileParam,
            ) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_SaveImageToFile\0"));
        CResult::Ok(func(self.handle, file_param))
    }
    /// # 保存图片，支持Bmp和Jpeg.编码质量在50-99之前
    /// ```
    /// 参数
    /// handle [IN] 设备句柄  
    /// pstSaveParam [IN][OUT] 保存图片参数结构体
    /// 备注
    ///  • 通过将接口可以将从设备采集到的原始图像数据转换成JPEG或者BMP等格式并存放在指定内存中，然后用户可以将转换之后的数据直接保存成图片文件。该接口调用无接口顺序要求，有图像源数据就可以进行转换，可以先调用图像采集 接口，获取一帧图像数据，然后再通过该接口转换格式。
    ///  • MV_CC_SaveImageEx2() 比 MV_CC_SaveImageEx() 增加参数handle，为了保证与其他接口的统一。
    /// ```
    pub unsafe extern "C" fn save_image_ex2(
        &self,
        save_param: *mut MvSaveImageParamEx,
    ) -> CResult<c_int> {
        // MV_CAMCTRL_API int __stdcall MV_CC_DisplayOneFrame  ( IN void *  handle,  IN MV_DISPLAY_FRAME_INFO *  pstDisplayInfo  )
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                handle: *const c_void,
                pSaveParam: *mut MvSaveImageParamEx,
            ) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_SaveImageEx2\0"));
        CResult::Ok(func(self.handle, save_param))
    }
    /// # 采用超时机制获取一帧图片，SDK内部等待直到有数据时返回
    /// ```
    /// 参数
    /// handle [IN] 设备句柄  
    /// pData [OUT] 图像数据接收指针  
    /// nDataSize [IN] 接收缓存大小  
    /// pstFrameInfo [OUT] 图像信息结构体  
    /// nMsec [IN] 等待超时时间  
    /// 备注
    ///  • 调用该接口获取图像数据帧之前需要先调用 MV_CC_StartGrabbing() 启动图像采集。该接口为主动式获取帧数据，上层应用程序需要根据帧率，控制好调用该接口的频率。该接口支持设置超时时间，SDK内部等待直到有数据时返回，可以增加取流平稳性，适合用于对平稳性要求较高的场合。
    ///  • 该接口对于U3V、GIGE设备均可支持。
    ///  • 该接口不支持CameraLink设备。
    /// ```
    pub unsafe extern "C" fn get_one_frame_timeout(
        &self,
        data: *mut c_uchar,
        data_size: c_uint,
        frame_info: *mut MvFrameOutInfoEx,
        timeout: c_uint,
    ) -> CResult<c_int> {
        let func: Symbol<
            '_,
            unsafe extern "C" fn(
                handle: *const c_void,
                pData: *mut c_uchar,
                nDataSize: c_uint,
                pstFrameInfo: *mut MvFrameOutInfoEx,
                nMsec: c_uint,
            ) -> c_int,
        > = auto_any_res_c!(self.lib().get(b"MV_CC_GetOneFrameTimeout\0"));
        CResult::Ok(func(self.handle, data, data_size, frame_info, timeout))
    }
}
