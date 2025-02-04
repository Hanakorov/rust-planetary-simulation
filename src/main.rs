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

fn draw_planet_with_shadow(body: &Body, light_source: Vec2) {
    // Отрисовка основного тела планеты
    draw_circle(body.position.x, body.position.y, body.radius, body.color);

    // Рассчёт направления на свет
    let light_dir = (light_source - body.position).normalize();

    // Смещение тени
    let shadow_offset = light_dir * (body.radius * 0.5);

    // Отрисовка тёмной части (тень)
    draw_circle(
        body.position.x - shadow_offset.x,
        body.position.y - shadow_offset.y,
        body.radius,
        Color::new(0.0, 0.0, 0.0, 0.5),
    );
}

fn draw_orbit(center: Vec2, radius: f32, color: Color) {
    draw_circle_lines(center.x, center.y, radius, 1.0, color);
}

#[macroquad::main("Orbital Simulation")]
async fn main() {
    let center = vec2(screen_width() / 2.0, screen_height() / 2.0);

    let planets = vec![Body {
        position: center,
        velocity: vec2(0.0, 0.0),
        radius: 50.0, // Радиус звезды уменьшен до 50
        color: YELLOW,
    }];

    let mut satellite_1 = create_satellite(center, 100.0, BLUE);
    let mut satellite_2 = create_satellite(center, 200.0, RED);

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        // Обновление позиций
        satellite_1.update_position(&planets, dt);
        satellite_2.update_position(&planets, dt);

        // Отрисовка звезды
        draw_circle(center.x, center.y, planets[0].radius, planets[0].color);

        // Отрисовка орбит
        draw_orbit(center, 100.0, DARKGRAY);
        draw_orbit(center, 200.0, DARKGRAY);

        // Отрисовка планет с тенями
        draw_planet_with_shadow(&satellite_1, center);
        draw_planet_with_shadow(&satellite_2, center);

        next_frame().await;
    }
}
