using Gst;
using ThreatDetect;

public static int main (string[] args) {
  // Initialize GStreamer:
  Gst.init (ref args);

  // Build the pipeline:
  Gst.Element pipeline;
  try {
    pipeline = Gst.parse_launch ("playbin uri=https://gstreamer.freedesktop.org/media/large/starwars.mkv");
  } catch (Error e) {
    stderr.printf ("Error: %s\n", e.message);
    return 0;
  }

  // Start playing:
  int res = ThreatDetect.foo();
  stdout.printf("Res: %d\n", res);
  pipeline.set_state (Gst.State.PLAYING);

  // Wait until error or EOS:
  Gst.Bus bus = pipeline.get_bus ();
  bus.timed_pop_filtered (Gst.CLOCK_TIME_NONE, Gst.MessageType.ERROR | Gst.MessageType.EOS);

  // Free resources:
  pipeline.set_state (Gst.State.NULL);
  return 0;
}
