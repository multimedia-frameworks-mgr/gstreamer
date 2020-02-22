extern crate gstreamer as gst;
use gst::prelude::*;

fn main() {
    gst::init().unwrap();
    let pipeline = gst::parse_launch(
        "v4l2src device=/dev/video0 ! videoconvert name=src ! video/x-raw,format=I420 ! autovideosink",
    )
    .unwrap();
    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();
    let src = pipeline.get_by_name("src").unwrap();
    let src_pad = src.get_static_pad("src").unwrap();

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

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
    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}
