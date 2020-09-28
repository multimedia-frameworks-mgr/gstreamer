extern crate gstreamer as gst;
use gst::prelude::*;
use std::collections::HashMap;

use std::time::{Duration, Instant};

fn get_faces_num(structure: &gst::StructureRef) -> usize {
    match structure.get_optional::<gst::List>("faces") {
        Ok(Some(res)) => res.as_slice().len(),
        _ => 0,
    }
}

struct SelectorController {
    last_change: Instant,
    active_pad: gst::Pad,
    faces_per_pad: HashMap<gst::Pad, usize>,
}

impl SelectorController {
    fn new(pad: gst::Pad) -> Self {
        let mut faces = HashMap::new();
        faces.insert(pad.clone(), 0);

        Self {
            last_change: Instant::now(),
            active_pad: pad,
            faces_per_pad: faces,
        }
    }

    fn update(&mut self, pad: gst::Pad, faces: usize) {
        self.faces_per_pad.insert(pad.clone(), faces);
    }

    fn pad_to_activate(&mut self) -> Option<gst::Pad> {
        // println!("{:?}", self.faces_per_pad);
        if self.last_change.elapsed() < Duration::from_millis(200) {
            return None;
        }
        let top_pad = self
            .faces_per_pad
            .iter()
            .max_by(|(_a_pad, a_faces), (_b_pad, b_faces)| a_faces.cmp(b_faces));

        let result = top_pad.and_then(|(pad, _faces)| {
            if *pad != self.active_pad {
                Some(pad.clone())
            } else {
                None
            }
        });

        if let Some(pad) = &result {
            self.active_pad = pad.clone();
            self.last_change = Instant::now();
        };

        result
    }
}

pub fn run(streams_num: i32) {
    const WIDTH: i32 = 360;
    const HEIGHT: i32 = 240;
    const FPS: i32 = 25;
    const XML: &str = "haarcascade_frontalface_alt.xml";

    gst::init().unwrap();
    let mut pipe_string = format!(
        "input-selector name=selector sync-streams=true sync-mode=1 ! videoconvert ! queue  name=out ! autovideosink sync=false
        "
    );

    for i in 0..streams_num {
        pipe_string.push_str(&format!(
            "pushfilesrc location=face{index}.h264
            ! video/x-h264,width={width},height={height},framerate={fps}/1,stream-format=byte-stream
            ! h264parse
            ! avdec_h264
            ! videoconvert
            ! identity sync=true
            ! video/x-raw,width={width},height={height},framerate={fps}/1
            ! facedetect min-neighbors=2 scale-factor=1.1 updates=1 display=0 profile={xml}
            ! queue leaky=2
            ! selector.
            ",
            index = i,
            width = WIDTH,
            height = HEIGHT,
            fps = FPS,
            xml = XML
        ));
    }

    //println!("{}", pipe_string);
    let pipeline = gst::parse_launch(&pipe_string).unwrap();
    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();
    // let src = pipeline.get_by_name("src").unwrap();

    let time = Instant::now();
    let out = pipeline.get_by_name("out").unwrap();
    let sink_pad = out.get_static_pad("sink").unwrap();
    sink_pad.add_probe(gst::PadProbeType::BUFFER, move |_, _probe_info| {
        println!("{:?}", time.elapsed().as_millis());
        gst::PadProbeReturn::Ok
    });

    let selector = pipeline.get_by_name("selector").unwrap();
    let first_active_pad = selector.get_static_pad("sink_0").unwrap();

    let mut controller = SelectorController::new(first_active_pad);
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
                    continue;
                }
                println!("{}", get_faces_num(structure));
                let pad = element
                    .get_src()
                    .unwrap()
                    .downcast::<gst::Element>()
                    .unwrap()
                    .get_static_pad("src")
                    .unwrap()
                    .get_peer()
                    .unwrap()
                    .get_parent()
                    .unwrap()
                    .downcast::<gst::Element>()
                    .unwrap()
                    .get_static_pad("src")
                    .unwrap()
                    .get_peer()
                    .unwrap();

                controller.update(pad, get_faces_num(structure));
                if let Some(new_pad) = controller.pad_to_activate() {
                    println!("Swapping to pad {:?}", new_pad.get_name());
                    selector.set_property("active_pad", &new_pad).unwrap()
                }
            }
            _ => (),
        }
    }

    // Shutdown pipeline
    pipeline.set_state(gst::State::Null).unwrap();
}
