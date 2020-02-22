#include "opencv2/highgui/highgui.hpp"
#include "opencv2/imgproc/imgproc.hpp"
#include "opencv2/objdetect/objdetect.hpp"
#include "threatdetect.hpp"
#include <iostream>
#include <stdio.h>

using namespace std;
using namespace cv;

extern "C" {
  String face_cascade_name = "haarcascade_frontalface_alt.xml";
  CascadeClassifier face_cascade;
  string window_name = "Capture - Face detection";

  RNG rng(12345);

  int init() {
    if (!face_cascade.load(face_cascade_name)) {
      return -1;
    };
    return 0;
  }

  void detectAndDisplay(uint8_t * frame, size_t frame_size) {
    std::vector<Rect> faces;
    vector<uint8_t> frame_vec;
    frame_vec.assign(frame, frame + frame_size);
    equalizeHist(frame_vec, frame_vec);
    //-- Detect faces
    face_cascade.detectMultiScale(frame_vec, faces, 1.1, 2,
        0 | CASCADE_SCALE_IMAGE, Size(30, 30));
    // Return faces somehow
  }

  int foo() {
    cout << "Hello!" << endl;
    return 0;
  }
}
