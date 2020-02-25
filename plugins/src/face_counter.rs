use gst;
use opencv::{core::*, imgproc, objdetect, prelude::*, types};
use std::ffi::c_void;
use std::mem;
use std::sync::Mutex;

#[derive(Copy, Clone)]
pub struct ImageDims {
    pub width: i32,
    pub height: i32,
}

const XML: &str = "/usr/share/opencv4/haarcascades/haarcascade_frontalface_alt.xml";

pub struct FaceCounter {
    classifier: Mutex<objdetect::CascadeClassifier>,
}

// unsafe impl Sync for FaceCounter {}

impl FaceCounter {
    pub fn new() -> Self {
        Self {
            classifier: Mutex::new(objdetect::CascadeClassifier::new(&XML).unwrap()),
        }
    }

    pub fn detect_faces(&self, buffer: gst::Buffer, dims: ImageDims) -> i32 {
        // At this point, buffer is only a reference to an existing memory region somewhere.
        // When we want to access its content, we have to map it while requesting the required
        // mode of access (read, read/write).
        // This type of abstraction is necessary, because the buffer in question might not be
        // on the machine's main memory itself, but rather in the GPU's memory.
        // So mapping the buffer makes the underlying memory region accessible to us.
        // See: https://gstreamer.freedesktop.org/documentation/plugin-development/advanced/allocation.html
        let map = buffer.into_mapped_buffer_readable().unwrap();
        let data = map.as_ptr() as *const c_void;
        let gray_frame = Mat::new_rows_cols_with_data(
            dims.height,
            dims.width,
            CV_8UC1,
            unsafe { mem::transmute(data) },
            Mat_AUTO_STEP,
        )
        .unwrap()
        .get_umat(
            AccessFlag::ACCESS_READ,
            UMatUsageFlags::USAGE_ALLOCATE_DEVICE_MEMORY,
        )
        .unwrap();
        let mut faces = types::VectorOfRect::new();

        unsafe {
            let mut norm_gray_frame = UMat::new_rows_cols(
                dims.height,
                dims.width,
                CV_8UC1,
                UMatUsageFlags::USAGE_ALLOCATE_DEVICE_MEMORY,
            )
            .unwrap();
            imgproc::equalize_hist(&gray_frame, &mut norm_gray_frame).unwrap();

            self.classifier
                .lock()
                .unwrap()
                .detect_multi_scale(
                    &norm_gray_frame,
                    &mut faces,
                    1.2,
                    3,
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
        };
        faces.len() as i32
    }
}
