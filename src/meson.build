sources = files('main.vala')

vapi_dir = meson.current_source_dir() / 'vapi'

add_project_arguments(['--vapidir', vapi_dir], language: 'vala')

libthreatdetect_proj = subproject('libthreatdetect')
libthreatdetect_dep = libthreatdetect_proj.get_variable('libthreatdetect_dep')
deps = vala_deps + [
  dependency('gstreamer-1.0'),
  libthreatdetect_dep,
  meson.get_compiler('vala').find_library('threatdetect', dir: vapi_dir)
]

executable('cctvselector', sources, dependencies: deps)
