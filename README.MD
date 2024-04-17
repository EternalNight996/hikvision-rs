<img src="public/ico/white_64x64.ico" alt="e-utils"/>

### 📄 [中文](docs/README.zh.md)  | 📄  [English](README.md)

# ⚡ what this ?
**This is the Hikvision Camera Rust SDK, which supports universal network cameras, universal USB cameras, IoT cameras, and industrial cameras (USB, network, CamL)**

### Support SDK
<table style="background:#000">
  <tr>
    <th><h3 style="color:#fff"><center>SDK</center></h3></th>
    <th><h3 style="color:#fff"><center>Windows10</center></h3></th>
    <th><h3 style="color:#fff"><center>Unix</center></h3></th>
    <th><h3 style="color:#fff"><center>-</center></h3></th>
  </tr>
  <tr>
    <td><center>MVS</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:red"><center>X</center></h4></td>
    <td><h4 style="color:#aaa"><center>Industrial Capture Camera (USB、CamL、GigE)</center></h4></td>
  </tr>
  <tr>
    <td><center>NET</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:red"><center>X</center></h4></td>
    <td><h4 style="color:#aaa"><center>Universal Network Camera</center></h4></td>
  </tr>
  <tr>
      <td><center>USB</center></td>
      <td><h4 style="color:red"><center>X</center></h4></td>
      <td><h4 style="color:red"><center>X</center></h4></td>
      <td><h4 style="color:#aaa"><center>USB Camera</center></h4></td>
  </tr>
  <tr>
      <td><center>OTAP</center></td>
      <td><h4 style="color:red"><center>X</center></h4></td>
      <td><h4 style="color:red"><center>X</center></h4></td>
      <td><h4 style="color:#aaa"><center>Open Things Access Protocol</center></h4></td>
  </tr>
</table>

### SDK Support API
<table style="background:#000">
  <tr>
    <th><h3 style="color:#fff"><center>API</center></h3></th>
    <th><h3 style="color:#fff"><center>MVS</center></h3></th>
    <th><h3 style="color:#fff"><center>NET</center></h3></th>
    <th><h3 style="color:#fff"><center>USB</center></h3></th>
    <th><h3 style="color:#fff"><center>OTAP</center></h3></th>
    <th><h3 style="color:#fff"><center>-</center></h3></th>
  </tr>
  <tr>
    <td><center>Init SDK</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>Init SDK</center></h4></td>
  </tr>
  <tr>
    <td><center>Enumerate</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:red"><center>X</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>Enumerate Device</center></h4></td>
  </tr>
  <tr>
    <td><center>Login</center></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>Login Device</center></h4></td>
  </tr>
  <tr>
    <td><center>Open</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>Open Device</center></h4></td>
  </tr>
  <tr>
    <td><center>Handle</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>Crete Device Handle</center></h4></td>
  </tr>
  <tr>
    <td><center>Log</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>Log output</center></h4></td>
  </tr>
  <tr>
    <td><center>Clean SDK</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>Clean SDK</center></h4></td>
  </tr>
  <tr>
    <td><center>Clean All</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>Clean All</center></h4></td>
  </tr>
  <tr>
    <td><center>Destroy All</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>Destroy All</center></h4></td>
  </tr>
  <tr>
    <td><center>Callback Sream</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>Callback Sream</center></h4></td>
  </tr>
  <tr>
    <td><center>Sream</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>Data Sream</center></h4></td>
  </tr>
  <tr>
    <td><center>Fix Network</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:red"><center>X</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>Fix Network packet size</center></h4></td>
  </tr>
  <tr>
    <td><center>Preview</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>Display Camera Image</center></h4></td>
  </tr>
  <tr>
    <td><center>Capture Image</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>Capture Image</center></h4></td>
  </tr>
  </tr>
  <tr>
    <td><center>Save JPEG</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>Save One JPEG</center></h4></td>
  </tr>
  <tr>
    <td><center>GUI</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>GUI Window</center></h4></td>
  </tr>
</table>

# ✨ Features
```toml
[features]
net = []
mvs = []
default = []
```

# 📖 Example
```toml
[dependencies]
hikvision = {version="0.1", feature=["mvs","net"]}
```

### Examples
[Init Mvs SDK](examples/global_mvs.rs) | [Init Net SDK](examples/global_net.rs) 


## `💡!important：`
1. [Env.json](Examples/Env.json) **The Lib Path of System Env; `{origin}` of init app current path**
```json
{
  "envs": [
    {
      "key": "HCNET_LIB",
      "value": "{origin}/libs/HCNet/lib"
    },
    {
      "key": "HCNET_COM_LIB",
      "value": "{origin}/libs/HCNet/lib/HCNetSDKCom"
    },
    {
      "key": "HCMVS_LIB",
      "value": "{origin}/libs/HCMvs/Win64_x64"
    }
  ]
}
```
2. The Hikvision Camera Lib of hikvision-rs
```
# put the lib to libs/HCMvs OR libs/HCNet 
cargo run --example global_mvs
cargo run --example global_net
```
3. add your Cargo.toml 
```toml
[dependencies]
hikvision = {version="0.1", feature=["mvs","net"]}
```

# 🚀 fast running
```sh
# test global mvs init sdk
cargo run --example global_mvs

# test global net init sdk
cargo run --example global_net
```

# 🦊 Applied Projects
<!-- [E-NetScan](https://github.com/EternalNight996/e-netscan.git): The network scanning project (which supports both command line and cross platform graphical interface) is under development.. -->

# 🔭 why need to hikvision-rs?
<!-- At first, I wanted to complete a cross network scanning project to help me complete some work. I referred to many open source projects, but these projects have some defects that do not meet my needs, so I have e-libscanner.
(process host and port scanning, and support domain name resolution, route tracking, fingerprint scanning, service scanning, asynchronous scanning, scalability and more)
The bottom layer is by calling [npcap]（ https://nmap.org/npcap/ ）And [WinPcap]（ https://www.winpcap.org/ ）Packet capture service;
The service API is [libpnet]（ https://github.com/libpnet/libpnet ); -->

# 🙋 Reference items and materials
<!-- ✨ [RustScan] https://github.com/RustScan/RustScan : Rustlike nmapscan
✨ [netscan] https://github.com/shellrow/netscan : Rust Network Scanner
✨ [libpnet](libpnet) https://github.com/libpnet/libpnet ● The background base of the interplatform network - mainly using captivity services ([npcap]) https://nmap.org/npcap/ with [WinPcap]( https://www.winpcap.org/ ) -->