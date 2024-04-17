<img src="../public/ico/white_64x64.ico" alt="e-utils"/>

### 📄 [中文](README.zh.md)  | 📄  [English](../README.md)

# ⚡ 这是什么?
**这是一个通用功能库，集成了便捷功能**

### 支持 功能

<table style="background:#000">
  <tr>
    <th><h3 style="color:#fff">基础功能</h3></th>
    <th><h3 style="color:#fff">Windows 10</h3></th>
    <th><h3 style="color:#fff">Unix</h3></th>
    <th><h3 style="color:#fff">Macos</h3></th>
  </tr>
  <tr>
    <td>Uuid</td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
  </tr>
  <tr>
    <td>Base64</td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
  </tr>
  <tr>
    <td>Algorithm</td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
  </tr>
  <tr>
    <td>GUI</td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:red">×</h4></td>
    <td><h4 style="color:red">×</h4></td>
  </tr>
  <tr>
    <td>Image</td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
  </tr>
  <tr>
    <td>_</td>
    <td><h4 style="color:red">×</h4></td>
    <td><h4 style="color:red">×</h4></td>
    <td><h4 style="color:red">×</h4></td>
  </tr>
</table>

# ✨ 分支
```toml
[features]
std = []
alloc = []
fs = []
uuid_v4 = ["uuid/v4", "uuid/fast-rng", "uuid/macro-diagnostics", "uuid"]
base64 = []
algorithm = ["rand"]
ui = ["e-macros"]
macros = ["e-macros"]
images = ["image"]
http = ["reqwest"]
http-blocking = ["reqwest/blocking"]
http-json = ["reqwest/json"]
default = ["std"]
```

# 📖 示例
```toml
[dependencies]
e-utils = {version="0.3.0", feature=["algorithm"]}
```

```Rust
 // Exmaple `Nanoid`
 fn main() {
     use e_utils::algorithm;
     println!("nanoid -> {}", algorithm!(nanoid));
     println!("nanoid 16bytes -> {}", algorithm!(nanoid 16));
     println!("nanoid 16bytes -> {}", algorithm!(nanoid 16));
     println!("nanoid 10bytes [alphabet:expr] -> {}", algorithm!(nanoid 16, &['1', 'b', 'c', '7']));
     println!("nanoid unsafe 10bytes -> {}", algorithm!(nanoid unsafe 10));
     println!("nanoid unsafe 10bytes [alphabet:expr]-> {}", algorithm!(nanoid unsafe 10, &['1','0']));
 }
 // Exmaple `algorithm`
 fn main2() {
    use e_utils::algorithm;
    println!("random bool -> {}", algorithm!());
    println!("random type -> {}", algorithm!(#u32));
    println!("random type[] -> {:?}", algorithm!([u32; 10]));
    println!("random range 0-13 -> {}", algorithm!(13));
    println!("random range -> {}", algorithm!(0i32..13));
    println!("random rgb range -> {:?}", algorithm!(rgb 100,255));
 }
```

```toml
[dependencies]
e-utils = {version="0.3.0", feature=["algorithm","images"]}
```

### 示例把内存中的图像提取出来并转换成base64
```Rust
use std::path::PathBuf;
use e_utils::{
  algorithm,
  images::ImageSource,
  parse::{AutoPath as _, ParseResult as _},
  Result,
};
use serde_json::{json, Value};
use super::{SnPicture, Store};

/// 处理图像数据
pub fn store_save_image<T>(
  pic_buf_ptr: *const T,
  buf_size: usize,
  cache_dir: PathBuf,
  store: &Store,
) -> Result<Value> {
  // 从内存中获取数据
  let reader = unsafe { ImageSource::from_raw_parts_reader(pic_buf_ptr, buf_size) }?;
  let iformat = reader.format().res()?;
  let suffix = iformat.extensions_str();
  let mime_type = iformat.to_mime_type();
  let image = reader.decode().map_err(|e| e.to_string())?;
  let nanoid = algorithm!(nanoid 12);

  cache_dir.auto_create_dir()?;
  let fpath = cache_dir.join(format!("{nanoid}.{}", suffix[0]));
  image
    .save_with_format(&fpath, iformat)
    .map_err(|e| e.to_string())?;
  let base64_str = ImageSource::image_to_base64(&image, iformat)?;
  let data = Value::String(format!("data:{mime_type};base64,{}", base64_str));
  let id = format!("image:{nanoid}");

  let _ = store.lock().res()?.insert(
    id.clone(),
    serde_json::to_value(&SnPicture {
      fpath,
      suffix: suffix[0].to_string(),
      mime_type: mime_type.to_string(),
    })?,
  );
  Ok(json!({"key": id, "value": data}))
}
```

## `💡!重要：`
#### xxx
<!-- 您必须使用使用MSVC工具链的Rust版本
您必须安装[WinPcap](https://www.winpcap.org/)或[npcap](https://nmap.org/npcap/)（使用[WinPcap](https://www.winpcap.org/) 4.1.3版进行测试）（如果使用[npcap](https://nmap.org/npcap/)，请确保使用“在[WinPcap](https://www.winpcap.org/) API兼容模式下安装[npcap](https://nmap.org/npcap/)”）
你必须把它放在包里。[WinPcap](https://www.winpcap.org/)开发者包中的lib位于该存储库根目录中名为lib的目录中。或者，您可以使用%LIB%/$Env:LIB环境变量中列出的任何位置。对于64位工具链，它位于WpdPack/Lib/x64/Packet中。对于32位工具链，它位于WpdPack/lib/Packet.lib中。
```
# 1.安装npcap服务 https://npcap.com/dist/npcap-1.70.exe
setx LIB E:\libs\LIB
# 下载并解压 https://npcap.com/dist/npcap-sdk-1.13.zip
# 将npcap-sdk-1.13\Lib\x64\Packet.lib放到E:\libs\LIB
``` -->

# 🚀 快速运行
<!-- ```sh
# 主机/端口扫描
cargo run --example host_scan
cargo run --example port_scan
``` -->


# 🦊 已运用项目
<!-- [E-NetScan](https://github.com/EternalNight996/e-netscan.git): 网络扫描项目（同时支持命令行与跨平台图形化界面）正在开发中。。 -->

# 🔭 为什么需要e-utils?
<!-- 起初是想完成一个跨网络扫描项目，帮助自己完成一些工作，参考许多开源项目,但这些项目多少有些缺陷并不满足自己需求，所以有了e-libscanner。
(处理主机和端口扫描，同时支持域名解析、路由跟踪、指纹扫描、服务扫描、异步扫描、可扩展更多)
底层是通过调用[npcap](https://nmap.org/npcap/)与[WinPcap](https://www.winpcap.org/)抓包服务；
服务api为[libpnet](https://github.com/libpnet/libpnet); -->

# 🙋 参考项目与资料
<!-- ✨[RustScan](https://github.com/RustScan/RustScan) :Rust仿nmap扫描库
✨[netscan](https://github.com/shellrow/netscan) :Rust 网络扫描库
✨[libpnet](https://github.com/libpnet/libpnet) 跨平台网络底层库--主要是调用抓包服务([npcap](https://nmap.org/npcap/)与[WinPcap](https://www.winpcap.org/)) -->