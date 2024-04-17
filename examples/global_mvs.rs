use e_utils::parse::{AutoParse as _, ParseResult as _};
use e_utils::system::env_path_join;
use e_utils::Result;
use hikvision::mvs_sdk::HcMvsCoreSdk;
use hikvision::net_sdk::net::{HcNetCoreSdk, HcnetLevel, NetDvrLocalSdkPath, NetSdkInitCfgType};
use hikvision::net_sdk::play::HcNetPlayCoreSdk;
use hikvision::net_sdk::{
    HCNET_CORE_LIB, HCNET_LIBEAY_LIB, HCNET_LIB_ENV, HCNET_SSLEAY_LIB, HCPLAY_CORE_LIB,
};
use hikvision::{
    mvs_sdk::constant::{HCMVS_CAMERA_CONTROL_LIB, HCMVS_LIB_ENV},
    Lib,
};

use lazy_static::lazy_static;
use std::env::set_current_dir;
use std::sync::{Arc, RwLock};
mod config;
use config::Config;

lazy_static! {
  // Global MVS Camera sdk
  pub static ref HC_MVS_CORE_SDK: Arc<RwLock<HcMvsCoreSdk>> =
    Arc::new(RwLock::new(HcMvsCoreSdk::default()));
}

fn main() -> Result<()> {
    e_utils::system::init_original_dir()?;
    let cf = Config::new();
    cf.init_env()?;
    init_mvs_sdk()?;
    HC_MVS_CORE_SDK.read().res()?.lib();
    HC_MVS_CORE_SDK.write().res()?.free().unwrap();
    println!("MVS SDK OK");
    Ok(())
}

/// Init SDK
pub fn init_mvs_sdk() -> Result<()> {
    set_current_dir(std::env::var(HCMVS_LIB_ENV)?)?;
    let lib = Lib::new(env_path_join(HCMVS_LIB_ENV, HCMVS_CAMERA_CONTROL_LIB)?);
    HC_MVS_CORE_SDK.write().res()?.set_lib(lib);
    Ok(())
}

/// Init net sdk
pub fn init_net_sdk<S: AsRef<str>>(log_dir: S, level: HcnetLevel) -> Result<()> {
    set_current_dir(std::env::var(HCNET_LIB_ENV)?)?;
    let lib = Lib::new(env_path_join(HCNET_LIB_ENV, HCNET_CORE_LIB)?);
    let mut sdk = HC_NET_CORE_SDK.write().res()?;
    sdk.set_lib(lib);
    init_hikvision_sdk(&mut sdk, log_dir, level)?;
    drop(sdk);
    let lib = Lib::new(env_path_join(HCNET_LIB_ENV, HCPLAY_CORE_LIB)?);
    let mut sdk = HC_NET_PLAY_CORE_SDK.write().res()?;
    sdk.set_lib(lib);
    Ok(())
}

/// 初始化项目
pub fn init_hikvision_sdk<S: AsRef<str>>(
    sdk: &mut HcNetCoreSdk,
    log_dir: S,
    level: HcnetLevel,
) -> Result<()> {
    let add_env = |a: &str| -> Result<String> {
        Ok(env_path_join(HCNET_LIB_ENV, a)?.to_str().res()?.to_string())
    };
    unsafe {
        let fpath = add_env(HCNET_CORE_LIB)?;
        let sdk_path = NetDvrLocalSdkPath::new(fpath).res()?;
        let res = sdk
            .init_cfg(NetSdkInitCfgType::NetSdkInitCfgSdkPath(sdk_path))
            .res()?;
        if res {
            println!("NET_DVR_SetSDKInitCfg load");
        } else {
            eprintln!("NET_DVR_SetSDKInitCfg load");
            return Err("NetSdkInitCfgSdkPath".into());
        }
        if cfg!(target_os = "windows") {
            let path = add_env(HCNET_LIBEAY_LIB)?.to_c_char();
            let res = sdk
                .init_cfg(NetSdkInitCfgType::NetSdkInitCfgLibeayPath(path))
                .res()?;
            println!("NET_DVR_SetSDKInitCfg load {}: {}", HCNET_LIBEAY_LIB, res);
            if !res {
                return Err("NetSdkInitCfgLibeayPath".into());
            }
            let path = add_env(HCNET_SSLEAY_LIB)?.to_c_char();
            let res = sdk
                .init_cfg(NetSdkInitCfgType::NetSdkInitCfgSsleayPath(path))
                .res()?;
            println!("NET_DVR_SetSDKInitCfg load {}: {}", HCNET_SSLEAY_LIB, res);
            if !res {
                return Err("NetSdkInitCfgSsleayPath".into());
            }
        } else if cfg!(target_os = "linux") {
            let path = add_env(HCNET_LIBEAY_LIB)?.to_c_char();
            let res = sdk
                .init_cfg(NetSdkInitCfgType::NetSdkInitCfgLibeayPath(path))
                .res()?;
            println!("NET_DVR_SetSDKInitCfg load {}: {}", HCNET_LIBEAY_LIB, res);
            if !res {
                return Err("NetSdkInitCfgLibeayPath".into());
            }
            let path = add_env(HCNET_SSLEAY_LIB)?.to_c_char();
            let res = sdk
                .init_cfg(NetSdkInitCfgType::NetSdkInitCfgSsleayPath(path))
                .res()?;
            println!("NET_DVR_SetSDKInitCfg load {}: {}", HCNET_SSLEAY_LIB, res);
            if !res {
                return Err("NetSdkInitCfgSsleayPath".into());
            }
        } else {
            return Err(e_utils::Error::Unsupport("不支持当前平台".into()));
        }
        let res = sdk.init_sdk().res()?;
        if res {
            println!("net_dvr_init_sdk");
        } else {
            eprintln!("net_dvr_init_sdk");
            return Err("net_dvr_init_sdk".into());
        }
        let res = sdk
            .set_log_to_file_sdk(log_dir.as_ref().to_c_char(), level, true)
            .res()?;
        if res {
            println!("net_dvr_set_log_to_file_sdk");
        } else {
            eprintln!("net_dvr_set_log_to_file_sdk");
            return Err("net_dvr_set_log_to_file_sdk".into());
        }
        Ok(())
    }
}
