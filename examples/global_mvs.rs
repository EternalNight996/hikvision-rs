use e_utils::parse::ParseResult as _;
use e_utils::system::env_path_join;
use e_utils::Result;
use hikvision::mvs_sdk::constant::{HCMVS_CAMERA_CONTROL_LIB, HCMVS_LIB_ENV};
use hikvision::mvs_sdk::HcMvsCoreSdk;
use hikvision::Lib;
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
    println!("Unload HC_MVS_CORE_SDK");
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
