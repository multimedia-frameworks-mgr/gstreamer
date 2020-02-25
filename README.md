# gstreamer
Processing pipeline implemented in GStreamer

WIP:

```bash
cd plugins
cargo build --release
export GST_PLUGIN_PATH="`pwd`/target/release"
gst-launch-1.0 v4l2src device=/dev/video0 ! video/x-raw,width=320,height=240 ! videoconvert ! queue ! sel. videotestsrc ! rsFaceSelector name=sel ! videoconvert ! autovideosink
```
