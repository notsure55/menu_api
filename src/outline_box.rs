use crate::{ Rect, Menu, Vertex, Vec4, Draw, InBounds, Hovering, Clicked, MenuOptions, Options };
use glium::{ Surface, uniform, Frame };

pub struct OutlineBox {
    options: MenuOptions,
    rect: Rect,
    color: Vec4,
    thickness: f32,
}

impl OutlineBox {
    pub fn new(options: MenuOptions, rect: Rect, color: Vec4, thickness: f32) -> Self {
        Self {
            options,
            rect,
            color,
            thickness,
        }
    }
}

impl InBounds for OutlineBox {
    fn in_bounds(
        &self,
        menu: &Menu
    ) -> bool {
        if menu.mouse_pos.0 < self.rect.top_left.p[0] + self.rect.width && menu.mouse_pos.0 > self.rect.top_left.p[0]
        && menu.mouse_pos.1 < self.rect.top_left.p[1] + self.rect.height && menu.mouse_pos.1 > self.rect.top_left.p[1] {
            true
        } else {
            false
        }
    }
}

impl Hovering for OutlineBox {
    fn is_hovering(
        &self,
        menu: &mut Menu,
        frame: &mut Frame,
    ) {
        if self.in_bounds(menu) {
            //let top_left = Vertex { position: [ top_left.position[0] - 2.0, top_left.position[1] - 2.0] };
            //let outline = OutlineBox::new(frame, top_left, width + 4.0, height + 4.0);
            //menu.add_to_draw_list(MenuObject::OutlineBox(outline));
        }
    }
}

impl Clicked for OutlineBox {
    fn clicked(
        &self,
        menu: &mut Menu,
        frame: &mut Frame,
    ) -> bool {
        if self.in_bounds(&menu) && menu.clicked {
            return true
        } else {
            return false
        }
    }
}

impl Options for OutlineBox {
    fn get_options(&self) -> &MenuOptions {
        &self.options
    }
}

impl Draw for OutlineBox {
    fn draw(
        &self,
        menu: &mut Menu,
        frame: &mut Frame
    ) {
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
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineLoop);

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

        let params = glium::DrawParameters {
            line_width: Some(self.thickness),
            .. Default::default()
        };

        let program = glium::Program::from_source(&menu.display, vertex_shader_src, fragment_shader_src, None).unwrap();

        frame.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &params
        ).unwrap();
    }
}
