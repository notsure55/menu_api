use crate::{ MenuObject, Rect, Menu, Vertex, Vec4, Draw, InBounds, Hovering, Clicked, MenuOptions, Options, outline_box, Draggable, label };

use glium::{ Surface, uniform, Frame };

#[derive(Default)]
pub struct FilledBox {
    options: MenuOptions,
    pub rect: Rect,
    color: Vec4,
    label: Option<label::Label>,
}

impl FilledBox {
    pub fn new(options: MenuOptions, rect: Rect, color: Vec4, label: Option<label::Label>) -> Self {
        Self {
            options,
            rect,
            color,
            label,
        }
    }
}

impl InBounds for FilledBox {
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

impl Hovering for FilledBox {
    fn is_hovering(
        &self,
        menu: &mut Menu,
        frame: &mut Frame,
    ) {
        if self.in_bounds(menu) {
            let top_left = Vertex { p: [ self.rect.top_left.p[0] - 2.0, self.rect.top_left.p[1] - 2.0] };
            let mut outline = outline_box::OutlineBox::new(
                MenuOptions::new(false, false, false, true),
                Rect::new(top_left, self.rect.width + 4.0, self.rect.height + 4.0),
                Vec4::new(1.0, 0.0, 0.0, 1.0),
                4.0,
                None,
            );
            outline.draw(menu, frame);
        }
    }
}

impl Draggable for FilledBox {
    fn is_dragging(
        &mut self,
        menu: &mut Menu,
    ) {
        if menu.mouse_pos.0 != menu.cached_mouse_pos.0 || menu.mouse_pos.1 != menu.cached_mouse_pos.1 {
            // make so we cant move it out of base_me
            let new_x = self.rect.top_left.p[0] + menu.mouse_pos.0 - menu.cached_mouse_pos.0;
            let new_y = self.rect.top_left.p[1] + menu.mouse_pos.1 - menu.cached_mouse_pos.1;
            if self.in_bounds(&menu) && menu.dragging
                && new_x < menu.base.rect.top_left.p[0] + menu.base.rect.width - self.rect.width
                && new_x > menu.base.rect.top_left.p[0]
                && new_y < menu.base.rect.top_left.p[1] + menu.base.rect.height - self.rect.height
                && new_y > menu.base.rect.top_left.p[1]
            {
                self.rect.top_left.p[0] = new_x;
                self.rect.top_left.p[1] = new_y;
                menu.cached_mouse_pos = menu.mouse_pos;
            }
        }
    }
}

impl Clicked for FilledBox {
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

impl Options for FilledBox {
    fn get_options(&self) -> MenuOptions {
        self.options
    }
}

impl Draw for FilledBox {
    fn draw(
        &mut self,
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

        match &self.label {
            Some(label) => label.draw(&self.rect, menu, frame),
            None => (),
        };
    }
}
