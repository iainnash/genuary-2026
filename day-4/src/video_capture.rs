use anyhow::{Context, Result};
use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_app as gst_app;
use std::sync::{Arc, Mutex};

const VIDEO_WIDTH: u32 = 1280;
const VIDEO_HEIGHT: u32 = 1024;

#[derive(Clone)]
pub struct SharedFrame {
    current_frame: Arc<Mutex<Option<Vec<u8>>>>,
}

impl SharedFrame {
    pub fn new() -> Self {
        Self {
            current_frame: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_current(&self, data: Vec<u8>) {
        *self.current_frame.lock().unwrap() = Some(data);
    }

    pub fn take_current(&self) -> Option<Vec<u8>> {
        self.current_frame.lock().unwrap().take()
    }
}

pub fn build_pipeline(path: Option<&str>, shared: SharedFrame) -> Result<gst::Pipeline> {
    gst::init()?;

    let desc = if let Some(path) = path {
        format!(
            "filesrc location=\"{}\" ! decodebin ! videoconvert ! videoscale \
            ! video/x-raw,format=RGBA,width={},height={} \
            ! appsink name=sink sync=false max-buffers=1 drop=true",
            path, VIDEO_WIDTH, VIDEO_HEIGHT
        )
    } else {
        format!(
            "avfvideosrc device-index=0 do-timestamp=true \
            ! videorate max-rate=30 \
            ! videoconvert \
            ! videoscale method=nearest-neighbour \
            ! video/x-raw,format=RGBA,width={},height={},framerate=30/1 \
            ! appsink name=sink sync=false max-buffers=1 drop=true",
            VIDEO_WIDTH, VIDEO_HEIGHT
        )
    };

    let pipeline = gst::parse::launch(&desc)?
        .downcast::<gst::Pipeline>()
        .map_err(|_| anyhow::anyhow!("Not a pipeline"))?;

    let sink = pipeline
        .by_name("sink")
        .context("sink not found")?
        .downcast::<gst_app::AppSink>()
        .map_err(|_| anyhow::anyhow!("sink is not an AppSink"))?;

    sink.set_callbacks(
        gst_app::AppSinkCallbacks::builder()
            .new_sample(move |sink| {
                let sample = sink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                let buffer = sample.buffer().ok_or(gst::FlowError::Error)?;
                let map = buffer.map_readable().map_err(|_| gst::FlowError::Error)?;
                shared.set_current(map.as_slice().to_vec());
                Ok(gst::FlowSuccess::Ok)
            })
            .build(),
    );

    Ok(pipeline)
}

pub const WIDTH: u32 = VIDEO_WIDTH;
pub const HEIGHT: u32 = VIDEO_HEIGHT;
