use macroquad::{prelude::*, miniquad::gl::GL_FRAGMENT_SHADER};
use glam::vec3;
use shaders::{FRAGMENT_SHADER, VERTEX_SHADER};

mod shaders;

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;

struct Ball {
    pos: Vec3,
    vel : Vec3,
    radius: f32,
    color: Color
}

impl Ball {
    fn new(pos: Vec3, vel: Vec3, radius: f32, color: Color) -> Ball {
        Ball { pos, vel, radius, color }
    }

    fn update(&mut self) {
        self.pos += self.vel;

        if self.pos.x > 10. - self.radius || self.pos.x < -10. + self.radius {
            self.vel.x *= -1.;
        }
        if self.pos.y > 10. - self.radius || self.pos.y < -10. + self.radius {
            self.vel.y *= -1.;
        }
        if self.pos.z > 10. - self.radius || self.pos.z < -10. + self.radius {
            self.vel.z *= -1.;
        }
    }
}

struct Boundary {
    pos: Vec3,
    size: Vec3
}

impl Boundary {
    fn new(pos: Vec3, size: Vec3) -> Boundary {
        Boundary {
            pos,
            size
        }
    }
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Balls but 3D"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut x = 0.0;
    let mut switch = false;
    let bounds = 8.0;

    let world_up = vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;

    let mut front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let mut right = front.cross(world_up).normalize();
    let mut up;

    let mut position = vec3(0.0, 1.0, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);

    // Walls + Floor + Roof
    let bound_arr = [
        Boundary::new(vec3(-10.0, -0., 0.0), vec3(0.1, 20.0, 20.0)),
        Boundary::new(vec3(10.0, -0., 0.0), vec3(0.1, 20.0, 20.0)),
        Boundary::new(vec3(0.0, -0., -10.0), vec3(20.0, 20.0, 0.1)),
        Boundary::new(vec3(0.0, 0., 10.0), vec3(20.0, 20.0, 0.1)),
        Boundary::new(vec3(0.0, -10., 0.0), vec3(20.0, 0.1, 20.0)),
        Boundary::new(vec3(0.0, 10., 0.0), vec3(20.0, 0.1, 20.0))
    ];
    let mut ball_vec: Vec<Ball> = vec![
        Ball::new(vec3(0., 0., 0.), vec3(0.1, 0.2, 0.3), 0.5, YELLOW)
    ];

    let mut fragment_shader = FRAGMENT_SHADER.to_string();
    let mut vertex_shader = VERTEX_SHADER.to_string();
    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };
    let mut material = load_material(
        &vertex_shader,
        &fragment_shader,
        MaterialParams {
            pipeline_params,
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        if is_key_down(KeyCode::W) {
            position += front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::S) {
            position -= front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::A) {
            position -= right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::D) {
            position += right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Space) {
            position.y += MOVE_SPEED;
        }
        if is_key_down(KeyCode::LeftControl) {
            position.y -= MOVE_SPEED;
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;
        last_mouse_position = mouse_position;

        yaw += mouse_delta.x * delta * LOOK_SPEED;
        pitch += mouse_delta.y * delta * -LOOK_SPEED;

        pitch = if pitch > 1.5 { 1.5 } else { pitch };
        pitch = if pitch < -1.5 { -1.5 } else { pitch };

        front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();

        right = front.cross(world_up).normalize();
        up = right.cross(front).normalize();

        x += if switch { 0.04 } else { -0.04 };
        if x >= bounds || x <= -bounds {
            switch = !switch;
        }

        clear_background(LIGHTGRAY);

        gl_use_default_material();

        // 3D
        set_camera(&Camera3D {
            position,
            up,
            target: position + front,
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        for element in &bound_arr {
            draw_cube(element.pos, element.size, None, BLACK);
            draw_cube_wires(element.pos, element.size, GREEN)
        }
        for item in &mut ball_vec {
            item.update();
            draw_sphere(item.pos, item.radius, None, item.color);
        }

        // Back to screen space, render some text
        set_default_camera();

        next_frame().await
    }
}
