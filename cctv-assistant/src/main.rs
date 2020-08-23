mod pipelines {
    pub mod opencv_face_detect;
    pub mod rs_face_selector;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let streams_num: i32 = match args.get(2).map( |s| s.parse()) {
        Some(Ok(number)) => number,
        _ => 3
    };

    match args.get(1).map(|s| s.as_ref()) {
        Some("rs") =>  pipelines::rs_face_selector::run(streams_num),
        Some("opencv") => pipelines::opencv_face_detect::run(streams_num),
        _ => pipelines::rs_face_selector::run(streams_num),
    }
}
