
# RustGLM: 基于智谱的 ChatGLM Rust SDK - [English Doc](https://github.com/blueokanna/RustGLM/blob/main/README.md)
> 高性能、高品质体验和可靠的 Rust 语言的智谱 ChatGLM 自然大语言处理开发套件

<br>

### ❌ 注意! RustGLM 0.1.0 版本不可用! 请更新到最新版本进行使用!

<br>

## 1. 准备开始

### 1.1 安装 Rust-up 可删减程序（ 👇 此处仅显示 Windows 和 Android 文件）

[Rust-up-Windows-x64-Installation](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)

[Rust-up-Windows-x32-Installation](https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe)

[Rust-up-aarch64-android-Installation](https://static.rust-lang.org/rustup/dist/aarch64-linux-android/rustup-init)

> 如果你是 `Linux` 用户 or `MacOS` 用户, 你可以点击这里进行查看: [用户安装手册](https://forge.rust-lang.org/infra/other-installation-methods.html)

<br>
<br>

1️⃣ 安装后，请使用 `命令行` 检查 Rust 版本：

```
cargo -V
```
or
```
cargo --version
```
<br>
<br>

2️⃣ **然后就可以使用命令将库添加到自己的项目中：**
```
cargo add RustGLM
```
or use
```
RustGLM = "0.1.4"
```

#### 您可能需要的其他 RustGLM 文档： 👉 :link: [RustGLM Documation](https://docs.rs/RustGLM/0.1.1/RustGLM/struct.RustGLM.html)
<br>
<br>

### 1.2 Rust NTP 时间服务器

它通过互联网或局域网上的时间服务器提供高度准确和安全的时间信息，确保所有设备使用相同的时间至关重要。这里的应用是通过以下方式进行 `JWT` 身份验证：

```
pub fn time_sync() -> i64 {
    let client = SntpClient::new();
    let result = client.synchronize("ntp.aliyun.com").unwrap();

    let local_time: DateTime<Local> =