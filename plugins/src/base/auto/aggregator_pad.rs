// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use super::super::gst_base_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gst;
use gst_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct AggregatorPad(Object<gst_base_sys::GstAggregatorPad, gst_base_sys::GstAggregatorPadClass, AggregatorPadClass>) @extends gst::Pad, gst::Object;

    match fn {
        get_type => || gst_base_sys::gst_aggregator_pad_get_type(),
    }
}

unsafe impl Send for AggregatorPad {}
unsafe impl Sync for AggregatorPad {}

pub const NONE_AGGREGATOR_PAD: Option<&AggregatorPad> = None;

pub trait AggregatorPadExt: 'static {
    fn drop_buffer(&self) -> bool;

    fn has_buffer(&self) -> bool;

    fn is_eos(&self) -> bool;

    fn peek_buffer(&self) -> Option<gst::Buffer>;

    fn pop_buffer(&self) -> Option<gst::Buffer>;

    fn get_property_emit_signals(&self) -> bool;

    fn set_property_emit_signals(&self, emit_signals: bool);

    fn connect_buffer_consumed<F: Fn(&Self, &gst::Buffer) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_emit_signals_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<AggregatorPad>> AggregatorPadExt for O {
    fn drop_buffer(&self) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_aggregator_pad_drop_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_buffer(&self) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_aggregator_pad_has_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_eos(&self) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_aggregator_pad_is_eos(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn peek_buffer(&self) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(gst_base_sys::gst_aggregator_pad_peek_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pop_buffer(&self) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(gst_base_sys::gst_aggregator_pad_pop_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_property_emit_signals(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"emit-signals\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `emit-signals` getter")
                .unwrap()
        }
    }

    fn set_property_emit_signals(&self, emit_signals: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"emit-signals\0".as_ptr() as *const _,
                Value::from(&emit_signals).to_glib_none().0,
            );
        }
    }

    fn connect_buffer_consumed<F: Fn(&Self, &gst::Buffer) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn buffer_consumed_trampoline<
            P,
            F: Fn(&P, &gst::Buffer) + Send + Sync + 'static,
        >(
            this: *mut gst_base_sys::GstAggregatorPad,
            object: *mut gst_sys::GstBuffer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AggregatorPad>,
        {
            let f: &F = &*(f as *const F);
            f(
                &AggregatorPad::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"buffer-consumed\0".as_ptr() as *const _,
                Some(transmute(buffer_consumed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_emit_signals_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_emit_signals_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstAggregatorPad,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AggregatorPad>,
        {
            let f: &F = &*(f as *const F);
            f(&AggregatorPad::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::emit-signals\0".as_ptr() as *const _,
                Some(transmute(
                    notify_emit_signals_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}
