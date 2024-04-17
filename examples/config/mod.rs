#![allow(unused)]
use e_log::_log::TauriBuilder;
use e_log::{init_layer_log, Level};
use e_utils::parse::{AutoJson as _, AutoPath as _, MyParseFormat as _};
use e_utils::system::{get_original_dir, init_original_dir};
use e_utils::ui::UiConfig;
use e_utils::{Error, Result};
use hikvision::mvs_sdk::types::{MvSaveIamgeMethodValue, MvSaveIamgeType};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
pub mod env;
use self::env::MyEnv;
pub mod tag;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct CameraLoginInfo {
  pub host: String,
  pub port: u16,
  pub uname: String,
  pub passwd: String,
}
impl core::fmt::Display for CameraLoginInfo {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut info = self.clone();
    info.passwd = "*".repeat(info.passwd.len());
    write!(f, "{}", info.to_string())
  }
}

#[derive(Deserialize, Debug, Clone, Default, Serialize, e_utils::Json)]
pub struct CpuConf {
  // Read from environment variable first in multi-threaded mode.
  // Default to lazy auto-detection (one thread per CPU core)
  pub core: usize,
}
#[derive(Deserialize, Debug, Clone, Default, Serialize, e_utils::Json)]
#[serde(rename_all = "camelCase")]
pub struct CameraMvsConf {
  pub quality: u32,
  pub image_type: MvSaveIamgeType,
  pub method_value: MvSaveIamgeMethodValue,
  pub m_sec: u64,
}
#[derive(Deserialize, Debug, Clone, Default, Serialize, e_utils::Json)]
#[serde(rename_all = "camelCase")]
pub struct CameraNetConf {
  pub buf_size: usize,
  pub max_cache: usize,
}
#[derive(Deserialize, Debug, Clone, Default, Serialize, e_utils::Json)]
#[serde(rename_all = "camelCase")]
pub struct BiosConf {
  pub sn_rule: String,
  pub sn_len: usize,
  pub board_sn_rule: String,
  pub board_sn_len: usize,
}
#[derive(Deserialize, Debug, Clone, Serialize)]
pub enum CameraApiType {
  HikNet,
  HikMvs,
  Unknow,
}
impl Default for CameraApiType {
  fn default() -> Self {
    Self::Unknow
  }
}
#[derive(Deserialize, Debug, Clone, Default, Serialize, e_utils::Json)]
#[serde(rename_all = "camelCase")]
pub struct CameraConf {
  pub ui: UiConfig,
  pub login_info: CameraLoginInfo,
  pub log_dir: String,
  pub cache_dir: String,
  pub auto_capture: bool,
  pub api_type: Option<CameraApiType>,
  pub mvs: CameraMvsConf,
  pub net: CameraNetConf,
}

/// 窗口主题
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum WindowThemeType {
  Mask,
  Line,
  Cat,
  Auto,
}
impl Default for WindowThemeType {
  fn default() -> Self {
    Self::Auto
  }
}
/// 主程序窗口配置
#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct WindowConf {
  /// 窗口主题
  pub theme: WindowThemeType,
  /// 窗口UI配置
  pub ui: UiConfig,
}
/// 主程序包信息
#[derive(Deserialize, Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PackageConf {
  /// 名称
  pub product_name: String,
  /// 版本
  pub version: String,
}
/// Mes信息
#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct MesSettingsConf {
  /// 站位名
  #[serde(rename = "StationName")]
  pub station_name: String,
  /// Prod域名
  #[serde(rename = "ProdUrl")]
  pub prod_url: String,
  /// Sys域名
  #[serde(rename = "SysUrl")]
  pub sys_url: String,
  /// 令牌名
  #[serde(rename = "TokenName")]
  pub tname: String,
  /// 用户名
  #[serde(rename = "UserName")]
  pub uname: String,
  /// 密码
  #[serde(rename = "PassWord")]
  pub passwd: String,
  /// 公司标识
  #[serde(rename = "CauseNo")]
  pub cause_no: String,
  /// 工代号
  #[serde(rename = "WorkCenter")]
  pub work_center: String,
  /// 测试机台id
  #[serde(rename = "TestFrameidNo")]
  pub test_frameid_no: String,
  /// 机台编号
  #[serde(rename = "WksNo")]
  pub wks_no: String,
}
/// Mes配置
#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct MesConf {
  /// Mes配置
  #[serde(rename = "MESSettings")]
  pub settings: MesSettingsConf,
  pub key: String,
  pub enable: bool,
  pub ignores: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, Default, Serialize, e_utils::Json)]
#[serde(rename_all = "camelCase")]
pub struct Conf {
  pub package: PackageConf,
  pub window: WindowConf,
  pub log: LogConf,
  pub camera: CameraConf,
}
#[derive(Deserialize, Debug, Clone, Default, Serialize, e_utils::Json)]
#[serde(rename_all = "camelCase")]
pub struct LogConf {
  pub level: Level,
  pub folder: String,
  pub fname: String,
  pub format: String,
  pub output_list: Vec<String>,
}

