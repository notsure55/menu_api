use windows::Win32::Foundation:: { HWND };
use std::io::{Error, ErrorKind};
use std::ffi::c_void;
use winit::raw_window_handle::HasWindowHandle;

use winit::{ window::{ Window, WindowAttributes, WindowLevel } , event_loop::EventLoop };

use winit::dpi::{ Position::Logical, LogicalSize, LogicalPosition };

use glium::Surface;

use glium::backend::glutin::Display;
use glutin::surface::WindowSurface;
use glium::implement_vertex;

pub mod windows_api;
pub mod rusttype;
pub mod check_box;

pub fn create_overlay(hwnd: HWND, overlay_name: &str) ->
    Result<(
        EventLoop<()>,
        Window,
        Display<WindowSurface>,
        HWND
    ), Error>
{
    // calculating window dimensions
    let window_size = windows_api::grab_window_dimensions(hwnd);
    let width = window_size.right - window_size.left - 15;
    let height = window_size.bottom - window_size.top - 40;

    #[allow(deprecated)]
    let window_attributes = WindowAttributes::new()
        .with_title(overlay_name)
        .with_inner_size(LogicalSize::new(width as f32, height as f32))
        .with_position(Logical(LogicalPosition::new(window_size.left.into(), window_size.top.into())))
        .with_transparent(true)
        .with_decorations(false)
        .with_window_level(WindowLevel::AlwaysOnTop);

    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop failed to build!, probaly trying to build not within main thread");

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_attributes)
        .build(&event_loop);

    let window_handle = match window.window_handle() {
        Ok(wh) => wh,
        Err(_e) => return Err(Error::new(ErrorKind::Other, "Raw Window handle is invald!")),
    };

    let handle = match windows_api::grab_handle(window_handle) {
        Some(h) => h,
        None => return Err(Error::new(ErrorKind::Other, "HWND is invald!")),
    };

    let hwnd: winit::platform::windows::HWND = handle.into();

    let overlay_handle = HWND(hwnd as *mut c_void);

    windows_api::make_window_click_through(overlay_handle);

    return Ok( (event_loop, window, display, overlay_handle) )
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub p: [f32; 2],
}

implement_vertex!(Vertex, p);

pub enum MenuObject {
    CheckBox(check_box::CheckBox),
  /*FilledBox(filled_box::FilledBox),
    OutlineBox(outline_box::OutlineBox),
    FloatSlider(float_slider::FloatSlider),*/
}

pub struct Vec4 {
    v: [f32; 4]
}

impl Vec4 {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            v:
            [
                r,
                g,
                b,
                a,
            ],
        }
    }
}

pub struct Rect {
    top_left: Vertex,
    width: f32,
    height: f32,
}

impl Rect {
    pub fn new(top_left: Vertex, width: f32, height: f32) -> Self {
        Self {
            top_left,
            width,
            height,
        }
    }
}

pub struct Menu {
    pub display: Display<WindowSurface>,
    pub window_size: (u32, u32),
    pub system: rusttype::TextSystem,
    pub font: rusttype::FontTexture,
    pub handle: HWND,
    pub mouse_pos: (f32, f32),
    pub base_size: (f32, f32),
    objects: Vec<MenuObject>
}

impl Menu {
    pub fn new(
        display: Display<WindowSurface>,
        system: rusttype::TextSystem,
        font: rusttype::FontTexture,
        handle: HWND,
        base_size: (f32, f32)
    ) -> Self {

        let window_size = display.get_framebuffer_dimensions();

        Self {
            display,
            window_size,
            system,
            font,
            handle,
            mouse_pos: (0.0, 0.0),
            base_size,
            objects: Vec::new()
        }
    }
    pub fn draw_menu(&mut self) {
        let mut frame = self.display.draw();

        frame.clear_color(0.0, 0.0, 0.0, 0.0);

        let objects = std::mem::take(&mut self.objects);

        for object in objects.iter() {
            match object {
                MenuObject::CheckBox(check_box) => check_box.draw(self, &mut frame),
            }
        }
        // draw_menu

        self.objects = objects;

        frame.finish().unwrap();
    }
    pub fn add_to_draw_list(&mut self, object: MenuObject) {
        self.objects.push(object);
    }
}
