use crate::{ MenuObject, Rect, Menu, Vertex, Vec4, Draw, InBounds, Hovering, Clicked, MenuOptions, Options, outline_box, Draggable, label };

use glium::{ Surface, uniform, Frame };

pub struct LineList {
    vertexs: Vec<Vertex>,
    color: Vec4,
}

impl LineList {
    pub fn new(
        vertexs: Vec<Vertex>,
        color: Vec4,
    ) -> Self {
        Self {
            vertexs,
            color,
        }
    }
}

impl Draw for LineList {
    fn draw(
        &mut self,
        menu: &mut Menu,
        frame: &mut Frame
    ) {
        let uniforms = uniform! {
            screen_size: [menu.window_size.0 as f32, menu.window_size.1 as f32],
            color_input: [self.color.v[0], self.color.v[1], self.color.v[2], self.color.v[3]]
        };

        let vertex_buffer = glium::VertexBuffer::new(&menu.display, &self.vertexs).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

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
