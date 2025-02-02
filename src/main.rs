use macroquad::prelude::*;

#[derive(Clone)]
struct Body {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    color: Color,
}

impl Body {
    fn update_position(&mut self, planets: &Vec<Body>, dt: f32) {
        for planet in planets {
            if planet.position != self.position {
                let direction = planet.position - self.position;
                let distance = direction.length().max(1.0);
                let force = direction.normalize() * (5000.0 / distance.powi(2));

                self.velocity += force * dt; 
            }
        }
        self.position += self.velocity * dt;
    }

    fn speed(&self) -> f32 {
        self.velocity.length()
    }
}

#[macroquad::main("Orbital Simulation")]
async fn main() {
    let center = vec2(screen_width() / 2.0, screen_height() / 2.0);

    let planets = vec![
        Body {
            position: center,
            velocity: vec2(0.0, 0.0),
            radius: 100.0,
            color: YELLOW,
        },
    ];

    let orbit_radius_1 = 100.0;
    let orbital_speed_1 = (5000.0_f32 / orbit_radius_1).sqrt();

    let start_pos_1 = vec2(center.x + orbit_radius_1, center.y);
    let direction_1 = (start_pos_1 - center).normalize();
    let velocity_1 = vec2(-direction_1.y, direction_1.x) * orbital_speed_1;

    let mut satellite_1 = Body {
        position: start_pos_1,
        velocity: velocity_1,
        radius: 10.0,
        color: BLUE,
    };

    let orbit_radius_2 = 200.0;
    let orbital_speed_2 = (5000.0_f32 / orbit_radius_2).sqrt(); 

    let start_pos_2 = vec2(center.x + orbit_radius_2, center.y);
    let direction_2 = (start_pos_2 - center).normalize();
    let velocity_2 = vec2(-direction_2.y, direction_2.x) * orbital_speed_2;

    let mut satellite_2 = Body {
        position: start_pos_2,
        velocity: velocity_2,
        radius: 10.0,
        color: RED,
    };

    let mut trail_1: Vec<Vec2> = Vec::new();
    let mut trail_2: Vec<Vec2> = Vec::new();

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        satellite_1.update_position(&planets, dt);
        satellite_2.update_position(&planets, dt);

        trail_1.push(satellite_1.position);
        trail_2.push(satellite_2.position);

        if trail_1.len() > 500 {
            trail_1.remove(0);
        }
        if trail_2.len() > 500 {
            trail_2.remove(0);
        }

        draw_circle_lines(center.x, center.y, orbit_radius_1, 1.0, DARKGRAY);
        draw_circle_lines(center.x, center.y, orbit_radius_2, 1.0, DARKGRAY);

        for i in 1..trail_1.len() {
            draw_line(trail_1[i - 1].x, trail_1[i - 1].y, trail_1[i].x, trail_1[i].y, 2.0, BLUE);
        }
        for i in 1..trail_2.len() {
            draw_line(trail_2[i - 1].x, trail_2[i - 1].y, trail_2[i].x, trail_2[i].y, 2.0, RED);
        }

        draw_circle(center.x, center.y, 30.0, YELLOW);

        draw_circle(satellite_1.position.x, satellite_1.position.y, satellite_1.radius, satellite_1.color);
        draw_circle(satellite_2.position.x, satellite_2.position.y, satellite_2.radius, satellite_2.color);

        // Добавление текста слева
        draw_text(
            &format!("Planet 1: Radius: {:.1}, Speed: {:.2}", satellite_1.radius, satellite_1.speed()),
            20.0,
            40.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Planet 2: Radius: {:.1}, Speed: {:.2}", satellite_2.radius, satellite_2.speed()),
            20.0,
            80.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
}
