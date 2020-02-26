extern crate gstreamer as gst;
use gst::prelude::*;

fn get_faces_num(structure: &gst::StructureRef) -> usize {
    match structure.get_optional::<gst::List>("faces") {
        Ok(Some(res)) => res.as_slice().len(),
        _ => 0,
    }
}

pub fn run() {
    const WIDTH: i32 = 320;
    const HEIGHT: i32 = 240;

    gst::init().unwrap();
    let pipeline = gst::parse_launch(&format!(
        "input-selector name=selector ! autovideosink sync=false
        v4l2src do-timestamp=true device=/dev/video0
            ! videoconvert 
            ! queue name=src 
            ! video/x-raw,width={width},height={height},framerate=30/1
            ! facedetect updates=1 profile=/usr/share/opencv4/haarcascades/haarcascade_frontalface_alt.xml
            ! videoconvert
            ! selector.sink_0
        videotestsrc is-live=1 ! queue ! video/x-raw,format=I420,width={width},height={height},framerate=30/1 ! selector.sink_1",
        width = WIDTH,
        height = HEIGHT
    ))
    .unwrap();
    // let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();
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
            MessageView::Element(element) => {
                let structure = element.get_structure().unwrap();
                if structure.get_name() != "facedetect" {
                    println!("not facedetect");
                    continue;
                }
                println!("{}", get_faces_num(structure));
            }
            _ => (),
        }
    }

    // Shutdown pipeline
    pipeline.set_state(gst::State::Null).unwrap();
}
