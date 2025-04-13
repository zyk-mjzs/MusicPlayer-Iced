## 预览ui
`cargo run -p iced_ui`
`cargo watch -x "run -p iced_ui"`

cargo add cargo-bundle

// 编译成 mac 平台的可执行文件
cargo bundle --target x86_64-apple-darwin
<!-- cargo bundle --target x86_64-apple-darwin -->

// 编译成 Windows 平台的可执行文件
rustup target add x86_64-pc-windows-gnu
cargo bundle --target x86_64-pc-windows-gnu --release
或
rustup target add x86_64-pc-windows-msvc
cargo bundle --target x86_64-pc-windows-msvc --release
