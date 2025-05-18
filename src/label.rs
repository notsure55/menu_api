use crate::{ MenuObject, Rect, Menu, Vertex, Vec4, Draw, InBounds, Hovering, Clicked, MenuOptions, Options, outline_box, Draggable, rusttype };

use glium::{ Surface, uniform, Frame };

#[derive(Default)]
pub enum Direction {
    #[default]
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Default)]
pub struct Label {
    color: Vec4,
    direction: Direction,
    text: String,
    scale: f32
}

impl Label {
    pub fn new(color: Vec4, direction: Direction, text: &str, scale: f32) -> Self {
        Self {
            color,
            direction,
            text: String::from(text),
            scale,
        }
    }
    pub fn draw(
        &self,
        rect: &Rect,
        menu: &mut Menu,
        frame: &mut Frame,
    ) {
        let mut top_left = Vertex { p: [ rect.top_left.p[0], rect.top_left.p[1] ] };
        match self.direction {
            Direction::Top => {
                top_left = Vertex { p: [ rect.top_left.p[0], rect.top_left.p[1] - rect.height * 0.10] };
            },
            Direction::Left => {
                top_left = Vertex { p: [ rect.top_left.p[0] - rect.width, rect.top_left.p[1] + rect.height * 0.50 ] };
            },
            Direction::Right => {
                top_left = Vertex { p: [ rect.top_left.p[0] + rect.width, rect.top_left.p[1] + rect.height * 0.50 ] };
            },
            Direction::Bottom => {
                top_left = Vertex { p: [ rect.top_left.p[0], rect.top_left.p[1] + rect.height * 2.0 ] };
            },
        }
        let text = rusttype::TextDisplay::new(&menu.system, &menu.font, &self.text);
        let text_width = text.get_width();

        let sx = self.scale / (menu.window_size.0 as f32 / 2.0);
        let sy = self.scale / (menu.window_size.1 as f32 / 2.0);

        let x_ndc = (top_left.p[0] / menu.window_size.0 as f32) * 2.0 - 1.0;
        let y_ndc = -((top_left.p[1] / menu.window_size.1 as f32) * 2.0 - 1.0);

        let matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
            sx,  0.0, 0.0, 0.0,
            0.0, sy,  0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            x_ndc, y_ndc, 0.0, 1.0,
        ).into();

        rusttype::draw(
            &text,
            &menu.system,
            frame,
            matrix,
            self.color.v
                .into()
        ).unwrap();
    }
}
