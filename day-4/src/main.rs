mod renderer;
mod shader;
mod video_capture;

use gstreamer::prelude::*;
use nannou::prelude::*;
use renderer::VideoRenderer;
use video_capture::SharedFrame;

struct Model {
    renderer: VideoRenderer,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    // Create window
    app.new_window()
        .title("GStreamer Video - HD")
        .size(1280, 720)
        .view(view)
        .build()
        .unwrap();

    let window = app.main_window();

    // Setup video capture
    let shared_frame = SharedFrame::new();
    let gst_pipeline = video_capture::build_pipeline(None, shared_frame.clone()).unwrap();
    gst_pipeline
        .set_state(gstreamer::State::Playing)
        .expect("Unable to set pipeline to Playing state");

    // Create renderer
    let renderer = VideoRenderer::new(&window, shared_frame, gst_pipeline);

    Model { renderer }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.renderer.update(app);
}

fn view(app: &App, model: &Model, frame: Frame) {
    model.renderer.render(app, frame);
}
