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
//use std::sync::Mutex;

const IMAGE_WIDTH: i32 = 320;
const IMAGE_HEIGHT: i32 = 240;

struct FaceSelector {}

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
                buffers.push(buffer);
            }
            true
        });

        if let Some(buffer) = buffers.first() {
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
        Self {}
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
