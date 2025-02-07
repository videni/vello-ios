A demo to use Vello with SwiftUI


## Quick start

### 1. Build

```
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
cargo build --target aarch64-apple-ios --release  && cp ./target/aarch64-apple-ios/release/libvello_ios.a ios/VelloDemo/libs
```

### 2. Run with XCode

My Mac(Designed for Ipad)