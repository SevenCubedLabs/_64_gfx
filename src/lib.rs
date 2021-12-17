#![no_std]
#![allow(nonstandard_style)]
#![allow(improper_ctypes)]

#[allow(dead_code)]
mod ctypes;

include!("bindings.rs");

pub struct Context {
    pub window: *mut SDL_Window,
    context: SDL_GLContext,
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
                let context = SDL_GL_CreateContext(window);
                SDL_GL_MakeCurrent(window, context);
                Ok(Context { window, context })
            }
        }
    }
}
