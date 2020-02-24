extern crate gstreamer as gst;
use core::sync::atomic::{AtomicU8, Ordering};
use gst::prelude::*;
use opencv::{core::*, imgproc, objdetect, prelude::*, types};
use std::ffi::c_void;
use std::mem;
use std::sync::Mutex;
use std::time::{Duration, Instant};

const XML: &str = "/usr/share/opencv4/haarcascades/haarcascade_frontalface_alt.xml";
const WIDTH: i32 = 320;
const HEIGHT: i32 = 240;

fn main() {
    gst::init().unwrap();
    let pipeline = gst::parse_launch(&format!(
        "input-selector name=selector ! autovideosink
        v4l2src device=/dev/video0
            ! videoscale 
            ! videoconvert 
            ! queue name=src 
            ! video/x-raw,format=I420,width={width},height={height}
            ! selector.sink_0
        videotestsrc ! video/x-raw,format=I420,width={width},height={height} ! selector.sink_1",
        width = WIDTH,
        height = HEIGHT
    ))
    .unwrap();
    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();
    let src = pipeline.get_by_name("src").unwrap();
    let selector = pipeline.get_by_name("selector").unwrap();
    let src_pad = src.get_static_pad("src").unwrap();

    let face = Mutex::new(objdetect::CascadeClassifier::new(&XML).unwrap());
    let counter = AtomicU8::new(0);
    src_pad.add_probe(gst::PadProbeType::BUFFER, move |_, probe_info| {
        let start = Instant::now();
        let buf_number = counter.fetch_add(1, Ordering::Relaxed);
        if buf_number % 10 != 0 {
            return gst::PadProbeReturn::Ok;
        }
        if let Some(gst::PadProbeData::Buffer(ref buffer)) = probe_info.data {
            // At this point, buffer is only a reference to an existing memory region somewhere.
            // When we want to access its content, we have to map it while requesting the required
            // mode of access (read, read/write).
            // This type of abstraction is necessary, because the buffer in question might not be
            // on the machine's main memory itself, but rather in the GPU's memory.
            // So mapping the buffer makes the underlying memory region accessible to us.
            // See: https://gstreamer.freedesktop.org/documentation/plugin-development/advanced/allocation.html
            let map = buffer.map_readable().unwrap();
            let data = map.as_ptr() as *const c_void;
            let gray_frame = Mat::new_rows_cols_with_data(
                HEIGHT,
                WIDTH,
                CV_8UC1,
                unsafe { mem::transmute(data) },
                Mat_AUTO_STEP,
            )
            .unwrap();
            let mut faces = types::VectorOfRect::new();

            unsafe {
                let mut norm_gray_frame = Mat::new_rows_cols(HEIGHT, WIDTH, CV_8UC1).unwrap();
                imgproc::equalize_hist(&gray_frame, &mut norm_gray_frame).unwrap();

                face.lock()
                    .unwrap()
                    .detect_multi_scale(
                        &norm_gray_frame,
                        &mut faces,
                        1.1,
                        2,
                        objdetect::CASCADE_SCALE_IMAGE,
                        Size {
                            width: 30,
                            height: 30,
                        },
                        Size {
                            width: 0,
                            height: 0,
                        },
                    )
                    .unwrap();
            }
            println!("Faces: {}", faces.len());
            if faces.len() > 0 {
                let pad = selector.get_static_pad("sink_0").unwrap();
                selector.set_property("active-pad", &pad).unwrap();
            } else {
                let pad = selector.get_static_pad("sink_1").unwrap();
                selector.set_property("active-pad", &pad).unwrap();
            }
        }

        println!("Elapsed: {} microseconds", start.elapsed().as_micros());
        gst::PadProbeReturn::Ok
    });

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
