# HikRobot

HikRobot MVS 相机 SDK 的 Rust 封装。

## 项目结构

```text
HikRobot/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── docs/
│   ├── c-sdk
│   └── rust-sdk
└── crates/
    ├── hikrobot/
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       ├── camera.rs
    │       ├── device.rs
    │       ├── error.rs
    │       └── system.rs
    └── hikrobot-sys/
        ├── Cargo.toml
        ├── build.rs
        ├── MvCamera.h
        ├── src/
        │   └── lib.rs
        ├── includes/
        └── lib/
            ├── win32/
            └── win64/
```

## 文档

- `docs/c-sdk`：海康 C SDK 头文件说明
- `docs/rust-sdk/system.md`：SDK 初始化、版本、设备入口
- `docs/rust-sdk/device.md`：设备枚举和设备信息
- `docs/rust-sdk/camera.md`：相机控制、取流、图像处理、录像
- `docs/rust-sdk/error.md`：统一错误类型和 C SDK 返回码翻译
