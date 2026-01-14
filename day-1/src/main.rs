use nannou::prelude::*;

mod star;
use star::Star;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    for point in star::Star::new(200., 200.).points().iter().step_by(10) {
        let star = Star::new(
            4. + point.y.abs() / (app.time.sin().abs() + 0.55),
            6. + 10. * (1. - app.time.sin()).abs(),
        );
        draw.polygon()
            .color(RED)
            .points(star.points())
            .x_y(point.x, point.y);
    }

    draw.to_frame(app, &frame).unwrap();
}