/// 全局配置
#[derive(Clone, Debug, Serialize, Deserialize, e_utils::Json)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  pub conf: Conf,
  pub env: MyEnv,
}
impl Default for Config {
  fn default() -> Self {
    let home = current_dir().unwrap();
    let conf: Conf = home
      .join(ConfigType::Main.to_string())
      .auto_read_json()
      .unwrap();
    // 解析Env.json环境变量
    let myenv = MyEnv::new(home.join(ConfigType::Env.to_string())).unwrap();
    println!("{:?}", myenv);
    Self { conf, env: myenv }
  }
}
/// 配置类型
pub enum ConfigType {
  /// 默认Conf.json
  Main,
  /// 默认 Env.json
  Env,
}
impl ToString for ConfigType {
  fn to_string(&self) -> String {
    match self {
      ConfigType::Main => "Conf.json",
      ConfigType::Env => "Env.json",
    }
    .to_string()
  }
}

impl Config {
  /// 新
  pub fn new() -> Self {
    Self::default()
  }
  /// 筛选
  pub fn parse_conf<S: AsRef<str>>(&self, value: S) -> Result<String> {
    let match_fn = |k: String| -> String {
      match &*k {
        "version" => self.conf.package.version.clone(),
        "name" => self.get_product_name().unwrap_or_default(),
        _ => String::new(),
      }
    };
    value.as_ref().parse_replace('#', '#', match_fn)
  }
  /// 获取日志文件
  pub fn get_product_name(&self) -> Result<String> {
    Ok(
      self
        .parse_conf(&self.conf.package.product_name)?
        .parse_all()?
        .to_uppercase(),
    )
  }
  /// 获取日志文件
  pub fn get_window_title(&self) -> Result<String> {
    Ok(
      self
        .parse_conf(&self.conf.window.ui.title)?
        .parse_all()?
        .to_uppercase(),
    )
  }
  /// 获取摄像头Ui标题
  #[allow(unused)]
  pub fn get_camera_title(&self) -> Result<String> {
    Ok(
      self
        .parse_conf(&self.conf.camera.ui.title)?
        .parse_all()?
        .to_uppercase(),
    )
  }
  /// 获取摄像头日志目录
  pub fn get_camera_log_folder(&self) -> Result<String> {
    self.parse_conf(&self.conf.camera.log_dir)?.parse_all()
  }
  /// 获取摄像头临时目录
  pub fn get_camera_cache_folder(&self) -> Result<String> {
    self.parse_conf(&self.conf.camera.cache_dir)?.parse_all()
  }
  /// 获取日志目录
  pub fn get_log_folder(&self) -> Result<String> {
    self.parse_conf(&self.conf.log.folder)?.parse_all()
  }
  /// 获取日志文件
  pub fn get_log_fname(&self) -> Result<String> {
    self.parse_conf(&self.conf.log.fname)?.parse_all()
  }

  /// 清空日志
  pub fn clean_log(&self) -> Result<PathBuf> {
    let folder = self.get_log_folder()?;
    let fname = self.get_log_fname()?;
    let target = Path::new(&folder).join(fname);
    if Path::new(&folder).exists() {
      OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&target)?
        .set_len(0)?;
      Ok(target)
    } else {
      Err(Error::NotFound(folder.into()))
    }
  }
  /// 写入配置
  #[allow(unused)]
  pub fn dump(&self, ctype: ConfigType) -> Result<()> {
    let home = get_original_dir()?;
    let ctype_str = ctype.to_string();
    match ctype {
      ConfigType::Main => self.conf.auto_write_json(home, ctype_str)?,
      ConfigType::Env => self.env.auto_write_json(home, ctype_str)?,
    }
    Ok(())
  }
  /// 读取并同步配置
  #[allow(unused)]
  pub fn sync(&mut self, ctype: ConfigType) -> Result<()> {
    let target = get_original_dir()?.join(ctype.to_string());
    match ctype {
      ConfigType::Main => self.conf = Conf::auto_read_json(target)?,
      ConfigType::Env => self.env = MyEnv::auto_read_json(target)?,
    }
    Ok(())
  }
  /// 读取配置
  #[allow(unused)]
  pub fn read<R>(&self, ctype: ConfigType) -> Result<R>
  where
    R: DeserializeOwned,
  {
    let home = get_original_dir()?;
    let value = home.join(ctype.to_string()).auto_read_json::<R>()?;
    Ok(value)
  }
  /// 添加日志
  #[inline]
  pub fn init_layer_log(&self) -> Result<TauriBuilder> {
    let folder = self.get_log_folder()?;
    let fname = self.get_log_fname()?;
    folder.auto_create_dir()?;
    let log = init_layer_log(
      Some(fname),
      folder,
      self.conf.log.level,
      self.conf.log.format.clone(),
      self.conf.log.output_list.clone(),
    )?;
    Ok(log)
  }
  /// 初始化环境
  pub fn init_env(&self) -> Result<()> {
    self.env.init()
  }
}
