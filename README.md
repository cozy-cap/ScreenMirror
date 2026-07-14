# Dependencies
[Cargo](https://man.archlinux.org/man/cargo.1.en)

# Install
```bash
git clone https://github.com/cozy-cap/ScreenMirror.git
cd ScreenMirror/src/
cargo build
```

# Using
## Host (Windows)
```bash
/path/to/ScreenMirror/target/release/second-monitor-steam host <Client_IP> [monitor_index]
```

## Client (Linux)
```bash
/path/to/ScreenMirror/target/release/second-monitor-stream client
```
