// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`StyleSchemeChooser`](crate::StyleSchemeChooser) interface.

use super::PtrHolder;
use crate::{StyleScheme, StyleSchemeChooser};
use glib::subclass::prelude::*;
use glib::{once_cell::sync::Lazy, Cast, Quark};
use glib::{prelude::*, translate::*};

pub trait StyleSchemeChooserImpl: ObjectImpl {
    fn style_scheme(&self) -> StyleScheme {
        self.parent_style_scheme()
    }

    fn set_style_scheme(&self, style_scheme: &StyleScheme) {
        self.parent_set_style_scheme(style_scheme)
    }
}

pub trait StyleSchemeChooserImplExt: ObjectSubclass {
    fn parent_style_scheme(&self) -> StyleScheme;

    fn parent_set_style_scheme(&self, style_scheme: &StyleScheme);
}

impl<T: StyleSchemeChooserImpl> StyleSchemeChooserImplExt for T {
    fn parent_style_scheme(&self) -> StyleScheme {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<StyleSchemeChooser>()
                as *const ffi::GtkSourceStyleSchemeChooserInterface;

            let func = (*parent_iface)
                .get_style_scheme
                .expect("no parent \"get_style_scheme\" implementation");

            from_glib_none(func(
                self.obj()
                    .unsafe_cast_ref::<StyleSchemeChooser>()
                    .to_glib_none()
                    .0,
            ))
        }
    }

    fn parent_set_style_scheme(&self, style_scheme: &StyleScheme) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<StyleSchemeChooser>()
                as *const ffi::GtkSourceStyleSchemeChooserInterface;

            let func = (*parent_iface)
                .set_style_scheme
                .expect("no parent \"set_style_scheme\" implementation");

            func(
                self.obj()
                    .unsafe_cast_ref::<StyleSchemeChooser>()
                    .to_glib_none()
                    .0,
                style_scheme.to_glib_none().0,
            )
        }
    }
}

unsafe impl<T: StyleSchemeChooserImpl> IsImplementable<T> for StyleSchemeChooser {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.get_style_scheme = Some(style_scheme_chooser_get_style_scheme::<T>);
        iface.set_style_scheme = Some(style_scheme_chooser_set_style_scheme::<T>);
    }
}

static STYLE_SCHEME_CHOOSER_GET_STYLE_SCHEME: Lazy<Quark> =
    Lazy::new(|| Quark::from_str("sourceview5-rs-subclass-style-scheme-chooser-get-style-scheme"));
unsafe extern "C" fn style_scheme_chooser_get_style_scheme<T: StyleSchemeChooserImpl>(
    style_scheme_chooser: *mut ffi::GtkSourceStyleSchemeChooser,
) -> *mut ffi::GtkSourceStyleScheme {
    let instance = &*(style_scheme_chooser as *mut T::Instance);
    let imp = instance.imp();

    let style_scheme = imp.style_scheme().to_glib_full();

    // It's a transfer none so we have to keep an instance of it around
    imp.obj().set_qdata(
        *STYLE_SCHEME_CHOOSER_GET_STYLE_SCHEME,
        PtrHolder(style_scheme, |ptr| {
            glib::gobject_ffi::g_object_unref(ptr as *mut _)
        }),
    );

    style_scheme
}

unsafe extern "C" fn style_scheme_chooser_set_style_scheme<T: StyleSchemeChooserImpl>(
    style_scheme_chooser: *mut ffi::GtkSourceStyleSchemeChooser,
    style_scheme: *mut ffi::GtkSourceStyleScheme,
) {
    let instance = &*(style_scheme_chooser as *mut T::Instance);
    let imp = instance.imp();

    imp.set_style_scheme(&from_glib_borrow::<_, StyleScheme>(style_scheme))
}
