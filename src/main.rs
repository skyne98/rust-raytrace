use notan::prelude::*;

//language=glsl
const VERT: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450

    layout(location = 0) in vec4 a_position;
    layout(location = 1) in vec2 a_texcoord;

    layout(location = 0) out vec2 v_texcoord;

    void main() {
        v_texcoord = a_texcoord;
        gl_Position = vec4(a_position.x, a_position.y * -1.0, a_position.z, 1.0);
    }
    "#
};

//language=glsl
const FRAG: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec2 v_texcoord;

    layout(location = 0) out vec4 outColor;

    layout(set = 0, binding = 1) uniform u_data {
        float data[4];
    };

    void main() {
        // outColor = vec4(v_texcoord, 0.0, 1.0);
        outColor = vec4(data[0], data[1], data[2], data[3]);
    }
    "#
};

#[derive(AppState)]
struct State {
    clear_options: ClearOptions,
    pipeline: Pipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    uniform: Buffer,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup).draw(draw).build()
}

fn setup(gfx: &mut Graphics) -> State {
    let clear_options = ClearOptions::color(Color::new(0.1, 0.2, 0.3, 1.0));

    let vertex_info = VertexInfo::new()
        .attr(0, VertexFormat::Float32x3)
        .attr(1, VertexFormat::Float32x2);

    let pipeline = gfx
        .create_pipeline()
        .from(&VERT, &FRAG)
        .with_vertex_info(&vertex_info)
        .with_color_blend(BlendMode::NORMAL)
        .with_texture_location(0, "u_texture")
        .build()
        .unwrap();

    #[rustfmt::skip]
    let vertices = [
        //pos               //coords
        1.0,  1.0, 0.0,     1.0, 1.0,
        1.0, -1.0, 0.0,     1.0, 0.0,
        -1.0, -1.0, 0.0,    0.0, 0.0,
        -1.0,  1.0, 0.0,    0.0, 1.0
    ];

    #[rustfmt::skip]
    let indices = [
        0, 1, 3,
        1, 2, 3,
    ];

    let vertex_buffer = gfx
        .create_vertex_buffer()
        .with_info(&vertex_info)
        .with_data(&vertices)
        .build()
        .unwrap();

    let index_buffer = gfx
        .create_index_buffer()
        .with_data(&indices)
        .build()
        .unwrap();

    let uniform_buffer = gfx
        .create_uniform_buffer(1, "u_data")
        .with_data(&[1.0f32, 0.0, 0.0, 1.0])
        .build()
        .expect("Error creating the uniform buffer");

    State {
        clear_options,
        pipeline,
        vertex_buffer,
        index_buffer,
        uniform: uniform_buffer,
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut renderer = gfx.create_renderer();

    renderer.begin(Some(state.clear_options));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_buffers(&[&state.vertex_buffer, &state.index_buffer]);
    renderer.bind_buffer(&state.uniform);
    renderer.draw(0, 6);
    renderer.end();

    gfx.render(&renderer);
}
