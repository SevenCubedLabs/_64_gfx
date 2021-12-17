//bindgen --use-core --no-prepend-enum-name --ctypes-prefix=ctypes --with-derive-default
//--no-debug \(.*\)? --blocklist-item FP_NAN --blocklist-item FP_ZERO --blocklist-item FP_INFINITE
//--blocklist-item FP_SUBNORMAL --blocklist-item FP_NORMAL src/bindings.h -o src/bindings.rs

#include <sys/time.h>
#include <SDL2/SDL.h>
#define GL_GLEXT_PROTOTYPES
#include <SDL2/SDL_opengl.h>
