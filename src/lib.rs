#![no_std]
#![allow(nonstandard_style)]
#![allow(improper_ctypes)]

#[allow(dead_code)]
mod ctypes;

include!("bindings.rs");

extern crate alloc;
use alloc::vec::Vec;

pub struct Context {
    pub window: *mut SDL_Window,
    _context: SDL_GLContext,
}

impl Context {
    pub fn new(name: *const u8, width: i32, height: i32) -> Result<Self, ()> {
        unsafe {
            SDL_Init(SDL_INIT_VIDEO);

            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 4);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 6);

            let window = SDL_CreateWindow(
                name as *const i8,
                SDL_WINDOWPOS_UNDEFINED_MASK as _,
                SDL_WINDOWPOS_UNDEFINED_MASK as _,
                width,
                height,
                SDL_WINDOW_OPENGL | SDL_WINDOW_SHOWN,
            );

            if window.is_null() {
                SDL_Quit();
                Err(())
            } else {
                let _context = SDL_GL_CreateContext(window);
                SDL_GL_MakeCurrent(window, _context);
                Ok(Context { window, _context })
            }
        }
    }

    pub fn swap(&self) {
        unsafe {
            SDL_GL_SwapWindow(self.window);
        }
    }
}

pub struct Vertex {
    pub position: [f32; 3],
}

pub struct Mesh {
    _vertices: Vec<Vertex>,
    indices: Vec<u32>,
    VAO: GLuint,
    _VBO: GLuint,
    EBO: GLuint,
}

impl Mesh {
    pub fn new(_vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        unsafe {
            let mut VAO: GLuint = 0;
            glGenVertexArrays(1, &mut VAO);
            glBindVertexArray(VAO);

            let mut _VBO: GLuint = 0;
            glGenBuffers(1, &mut _VBO);
            glBindBuffer(GL_ARRAY_BUFFER, _VBO);
            glVertexAttribPointer(
                0,
                3,
                GL_FLOAT,
                GL_FALSE as _,
                3 * core::mem::size_of::<f32>() as i32,
                0 as *const core::ffi::c_void,
            );
            glEnableVertexAttribArray(0);
            glBufferData(
                GL_ARRAY_BUFFER,
                (core::mem::size_of::<Vertex>() * _vertices.len()) as _,
                _vertices.as_ptr() as *const core::ffi::c_void,
                GL_STATIC_DRAW,
            );

            let mut EBO: GLuint = 0;
            glGenBuffers(1, &mut EBO);
            glBindBuffer(GL_ARRAY_BUFFER, EBO);
            glBufferData(
                GL_ARRAY_BUFFER,
                (core::mem::size_of::<u32>() * indices.len()) as _,
                indices.as_ptr() as *const core::ffi::c_void,
                GL_STATIC_DRAW,
            );

            Self {
                _vertices,
                indices,
                VAO,
                _VBO,
                EBO,
            }
        }
    }

    pub fn draw(&self) {
        unsafe {
            glBindVertexArray(self.VAO);
            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.EBO);
            glDrawElements(
                GL_TRIANGLES,
                self.indices.len() as _,
                GL_UNSIGNED_INT,
                0 as *const core::ffi::c_void,
            );
        }
    }
}
