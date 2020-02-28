use std::thread;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::GLProfile;
use sdl2::VideoSubsystem;

mod shader;

fn main() {
    let sdl_context = sdl2::init().expect("SDL ライブラリの初期化に失敗しました");

    let video_subsystem = sdl_context
        .video()
        .expect("Video Subsystem の初期化に失敗しました");

    setup_opengl(&video_subsystem);

    let window = video_subsystem
        .window("Rust OpenGL", 640, 480)
        .position_centered()
        .build()
        .expect("ウィンドウの生成に失敗しました");

    let _gl_context = window
        .gl_create_context()
        .expect("OpenGL コンテキストの生成に失敗しました");
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    let _shader = shader::Shader::new("shaders/vertex.glsl", "shaders/fragment.glsl");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("キャンバスの生成に失敗しました");

    // 白色で全体を塗りつぶし
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    // バッファの内容を画面にレンダリングする
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'event: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'event,
                _ => {}
            }
        }

        canvas.present();

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

/** プロファイルとバージョンの設定 */
fn setup_opengl(video_subsystem: &VideoSubsystem) {
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 1);
    let (major, minor) = gl_attr.context_version();
    println!("OpenGL is ready (version {}.{})", major, minor);
}
