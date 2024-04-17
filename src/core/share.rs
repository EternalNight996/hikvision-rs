use e_utils::Result;
use libloading::Library;
use std::{ffi::OsStr, path::PathBuf};

/// 加载动态链接库
pub fn load_library<S>(dll_path: S) -> Result<Library>
where
  S: AsRef<OsStr>,
{
  // 加载动态链接库
  unsafe { libloading::Library::new(dll_path) }.map_err(|e| e.to_string().into())
}

/// LIB
#[derive(Default)]
pub struct Lib {
  lib_path: PathBuf,
  lib: Option<Library>,
}
unsafe impl Send for Lib {}
unsafe impl Sync for Lib {}
impl Lib {
  /// 新构建
  pub fn new(lib_path: PathBuf) -> Self {
    let lib = load_library(lib_path.clone()).ok();
    Self { lib_path, lib }
  }
  /// 设置链接库
  pub fn set_lib(&mut self, lib: Option<Library>) {
    self.lib = lib
  }
  /// 设置路径
  pub fn set_path(&mut self, path: PathBuf) {
    self.lib_path = path
  }
  /// 释放lib
  pub fn free(&mut self) -> Option<()> {
    self.lib.take()?.close().ok()
  }
}
impl Lib {
  /// 强制解包
  pub fn get(&self) -> &Library {
    self
      .lib
      .as_ref()
      .expect(&format!("{} 链接库无法找到", self.lib_path.display(),))
  }
  /// 获取路径
  pub fn get_path(&self) -> PathBuf {
    self.lib_path.clone()
  }
}
