<img src="public/ico/white_64x64.ico" alt="e-utils"/>

### 📄 [中文](docs/README.zh.md)  | 📄  [English](README.md)

# ⚡ what this ?
**This is a Hikvision Camera Rust SDK that supports both civilian network cameras and industrial cameras**

### Support app
<table style="background:#000">
  <tr>
    <th><h3 style="color:#fff">APP</h3></th>
    <th><h3 style="color:#fff">Windows 10</h3></th>
    <th><h3 style="color:#fff">Unix</h3></th>
    <th><h3 style="color:#fff">Macos</h3></th>
    <th><h3 style="color:#fff">IOS</h3></th>
    <th><h3 style="color:#fff">Android</h3></th>
  </tr>
  <tr>
    <td>MVS</td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:red">x</h4></td>
    <td><h4 style="color:red">x</h4></td>
    <td><h4 style="color:red">x</h4></td>
    <td><h4 style="color:red">x</h4></td>
  </tr>
  <tr>
    <td>Network</td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:red">x</h4></td>
    <td><h4 style="color:red">x</h4></td>
    <td><h4 style="color:red">x</h4></td>
    <td><h4 style="color:red">x</h4></td>
  </tr>
  <tr>
    <td>_</td>
    <td><h4 style="color:red">×</h4></td>
    <td><h4 style="color:red">×</h4></td>
    <td><h4 style="color:red">×</h4></td>
    <td><h4 style="color:red">x</h4></td>
    <td><h4 style="color:red">x</h4></td>
  </tr>
</table>

# ✨ Features
```toml
[features]
net_sdk = []
mvs_sdk = []
default = []

```

# 📖 Example
```toml
[dependencies]
e-utils = {version="0.3.0", feature=["algorithm"]}
```

```rust
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

### Example Image save from Memory then switch to base64
```rust
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

## `💡!important：`
<!-- ####There are three requirements for building on the windows system environment:

You must use the rust version using the MSVC toolchain

You must install [WinPcap]（ https://www.winpcap.org/ ）Or [npcap]（ https://nmap.org/npcap/ ）(using [WinPcap]（ https://www.winpcap.org/ ）Version 4.1.3) (if using [npcap]（ https://nmap.org/npcap/ ）, please make sure to use "in [WinPcap]（ https://www.winpcap.org/ ）Install [npcap] in API compatibility mode（ https://nmap.org/npcap/ )”）

You must put it in your bag. [WinPcap]( https://www.winpcap.org/ ）The Lib in the developer package is located in the directory named Lib in the root directory of the repository. Alternatively, you can use any location listed in the% lib% / $env: lib environment variable. For the 64 bit toolchain, it is located in wpdpack / lib / x64 / packet. For the 32-bit toolchain, it is located in wpdpack / lib / packet.lib.
```
# 1.install npcap server https://npcap.com/dist/npcap-1.70.exe
setx LIB E:\libs\LIB
# download and decompression https://npcap.com/dist/npcap-sdk-1.13.zip
# npcap-sdk-1.13\Lib\x64\Packet.lib put to E:\libs\LIB
``` -->

# 🚀 fast running
<!-- ```sh
# host/port scan
cargo run --example host_scan
cargo run --example port_scan
# async scan
cargo run --example async_host_scan --features="async"
cargo run --example async_port_scan --features="async"
# Fingerprint scan
cargo run --example os --features="os"
# Service Scan
cargo run --example service_detection --features="service"
# DNS Parse
cargo run --example dns
# Trace Route
cargo run --example tracert
``` -->

# 🦊 Applied Projects
<!-- [E-NetScan](https://github.com/EternalNight996/e-netscan.git): The network scanning project (which supports both command line and cross platform graphical interface) is under development.. -->

# 🔭 why need to e-utils?
<!-- At first, I wanted to complete a cross network scanning project to help me complete some work. I referred to many open source projects, but these projects have some defects that do not meet my needs, so I have e-libscanner.
(process host and port scanning, and support domain name resolution, route tracking, fingerprint scanning, service scanning, asynchronous scanning, scalability and more)
The bottom layer is by calling [npcap]（ https://nmap.org/npcap/ ）And [WinPcap]（ https://www.winpcap.org/ ）Packet capture service;
The service API is [libpnet]（ https://github.com/libpnet/libpnet ); -->

# 🙋 Reference items and materials
<!-- ✨ [RustScan] https://github.com/RustScan/RustScan : Rustlike nmapscan
✨ [netscan] https://github.com/shellrow/netscan : Rust Network Scanner
✨ [libpnet](libpnet) https://github.com/libpnet/libpnet ● The background base of the interplatform network - mainly using captivity services ([npcap]) https://nmap.org/npcap/ with [WinPcap]( https://www.winpcap.org/ ) -->