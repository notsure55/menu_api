extern crate menu_api;

use std::io::Error;
use menu_api::windows_api;
use menu_api::rusttype;
use menu_api::check_box;
use windows::Win32::UI::Input::KeyboardAndMouse::{ GetAsyncKeyState };

use std::cell::RefCell;
use std::rc::Rc;

fn main() -> Result<(), Error> {
    let hwnd = windows_api::grab_game_hwnd("Counter-Strike 2");
    let (event_loop, window, display, overlay_hwnd) = menu_api::create_overlay(hwnd, "Black Overlay").unwrap();

    let system = rusttype::TextSystem::new(&display);

    let font = rusttype::FontTexture::new(
        &display,
        &include_bytes!("../fonts/arialbd.ttf")[..], 70,
        rusttype::FontTexture::ascii_character_list()
    ).unwrap();

    let mut black = false;

    let mut menu = menu_api::Menu::new(display, system, font, overlay_hwnd, (600.0, 450.0));
    let check_box = check_box::CheckBox::new(
        menu_api::Rect::new(menu_api::Vertex { p: [ 100.0, 100.0] }, 200.0, 200.0 ),
        menu_api::Vec4::new(1.0, 0.0, 0.0, 1.0),
        Rc::new(RefCell::new(black))
    );

    menu.add_to_draw_list(menu_api::MenuObject::CheckBox(check_box));

    #[allow(deprecated)]
    event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::CursorMoved { position, .. } => {
                    menu.mouse_pos = (position.x as f32, position.y as f32);
                },
                glium::winit::event::WindowEvent::RedrawRequested => {
                    menu.draw_menu();
                    unsafe {
                        if GetAsyncKeyState(0x2D) & 0x01 > 0  {
                            menu.toggle_overlay();
                        }
                    }
                    window.request_redraw()
                },
                _ => (),
            },
            _ => (),
        };
    })
    .unwrap();

    Ok(())
}
