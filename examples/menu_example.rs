extern crate menu_glium_api as menu_api;
use menu_api::Menu;

use std::io::Error;
use menu_api::{ windows_api, filled_box, rusttype, check_box, float_slider, label, outline_box };
use windows::Win32::UI::Input::KeyboardAndMouse::{ GetAsyncKeyState };

use std::cell::RefCell;
use std::rc::Rc;

fn main() -> Result<(), Error> {
    // optional
    let hwnd = windows_api::grab_game_hwnd("Counter-Strike 2");         // Some(hwnd)
    let (event_loop, window, display, overlay_hwnd) = menu_api::create_overlay(Some(hwnd), "Black Overlay").unwrap();

    let system = rusttype::TextSystem::new(&display);

    let font = rusttype::FontTexture::new(
        &display,
        &include_bytes!("../fonts/arialbd.ttf")[..], 70,
        rusttype::FontTexture::ascii_character_list()
    ).unwrap();

    let black = Rc::new(RefCell::new(false));
    let float = Rc::new(RefCell::new(10.0));

    let mut menu = menu_api::Menu::new(display, system, font, overlay_hwnd, (600.0, 450.0));

    build_menu(&mut menu, Rc::clone(&black), Rc::clone(&float));

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
                    if *black.borrow() {
                        println!("We are black!");
                    }
                    println!("{}", *float.borrow());
                    cheat_loop(&mut menu);
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

fn build_menu(menu: &mut Menu, black: Rc<RefCell<bool>>, float: Rc<RefCell<f32>>) {
    let esp = label::Label::new(
        menu_api::Vec4::new(1.0, 0.0, 0.0, 1.0),
        label::Direction::Top,
        "Esp",
        20.0,
        0.10,
    );
    let smoothing = label::Label::new(
        menu_api::Vec4::new(1.0, 1.0, 0.0, 1.0),
        label::Direction::Bottom,
        "smoothing",
        20.0,
        0.5,
    );
    let fanboy = label::Label::new(
        menu_api::Vec4::new(1.0, 1.0, 0.0, 1.0),
        label::Direction::Top,
        "Fanboy",
        20.0,
        0.5,
    );
    let check_box = check_box::CheckBox::new(
        menu_api::MenuOptions::new(true, true, false, true),
        menu_api::Rect::new(menu_api::Vertex { p: [ menu.base.rect.top_left.p[0] + 15.0,
                                                    menu.base.rect.top_left.p[1] + 15.0] }, 30.0, 30.0 ),
        menu_api::Vec4::new(0.0, 1.0, 0.7, 1.0),
        Rc::clone(&black),
        Some(esp)
    );
    let filled_box = filled_box::FilledBox::new(
        menu_api::MenuOptions::new(true, true, false, true),
        menu_api::Rect::new(menu_api::Vertex { p: [ menu.base.rect.top_left.p[0] + 100.0,
                                                    menu.base.rect.top_left.p[1] + 100.0] }, 100.0, 100.0 ),
        menu_api::Vec4::new(1.0, 1.0, 0.7, 1.0),
        Some(label::Label::new(
            menu_api::Vec4::new(1.0, 1.0, 0.0, 1.0),
            label::Direction::Left,
            "Fanboy",
            20.0
        ))
    );
    let filled_box1 = filled_box::FilledBox::new(
        menu_api::MenuOptions::new(true, true, false, true),
        menu_api::Rect::new(menu_api::Vertex { p: [ menu.base.rect.top_left.p[0] + 300.0,
                                                    menu.base.rect.top_left.p[1] + 300.0] }, 100.0, 100.0 ),
        menu_api::Vec4::new(1.0, 0.0, 1.0, 1.0),
        None
    );
    let slider = float_slider::FloatSlider::new(
        menu_api::MenuOptions::new(true, true, false, true),
        menu_api::Rect::new(menu_api::Vertex { p: [ menu.base.rect.top_left.p[0] + 300.0,
                                                    menu.base.rect.top_left.p[1] + 300.0] }, 100.0, 10.0 ),
        menu_api::Vec4::new(1.0, 0.0, 1.0, 1.0),
        Rc::clone(&float),
        0.0,
        100.0,
        Some(smoothing)
    );
    let outline_box = outline_box::OutlineBox::new(
        menu_api::MenuOptions::new(false, false, false, false),
        menu_api::Rect::new(menu_api::Vertex { p: [ menu.base.rect.top_left.p[0] + 500.0,
                                                    menu.base.rect.top_left.p[1] + 500.0] }, 100.0, 100.0 ),
        menu_api::Vec4::new(1.0, 0.0, 1.0, 1.0),
        4.0,
        Some(fanboy)
    );

    menu.add_to_draw_list(menu_api::MenuObject::CheckBox(check_box));
    menu.add_to_draw_list(menu_api::MenuObject::FilledBox(filled_box));
    menu.add_to_draw_list(menu_api::MenuObject::FilledBox(filled_box1));
    menu.add_to_draw_list(menu_api::MenuObject::FloatSlider(slider));
    menu.add_to_draw_list(menu_api::MenuObject::OutlineBox(outline_box));
}

fn cheat_loop(menu: &mut Menu) {
    menu.draw_menu();
    unsafe {
        // insert key
        if GetAsyncKeyState(0x2D) & 0x01 > 0  {
            menu.toggle_overlay();
        }
    }
}
