use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rodio::{Decoder, OutputStream, Source};
use std::fs;
use std::io::{self, BufWriter, Write, stdout};
use std::thread;
use std::time::{Duration, Instant};

const ASCII_FRAMES_PATH: &str = "F:\\Develop\\rust\\bad_apple\\resources\\bad_apple_frames.txt";
const AUDIO_PATH: &str = "F:\\Develop\\rust\\bad_apple\\resources\\bad_apple.mp3";

fn main() -> io::Result<()> {
    let frames_data = fs::read_to_string(ASCII_FRAMES_PATH)?;
    let frames: Vec<&str> = frames_data.split("---FRAME---").skip(1).collect();

    // 初始化音频
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let audio_file = fs::File::open(AUDIO_PATH).unwrap();
    let source = Decoder::new(audio_file).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();

    // 设置终端模式
    let mut stdout = BufWriter::new(stdout());
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(Hide)?;
    stdout.flush()?;

    let start_time = Instant::now();
    let frame_duration = Duration::from_millis(33); // 30fps约为33ms/帧
    let mut next_frame_time = start_time;

    for frame_idx in 0..frames.len() {
        render_frame(&mut stdout, frames[frame_idx])?; // 渲染当前帧
        next_frame_time += frame_duration; // 更新下一帧时间
        let now = Instant::now();
        if now < next_frame_time {
            thread::sleep(next_frame_time - now);
        }
    }

    // 恢复终端设置
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    stdout.flush()?;

    Ok(())
}

// 渲染函数
fn render_frame(stdout: &mut BufWriter<io::Stdout>, frame: &str) -> io::Result<()> {
    stdout.queue(MoveTo(0, 0))?;
    let cleaned_frame = frame.replace('\n', "\x1B[K\n"); // 行尾清除，避免重影
    write!(stdout, "{}\x1B[K", cleaned_frame)?;
    stdout.flush()
}
