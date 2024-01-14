/* automatically generated by rust-bindgen 0.69.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Image_draw(
        arg1: *mut Fl_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Image_draw_ext(
        arg1: *mut Fl_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        cx: ::std::os::raw::c_int,
        cy: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Image_width(arg1: *mut Fl_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Image_height(arg1: *mut Fl_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Image_delete(arg1: *mut Fl_Image);
}
extern "C" {
    pub fn Fl_Image_count(self_: *mut Fl_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Image_data(self_: *mut Fl_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Image_copy(self_: *mut Fl_Image) -> *mut Fl_Image;
}
extern "C" {
    pub fn Fl_Image_copy_sized(
        self_: *mut Fl_Image,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    ) -> *mut Fl_Image;
}
extern "C" {
    pub fn Fl_Image_scale(
        self_: *mut Fl_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Image_fail(self_: *mut Fl_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Image_data_w(self_: *const Fl_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Image_data_h(self_: *const Fl_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Image_d(self_: *const Fl_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Image_ld(self_: *const Fl_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Image_inactive(self_: *mut Fl_Image);
}
extern "C" {
    pub fn Fl_Image_from_dyn_ptr(other: *mut Fl_Image) -> *mut Fl_Image;
}
extern "C" {
    pub fn Fl_Image_set_scaling_algorithm(algorithm: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Image_scaling_algorithm() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_set_scaling_algorithm(algorithm: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_RGB_Image_scaling_algorithm() -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_JPEG_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_JPEG_Image_draw(
        arg1: *mut Fl_JPEG_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_JPEG_Image_draw_ext(
        arg1: *mut Fl_JPEG_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        cx: ::std::os::raw::c_int,
        cy: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_JPEG_Image_width(arg1: *mut Fl_JPEG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_height(arg1: *mut Fl_JPEG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_delete(arg1: *mut Fl_JPEG_Image);
}
extern "C" {
    pub fn Fl_JPEG_Image_count(self_: *mut Fl_JPEG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_data(self_: *mut Fl_JPEG_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_JPEG_Image_copy(self_: *mut Fl_JPEG_Image) -> *mut Fl_JPEG_Image;
}
extern "C" {
    pub fn Fl_JPEG_Image_copy_sized(
        self_: *mut Fl_JPEG_Image,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    ) -> *mut Fl_JPEG_Image;
}
extern "C" {
    pub fn Fl_JPEG_Image_scale(
        self_: *mut Fl_JPEG_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_JPEG_Image_fail(self_: *mut Fl_JPEG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_data_w(self_: *const Fl_JPEG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_data_h(self_: *const Fl_JPEG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_d(self_: *const Fl_JPEG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_ld(self_: *const Fl_JPEG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_inactive(self_: *mut Fl_JPEG_Image);
}
extern "C" {
    pub fn Fl_JPEG_Image_from_dyn_ptr(other: *mut Fl_Image) -> *mut Fl_JPEG_Image;
}
extern "C" {
    pub fn Fl_JPEG_Image_new(filename: *const ::std::os::raw::c_char) -> *mut Fl_JPEG_Image;
}
extern "C" {
    pub fn Fl_JPEG_Image_from(data: *const ::std::os::raw::c_uchar) -> *mut Fl_JPEG_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_PNG_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_PNG_Image_draw(
        arg1: *mut Fl_PNG_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_PNG_Image_draw_ext(
        arg1: *mut Fl_PNG_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        cx: ::std::os::raw::c_int,
        cy: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_PNG_Image_width(arg1: *mut Fl_PNG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_height(arg1: *mut Fl_PNG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_delete(arg1: *mut Fl_PNG_Image);
}
extern "C" {
    pub fn Fl_PNG_Image_count(self_: *mut Fl_PNG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_data(self_: *mut Fl_PNG_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_PNG_Image_copy(self_: *mut Fl_PNG_Image) -> *mut Fl_PNG_Image;
}
extern "C" {
    pub fn Fl_PNG_Image_copy_sized(
        self_: *mut Fl_PNG_Image,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    ) -> *mut Fl_PNG_Image;
}
extern "C" {
    pub fn Fl_PNG_Image_scale(
        self_: *mut Fl_PNG_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_PNG_Image_fail(self_: *mut Fl_PNG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_data_w(self_: *const Fl_PNG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_data_h(self_: *const Fl_PNG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_d(self_: *const Fl_PNG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_ld(self_: *const Fl_PNG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_inactive(self_: *mut Fl_PNG_Image);
}
extern "C" {
    pub fn Fl_PNG_Image_from_dyn_ptr(other: *mut Fl_Image) -> *mut Fl_PNG_Image;
}
extern "C" {
    pub fn Fl_PNG_Image_new(filename: *const ::std::os::raw::c_char) -> *mut Fl_PNG_Image;
}
extern "C" {
    pub fn Fl_PNG_Image_from(
        data: *const ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    ) -> *mut Fl_PNG_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_SVG_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_SVG_Image_draw(
        arg1: *mut Fl_SVG_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_SVG_Image_draw_ext(
        arg1: *mut Fl_SVG_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        cx: ::std::os::raw::c_int,
        cy: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_SVG_Image_width(arg1: *mut Fl_SVG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_height(arg1: *mut Fl_SVG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_delete(arg1: *mut Fl_SVG_Image);
}
extern "C" {
    pub fn Fl_SVG_Image_count(self_: *mut Fl_SVG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_data(self_: *mut Fl_SVG_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_SVG_Image_copy(self_: *mut Fl_SVG_Image) -> *mut Fl_SVG_Image;
}
extern "C" {
    pub fn Fl_SVG_Image_copy_sized(
        self_: *mut Fl_SVG_Image,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    ) -> *mut Fl_SVG_Image;
}
extern "C" {
    pub fn Fl_SVG_Image_scale(
        self_: *mut Fl_SVG_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_SVG_Image_fail(self_: *mut Fl_SVG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_data_w(self_: *const Fl_SVG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_data_h(self_: *const Fl_SVG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_d(self_: *const Fl_SVG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_ld(self_: *const Fl_SVG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_inactive(self_: *mut Fl_SVG_Image);
}
extern "C" {
    pub fn Fl_SVG_Image_from_dyn_ptr(other: *mut Fl_Image) -> *mut Fl_SVG_Image;
}
extern "C" {
    pub fn Fl_SVG_Image_new(filename: *const ::std::os::raw::c_char) -> *mut Fl_SVG_Image;
}
extern "C" {
    pub fn Fl_SVG_Image_from(data: *const ::std::os::raw::c_char) -> *mut Fl_SVG_Image;
}
extern "C" {
    pub fn Fl_SVG_Image_normalize(self_: *mut Fl_SVG_Image);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_BMP_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_BMP_Image_draw(
        arg1: *mut Fl_BMP_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_BMP_Image_draw_ext(
        arg1: *mut Fl_BMP_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        cx: ::std::os::raw::c_int,
        cy: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_BMP_Image_width(arg1: *mut Fl_BMP_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_height(arg1: *mut Fl_BMP_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_delete(arg1: *mut Fl_BMP_Image);
}
extern "C" {
    pub fn Fl_BMP_Image_count(self_: *mut Fl_BMP_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_data(self_: *mut Fl_BMP_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_BMP_Image_copy(self_: *mut Fl_BMP_Image) -> *mut Fl_BMP_Image;
}
extern "C" {
    pub fn Fl_BMP_Image_copy_sized(
        self_: *mut Fl_BMP_Image,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    ) -> *mut Fl_BMP_Image;
}
extern "C" {
    pub fn Fl_BMP_Image_scale(
        self_: *mut Fl_BMP_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_BMP_Image_fail(self_: *mut Fl_BMP_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_data_w(self_: *const Fl_BMP_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_data_h(self_: *const Fl_BMP_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_d(self_: *const Fl_BMP_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_ld(self_: *const Fl_BMP_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_inactive(self_: *mut Fl_BMP_Image);
}
extern "C" {
    pub fn Fl_BMP_Image_from_dyn_ptr(other: *mut Fl_Image) -> *mut Fl_BMP_Image;
}
extern "C" {
    pub fn Fl_BMP_Image_new(filename: *const ::std::os::raw::c_char) -> *mut Fl_BMP_Image;
}
extern "C" {
    pub fn Fl_BMP_Image_from(
        data: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_long,
    ) -> *mut Fl_BMP_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_GIF_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_GIF_Image_draw(
        arg1: *mut Fl_GIF_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_GIF_Image_draw_ext(
        arg1: *mut Fl_GIF_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        cx: ::std::os::raw::c_int,
        cy: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_GIF_Image_width(arg1: *mut Fl_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_height(arg1: *mut Fl_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_delete(arg1: *mut Fl_GIF_Image);
}
extern "C" {
    pub fn Fl_GIF_Image_count(self_: *mut Fl_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_data(self_: *mut Fl_GIF_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_GIF_Image_copy(self_: *mut Fl_GIF_Image) -> *mut Fl_GIF_Image;
}
extern "C" {
    pub fn Fl_GIF_Image_copy_sized(
        self_: *mut Fl_GIF_Image,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    ) -> *mut Fl_GIF_Image;
}
extern "C" {
    pub fn Fl_GIF_Image_scale(
        self_: *mut Fl_GIF_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_GIF_Image_fail(self_: *mut Fl_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_data_w(self_: *const Fl_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_data_h(self_: *const Fl_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_d(self_: *const Fl_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_ld(self_: *const Fl_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_inactive(self_: *mut Fl_GIF_Image);
}
extern "C" {
    pub fn Fl_GIF_Image_from_dyn_ptr(other: *mut Fl_Image) -> *mut Fl_GIF_Image;
}
extern "C" {
    pub fn Fl_GIF_Image_new(filename: *const ::std::os::raw::c_char) -> *mut Fl_GIF_Image;
}
extern "C" {
    pub fn Fl_GIF_Image_from(
        data: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_ulong,
    ) -> *mut Fl_GIF_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Anim_GIF_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_draw(
        arg1: *mut Fl_Anim_GIF_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_draw_ext(
        arg1: *mut Fl_Anim_GIF_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        cx: ::std::os::raw::c_int,
        cy: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_width(arg1: *mut Fl_Anim_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_height(arg1: *mut Fl_Anim_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_delete(arg1: *mut Fl_Anim_GIF_Image);
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_count(self_: *mut Fl_Anim_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_data(
        self_: *mut Fl_Anim_GIF_Image,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_copy(self_: *mut Fl_Anim_GIF_Image) -> *mut Fl_Anim_GIF_Image;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_copy_sized(
        self_: *mut Fl_Anim_GIF_Image,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    ) -> *mut Fl_Anim_GIF_Image;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_scale(
        self_: *mut Fl_Anim_GIF_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_fail(self_: *mut Fl_Anim_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_data_w(self_: *const Fl_Anim_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_data_h(self_: *const Fl_Anim_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_d(self_: *const Fl_Anim_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_ld(self_: *const Fl_Anim_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_inactive(self_: *mut Fl_Anim_GIF_Image);
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_from_dyn_ptr(other: *mut Fl_Image) -> *mut Fl_Anim_GIF_Image;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_new(
        filename: *const ::std::os::raw::c_char,
        canvas: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_ushort,
    ) -> *mut Fl_Anim_GIF_Image;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_from(
        imagename: *const ::std::os::raw::c_char,
        data: *const ::std::os::raw::c_uchar,
        length: ::std::os::raw::c_ulong,
        canvas: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_ushort,
    ) -> *mut Fl_Anim_GIF_Image;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_delay(
        self_: *const Fl_Anim_GIF_Image,
        frame_: ::std::os::raw::c_int,
    ) -> f64;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_set_delay(
        self_: *mut Fl_Anim_GIF_Image,
        frame: ::std::os::raw::c_int,
        delay: f64,
    );
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_is_animated(self_: *const Fl_Anim_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_set_speed(self_: *mut Fl_Anim_GIF_Image, speed: f64);
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_speed(self_: *mut Fl_Anim_GIF_Image) -> f64;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_start(self_: *mut Fl_Anim_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_stop(self_: *mut Fl_Anim_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_next(self_: *mut Fl_Anim_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Anim_GIF_Image_playing(self_: *const Fl_Anim_GIF_Image) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Pixmap {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Pixmap_draw(
        arg1: *mut Fl_Pixmap,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Pixmap_draw_ext(
        arg1: *mut Fl_Pixmap,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        cx: ::std::os::raw::c_int,
        cy: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Pixmap_width(arg1: *mut Fl_Pixmap) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pixmap_height(arg1: *mut Fl_Pixmap) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pixmap_delete(arg1: *mut Fl_Pixmap);
}
extern "C" {
    pub fn Fl_Pixmap_count(self_: *mut Fl_Pixmap) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pixmap_data(self_: *mut Fl_Pixmap) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Pixmap_copy(self_: *mut Fl_Pixmap) -> *mut Fl_Pixmap;
}
extern "C" {
    pub fn Fl_Pixmap_copy_sized(
        self_: *mut Fl_Pixmap,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    ) -> *mut Fl_Pixmap;
}
extern "C" {
    pub fn Fl_Pixmap_scale(
        self_: *mut Fl_Pixmap,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Pixmap_fail(self_: *mut Fl_Pixmap) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pixmap_data_w(self_: *const Fl_Pixmap) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pixmap_data_h(self_: *const Fl_Pixmap) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pixmap_d(self_: *const Fl_Pixmap) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pixmap_ld(self_: *const Fl_Pixmap) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Pixmap_inactive(self_: *mut Fl_Pixmap);
}
extern "C" {
    pub fn Fl_Pixmap_from_dyn_ptr(other: *mut Fl_Image) -> *mut Fl_Pixmap;
}
extern "C" {
    pub fn Fl_Pixmap_new(D: *const *const ::std::os::raw::c_char) -> *mut Fl_Pixmap;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_XPM_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_XPM_Image_draw(
        arg1: *mut Fl_XPM_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_XPM_Image_draw_ext(
        arg1: *mut Fl_XPM_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        cx: ::std::os::raw::c_int,
        cy: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_XPM_Image_width(arg1: *mut Fl_XPM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_XPM_Image_height(arg1: *mut Fl_XPM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_XPM_Image_delete(arg1: *mut Fl_XPM_Image);
}
extern "C" {
    pub fn Fl_XPM_Image_count(self_: *mut Fl_XPM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_XPM_Image_data(self_: *mut Fl_XPM_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_XPM_Image_copy(self_: *mut Fl_XPM_Image) -> *mut Fl_XPM_Image;
}
extern "C" {
    pub fn Fl_XPM_Image_copy_sized(
        self_: *mut Fl_XPM_Image,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    ) -> *mut Fl_XPM_Image;
}
extern "C" {
    pub fn Fl_XPM_Image_scale(
        self_: *mut Fl_XPM_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_XPM_Image_fail(self_: *mut Fl_XPM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_XPM_Image_data_w(self_: *const Fl_XPM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_XPM_Image_data_h(self_: *const Fl_XPM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_XPM_Image_d(self_: *const Fl_XPM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_XPM_Image_ld(self_: *const Fl_XPM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_XPM_Image_inactive(self_: *mut Fl_XPM_Image);
}
extern "C" {
    pub fn Fl_XPM_Image_from_dyn_ptr(other: *mut Fl_Image) -> *mut Fl_XPM_Image;
}
extern "C" {
    pub fn Fl_XPM_Image_new(filename: *const ::std::os::raw::c_char) -> *mut Fl_XPM_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_XBM_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_XBM_Image_draw(
        arg1: *mut Fl_XBM_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_XBM_Image_draw_ext(
        arg1: *mut Fl_XBM_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        cx: ::std::os::raw::c_int,
        cy: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_XBM_Image_width(arg1: *mut Fl_XBM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_XBM_Image_height(arg1: *mut Fl_XBM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_XBM_Image_delete(arg1: *mut Fl_XBM_Image);
}
extern "C" {
    pub fn Fl_XBM_Image_count(self_: *mut Fl_XBM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_XBM_Image_data(self_: *mut Fl_XBM_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_XBM_Image_copy(self_: *mut Fl_XBM_Image) -> *mut Fl_XBM_Image;
}
extern "C" {
    pub fn Fl_XBM_Image_copy_sized(
        self_: *mut Fl_XBM_Image,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    ) -> *mut Fl_XBM_Image;
}
extern "C" {
    pub fn Fl_XBM_Image_scale(
        self_: *mut Fl_XBM_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_XBM_Image_fail(self_: *mut Fl_XBM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_XBM_Image_data_w(self_: *const Fl_XBM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_XBM_Image_data_h(self_: *const Fl_XBM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_XBM_Image_d(self_: *const Fl_XBM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_XBM_Image_ld(self_: *const Fl_XBM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_XBM_Image_inactive(self_: *mut Fl_XBM_Image);
}
extern "C" {
    pub fn Fl_XBM_Image_from_dyn_ptr(other: *mut Fl_Image) -> *mut Fl_XBM_Image;
}
extern "C" {
    pub fn Fl_XBM_Image_new(filename: *const ::std::os::raw::c_char) -> *mut Fl_XBM_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_PNM_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_PNM_Image_draw(
        arg1: *mut Fl_PNM_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_PNM_Image_draw_ext(
        arg1: *mut Fl_PNM_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        cx: ::std::os::raw::c_int,
        cy: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_PNM_Image_width(arg1: *mut Fl_PNM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNM_Image_height(arg1: *mut Fl_PNM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNM_Image_delete(arg1: *mut Fl_PNM_Image);
}
extern "C" {
    pub fn Fl_PNM_Image_count(self_: *mut Fl_PNM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNM_Image_data(self_: *mut Fl_PNM_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_PNM_Image_copy(self_: *mut Fl_PNM_Image) -> *mut Fl_PNM_Image;
}
extern "C" {
    pub fn Fl_PNM_Image_copy_sized(
        self_: *mut Fl_PNM_Image,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    ) -> *mut Fl_PNM_Image;
}
extern "C" {
    pub fn Fl_PNM_Image_scale(
        self_: *mut Fl_PNM_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_PNM_Image_fail(self_: *mut Fl_PNM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNM_Image_data_w(self_: *const Fl_PNM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNM_Image_data_h(self_: *const Fl_PNM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNM_Image_d(self_: *const Fl_PNM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNM_Image_ld(self_: *const Fl_PNM_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNM_Image_inactive(self_: *mut Fl_PNM_Image);
}
extern "C" {
    pub fn Fl_PNM_Image_from_dyn_ptr(other: *mut Fl_Image) -> *mut Fl_PNM_Image;
}
extern "C" {
    pub fn Fl_PNM_Image_new(filename: *const ::std::os::raw::c_char) -> *mut Fl_PNM_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Tiled_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Tiled_Image_draw(
        arg1: *mut Fl_Tiled_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tiled_Image_draw_ext(
        arg1: *mut Fl_Tiled_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        cx: ::std::os::raw::c_int,
        cy: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tiled_Image_width(arg1: *mut Fl_Tiled_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tiled_Image_height(arg1: *mut Fl_Tiled_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tiled_Image_delete(arg1: *mut Fl_Tiled_Image);
}
extern "C" {
    pub fn Fl_Tiled_Image_count(self_: *mut Fl_Tiled_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tiled_Image_data(self_: *mut Fl_Tiled_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Tiled_Image_copy(self_: *mut Fl_Tiled_Image) -> *mut Fl_Tiled_Image;
}
extern "C" {
    pub fn Fl_Tiled_Image_copy_sized(
        self_: *mut Fl_Tiled_Image,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    ) -> *mut Fl_Tiled_Image;
}
extern "C" {
    pub fn Fl_Tiled_Image_scale(
        self_: *mut Fl_Tiled_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Tiled_Image_fail(self_: *mut Fl_Tiled_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tiled_Image_data_w(self_: *const Fl_Tiled_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tiled_Image_data_h(self_: *const Fl_Tiled_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tiled_Image_d(self_: *const Fl_Tiled_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tiled_Image_ld(self_: *const Fl_Tiled_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Tiled_Image_inactive(self_: *mut Fl_Tiled_Image);
}
extern "C" {
    pub fn Fl_Tiled_Image_from_dyn_ptr(other: *mut Fl_Image) -> *mut Fl_Tiled_Image;
}
extern "C" {
    pub fn Fl_Tiled_Image_new(
        i: *mut Fl_Image,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    ) -> *mut Fl_Tiled_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_RGB_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_RGB_Image_draw(
        arg1: *mut Fl_RGB_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_RGB_Image_draw_ext(
        arg1: *mut Fl_RGB_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        cx: ::std::os::raw::c_int,
        cy: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_RGB_Image_width(arg1: *mut Fl_RGB_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_height(arg1: *mut Fl_RGB_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_delete(arg1: *mut Fl_RGB_Image);
}
extern "C" {
    pub fn Fl_RGB_Image_count(self_: *mut Fl_RGB_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_data(self_: *mut Fl_RGB_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_RGB_Image_copy(self_: *mut Fl_RGB_Image) -> *mut Fl_RGB_Image;
}
extern "C" {
    pub fn Fl_RGB_Image_copy_sized(
        self_: *mut Fl_RGB_Image,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    ) -> *mut Fl_RGB_Image;
}
extern "C" {
    pub fn Fl_RGB_Image_scale(
        self_: *mut Fl_RGB_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_RGB_Image_fail(self_: *mut Fl_RGB_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_data_w(self_: *const Fl_RGB_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_data_h(self_: *const Fl_RGB_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_d(self_: *const Fl_RGB_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_ld(self_: *const Fl_RGB_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_inactive(self_: *mut Fl_RGB_Image);
}
extern "C" {
    pub fn Fl_RGB_Image_from_dyn_ptr(other: *mut Fl_Image) -> *mut Fl_RGB_Image;
}
extern "C" {
    pub fn Fl_RGB_Image_new(
        bits: *const ::std::os::raw::c_uchar,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        depth: ::std::os::raw::c_int,
        ld: ::std::os::raw::c_int,
    ) -> *mut Fl_RGB_Image;
}
extern "C" {
    pub fn Fl_RGB_Image_from_data(
        bits: *const ::std::os::raw::c_uchar,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        depth: ::std::os::raw::c_int,
        ld: ::std::os::raw::c_int,
    ) -> *mut Fl_RGB_Image;
}
extern "C" {
    pub fn Fl_RGB_Image_from_pixmap(image: *const Fl_Pixmap) -> *mut Fl_RGB_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Shared_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Shared_Image_draw(
        arg1: *mut Fl_Shared_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Shared_Image_draw_ext(
        arg1: *mut Fl_Shared_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        cx: ::std::os::raw::c_int,
        cy: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Shared_Image_width(arg1: *mut Fl_Shared_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Shared_Image_height(arg1: *mut Fl_Shared_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Shared_Image_delete(arg1: *mut Fl_Shared_Image);
}
extern "C" {
    pub fn Fl_Shared_Image_count(self_: *mut Fl_Shared_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Shared_Image_data(
        self_: *mut Fl_Shared_Image,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Shared_Image_copy(self_: *mut Fl_Shared_Image) -> *mut Fl_Shared_Image;
}
extern "C" {
    pub fn Fl_Shared_Image_copy_sized(
        self_: *mut Fl_Shared_Image,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    ) -> *mut Fl_Shared_Image;
}
extern "C" {
    pub fn Fl_Shared_Image_scale(
        self_: *mut Fl_Shared_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Shared_Image_fail(self_: *mut Fl_Shared_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Shared_Image_data_w(self_: *const Fl_Shared_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Shared_Image_data_h(self_: *const Fl_Shared_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Shared_Image_d(self_: *const Fl_Shared_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Shared_Image_ld(self_: *const Fl_Shared_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Shared_Image_inactive(self_: *mut Fl_Shared_Image);
}
extern "C" {
    pub fn Fl_Shared_Image_from_dyn_ptr(other: *mut Fl_Image) -> *mut Fl_Shared_Image;
}
extern "C" {
    pub fn Fl_Shared_Image_get(
        name: *const ::std::os::raw::c_char,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    ) -> *mut Fl_Shared_Image;
}
extern "C" {
    pub fn Fl_Shared_Image_from_rgb(
        rgb: *mut Fl_RGB_Image,
        own_it: ::std::os::raw::c_int,
    ) -> *mut Fl_Shared_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_ICO_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_ICO_Image_draw(
        arg1: *mut Fl_ICO_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_ICO_Image_draw_ext(
        arg1: *mut Fl_ICO_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        cx: ::std::os::raw::c_int,
        cy: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_ICO_Image_width(arg1: *mut Fl_ICO_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_ICO_Image_height(arg1: *mut Fl_ICO_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_ICO_Image_delete(arg1: *mut Fl_ICO_Image);
}
extern "C" {
    pub fn Fl_ICO_Image_count(self_: *mut Fl_ICO_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_ICO_Image_data(self_: *mut Fl_ICO_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_ICO_Image_copy(self_: *mut Fl_ICO_Image) -> *mut Fl_ICO_Image;
}
extern "C" {
    pub fn Fl_ICO_Image_copy_sized(
        self_: *mut Fl_ICO_Image,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    ) -> *mut Fl_ICO_Image;
}
extern "C" {
    pub fn Fl_ICO_Image_scale(
        self_: *mut Fl_ICO_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_ICO_Image_fail(self_: *mut Fl_ICO_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_ICO_Image_data_w(self_: *const Fl_ICO_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_ICO_Image_data_h(self_: *const Fl_ICO_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_ICO_Image_d(self_: *const Fl_ICO_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_ICO_Image_ld(self_: *const Fl_ICO_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_ICO_Image_inactive(self_: *mut Fl_ICO_Image);
}
extern "C" {
    pub fn Fl_ICO_Image_from_dyn_ptr(other: *mut Fl_Image) -> *mut Fl_ICO_Image;
}
extern "C" {
    pub fn Fl_ICO_Image_new(
        filename: *const ::std::os::raw::c_char,
        id: ::std::os::raw::c_int,
    ) -> *mut Fl_ICO_Image;
}
extern "C" {
    pub fn Fl_ICO_Image_from_data(
        bits: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_ulong,
        id: ::std::os::raw::c_int,
    ) -> *mut Fl_ICO_Image;
}
extern "C" {
    pub fn Fl_ICO_Image_icondirentry(
        self_: *const Fl_ICO_Image,
        size: *mut ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_register_images();
}
