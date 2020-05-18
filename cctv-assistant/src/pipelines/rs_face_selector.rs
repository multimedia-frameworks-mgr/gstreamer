extern crate gstreamer as gst;
use gst::prelude::*;

const WIDTH: i32 = 320;
const HEIGHT: i32 = 240;

pub fn run() {
    gst::init().unwrap();
    let pipeline = gst::parse_launch(&format!(
        "rsFaceSelector name=selector ! autovideosink
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
    // let pipeline = pipeline.dynamic_cast::<gst::pipeline>().unwrap();
    // let src = pipeline.get_by_name("src").unwrap();
    // let selector = pipeline.get_by_name("selector").unwrap();
    // let src_pad = src.get_static_pad("src").unwrap();

    pipeline.set_state(gst::State::Playing).unwrap();

    // Wait until error or EOS
    let bus = pipeline.get_bus().unwrap();
    let bin = pipeline.dynamic_cast::<gst::Bin>().unwrap();
    println!(
        "{}",
        bin.debug_to_dot_data(gstreamer::DebugGraphDetails::all())
    );
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
    bin.set_state(gst::State::Null).unwrap();
}
