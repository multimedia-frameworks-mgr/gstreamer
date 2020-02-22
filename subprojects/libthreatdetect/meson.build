project('libthreatdetect', 'cpp')

inc = include_directories('include')
sources = files('threatdetect.cpp')
deps = [
  dependency('opencv4')
]

libthreatdetect = static_library(
  'threatdetect',
  sources,
  dependencies: deps,
  include_directories: inc
)

libthreatdetect_dep = declare_dependency(
  include_directories: inc,
  link_with: libthreatdetect,
  #dependencies: deps,
)
