use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;

use gl;
use gl::types::{GLchar, GLint};

#[derive(Debug)]
enum SourceType {
    Program,
    VertexShader,
    FragmentShader,
}

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vs_path: &str, fs_path: &str) -> Shader {
        let mut vs_file =
            File::open(vs_path).unwrap_or_else(|_| panic!("failed to open file: {}", vs_path));
        let mut fs_file =
            File::open(fs_path).unwrap_or_else(|_| panic!("failed to open file: {}", fs_path));

        let mut vs_content = String::new();
        vs_file
            .read_to_string(&mut vs_content)
            .unwrap_or_else(|_| panic!("failed to read vertex shader ({})", vs_path));

        let mut fs_content = String::new();
        fs_file
            .read_to_string(&mut fs_content)
            .unwrap_or_else(|_| panic!("failed to read fragment shader ({})", fs_path));

        let cstr_vs_content = CString::new(vs_content.as_bytes()).unwrap();
        let cstr_fs_content = CString::new(fs_content.as_bytes()).unwrap();

        let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        unsafe {
            gl::ShaderSource(vertex_shader, 1, &cstr_vs_content.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
            // check_compile_errors(vertex_shader, SourceType::VertexShader);
        }

        let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        unsafe {
            gl::ShaderSource(fragment_shader, 1, &cstr_fs_content.as_ptr(), ptr::null());
            gl::CompileShader(gl::FRAGMENT_SHADER);
            // check_compile_errors(fragment_shader, SourceType::FragmentShader);
        };

        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vertex_shader);
            gl::AttachShader(id, fragment_shader);
            gl::LinkProgram(id);
            // check_compile_errors(id, SourceType::Program);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Shader { id }
    }
}

unsafe fn check_compile_errors(shader: u32, source_type: SourceType) {
    let mut info_log = Vec::with_capacity(1024);
    info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character

    match source_type {
        SourceType::Program => {
            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                let info_log_string = String::from_utf8(info_log).unwrap_or_else(|err| {
                    panic!("failed to convert to compilation log from buffer: {}", err)
                });
                panic!(
                    "failed to compile shader code: type={:?}, log={}",
                    source_type, info_log_string
                );
            }
        }
        SourceType::VertexShader | SourceType::FragmentShader => {
            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                let info_log_string = String::from_utf8(info_log).unwrap_or_else(|err| {
                    panic!("failed to convert to link log from buffer: {}", err)
                });
                panic!(
                    "failed to link shader code: type={:?}, log={}",
                    source_type, info_log_string
                )
            }
        }
    }
}
