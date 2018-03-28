extern crate gl;
extern crate glfw;
extern crate stl_io;

use gl::types::*;
use glfw::Context;
use std::io::Read;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let mut window = glfw.create_window(500, 500, "Rusty Glider", glfw::WindowMode::Windowed)
        .unwrap()
        .0;

    gl::load_with(|s| window.get_proc_address(s) as *const std::os::raw::c_void);

    let vert_src = {
        let mut file = std::fs::File::open("vert.vert").unwrap();
        let mut src = String::new();
        file.read_to_string(&mut src).unwrap();
        src
    };

    let frag_src = {
        let mut file = std::fs::File::open("frag.frag").unwrap();
        let mut src = String::new();
        file.read_to_string(&mut src).unwrap();
        src
    };

    let mut vert_buffer: GLuint = 0;
    // let mut color_buffer: GLuint = 0;

    let boat_stl = {
        let mut file = std::fs::File::open("Glider DG-1000.stl").unwrap();
        stl_io::read_stl(&mut file).unwrap()
    };

    let mut verts: Vec<GLfloat> = Vec::with_capacity(boat_stl.vertices.len());

    for tri in boat_stl.faces {
        for vert in tri.vertices.iter() {
            for coord in boat_stl.vertices[*vert].iter() {
                verts.push(*coord);
            }
        }
    }

    // let verts: Vec<GLfloat> = vec![
    //      0.5,  0.5, -0.5,
    //     -0.5, -0.5, -0.5,
    //      0.5, -0.5, -0.5,
    //      0.5, -0.5,  0.5,
    //      0.5,  0.5,  0.5,
    //     -0.5,  0.5,  0.5,
    //     -0.5,  0.5, -0.5,
    //     -0.5, -0.5, -0.5
    // ];
    // let color: Vec<GLfloat> = vec![
    //     0.5, 0.5, 0.5,
    //     0.5, 0.0, 0.0,
    //     0.5, 0.5, 0.0,
    //     0.0, 0.5, 0.0,
    //     0.0, 0.5, 0.5,
    //     0.0, 0.0, 0.5,
    //     0.5, 0.0, 0.5,
    //     0.5, 0.0, 0.0,
    // ];

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::Enable(gl::DEPTH_TEST);

        let vert_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        {
            let s = std::mem::transmute::<&u8, *const GLchar>(&vert_src.as_bytes()[0]);
            gl::ShaderSource(
                vert_shader,
                1,
                &s as *const *const GLchar,
                std::mem::transmute::<&usize, *const GLint>(&vert_src.len()),
            );
        }
        {
            let s = std::mem::transmute::<&u8, *const GLchar>(&frag_src.as_bytes()[0]);
            gl::ShaderSource(
                frag_shader,
                1,
                &s as *const *const GLchar,
                std::mem::transmute::<&usize, *const GLint>(&frag_src.len()),
            );
        }

        gl::CompileShader(vert_shader);
        gl::CompileShader(frag_shader);

        let shader = gl::CreateProgram();

        gl::AttachShader(shader, vert_shader);
        gl::AttachShader(shader, frag_shader);

        gl::LinkProgram(shader);

        gl::UseProgram(shader);

        gl::EnableVertexAttribArray(0);
        // gl::EnableVertexAttribArray(1);

        gl::GenBuffers(1, &mut vert_buffer as *mut GLuint);
        gl::BindBuffer(gl::ARRAY_BUFFER, vert_buffer);

        // gl::BufferData(
        //     gl::ARRAY_BUFFER,
        //     4 * verts.len() as isize,
        //     verts.as_ptr() as *const std::os::raw::c_void,
        //     gl::STATIC_DRAW,
        // );

        gl::BufferData(
            gl::ARRAY_BUFFER,
            4 * verts.len() as isize,
            verts.as_ptr() as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

        // gl::GenBuffers(1, &mut color_buffer as *mut GLuint);
        // gl::BindBuffer(gl::ARRAY_BUFFER, color_buffer);
        // gl::BufferData(
        //     gl::ARRAY_BUFFER,
        //     4 * color.len() as isize,
        //     color.as_ptr() as *const std::os::raw::c_void,
        //     gl::STATIC_DRAW,
        // );
        // gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
    }

    while !window.should_close() {
        let cursor_pos = window.get_cursor_pos();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Uniform2f(
                0,
                (cursor_pos.0 / 250.0 - 1.0) as f32,
                (cursor_pos.1 / 250.0 - 1.0) as f32,
            );
            gl::DrawArrays(gl::TRIANGLES, 0, verts.len() as i32);
        }
        window.swap_buffers();
        glfw.poll_events();
    }
}
