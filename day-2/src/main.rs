use nannou::prelude::*;

// Helper function to calculate scale based on proximity to an edge
fn calculate_edge_scale(
    position: f32,
    edge_position: f32,
    dimension: f32,
    threshold_percent: f32,
    is_upper_edge: bool,
) -> f32 {
    let threshold_distance = dimension * threshold_percent;
    let distance_from_edge = if is_upper_edge {
        edge_position - position // For top/right edges
    } else {
        position - edge_position // For bottom/left edges
    };

    if distance_from_edge < threshold_distance && distance_from_edge >= 0.0 {
        map_range(distance_from_edge, 0.0, threshold_distance, 0.6, 1.0)
    } else {
        1.0
    }
}

struct Model {
    pub dvd_logo: wgpu::Texture,
    position: Vec2,
    base_velocity: Vec2,
    velocity: Vec2,
    scale: Vec2,
    dvd_wh: Vec2,
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let base_velocity = vec2(190., 80.);
    Model {
        dvd_logo: wgpu::Texture::from_path(_app, "assets/dvd_logo.png").unwrap(),
        position: vec2(0., 0.),
        base_velocity: base_velocity,
        velocity: base_velocity,
        scale: vec2(1., 1.),
        dvd_wh: vec2(0., 0.),
    }
}

fn min(a: f32, b: f32) -> f32 {
    if a < b {
        return a;
    }
    b
}

fn update(app: &App, model: &mut Model, update: Update) {
    let dt = update.since_last.as_secs_f32();

    model.position += model.velocity * dt;

    let window_rect = app.window_rect();

    model.dvd_wh = window_rect.wh() / 6. * model.scale;

    let dvd_rect = geom::Rect::from_xy_wh(model.position, model.dvd_wh);

    let velocity_boost_factor = 8.0;

    if dvd_rect.right() > window_rect.right() || dvd_rect.left() < window_rect.left() {
        model.velocity.x = -model.velocity.x * velocity_boost_factor;
    }

    if dvd_rect.top() > window_rect.top() || dvd_rect.bottom() < window_rect.bottom() {
        model.velocity.y = -model.velocity.y * velocity_boost_factor;
    }

    // Gradually return to base velocity
    let velocity_decay_rate = 0.18; // Decay rate per update
    let velocity_threshold = 0.1; // Threshold for considering velocities equal

    let x_diff = model.velocity.x.abs() - model.base_velocity.x.abs();
    if x_diff.abs() > velocity_threshold {
        if model.velocity.x.abs() > model.base_velocity.x.abs() {
            model.velocity.x *= velocity_decay_rate;
        } else {
            model.velocity.x /= velocity_decay_rate;
        }
    } else {
        model.velocity.x = model.base_velocity.x.abs() * model.velocity.x.signum();
    }

    let y_diff = model.velocity.y.abs() - model.base_velocity.y.abs();
    if y_diff.abs() > velocity_threshold {
        if model.velocity.y.abs() > model.base_velocity.y.abs() {
            model.velocity.y *= velocity_decay_rate;
        } else {
            model.velocity.y /= velocity_decay_rate;
        }
    } else {
        model.velocity.y = model.base_velocity.y.abs() * model.velocity.y.signum();
    }

    model.scale = vec2(1.0, 1.0);

    let edge_threshold_percent = 0.1;

    model.scale.y = min(
        calculate_edge_scale(
            model.position.y,
            window_rect.bottom(),
            window_rect.h(),
            edge_threshold_percent,
            false,
        ),
        calculate_edge_scale(
            model.position.y,
            window_rect.top(),
            window_rect.h(),
            edge_threshold_percent,
            true,
        ),
    );
    model.scale.x = min(
        calculate_edge_scale(
            model.position.x,
            window_rect.left(),
            window_rect.w(),
            edge_threshold_percent,
            false,
        ),
        calculate_edge_scale(
            model.position.x,
            window_rect.right(),
            window_rect.w(),
            edge_threshold_percent,
            true,
        ),
    );
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.texture(&model.dvd_logo)
        .wh(model.dvd_wh)
        .xy(model.position);

    draw.to_frame(app, &frame).unwrap();
}
