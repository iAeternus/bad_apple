use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{Hide, MoveTo, Show},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, size},
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
    stdout.execute(Hide)?;
    stdout.flush()?;

    // 获取终端尺寸
    let (_, rows) = size()?;

    // 设置帧率
    let start_time = Instant::now();
    let frame_duration = Duration::from_millis(33); // 30fps约为33ms/帧
    let mut next_frame_time = start_time;

    // 渲染每一帧
    for frame in frames.iter() {
        render_frame(&mut stdout, frame, rows)?;
        next_frame_time += frame_duration;
        let now = Instant::now();
        if now < next_frame_time {
            thread::sleep(next_frame_time - now);
        }
    }

    // 恢复终端模式
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    stdout.flush()?;

    Ok(())
}

// 渲染函数
fn render_frame(stdout: &mut BufWriter<io::Stdout>, frame: &str, rows: u16) -> io::Result<()> {
    let frame_lines: Vec<&str> = frame.lines().collect();
    let frame_height = frame_lines.len();
    let max_lines = rows as usize;

    // 渲染帧的每一行
    for y in 0..frame_height {
        stdout.queue(MoveTo(0, y as u16))?;
        write!(stdout, "{}", frame_lines[y])?;
    }

    // 清除剩余行
    for y in frame_height..max_lines {
        stdout.queue(MoveTo(0, y as u16))?;
        write!(stdout, "\x1B[K")?;
    }

    stdout.flush()
}
