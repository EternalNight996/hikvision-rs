use e_log::Level;
use e_utils::parse::{AutoJson as _, MyParseFormat as _};
use e_utils::Result;
use serde::{Deserialize, Serialize};
use std::{env, path::Path};

pub const ORIGN_KEY: &str = "HIKVISION";

#[derive(Clone, Debug, Deserialize, Default, Serialize, e_utils::Json)]
#[serde(rename_all = "camelCase")]
pub struct MyEnv {
  envs: Vec<MyEnvData>,
}
#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct MyEnvData {
  pub key: String,
  pub value: String,
}
impl MyEnv {
  pub fn new<P>(conf_path: P) -> Result<Self>
  where
    P: AsRef<Path>,
  {
    let conf = serde_json::from_value::<MyEnv>(conf_path.as_ref().auto_read_json()?)?;
    let envs: Vec<MyEnvData> = conf
      .envs
      .into_iter()
      .map(|mut x| {
        x.value = x.value.parse_all().unwrap();
        x
      })
      .collect();
    Ok(MyEnv { envs })
  }
  #[allow(unused)]
  pub fn get(&self, key: &str) -> Option<&MyEnvData> {
    self.envs.iter().find(|x| x.key == key)
  }
  pub fn init(&self) -> Result<()> {
    let res = self
      .envs
      .iter()
      .map(|x| {
        if env::var(&x.key).is_err() {
          env::set_var(&x.key, &x.value);
        }
        let v = env::var(&x.key);
        Level::Info.add(
          format!("env var[{},{:?}]", x.key, v),
          "MyEnv::init".to_owned(),
        );
        v
      })
      .find(|x| x.is_err());
    if let Some(x) = res {
      x?;
    };
    Ok(())
  }
}
