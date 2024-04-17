<img src="../public/ico/white_64x64.ico" alt="e-utils"/>

### 📄 [中文](README.zh.md)  | 📄  [English](../README.md)

# ⚡ 这是什么?
**这是海康威视Camera Rust SDK，支持通用网络摄像头、通用USB摄像头、物联网摄像头和工业摄像头（USB、网络、CamL）**

### 支持 SDK
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
    <td><h4 style="color:#aaa"><center>工业采集接口 (USB、CamL、GigE)</center></h4></td>
  </tr>
  <tr>
    <td><center>NET</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:red"><center>X</center></h4></td>
    <td><h4 style="color:#aaa"><center>通用网络接口</center></h4></td>
  </tr>
  <tr>
      <td><center>USB</center></td>
      <td><h4 style="color:red"><center>X</center></h4></td>
      <td><h4 style="color:red"><center>X</center></h4></td>
      <td><h4 style="color:#aaa"><center>通用USB接口</center></h4></td>
  </tr>
  <tr>
      <td><center>OTAP</center></td>
      <td><h4 style="color:red"><center>X</center></h4></td>
      <td><h4 style="color:red"><center>X</center></h4></td>
      <td><h4 style="color:#aaa"><center>物联网开放访问协议</center></h4></td>
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
    <td><h4 style="color:#aaa"><center>初始化SDK</center></h4></td>
  </tr>
  <tr>
    <td><center>Enumerate</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:red"><center>X</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>枚举设备</center></h4></td>
  </tr>
  <tr>
    <td><center>Login</center></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>登录设备</center></h4></td>
  </tr>
  <tr>
    <td><center>Open</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>打开设备</center></h4></td>
  </tr>
  <tr>
    <td><center>Handle</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>创建句柄</center></h4></td>
  </tr>
  <tr>
    <td><center>Log</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>日志输出</center></h4></td>
  </tr>
  <tr>
    <td><center>Clean SDK</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>清理SDK</center></h4></td>
  </tr>
  <tr>
    <td><center>Clean All</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>清理所有</center></h4></td>
  </tr>
  <tr>
    <td><center>Destroy All</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>销毁</center></h4></td>
  </tr>
  <tr>
    <td><center>Callback Sream</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>回调数据流</center></h4></td>
  </tr>
  <tr>
    <td><center>Sream</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>数据流</center></h4></td>
  </tr>
  <tr>
    <td><center>Fix Network</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:red"><center>X</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>自适应网络传输大小</center></h4></td>
  </tr>
  <tr>
    <td><center>Preview</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>预览图像</center></h4></td>
  </tr>
  <tr>
    <td><center>Capture Image</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>抓图</center></h4></td>
  </tr>
  </tr>
  <tr>
    <td><center>Save JPEG</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>存储JPEG格式</center></h4></td>
  </tr>
  <tr>
    <td><center>GUI</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:white"><center>-</center></h4></td>
    <td><h4 style="color:#aaa"><center>图形化图像界面</center></h4></td>
  </tr>
</table>



# ✨ 分支
```toml
[features]
net = []
mvs = []
default = []
```

# 📖 示例
```toml
[dependencies]
hikvision = {version="0.1", feature=["mvs","net"]}
```

### Examples
[初始化 Mvs SDK](examples/global_mvs.rs) | [初始化 Net SDK](examples/global_net.rs) 


## `💡!重要：`
#### xxx
1. [Env.json](Examples/Env.json) **环境变量; `{origin}` 是特殊的关键词，记录原始目录**
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

1. 添加海康威视摄像头的链接库到hikvision-rs
```sh
# 把链接库放到 libs/HCMvs 或 libs/HCNet 
cargo run --example global_mvs
cargo run --example global_net
```

1. 添加SDK Cargo.toml 
```toml
[dependencies]
hikvision = {version="0.1", feature=["mvs","net"]}
```


# 🚀 快速运行
```sh
# test global mvs init sdk
cargo run --example global_mvs

# test global net init sdk
cargo run --example global_net
```


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