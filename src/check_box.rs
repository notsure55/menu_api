use std::cell::RefCell;
use std::rc::Rc;

use crate::{ Rect, Menu, Vertex, Vec4 };

use glium::{ Surface, uniform, Frame };

pub struct CheckBox {
    rect: Rect,
    color: Vec4,
    toggle: Rc<RefCell<bool>>,
}

impl CheckBox {
    pub fn new(rect: Rect, color: Vec4, toggle: Rc<RefCell<bool>>) -> Self {
        Self {
            rect,
            toggle,
            color,
        }
    }
    pub fn toggle(&self) {
        let mut value = self.toggle.borrow_mut();
        *value = !*value;
    }
    pub fn draw(&self, menu: &mut Menu, frame: &mut Frame) {
        let uniforms = uniform! {
            screen_size: [menu.window_size.0 as f32, menu.window_size.1 as f32],
            color_input: [self.color.v[0], self.color.v[1], self.color.v[2], self.color.v[3]]
        };

        let shape = vec![
            Vertex { p: [ self.rect.top_left.p[0], self.rect.top_left.p[1] ] },
            Vertex { p: [ self.rect.top_left.p[0] + self.rect.width, self.rect.top_left.p[1]] },
            Vertex { p: [ self.rect.top_left.p[0] + self.rect.width, self.rect.top_left.p[1] + self.rect.height] },
            Vertex { p: [ self.rect.top_left.p[0], self.rect.top_left.p[1] + self.rect.height] },
        ];

        let vertex_buffer = glium::VertexBuffer::new(&menu.display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

        let vertex_shader_src = r#"
        #version 140

        in vec2 p;
        uniform vec2 screen_size;

        void main() {
        vec2 zero_to_one = p / screen_size;
        vec2 zero_to_two = zero_to_one * 2.0;
        vec2 clip_space = zero_to_two - 1.0;
        clip_space.y = -clip_space.y;

        gl_Position = vec4(clip_space, 0.0, 1.0);
        }
        "#;
        let fragment_shader_src = r#"
        #version 140

        uniform vec4 color_input;
        out vec4 color;

        void main() {
        color = color_input;
        }
        "#;

        let program = glium::Program::from_source(&menu.display, vertex_shader_src, fragment_shader_src, None).unwrap();

        frame.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &Default::default()
        ).unwrap();
    }
}
