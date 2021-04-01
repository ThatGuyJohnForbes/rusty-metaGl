extern crate sdl2;
extern crate gl;

use std::ffi::CString;

pub mod render_gl;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(2,1);

    let window = video_subsystem
        .window("test", 900, 700)
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();

    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();

    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();

    let shader_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();

    shader_program.set_used();

    'main: loop {
        for event in event_pump.poll_iter() {
            //handle user input here
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _=> {},
            }
        }

        // render window content here
        unsafe {
            gl::Viewport(0, 0, 900, 700); // set viewport
            gl::ClearColor(0.0,0.0,0.0,1.0);
        }

        window.gl_swap_window();
    }
}
