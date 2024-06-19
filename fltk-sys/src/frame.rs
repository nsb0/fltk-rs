/* automatically generated by rust-bindgen 0.69.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
pub type Fl_Callback = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::core::ffi::c_void),
>;
pub type custom_handler_callback = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut Fl_Widget,
        arg2: ::core::ffi::c_int,
        arg3: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type custom_draw_callback = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::core::ffi::c_void),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Box {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Box_new(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        title: *const ::core::ffi::c_char,
    ) -> *mut Fl_Box;
}
extern "C" {
    pub fn Fl_Box_x(arg1: *mut Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_y(arg1: *mut Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_width(arg1: *mut Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_height(arg1: *mut Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_label(arg1: *mut Fl_Box) -> *const ::core::ffi::c_char;
}
extern "C" {
    pub fn Fl_Box_set_label(arg1: *mut Fl_Box, title: *const ::core::ffi::c_char);
}
extern "C" {
    pub fn Fl_Box_redraw(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_show(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_hide(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_activate(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_deactivate(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_redraw_label(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_resize(
        arg1: *mut Fl_Box,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_Box_widget_resize(
        arg1: *mut Fl_Box,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_Box_tooltip(arg1: *mut Fl_Box) -> *const ::core::ffi::c_char;
}
extern "C" {
    pub fn Fl_Box_set_tooltip(arg1: *mut Fl_Box, txt: *const ::core::ffi::c_char);
}
extern "C" {
    pub fn Fl_Box_get_type(arg1: *mut Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_set_type(arg1: *mut Fl_Box, typ: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Box_color(arg1: *mut Fl_Box) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_Box_set_color(arg1: *mut Fl_Box, color: ::core::ffi::c_uint);
}
extern "C" {
    pub fn Fl_Box_measure_label(
        arg1: *const Fl_Box,
        arg2: *mut ::core::ffi::c_int,
        arg3: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_Box_label_color(arg1: *mut Fl_Box) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_Box_set_label_color(arg1: *mut Fl_Box, color: ::core::ffi::c_uint);
}
extern "C" {
    pub fn Fl_Box_label_font(arg1: *mut Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_set_label_font(arg1: *mut Fl_Box, font: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Box_label_size(arg1: *mut Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_set_label_size(arg1: *mut Fl_Box, sz: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Box_label_type(arg1: *mut Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_set_label_type(arg1: *mut Fl_Box, typ: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Box_box(arg1: *mut Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_set_box(arg1: *mut Fl_Box, typ: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Box_changed(arg1: *mut Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_set_changed(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_clear_changed(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_align(arg1: *mut Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_set_align(arg1: *mut Fl_Box, typ: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Box_delete(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_set_image(arg1: *mut Fl_Box, arg2: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_Box_handle(
        self_: *mut Fl_Box,
        cb: custom_handler_callback,
        data: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_Box_handle_event(self_: *mut Fl_Box, event: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_draw(
        self_: *mut Fl_Box,
        cb: custom_draw_callback,
        data: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_Box_resize_callback(
        self_: *mut Fl_Box,
        cb: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut Fl_Widget,
                x: ::core::ffi::c_int,
                y: ::core::ffi::c_int,
                w: ::core::ffi::c_int,
                h: ::core::ffi::c_int,
                arg2: *mut ::core::ffi::c_void,
            ),
        >,
        data: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_Box_set_when(arg1: *mut Fl_Box, arg2: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Box_when(arg1: *const Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_image(arg1: *const Fl_Box) -> *const ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Box_parent(self_: *const Fl_Box) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Box_selection_color(arg1: *mut Fl_Box) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_Box_set_selection_color(arg1: *mut Fl_Box, color: ::core::ffi::c_uint);
}
extern "C" {
    pub fn Fl_Box_do_callback(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_inside(
        self_: *const Fl_Box,
        arg1: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_window(arg1: *const Fl_Box) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Box_top_window(arg1: *const Fl_Box) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Box_takes_events(arg1: *const Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_user_data(arg1: *const Fl_Box) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Box_take_focus(self_: *mut Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_set_visible_focus(self_: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_clear_visible_focus(self_: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_visible_focus(self_: *mut Fl_Box, v: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Box_has_visible_focus(self_: *mut Fl_Box) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_Box_set_user_data(arg1: *mut Fl_Box, data: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_Box_draw_data(self_: *const Fl_Box) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Box_handle_data(self_: *const Fl_Box) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Box_set_draw_data(self_: *mut Fl_Box, data: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_Box_set_handle_data(self_: *mut Fl_Box, data: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_Box_damage(self_: *const Fl_Box) -> ::core::ffi::c_uchar;
}
extern "C" {
    pub fn Fl_Box_set_damage(self_: *mut Fl_Box, flag: ::core::ffi::c_uchar);
}
extern "C" {
    pub fn Fl_Box_set_damage_area(
        self_: *mut Fl_Box,
        flag: ::core::ffi::c_uchar,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_Box_clear_damage(self_: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_as_window(self_: *mut Fl_Box) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Box_as_group(self_: *mut Fl_Box) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Box_set_deimage(arg1: *mut Fl_Box, arg2: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_Box_deimage(arg1: *const Fl_Box) -> *const ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_Box_set_callback(
        arg1: *mut Fl_Box,
        arg2: Fl_Callback,
        arg3: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_Box_set_deleter(
        arg1: *mut Fl_Box,
        arg2: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>,
    );
}
extern "C" {
    pub fn Fl_Box_visible(self_: *const Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_visible_r(self_: *const Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_active(self_: *const Fl_Box) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_Box_active_r(self_: *const Fl_Box) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Box_callback(self_: *const Fl_Box) -> Fl_Callback;
}
extern "C" {
    pub fn Fl_Box_set_deletion_callback(
        self_: *mut Fl_Box,
        arg1: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::core::ffi::c_void),
        >,
        data: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_Box_from_dyn_ptr(ptr: *mut Fl_Widget) -> *mut Fl_Box;
}
extern "C" {
    pub fn Fl_Box_from_derived_dyn_ptr(ptr: *mut Fl_Widget) -> *mut Fl_Box;
}
extern "C" {
    pub fn Fl_Box_super_draw(ptr: *mut Fl_Widget, flag: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Box_super_draw_first(ptr: *mut Fl_Widget, flag: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_Box_super_handle_first(ptr: *mut Fl_Widget, flag: ::core::ffi::c_int);
}
