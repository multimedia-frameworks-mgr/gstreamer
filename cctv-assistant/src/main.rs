mod pipelines {
    pub mod opencv_face_detect;
    pub mod rs_face_selector;
}

fn main() {
    pipelines::rs_face_selector::run()
    // pipelines::opencvFaceDetect::run()
}
