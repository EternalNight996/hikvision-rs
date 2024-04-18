#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use e_utils::parse::AutoParse as _;
use serde::{Deserialize, Serialize};
use std::ffi::*;

/// 错误结构
#[repr(C)]
#[derive(Deserialize, Serialize, Debug, e_utils::C, e_utils::Json)]
pub struct PlayM4LastError {
  ///
  pub code: c_ushort,
  ///
  #[serde(with = "e_utils::parse::c_str_pointer_serializer")]
  pub msg: *const c_char,
}
impl Default for PlayM4LastError {
  fn default() -> Self {
    Self {
      code: 0,
      msg: "".to_c_char(),
    }
  }
}
/// 定义预览参数结构体
#[repr(C)]
#[derive(Deserialize, Serialize, Debug)]
pub struct FrameInfo {
  pub nWidth: c_long,
  pub nHeight: c_long,
  pub nStamp: c_long,
  pub nType: c_long,
  pub nFrameRate: c_long,
  pub dwFrameNum: c_uint,
}

/// 解码回调函数类型定义
pub type DecCBFunWin = extern "C" fn(
  nPort: c_long,
  pBuf: *const c_char,
  pSize: c_long,
  pFrameInfo: *const FrameInfo,
  nUser: c_long,
  nReserved2: c_long,
) -> *const c_void;

/// 播放模式
#[repr(u8)]
#[derive(Debug)]
pub enum PlayStreamOpenMode {
  StreameRealtime = 0,
  StreameFile = 1,
}
