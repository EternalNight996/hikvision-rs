#![allow(non_snake_case)]

/// 网络SDK
pub mod net;
/// 播放SDK
pub mod play;

pub const HCNET_LIB_ENV: &'static str = "HCNET_LIB";
pub const HCNET_COM_LIB_ENV: &'static str = "HCNET_COM_LIB";

#[cfg(target_os = "windows")]
pub const HCNET_CORE_LIB: &'static str = "HCNetSDK.dll";
#[cfg(target_os = "linux")]
pub const HCNET_CORE_LIB: &'static str = "libhcnetsdk.so";

#[cfg(target_os = "windows")]
pub const HCPLAY_CORE_LIB: &'static str = "PlayCtrl.dll";
#[cfg(target_os = "linux")]
pub const HCPLAY_CORE_LIB: &'static str = "libPlayCtrl.so";

#[cfg(target_os = "windows")]
pub const HCNET_LIBEAY_LIB: &'static str = "libcrypto-1_1-x64.dll";
#[cfg(target_os = "linux")]
pub const HCNET_LIBEAY_LIB: &'static str = "libcrypto.so.1.1";

#[cfg(target_os = "windows")]
pub const HCNET_SSLEAY_LIB: &'static str = "libssl-1_1-x64.dll";
#[cfg(target_os = "linux")]
pub const HCNET_SSLEAY_LIB: &'static str = "libssl.so.1.1";

/// Serializer for function pointer
pub mod function_login_callback_pointer_serializer {
  use serde::{Deserialize, Deserializer, Serializer};

  use crate::net_sdk::net::FLoginResultCallBack;

  /// Serializer for function pointer
  pub fn serialize<S>(func_ptr: &FLoginResultCallBack, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let func_ptr_address = unsafe { std::mem::transmute::<_, usize>(*func_ptr) };
    serializer.serialize_u64(func_ptr_address as u64)
  }
  /// Serializer for function pointer
  pub fn deserialize<'de, D>(deserializer: D) -> Result<FLoginResultCallBack, D::Error>
  where
    D: Deserializer<'de>,
  {
    let func_ptr_address: u64 = Deserialize::deserialize(deserializer)?;
    let func_ptr =
      unsafe { std::mem::transmute::<usize, FLoginResultCallBack>(func_ptr_address as usize) };
    Ok(func_ptr)
  }
}
