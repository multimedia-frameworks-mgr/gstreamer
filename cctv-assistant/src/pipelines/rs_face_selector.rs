extern crate gstreamer as gst;
use gst::prelude::*;

use std::sync::{Arc, Mutex};
use std::time::Instant;

const WIDTH: i32 = 320;
const HEIGHT: i32 = 240;

pub fn run() {
    gst::init().unwrap();
    let pipeline = gst::parse_launch(&format!(
        "rsFaceSelector name=selector ! autovideosink name=output sync=false
        v4l2src device=/dev/video0
            ! videoconvert 
            ! queue name=src 
            ! video/x-raw,format=I420,width={width},height={height},framerate=30/1
            ! selector.sink_0
        videotestsrc is-live=1 ! queue ! video/x-raw,format=I420,width={width},height={height},framerate=30/1 ! selector.sink_1",
        width = WIDTH,
        height = HEIGHT
    ))
    .unwrap();
    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();
    let output = pipeline.get_by_name("output").unwrap();
    // let selector = pipeline.get_by_name("selector").unwrap();
    let output_pad = output.get_static_pad("sink").unwrap();

    let start: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
    let probe = move |_: &gst::Pad, _probe_info: &mut gst::PadProbeInfo| {
        let mut ref_time = start.lock().unwrap();
        if let Some(time) = *ref_time {
            println!("{}", time.elapsed().as_micros());
        }
        *ref_time = Some(Instant::now());

        gst::PadProbeReturn::Ok
    };

    output_pad.add_probe(gst::PadProbeType::BUFFER, probe);

    pipeline.set_state(gst::State::Playing).unwrap();

    // Wait until error or EOS
    let bus = pipeline.get_bus().unwrap();
    while let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            _ => (),
        }
    }

    // Shutdown pipeline
    pipeline.set_state(gst::State::Null).unwrap();
}
