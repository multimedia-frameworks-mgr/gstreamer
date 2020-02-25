extern crate cc;
extern crate pkg_config;

fn main() {
    if cfg!(feature = "v1_18") {
        return;
    }

    let gstreamer = pkg_config::probe_library("gstreamer-1.0").unwrap();
    let includes = [gstreamer.include_paths];

    let files = ["src/base/gstaggregator.c"];

    let mut build = cc::Build::new();
    build.include("src/base");

    for f in files.iter() {
        build.file(f);
    }

    for p in includes.iter().flat_map(|i| i) {
        build.include(p);
    }

    build.define(
        "PACKAGE_BUGREPORT",
        "\"https://gitlab.freedesktop.org/gstreamer/gstreamer/issues/new\"",
    );
    build.define("GstAggregator", "GstAggregatorFallback");
    build.define("GstAggregatorClass", "GstAggregatorFallbackClass");
    build.define("GstAggregatorPrivate", "GstAggregatorFallbackPrivate");
    build.define("GstAggregatorPad", "GstAggregatorFallbackPad");
    build.define("GstAggregatorPadClass", "GstAggregatorFallbackPadClass");
    build.define("GstAggregatorPadPrivate", "GstAggregatorFallbackPadPrivate");
    build.define("GST_BASE_API", "G_GNUC_INTERNAL");

    build.flag("-w");
    build.compile("libgstaggregator-c.a");
}
