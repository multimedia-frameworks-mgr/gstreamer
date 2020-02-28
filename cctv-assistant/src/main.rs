mod pipelines {
    pub mod opencv_face_detect;
    pub mod rs_face_selector;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_ref()) {
        Some("rs") =>  pipelines::rs_face_selector::run(),
        Some("opencv") => pipelines::opencv_face_detect::run(),
        _ => pipelines::rs_face_selector::run(),
    }
}
