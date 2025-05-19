use windows::Win32::Foundation:: { HWND, RECT };
use std::io::{Error, ErrorKind};
use std::ffi::c_void;
use winit::raw_window_handle::HasWindowHandle;

use winit::{ window::{ Window, WindowAttributes, WindowLevel } , event_loop::EventLoop };

use winit::dpi::{ Position::Logical, LogicalSize, LogicalPosition };

use glium::{ Surface, Frame };

use glium::backend::glutin::Display;

use glutin::surface::WindowSurface;
use glium::implement_vertex;

use windows::Win32::UI::Input::KeyboardAndMouse::{ GetAsyncKeyState,
                                                   SendInput,
                                                   INPUT, INPUT_TYPE,
                                                   MOUSE_EVENT_FLAGS,
                                                   INPUT_0 };

pub mod windows_api;
pub mod rusttype;
pub mod check_box;
pub mod outline_box;
pub mod filled_box;
pub mod float_slider;
pub mod label;

pub fn create_overlay(hwnd: Option<HWND>, overlay_name: &str) ->
    Result<(
        EventLoop<()>,
        Window,
        Display<WindowSurface>,
        HWND
    ), Error>
{
    let mut window_size = RECT { left: 100, top: 100, right: 800, bottom: 800 };
    let mut width = window_size.right - window_size.left - 15;
    let mut height = window_size.bottom - window_size.top - 40;
    if hwnd.is_some() {
        window_size = windows_api::grab_window_dimensions(hwnd.unwrap());
        width = window_size.right - window_size.left - 15;
        height = window_size.bottom - window_size.top - 40;
    }
    // calculating window dimensions

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

pub trait Draw {
    fn draw(
        &mut self,
        menu: &mut Menu,
        frame: &mut Frame
    );
}
pub trait InBounds {
    fn in_bounds(
        &self,
        menu: &Menu
    ) -> bool;
}
pub trait Hovering {
    fn is_hovering(
        &self,
        menu: &mut Menu,
        frame: &mut Frame,
    );
}
pub trait Clicked {
    fn clicked(
        &self,
        menu: &mut Menu,
        frame: &mut Frame,
    ) -> bool;
}
pub trait Options {
    fn get_options(&self) -> MenuOptions;
}
pub trait Draggable {
    fn is_dragging(
        &mut self,
        menu: &mut Menu,
    );
}

impl Draw for MenuObject {
    fn draw(
        &mut self,
        menu: &mut Menu,
        frame: &mut Frame
    ) {
        match self {
            MenuObject::CheckBox(b) => b.draw(menu, frame),
            MenuObject::OutlineBox(b) => b.draw(menu, frame),
            MenuObject::FilledBox(b) => b.draw(menu, frame),
            MenuObject::FloatSlider(b) => b.draw(menu, frame),
        }
    }
}
impl InBounds for MenuObject {
    fn in_bounds(
        &self,
        menu: &Menu
    ) -> bool {
        match self {
            MenuObject::CheckBox(b) => b.in_bounds(menu),
            MenuObject::OutlineBox(b) => b.in_bounds(menu),
            MenuObject::FilledBox(b) => b.in_bounds(menu),
            MenuObject::FloatSlider(b) => b.in_bounds(menu),
        }
    }
}
impl Hovering for MenuObject {
    fn is_hovering(
        &self,
        menu: &mut Menu,
        frame: &mut Frame,
    ) {
        match self {
            MenuObject::CheckBox(b) => b.is_hovering(menu, frame),
            MenuObject::OutlineBox(b) => b.is_hovering(menu, frame),
            MenuObject::FilledBox(b) => b.is_hovering(menu, frame),
            MenuObject::FloatSlider(b) => b.is_hovering(menu, frame),
        }
    }
}
impl Clicked for MenuObject {
    fn clicked(
        &self,
        menu: &mut Menu,
        frame: &mut Frame,
    ) -> bool {
        match self {
            MenuObject::CheckBox(b) => b.clicked(menu, frame),
            MenuObject::OutlineBox(b) => b.clicked(menu, frame),
            MenuObject::FilledBox(b) => b.clicked(menu, frame),
            MenuObject::FloatSlider(b) => b.clicked(menu, frame),
        }
    }
}
impl Options for MenuObject {
    fn get_options(&self) -> MenuOptions {
        match self {
            MenuObject::CheckBox(b) => b.get_options(),
            MenuObject::OutlineBox(b) => b.get_options(),
            MenuObject::FilledBox(b) => b.get_options(),
            MenuObject::FloatSlider(b) => b.get_options(),
        }
    }
}
impl Draggable for MenuObject {
    fn is_dragging(
        &mut self,
        menu: &mut Menu,
    ) {
        match self {
            MenuObject::CheckBox(b) => b.is_dragging(menu),
            MenuObject::OutlineBox(b) => b.is_dragging(menu),
            MenuObject::FilledBox(b) => b.is_dragging(menu),
            MenuObject::FloatSlider(b) => b.is_dragging(menu),
        }
    }
}
#[derive(Copy, Clone, Default)]
pub struct Vertex {
    pub p: [f32; 2],
}

implement_vertex!(Vertex, p);

#[derive(Default, Copy, Clone)]
pub struct MenuOptions {
    pub draggable: bool,
    pub hover: bool,
    pub delete: bool,
    pub moveable: bool,
}

impl MenuOptions {
    pub fn new(draggable: bool, hover: bool, delete: bool, moveable: bool) -> Self {
        Self {
            draggable,
            hover,
            delete,
            moveable,
        }
    }
}

pub enum MenuObject {
    CheckBox(check_box::CheckBox),
    FilledBox(filled_box::FilledBox),
    OutlineBox(outline_box::OutlineBox),
    FloatSlider(float_slider::FloatSlider),
//    Label(label::Label),
}

#[derive(Default)]
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

#[derive(Default)]
pub struct Rect {
    pub top_left: Vertex,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(
        top_left: Vertex,
        width: f32,
        height: f32
    ) -> Self {
        Self {
            top_left,
            width,
            height,
        }
    }
    pub fn in_bounds(
        &self,
        menu: &Menu
    ) -> bool {
        if menu.mouse_pos.0 < self.top_left.p[0] + self.width && menu.mouse_pos.0 > self.top_left.p[0]
        && menu.mouse_pos.1 < self.top_left.p[1] + self.height && menu.mouse_pos.1 > self.top_left.p[1] {
            true
        } else {
            false
        }
    }
    pub fn is_hovering(
        &self,
        menu: &mut Menu,
        frame: &mut Frame,
    ) {
        if self.in_bounds(menu) {
            let top_left = Vertex { p: [ self.top_left.p[0] - 2.0, self.top_left.p[1] - 2.0] };
            let mut outline = outline_box::OutlineBox::new(
                MenuOptions::new(false, false, false, true),
                Rect::new(top_left, self.width + 4.0, self.height + 4.0),
                Vec4::new(1.0, 0.0, 0.0, 1.0),
                4.0,
                None
            );
            outline.draw(menu, frame);
        }
    }
}

pub struct Menu {
    pub display: Display<WindowSurface>,
    pub window_size: (f32, f32),
    pub system: rusttype::TextSystem,
    pub font: rusttype::FontTexture,
    pub handle: HWND,
    pub mouse_pos: (f32, f32),
    pub cached_mouse_pos: (f32, f32),
    pub base: filled_box::FilledBox,
    objects: Vec<MenuObject>,
    clickthrough: bool,
    pub clicked: bool,
    pub dragging: bool,
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
        let base = filled_box::FilledBox::new(
            MenuOptions::new(true, true, false, false),
            Rect::new(Vertex { p: [ 100.0, 100.0] }, base_size.0, base_size.1),
            Vec4::new(0.5, 0.5, 0.5, 1.0),
            None
        );

        Self {
            display,
            window_size: (window_size.0 as f32, window_size.1 as f32),
            system,
            font,
            handle,
            mouse_pos: (0.0, 0.0),
            cached_mouse_pos: (0.0, 0.0),
            base,
            objects: Vec::new(),
            clickthrough: true,
            clicked: false,
            dragging: false,
        }
    }
    pub fn draw_menu(&mut self) {
        self.check_clicks();

        let mut frame = self.display.draw();

        frame.clear_color(0.0, 0.0, 0.0, 0.0);

        let mut base = std::mem::take(&mut self.base);

        base.draw(self, &mut frame);
        self.base_dragging(&mut base);
        base.is_hovering(self, &mut frame);

        self.base = base;

        let mut objects = std::mem::take(&mut self.objects);

        let mut remove = vec![];

        for (i, object) in objects.iter_mut().enumerate() {
            object.draw(self, &mut frame);

            object.clicked(self, &mut frame);

            let options = object.get_options();

            if options.draggable {
                object.is_dragging(self);
            }
            if options.hover {
                object.is_hovering(self, &mut frame);
            }
            if options.delete {
                remove.push(i)
            }
        }

        remove.iter().for_each(|i|{
            objects.remove(*i);
        });

        self.objects = objects;

        frame.finish().unwrap();
    }
    pub fn add_to_draw_list(&mut self, object: MenuObject) {
        self.objects.push(object);
    }
    pub fn toggle_overlay(&mut self) {
        if self.clickthrough {
            windows_api::make_window_non_click_through(self.handle);
            self.clickthrough = false
        } else {
            windows_api::make_window_click_through(self.handle);
            self.clickthrough = true;
        }
        // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput
        let mut inputs = [INPUT { r#type: INPUT_TYPE(0), Anonymous: INPUT_0::default() }; 2];

        inputs[0].r#type = INPUT_TYPE(0);
        inputs[0].Anonymous.mi.dwFlags = MOUSE_EVENT_FLAGS(0x0002);

        inputs[1].r#type = INPUT_TYPE(0);
        inputs[1].Anonymous.mi.dwFlags = MOUSE_EVENT_FLAGS(0x0004);

        unsafe { SendInput(&inputs, std::mem::size_of_val::<INPUT>(&inputs[0]) as i32) };
    }
    pub fn check_clicks(&mut self) {
        // reset for next loop if not clicked again
        unsafe {
            if self.clicked {
                self.clicked = false;
            }
            if GetAsyncKeyState(0x01) & 0x01 > 0 {
                self.clicked = true;
            }
            if GetAsyncKeyState(0x02) < 0 {
                if !self.dragging {
                    self.dragging = true;
                    self.cached_mouse_pos = self.mouse_pos;
                }
            } else {
                self.dragging = false;
            }
        }
    }
    fn base_dragging(&mut self, base: &mut filled_box::FilledBox) {
        for object in self.objects.iter() {
            if object.in_bounds(&self) && self.dragging {
                return
            }
        }
        if self.mouse_pos.0 != self.cached_mouse_pos.0 || self.mouse_pos.1 != self.cached_mouse_pos.1 {
            if base.in_bounds(&self) && self.dragging {
                base.rect.top_left.p[0] += self.mouse_pos.0 - self.cached_mouse_pos.0;
                base.rect.top_left.p[1] += self.mouse_pos.1 - self.cached_mouse_pos.1;
                for object in self.objects.iter_mut() {
                    let options = object.get_options();

                    if options.moveable {
                        match object {
                            MenuObject::CheckBox(b) => {
                                b.rect.top_left.p[0] += self.mouse_pos.0 - self.cached_mouse_pos.0;
                                b.rect.top_left.p[1] += self.mouse_pos.1 - self.cached_mouse_pos.1;
                            },
                            MenuObject::OutlineBox(b) => {
                                b.rect.top_left.p[0] += self.mouse_pos.0 - self.cached_mouse_pos.0;
                                b.rect.top_left.p[1] += self.mouse_pos.1 - self.cached_mouse_pos.1;
                            },
                            MenuObject::FilledBox(b) => {
                                b.rect.top_left.p[0] += self.mouse_pos.0 - self.cached_mouse_pos.0;
                                b.rect.top_left.p[1] += self.mouse_pos.1 - self.cached_mouse_pos.1;
                            },
                            MenuObject::FloatSlider(b) => {
                                b.rect.top_left.p[0] += self.mouse_pos.0 - self.cached_mouse_pos.0;
                                b.rect.top_left.p[1] += self.mouse_pos.1 - self.cached_mouse_pos.1;
                            }
                        }
                    }
                }
                self.cached_mouse_pos = self.mouse_pos;
            }
        }
    }
}
