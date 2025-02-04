use macroquad::prelude::*;

#[derive(Clone)]
struct Body {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    color: Color,
}

impl Body {
    fn update_position(&mut self, planets: &[Body], dt: f32) {
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

fn create_satellite(center: Vec2, orbit_radius: f32, color: Color) -> Body {
    let start_pos = vec2(center.x + orbit_radius, center.y);
    let direction = (start_pos - center).normalize();
    let orbital_speed = (5000.0_f32 / orbit_radius).sqrt();
    let velocity = vec2(-direction.y, direction.x) * orbital_speed;

    Body {
        position: start_pos,
        velocity,
        radius: 10.0,
        color,
    }
}

fn draw_orbit_trail(trail: &[Vec2], color: Color) {
    for i in 1..trail.len() {
        draw_line(trail[i - 1].x, trail[i - 1].y, trail[i].x, trail[i].y, 2.0, color);
    }
}

fn update_trails(trails: &mut [Vec<Vec2>], satellites: &[Body]) {
    for (trail, satellite) in trails.iter_mut().zip(satellites) {
        trail.push(satellite.position);
        if trail.len() > 500 {
            trail.remove(0);
        }
    }
}

#[macroquad::main("Orbital Simulation")]
async fn main() {
    let center = vec2(screen_width() / 2.0, screen_height() / 2.0);

    let planets = vec![Body {
        position: center,
        velocity: vec2(0.0, 0.0),
        radius: 100.0,
        color: YELLOW,
    }];

    let mut satellite_1 = create_satellite(center, 100.0, BLUE);
    let mut satellite_2 = create_satellite(center, 200.0, RED);

    let  trail_1: Vec<Vec2> = Vec::new();
    let  trail_2: Vec<Vec2> = Vec::new();

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        // Обновление позиций
        satellite_1.update_position(&planets, dt);
        satellite_2.update_position(&planets, dt);

        // Обновление траекторий
        update_trails(&mut [trail_1.clone(), trail_2.clone()], &[satellite_1.clone(), satellite_2.clone()]);

        // Отрисовка орбит
        draw_circle_lines(center.x, center.y, 100.0, 1.0, DARKGRAY);
        draw_circle_lines(center.x, center.y, 200.0, 1.0, DARKGRAY);

        // Отрисовка траекторий
        draw_orbit_trail(&trail_1, BLUE);
        draw_orbit_trail(&trail_2, RED);

        // Отрисовка объектов
        draw_circle(center.x, center.y, 30.0, YELLOW);
        draw_circle(satellite_1.position.x, satellite_1.position.y, satellite_1.radius, satellite_1.color);
        draw_circle(satellite_2.position.x, satellite_2.position.y, satellite_2.radius, satellite_2.color);

        // Отображение информации
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
