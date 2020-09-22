# gstreamer
Processing pipeline implemented in GStreamer

## Dependencies:
* GStreamer 1.16
  * gstreamer
  * gst-plugins-base
  * gst-plugins-base-libs
  * gst-plugins-good
  * gst-plugins-bad
  * gst-plugins-ugly
* OpenCV 4
* gst-plugins-opencv

## Running

### Preps
```bash
brew install rustup
rustup toolchain install stable
cargo build --release
```

Additionally, pipelines expect file `faceX.h264` where X is number of input starting from 0
Parameters of video are hardcoded in pipelines and plugin:
- cctv-assistant/src/pipelines/opencv_face_detect.rs
- cctv-assistant/src/pipelines/rs_face_selector.rs
- plugins/src/face_selector.rs

### Running GstAggregate-base plugin version
```bash
cargo build --release
export GST_PLUGIN_PATH="`pwd`/target/release"
./target/release/cctv-assistant rs <number of inputs>
```

### Running opencv-plugin version
```bash
cargo build --release
./target/release/cctv-assistant opencv <number_of_inputs>
```

## Measurements

```bash
sar -P ALL 5 20 >measurements/proc_rs_2.txt & ; ./target/release/cctv-assistant | tee measurements/times_rs_2.txt && pkill -SIGINT sar
```