mod renderer;
mod shader;
mod video_capture;

use gstreamer::prelude::*;
use nannou::prelude::*;
use renderer::VideoRenderer;
use video_capture::SharedFrame;

use std::time::{Duration, Instant};

struct Model {
    renderer: VideoRenderer,
    last_update: Instant,
    frame_duration: Duration, // Target duration between frames
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    // Create window
    app.new_window()
        .title("Video Partial Updates - Random Squares")
        .size(1280, 1024)
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

    // Set up frame rate limiter (target 30 FPS)
    Model { 
        renderer,
        last_update: Instant::now(),
        frame_duration: Duration::from_secs_f64(1.0 / 30.0), // 30 FPS
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Frame rate limiting
    let now = Instant::now();
    let elapsed = now.duration_since(model.last_update);
    
    // Only update if enough time has passed
    if elapsed >= model.frame_duration {
        model.renderer.update(app);
        model.last_update = now;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    model.renderer.render(app, frame);
}
