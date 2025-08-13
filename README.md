## Bad Apple in Rust

### Quick Start

```
cargo run --bin show
```

### 预处理视频

```shell
ffmpeg -i "BadApple.mp4" -vf "fps=30,scale=120:60" frames/%04d.png
ffmpeg -i "BadApple.mp4" -q:a 0 -map a bad_apple.mp3
```

