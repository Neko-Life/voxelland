use crate::{game::Game, shader::Shader, text::Text, texture::Texture};
use glam::Vec2;
use glfw::{Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};
use std::{sync::{Arc, RwLock}, time::{Duration, Instant}};
use imgui::*;
use imgui_opengl_renderer::Renderer;
pub struct WindowAndKeyContext {
    pub width: u32,
    pub height: u32,
    pub game: Option<Game>,

    pub previous_time: Instant,
    pub delta_time: f32,

    pub glfw: Glfw,
    pub window: Arc<RwLock<PWindow>>,
    pub events: GlfwReceiver<(f64, WindowEvent)>,

    pub imgui: imgui::Context,
    pub guirenderer: imgui_opengl_renderer::Renderer
}

impl WindowAndKeyContext {
    pub fn new(windowname: &'static str, width: u32, height: u32) -> Self {
        
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        let (mut window, events) = glfw
            .create_window(width, height, windowname, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.make_current();

        // Initialize ImGui
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
        let mut renderer = Renderer::new(&mut imgui, |s| window.get_proc_address(s) as *const _);


        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::FrontFace(gl::CW);
        }

   

        let mut wak = WindowAndKeyContext {
            width,
            height,
            game: None,
            previous_time: Instant::now(),
            delta_time: 0.0,
            glfw,
            window: Arc::new(RwLock::new(window)),
            events,
            imgui,
            guirenderer: renderer
        };

        wak
    }

    

    pub fn run(&mut self) {
        self.glfw.poll_events();

        let current_time = Instant::now();
        self.delta_time = current_time
            .duration_since(self.previous_time)
            .as_secs_f32();
        self.previous_time = current_time;

        let g = self.game.as_mut().unwrap();
        g.update();

        if g.vars.menu_open {

            let cb = g.currentbuttons.clone();

            self.imgui.io_mut().update_delta_time(Duration::from_secs_f32(self.delta_time));

            let (width, height) = self.window.read().unwrap().get_framebuffer_size();
            self.imgui.io_mut().display_size = [width as f32, height as f32];
            
            // Start the ImGui frame
            let ui = self.imgui.frame();

            let window_flags = WindowFlags::NO_DECORATION
                | WindowFlags::NO_MOVE
                | WindowFlags::NO_RESIZE
                | WindowFlags::NO_SCROLLBAR
                | WindowFlags::NO_TITLE_BAR
                | WindowFlags::NO_BACKGROUND;

            let window_pos = [width as f32 / 2.0 - 50.0, height as f32 / 2.0 - 100.0];

            ui.window("Transparent Window")
                .size([100.0, 200.0], Condition::Always)
                .position(window_pos, Condition::Always)
                .flags(window_flags)
                .build(|| {

                    for (buttonname, command) in cb {
                        if ui.button(buttonname) {
                            g.button_command(command);
                        }
                    }
                    
                });

            // Render the ImGui frame
            self.guirenderer.render(&mut self.imgui);
        }

        
        let io = self.imgui.io_mut();
        for (_, event) in glfw::flush_messages(&self.events) {

            

            match event {
                glfw::WindowEvent::MouseButton(mousebutton, action, _) => {
                    let index = match mousebutton {
                        glfw::MouseButton::Button1 => 0,
                        glfw::MouseButton::Button2 => 1,
                        glfw::MouseButton::Button3 => 2,
                        glfw::MouseButton::Button4 => 3,
                        glfw::MouseButton::Button5 => 4,
                        glfw::MouseButton::Button6 => 5,
                        glfw::MouseButton::Button7 => 6,
                        glfw::MouseButton::Button8 => 7,
                        _ => return,
                    };
                    io.mouse_down[index] = action == glfw::Action::Press;

                    if !io.want_capture_mouse {
                        if mousebutton == glfw::MouseButtonLeft {
                            
                            if !io.want_capture_mouse {
                                self.window.write().unwrap().set_cursor_mode(glfw::CursorMode::Disabled);
                                self.game.as_mut().unwrap().set_mouse_focused(true);
                            }
                            
                        }
                        self.game
                            .as_mut()
                            .unwrap()
                            .mouse_button(mousebutton, action);
                    }
                        
                }
                glfw::WindowEvent::FramebufferSize(wid, hei) => {
                    self.width = wid as u32;
                    self.height = hei as u32;
                    unsafe {
                        gl::Viewport(0, 0, wid, hei);
                    }
                }
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    let g = self.game.as_mut().unwrap();
                    g.cursor_pos(xpos, ypos);
                    if !g.vars.mouse_focused {
                        io.mouse_pos = [xpos as f32, ypos as f32];
                    }
                    
                }
                glfw::WindowEvent::Key(key, scancode, action, _modifiers) => {

                    let pressed = action == glfw::Action::Press || action == glfw::Action::Repeat;
                    io.keys_down[scancode as usize] = pressed;

                    if !io.want_capture_keyboard && !io.want_text_input {
                        if key == Key::Escape {
                            self.window.write().unwrap().set_cursor_mode(glfw::CursorMode::Normal);
                            self.game.as_mut().unwrap().set_mouse_focused(false);
                        }
                        self.game.as_mut().unwrap().keyboard(key, action);
                    }
                    
                }
                glfw::WindowEvent::Scroll(x, y) => {
                    io.mouse_wheel_h += x as f32;
                    io.mouse_wheel += y as f32;

                    self.game.as_mut().unwrap().scroll(y);
                }
                _ => {}
            }
        }

        self.window.write().unwrap().swap_buffers();
    }
}
