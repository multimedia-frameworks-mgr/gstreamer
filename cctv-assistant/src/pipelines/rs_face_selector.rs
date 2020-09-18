extern crate gstreamer as gst;
use gst::prelude::*;

const WIDTH: i32 = 1080;
const HEIGHT: i32 = 720;
const FPS: i32 = 25;

pub fn run(streams_num: i32) {
    gst::init().unwrap();

    let mut pipe_string = format!("rsFaceSelector name=selector ! autovideosink");
    for i in 0..streams_num {
        pipe_string.push_str(&format!(
            "
            pushfilesrc location=face{index}.h264
            ! video/x-h264,width={width},height={height},framerate={fps}/1,stream-format=byte-stream
            ! h264parse
            ! avdec_h264
            ! videoconvert
            ! video/x-raw,format=I420,width={width},height={height},framerate={fps}/1
            ! identity sync=true
            ! queue leaky=2
            ! selector.
            ",
            index = i,
            width = WIDTH,
            height = HEIGHT,
            fps = FPS
        ));
    }
    let pipeline = gst::parse_launch(&pipe_string).unwrap();
    // let pipeline = pipeline.dynamic_cast::<gst::pipeline>().unwrap();
    // let src = pipeline.get_by_name("src").unwrap();
    // let selector = pipeline.get_by_name("selector").unwrap();
    // let src_pad = src.get_static_pad("src").unwrap();

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
