use glib;
use glib::subclass;
use glib::subclass::prelude::*;

use super::base as gst_base;
use gst;
use gst::prelude::*;
use gst::subclass::prelude::*;
// use gst_base;
use gst_base::prelude::*;
use gst_base::subclass::prelude::*;
use gst_video;

use std::i32;
use std::sync::Mutex;
use std::time::Instant;

use super::face_counter::*;

const IMAGE_WIDTH: i32 = 320;
const IMAGE_HEIGHT: i32 = 240;

struct FaceSelector {
    detector: FaceCounter,
    last_selected_pad: Mutex<String>,
}

impl FaceSelector {}

impl ObjectImpl for FaceSelector {
    glib_object_impl!();
}

impl ElementImpl for FaceSelector {}

impl AggregatorImpl for FaceSelector {
    fn aggregate(
        &self,
        aggregator: &gst_base::Aggregator,
        _timeout: bool,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let mut buffers = Vec::new();
        aggregator.foreach_sink_pad(|_elem, pad| {
            let agg_pad = pad.clone().downcast::<gst_base::AggregatorPad>().unwrap();
            if let Some(buffer) = agg_pad.pop_buffer() {
                buffers.push((buffer, agg_pad.get_name()));
            }
            true
        });

        if buffers.len() == 1 {
            let (buffer, pad_name) = buffers.first().unwrap();
            let mut buffer = buffer.copy();
            let mut last_selected_pad = self.last_selected_pad.lock().unwrap();

            if *last_selected_pad != pad_name.to_string() {
                buffer.make_mut().set_flags(gst::BufferFlags::DISCONT);
                *last_selected_pad = pad_name.to_string();
            }

            return aggregator.finish_buffer(buffer.to_owned());
        }

        // let start = Instant::now();
        let dims = ImageDims {
            width: IMAGE_WIDTH,
            height: IMAGE_HEIGHT,
        };
        let most_faces_buffer: Option<(gst::Buffer, glib::GString)> = buffers
            .into_iter()
            .max_by_key(|(buffer, _pad_name)| self.detector.detect_faces(buffer.copy(), dims));

        //println!("Elapsed: {} microseconds", start.elapsed().as_micros());
        if let Some((buffer, pad_name)) = most_faces_buffer {
            let mut last_selected_pad = self.last_selected_pad.lock().unwrap();
            let mut buffer = buffer.copy();

            if *last_selected_pad != pad_name.to_string() {
                buffer.make_mut().set_flags(gst::BufferFlags::DISCONT);
                *last_selected_pad = pad_name.to_string();
            }

            aggregator.finish_buffer(buffer.copy())
        } else {
            Err(gst_base::AGGREGATOR_FLOW_NEED_DATA)
        }
    }
}

impl ObjectSubclass for FaceSelector {
    const NAME: &'static str = "RsFaceSelector";
    type ParentType = gst_base::Aggregator;
    type Instance = gst::subclass::ElementInstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib_object_subclass!();

    fn new() -> Self {
        Self {
            detector: FaceCounter::new(),
            last_selected_pad: Mutex::new(String::from("sink_0")),
        }
    }

    fn class_init(klass: &mut subclass::simple::ClassStruct<Self>) {
        klass.set_metadata(
            "Face-detecting selector",
            "Generic",
            "Detects faces on sources and picks the source to output",
            "Bartosz Błaszków <bbartosz06@gmail.com>",
        );

        let caps = gst::Caps::builder("video/x-raw")
            .field("format", &gst_video::VideoFormat::I420.to_str())
            .field("width", &IMAGE_WIDTH)
            .field("height", &IMAGE_HEIGHT)
            .field("framerate", &gst::Fraction::new(30, 1))
            .build();

        let sink_pad_tmpl = gst::PadTemplate::new_with_gtype(
            "sink_%d",
            gst::PadDirection::Sink,
            gst::PadPresence::Request,
            &caps,
            gst_base::AggregatorPad::static_type(),
        )
        .unwrap();
        klass.add_pad_template(sink_pad_tmpl);

        let src_pad_template = gst::PadTemplate::new(
            "src",
            gst::PadDirection::Src,
            gst::PadPresence::Always,
            &caps,
        )
        .unwrap();
        klass.add_pad_template(src_pad_template);
    }
}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "rsFaceSelector",
        gst::Rank::None,
        FaceSelector::get_type(),
    )
}
