use nannou::prelude::*;

// Helper function to calculate scale based on proximity to an edge
fn calculate_edge_scale(position: f32, edge_position: f32, dimension: f32, threshold_percent: f32, is_upper_edge: bool) -> f32 {
    let threshold_distance = dimension * threshold_percent;
    let distance_from_edge = if is_upper_edge {
        edge_position - position // For top/right edges
    } else {
        position - edge_position // For bottom/left edges
    };
    
    if distance_from_edge < threshold_distance && distance_from_edge >= 0.0 {
        map_range(
            distance_from_edge,
            0.0,
            threshold_distance,
            0.6,
            1.0,
        )
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
    }
}

fn min(a: f32, b: f32) -> f32 {
    if a < b { return a; }
    b
}

fn update(app: &App, model: &mut Model, update: Update) {
    let dt = update.since_last.as_secs_f32();

    model.position += model.velocity * dt;

    let rect = app.window_rect();
    
    // Check for collisions with walls and increase velocity on bounce
    let velocity_boost_factor = 1.8; // Increase velocity by 50% on bounce
    
    // X-axis collision
    if model.position.x > rect.right() || model.position.x < rect.left() {
        // Reverse and boost x velocity
        model.velocity.x = -model.velocity.x * velocity_boost_factor;
    }
    
    // Y-axis collision
    if model.position.y > rect.top() || model.position.y < rect.bottom() {
        // Reverse and boost y velocity
        model.velocity.y = -model.velocity.y * velocity_boost_factor;
    }
    
    // Gradually return to base velocity
    let velocity_decay_rate = 0.98; // Decay rate per update
    let velocity_threshold = 0.1; // Threshold for considering velocities equal
    
    // X-axis velocity decay
    let x_diff = model.velocity.x.abs() - model.base_velocity.x.abs();
    if x_diff.abs() > velocity_threshold {
        // Gradually reduce velocity toward base velocity
        if model.velocity.x.abs() > model.base_velocity.x.abs() {
            model.velocity.x *= velocity_decay_rate;
        } else {
            model.velocity.x /= velocity_decay_rate;
        }
    } else {
        // Ensure correct direction is maintained
        model.velocity.x = model.base_velocity.x.abs() * model.velocity.x.signum();
    }
    
    // Y-axis velocity decay
    let y_diff = model.velocity.y.abs() - model.base_velocity.y.abs();
    if y_diff.abs() > velocity_threshold {
        // Gradually reduce velocity toward base velocity
        if model.velocity.y.abs() > model.base_velocity.y.abs() {
            model.velocity.y *= velocity_decay_rate;
        } else {
            model.velocity.y /= velocity_decay_rate;
        }
    } else {
        // Ensure correct direction is maintained
        model.velocity.y = model.base_velocity.y.abs() * model.velocity.y.signum();
    }

    // Reset scale to default
    model.scale = vec2(1.0, 1.0);
    
    // Apply edge scaling
    let edge_threshold_percent = 0.1; // 10% from edges
    
    // Check and apply scaling for each edge
    model.scale.y = min(
        calculate_edge_scale(model.position.y, rect.bottom(), rect.h(), edge_threshold_percent, false),
        calculate_edge_scale(model.position.y, rect.top(), rect.h(), edge_threshold_percent, true)
    );
    model.scale.x = min(
        calculate_edge_scale(model.position.x, rect.left(), rect.w(), edge_threshold_percent, false),
        calculate_edge_scale(model.position.x, rect.right(), rect.w(), edge_threshold_percent, true)
    );

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let window_size = app.window_rect().wh();

    draw.texture(&model.dvd_logo)
        .wh(window_size / 6. * model.scale)
        .xy(model.position);
    
    draw.to_frame(app, &frame).unwrap();
}
