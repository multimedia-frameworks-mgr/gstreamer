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
```bash
cargo build --release
export GST_PLUGIN_PATH="`pwd`/target/release"
./target/release/cctv-assistant
```
