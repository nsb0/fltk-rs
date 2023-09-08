use crate::enums::Align;
use crate::enums::{Color, FrameType};
use crate::prelude::*;
use crate::utils::FlString;
use crate::widget::Widget;
use fltk_sys::group::*;
use std::{
    ffi::{CStr, CString},
    mem,
    sync::atomic::{AtomicBool, Ordering},
};

static DEBUG: AtomicBool = AtomicBool::new(false);

/// Creates a group widget
#[derive(Debug)]
pub struct Group {
    inner: *mut Fl_Group,
    tracker: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Group, Fl_Group);
crate::macros::widget::impl_widget_base!(Group, Fl_Group);
crate::macros::widget::impl_widget_default!(Group);
crate::macros::group::impl_group_ext!(Group, Fl_Group);

impl Group {
    #[deprecated(since = "1.2.18", note = "please use `try_current` instead")]
    /// Get the current group
    pub fn current() -> Group {
        unsafe {
            let ptr = Fl_Group_current();
            assert!(!ptr.is_null());
            Group::from_widget_ptr(ptr as _)
        }
    }

    /// Tries to get the current group
    pub fn try_current() -> Option<Group> {
        unsafe {
            let ptr = Fl_Group_current();
            if ptr.is_null() {
                None
            } else {
                Some(Group::from_widget_ptr(ptr as _))
            }
        }
    }

    /// Sets the current GroupExt widget which will take children
    pub fn set_current(grp: Option<&impl GroupExt>) {
        unsafe {
            if let Some(grp) = grp {
                Fl_Group_set_current(grp.as_widget_ptr() as _)
            } else {
                Fl_Group_set_current(std::ptr::null_mut())
            }
        }
    }
}

/// Creates a widget pack
#[derive(Debug)]
pub struct Pack {
    inner: *mut Fl_Pack,
    tracker: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Pack, Fl_Pack);
crate::macros::widget::impl_widget_base!(Pack, Fl_Pack);
crate::macros::widget::impl_widget_default!(Pack);
crate::macros::group::impl_group_ext!(Pack, Fl_Pack);

/// Defines pack types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PackType {
    /// Vertical layout pack
    Vertical = 0,
    /// Horizontal layout pack
    Horizontal = 1,
}

crate::macros::widget::impl_widget_type!(PackType);

impl Pack {
    /// Get the spacing of the pack
    pub fn spacing(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Pack_spacing(self.inner) }
    }

    /// Set the spacing of the pack
    pub fn set_spacing(&mut self, spacing: i32) {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Pack_set_spacing(self.inner, spacing);
        }
    }

    /// Layout the children of the pack automatically.
    /// Must be called on existing children
    pub fn auto_layout(&mut self) {
        let children = self.children();
        if children == 0 {
            return;
        }
        let spacing = self.spacing() * (children - 1);
        let t = self.get_type::<PackType>();
        let w = (self.width() - spacing) / children;
        let h = (self.height() - spacing) / children;

        for i in 0..children {
            let mut c = self.child(i).unwrap();
            let c_w = c.width();
            let c_h = c.height();
            if t == PackType::Vertical {
                c.set_size(c_w, h);
            } else {
                c.set_size(w, c_h);
            }
        }
    }
}

/// Creates a scroll group
#[derive(Debug)]
pub struct Scroll {
    inner: *mut Fl_Scroll,
    tracker: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Scroll, Fl_Scroll);
crate::macros::widget::impl_widget_base!(Scroll, Fl_Scroll);
crate::macros::widget::impl_widget_default!(Scroll);
crate::macros::group::impl_group_ext!(Scroll, Fl_Scroll);

/// Defines Scroll types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ScrollType {
    /// Never show bars
    None = 0,
    /// Show vertical bar
    Horizontal = 1,
    /// Show vertical bar
    Vertical = 2,
    /// Show both horizontal and vertical bars
    Both = 3,
    /// Always show bars
    AlwaysOn = 4,
    /// Show horizontal bar always
    HorizontalAlways = 5,
    /// Show vertical bar always
    VerticalAlways = 6,
    /// Always show both horizontal and vertical bars
    BothAlways = 7,
}

crate::macros::widget::impl_widget_type!(ScrollType);

impl Scroll {
    /// Returns the vertical scrollbar
    pub fn scrollbar(&self) -> crate::valuator::Scrollbar {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Scroll_scrollbar(self.inner);
            assert!(!ptr.is_null());
            crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget)
        }
    }

    /// Returns the horizontal scrollbar
    pub fn hscrollbar(&self) -> crate::valuator::Scrollbar {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Scroll_hscrollbar(self.inner);
            assert!(!ptr.is_null());
            crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget)
        }
    }

    /// Returns the x position
    pub fn xposition(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Scroll_xposition(self.inner) }
    }

    /// Returns the y position
    pub fn yposition(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Scroll_yposition(self.inner) }
    }

    /// Scrolls to `x` and `y`
    pub fn scroll_to(&mut self, x: i32, y: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Scroll_scroll_to(self.inner, x, y) }
    }

    /// Gets the scrollbar size
    pub fn scrollbar_size(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Scroll_scrollbar_size(self.inner) }
    }

    /// Sets the scrollbar size
    pub fn set_scrollbar_size(&mut self, new_size: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Scroll_set_scrollbar_size(self.inner, new_size) }
    }
}

/// Defines how Tabs handle overflow
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TabsOverflow {
    /// Compress tabs
    Compress = 0,
    /// Clip tabs
    Clip,
    /// Create a pulldown
    Pulldown,
    /// Drag tabs
    Drag,
}

/// Creates a tab which can contain widgets
#[derive(Debug)]
pub struct Tabs {
    inner: *mut Fl_Tabs,
    tracker: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Tabs, Fl_Tabs);
crate::macros::widget::impl_widget_base!(Tabs, Fl_Tabs);
crate::macros::widget::impl_widget_default!(Tabs);
crate::macros::group::impl_group_ext!(Tabs, Fl_Tabs);

impl Tabs {
    /// Gets the currently visible group
    pub fn value(&self) -> Option<impl GroupExt> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Tabs_value(self.inner);
            if ptr.is_null() {
                None
            } else {
                Some(Group::from_widget_ptr(
                    ptr as *mut fltk_sys::widget::Fl_Widget,
                ))
            }
        }
    }

    /// Sets the currently visible group
    /// # Errors
    /// Errors when the value can't be set for the group widget
    pub fn set_value<Grp: GroupExt>(&mut self, w: &Grp) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        unsafe {
            match Fl_Tabs_set_value(
                self.inner,
                w.as_widget_ptr() as *mut fltk_sys::group::Fl_Widget,
            ) {
                0 => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                _ => Ok(()),
            }
        }
    }

    /// Returns the tab group for the tab the user has currently down-clicked
    pub fn push(&self) -> Option<impl GroupExt> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Tabs_push(self.inner);
            if ptr.is_null() {
                None
            } else {
                Some(Group::from_widget_ptr(
                    ptr as *mut fltk_sys::widget::Fl_Widget,
                ))
            }
        }
    }

    /// This is called by the tab widget's handle() method to set the tab group widget the user last pushed
    /// # Errors
    /// Errors if `set_push` can't be set for the group widget
    pub fn set_push<Grp: GroupExt>(&mut self, w: &Grp) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        unsafe {
            match Fl_Tabs_set_push(
                self.inner,
                w.as_widget_ptr() as *mut fltk_sys::group::Fl_Widget,
            ) {
                0 => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                _ => Ok(()),
            }
        }
    }

    /// Returns the position and size available to be used by its children
    pub fn client_area(&self) -> (i32, i32, i32, i32) {
        assert!(!self.was_deleted());
        unsafe {
            let mut i1 = 0;
            let mut i2 = 0;
            let mut i3 = 0;
            let mut i4 = 0;
            Fl_Tabs_client_area(self.inner, &mut i1, &mut i2, &mut i3, &mut i4);
            (i1, i2, i3, i4)
        }
    }

    /// Sets the tab label alignment
    pub fn set_tab_align(&mut self, a: Align) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tabs_set_tab_align(self.inner, a.bits()) }
    }

    /// Gets the tab label alignment.
    pub fn tab_align(&self) -> Align {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tabs_tab_align(self.inner)) }
    }

    /// Auto layout a tabs widget
    pub fn auto_layout(&mut self) {
        for c in self.clone().into_iter() {
            if let Some(mut c) = c.as_group() {
                c.resize(self.x(), self.y() + 30, self.w(), self.h() - 30);
            }
        }
        self.resize_callback(|t, x, y, w, h| {
            for c in t.clone().into_iter() {
                if let Some(mut c) = c.as_group() {
                    c.resize(x, y + 30, w, h - 30);
                }
            }
        });
    }

    /// Sets how the Tabs handles overflow
    pub fn handle_overflow(&mut self, ov: TabsOverflow) {
        unsafe { Fl_Tabs_handle_overflow(self.inner, ov as i32) }
    }
}

/// Creates a tile which can contain widgets. For the tiling to work correctly, the children of a Tile must cover the entire area of the widget, but not overlap. This means that all children must touch each other at their edges, and no gaps can be left inside the Tile.
/// More info can be found [here](https://www.fltk.org/doc-1.4/classFl__Tile.html#details)
#[derive(Debug)]
pub struct Tile {
    inner: *mut Fl_Tile,
    tracker: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Tile, Fl_Tile);
crate::macros::widget::impl_widget_base!(Tile, Fl_Tile);
crate::macros::widget::impl_widget_default!(Tile);
crate::macros::group::impl_group_ext!(Tile, Fl_Tile);

/// Creates a wizard widget
#[derive(Debug)]
pub struct Wizard {
    inner: *mut Fl_Wizard,
    tracker: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Wizard, Fl_Wizard);
crate::macros::widget::impl_widget_base!(Wizard, Fl_Wizard);
crate::macros::widget::impl_widget_default!(Wizard);
crate::macros::group::impl_group_ext!(Wizard, Fl_Wizard);

impl Wizard {
    /// Gets the next view of the wizard
    pub fn next(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Wizard_next(self.inner) }
    }

    /// Gets the previous view of the wizard
    pub fn prev(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Wizard_prev(self.inner) }
    }

    #[deprecated(since = "1.2.18", note = "please use `try_current_widget` instead")]
    /// Gets the underlying widget of the current view
    pub fn current_widget(&self) -> Widget {
        unsafe {
            assert!(!self.was_deleted());
            let ptr = Fl_Wizard_value(self.inner) as *mut fltk_sys::widget::Fl_Widget;
            assert!(!ptr.is_null());
            Widget::from_widget_ptr(ptr)
        }
    }

    /// Gets the underlying widget of the current view
    pub fn try_current_widget(&self) -> Option<impl WidgetExt> {
        unsafe {
            assert!(!self.was_deleted());
            let ptr = Fl_Wizard_value(self.inner) as *mut fltk_sys::widget::Fl_Widget;
            if ptr.is_null() {
                None
            } else {
                Some(Widget::from_widget_ptr(ptr))
            }
        }
    }

    /// Sets the underlying widget of the current view
    pub fn set_current_widget<W: WidgetExt>(&mut self, w: &W) {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Wizard_set_value(
                self.inner,
                w.as_widget_ptr() as *mut fltk_sys::group::Fl_Widget,
            )
        }
    }
}

/// Creates a color chooser widget
#[derive(Debug)]
pub struct ColorChooser {
    inner: *mut Fl_Color_Chooser,
    tracker: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(ColorChooser, Fl_Color_Chooser);
crate::macros::widget::impl_widget_base!(ColorChooser, Fl_Color_Chooser);
crate::macros::widget::impl_widget_default!(ColorChooser);
crate::macros::group::impl_group_ext!(ColorChooser, Fl_Color_Chooser);

impl ColorChooser {
    /// Return the rgb color
    pub fn rgb_color(&self) -> (u8, u8, u8) {
        unsafe {
            assert!(!self.was_deleted());
            let r = (Fl_Color_Chooser_r(self.inner) * 255.0) as u8;
            let g = (Fl_Color_Chooser_g(self.inner) * 255.0) as u8;
            let b = (Fl_Color_Chooser_b(self.inner) * 255.0) as u8;
            (r, g, b)
        }
    }

    /// Return the hex color
    pub fn hex_color(&self) -> u32 {
        assert!(!self.was_deleted());
        let (r, g, b) = self.rgb_color();
        crate::utils::rgb2hex(r, g, b)
    }

    /// Set the base color of the ColorChooser. Returns an error on failure to change the color (wrong input)
    pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        unsafe {
            let ret = Fl_Color_Chooser_set_rgb(
                self.inner,
                r as f64 / 255.0,
                g as f64 / 255.0,
                b as f64 / 255.0,
            );
            if ret == 1 {
                Ok(())
            } else {
                Err(FltkError::Internal(FltkErrorKind::FailedOperation))
            }
        }
    }

    /// Set the base color of the ColorChooser. Returns an error on failure to change the color (wrong input)
    pub fn set_tuple_rgb(&mut self, (r, g, b): (u8, u8, u8)) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        unsafe {
            let ret = Fl_Color_Chooser_set_rgb(
                self.inner,
                r as f64 / 255.0,
                g as f64 / 255.0,
                b as f64 / 255.0,
            );
            if ret == 1 {
                Ok(())
            } else {
                Err(FltkError::Internal(FltkErrorKind::FailedOperation))
            }
        }
    }

    /// Set the mode of the ColorChooser - color modes are rgb(0), byte(1), hex(2), or hsv(3) 
    pub fn set_mode(&mut self, mode: i32) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Color_Chooser_set_mode(
                self.inner, mode
            );
        }
    }
}

crate::macros::widget::impl_widget_type!(FlexType);

/// Defines Flex types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlexType {
    /// row direction
    Column = 0,
    /// column direction
    Row,
}

/**
    a Flexbox widget
    # Example
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn main() {
        let a = app::App::default();
        let mut win = window::Window::default().with_size(400, 300);
        let mut col = group::Flex::default().size_of_parent();
        col.set_type(group::FlexType::Column);
        let expanding = button::Button::default().with_label("Expanding");
        let mut normal = button::Button::default().with_label("Normal");
        col.set_size(&mut normal, 30);
        col.end();
        win.end();
        win.show();
        a.run().unwrap();
    }
    ```
*/
#[derive(Debug)]
pub struct Flex {
    inner: *mut Fl_Flex,
    tracker: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Flex, Fl_Flex);
crate::macros::widget::impl_widget_base!(Flex, Fl_Flex);
crate::macros::widget::impl_widget_default!(Flex);
crate::macros::group::impl_group_ext!(Flex, Fl_Flex);

impl Flex {
    /// Create a new Flex widget.
    /// This code is here for backward compatibility with initial Fl_Flex code, which defaulted to Row instead of Column.
    /// The behavior will be changed in fltk-rs version 2.
    fn new<T: Into<Option<&'static str>>>(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        title: T,
    ) -> Self {
        let mut f = <Flex as WidgetBase>::new(x, y, width, height, title).row();
        f.set_pad(5);
        f.debug_();
        f
    }
    /// Add a widget to the Flex box
    pub fn add<W: WidgetExt>(&mut self, widget: &W) {
        <Self as GroupExt>::add(self, widget);
        self.recalc();
    }

    /// Add a widget to the Flex box
    pub fn insert<W: WidgetExt>(&mut self, widget: &W, idx: i32) {
        <Self as GroupExt>::insert(self, widget, idx);
        self.recalc();
    }

    /// Set the size of the widget, same as `fixed` (before it was changed in FLTK 1.4)
    #[deprecated(since = "1.4.8", note = "please use `fixed` instead")]
    pub fn set_size<W: WidgetExt>(&mut self, w: &W, size: i32) {
        unsafe { Fl_Flex_set_size(self.inner, w.as_widget_ptr() as _, size) }
    }

    /// Set the size of the widget, same as `set_size`, but more inline with the new FLTK Fl_Flex api
    pub fn fixed<W: WidgetExt>(&mut self, w: &W, size: i32) {
        unsafe { Fl_Flex_set_size(self.inner, w.as_widget_ptr() as _, size) }
    }

    /// Debug the flex layout
    pub fn debug(flag: bool) {
        DEBUG.store(flag, Ordering::Release);
    }

    fn debug_(&mut self) {
        if DEBUG.load(Ordering::Relaxed) {
            self.set_frame(FrameType::BorderBox);
            if self.get_type::<FlexType>() == FlexType::Row {
                self.set_color(Color::from_rgb(200, 0, 0));
            } else {
                self.set_color(Color::from_rgb(0, 0, 200));
            }
        }
    }

    /// Set the type to be a column
    pub fn column(mut self) -> Self {
        self.set_type(FlexType::Column);
        self.debug_();
        self
    }

    /// Set the type to a row
    pub fn row(mut self) -> Self {
        self.set_type(FlexType::Row);
        self.debug_();
        self
    }

    /// Recalculate children's coords and sizes
    pub fn recalc(&self) {
        let mut s = self.clone();
        s.resize(self.x(), self.y(), self.w(), self.h());
        s.redraw();
    }

    /// Recalculate children's coords and sizes
    pub fn layout(&self) {
        self.recalc();
    }

    /// Set the margin
    pub fn set_margin(&mut self, m: i32) {
        unsafe { Fl_Flex_set_margin(self.inner, m) }
    }

    /// Get the margin
    pub fn margin(&self) -> i32 {
        unsafe { Fl_Flex_margin(self.inner) }
    }

    /// Set the padding
    pub fn set_pad(&mut self, p: i32) {
        unsafe { Fl_Flex_set_pad(self.inner, p) }
    }

    /// Get the padding
    pub fn pad(&self) -> i32 {
        unsafe { Fl_Flex_pad(self.inner) }
    }

    /// Set the padding
    pub fn set_spacing(&mut self, p: i32) {
        unsafe { Fl_Flex_set_pad(self.inner, p) }
    }

    /// Get the padding
    pub fn spacing(&self) -> i32 {
        unsafe { Fl_Flex_pad(self.inner) }
    }

    /// Set the margins
    pub fn set_margins(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe { Fl_Flex_set_margins(self.inner, left, top, right, bottom) }
    }

    /// Get the margins -> returns (left, top, right, bottom)
    pub fn margins(&self) -> (i32, i32, i32, i32) {
        let mut left = 0;
        let mut top = 0;
        let mut right = 0;
        let mut bottom = 0;
        unsafe {
            Fl_Flex_margins(self.inner, &mut left, &mut top, &mut right, &mut bottom);
        }
        (left, top, right, bottom)
    }
}

/**
    Defines a Vertical Grid (custom widget).
    Requires setting the params manually using the `set_params` method, which takes the rows, columns and spacing.
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn main() {
        let app = app::App::default();
        let mut win = window::Window::default().with_size(400, 300);
        let mut grid = group::VGrid::new(0, 0, 400, 300, "");
        grid.set_params(3, 3, 5);
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        grid.end();
        win.end();
        win.show();
        app.run().unwrap();
    }
    ```
*/
#[derive(Debug, Clone)]
pub struct VGrid {
    vpack: Pack,
    rows: i32,
    cols: i32,
    current: i32,
}

impl Default for VGrid {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, None)
    }
}

impl VGrid {
    /// Constructs a widget with the size of its parent
    pub fn default_fill() -> Self {
        Self::default().size_of_parent()
    }

    /// Creates a new vertical grid
    pub fn new<T: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: T) -> VGrid {
        let vpack = Pack::new(x, y, w, h, label);
        VGrid {
            vpack,
            rows: 1,
            cols: 1,
            current: 0,
        }
    }

    /// Sets the params for the grid
    pub fn set_params(&mut self, rows: i32, cols: i32, spacing: i32) {
        self.vpack.set_spacing(spacing);
        let rows = if rows < 1 { 1 } else { rows };
        let cols = if cols < 1 { 1 } else { cols };
        self.rows = rows;
        self.cols = cols;
        for _ in 0..rows {
            let mut p = Pack::new(0, 0, self.vpack.width(), 0, "");
            p.set_type(PackType::Horizontal);
            p.set_spacing(spacing);
            p.end();
            self.vpack.add(&p);
        }
    }

    /// Adds widgets to the grid
    pub fn add<W: WidgetExt>(&mut self, w: &W) {
        debug_assert!(self.current < self.rows * self.cols);
        let rem = (self.current - 1) / self.cols;
        if rem < self.rows {
            let hpack = self.vpack.child(rem).unwrap();
            let mut hpack = unsafe { Pack::from_widget_ptr(hpack.as_widget_ptr()) };
            hpack.end();
            hpack.add(w);
            hpack.auto_layout();
            self.vpack.auto_layout();
            self.current += 1;
        }
    }

    /// End the grid
    pub fn end(&mut self) {
        use std::collections::VecDeque;
        let children = self.vpack.children();
        self.current = children - self.rows;
        debug_assert!(self.current <= self.rows * self.cols);
        let mut v = VecDeque::new();
        for i in self.rows..children {
            let c = self.vpack.child(i).unwrap();
            v.push_back(c);
        }
        for i in 0..self.rows {
            let hpack = self.vpack.child(i).unwrap();
            let mut hpack = unsafe { Pack::from_widget_ptr(hpack.as_widget_ptr()) };
            hpack.end();
            for _j in 0..self.cols {
                if let Some(w) = v.pop_front() {
                    self.vpack.remove(&w);
                    hpack.add(&w);
                }
                hpack.auto_layout();
            }
        }
        self.vpack.auto_layout();
    }
}

crate::widget_extends!(VGrid, Pack, vpack);

/**
    Defines a Horizontal Grid (custom widget).
    Requires setting the params manually using the `set_params` method, which takes the rows, columns and spacing.
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn main() {
        let app = app::App::default();
        let mut win = window::Window::default().with_size(400, 300);
        let mut grid = group::HGrid::new(0, 0, 400, 300, "");
        grid.set_params(3, 3, 5);
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        button::Button::default();
        grid.end();
        win.end();
        win.show();
        app.run().unwrap();
    }
    ```
*/
#[derive(Debug, Clone)]
pub struct HGrid {
    hpack: Pack,
    rows: i32,
    cols: i32,
    current: i32,
}

impl Default for HGrid {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, None)
    }
}

impl HGrid {
    /// Constructs a widget with the size of its parent
    pub fn default_fill() -> Self {
        Self::default().size_of_parent()
    }

    /// Creates a new horizontal grid
    pub fn new<T: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: T) -> HGrid {
        let mut hpack = Pack::new(x, y, w, h, label);
        hpack.set_type(PackType::Horizontal);
        HGrid {
            hpack,
            rows: 1,
            cols: 1,
            current: 0,
        }
    }

    /// Sets the params for the grid
    pub fn set_params(&mut self, rows: i32, cols: i32, spacing: i32) {
        self.hpack.set_spacing(spacing);
        let rows = if rows < 1 { 1 } else { rows };
        let cols = if cols < 1 { 1 } else { cols };
        self.rows = rows;
        self.cols = cols;
        for _ in 0..cols {
            let mut p = Pack::new(0, 0, 0, self.hpack.height(), "");
            p.set_spacing(spacing);
            p.end();
            self.hpack.add(&p);
        }
    }

    /// Adds widgets to the grid
    pub fn add<W: WidgetExt>(&mut self, w: &W) {
        debug_assert!(self.current < self.rows * self.cols);
        let rem = (self.current - 1) / self.rows;
        if rem < self.cols {
            let vpack = self.hpack.child(rem).unwrap();
            let mut vpack = unsafe { Pack::from_widget_ptr(vpack.as_widget_ptr()) };
            vpack.end();
            vpack.add(w);
            vpack.auto_layout();
            self.hpack.auto_layout();
            self.current += 1;
        }
    }

    /// End the grid
    pub fn end(&mut self) {
        use std::collections::VecDeque;
        let children = self.hpack.children();
        self.current = children - self.cols;
        debug_assert!(self.current <= self.rows * self.cols);
        let mut v = VecDeque::new();
        for i in self.cols..children {
            let c = self.hpack.child(i).unwrap();
            v.push_back(c);
        }
        for i in 0..self.cols {
            let vpack = self.hpack.child(i).unwrap();
            let mut vpack = unsafe { Pack::from_widget_ptr(vpack.as_widget_ptr()) };
            vpack.end();
            for _j in 0..self.rows {
                if let Some(w) = v.pop_front() {
                    self.hpack.remove(&w);
                    vpack.add(&w);
                }
                vpack.auto_layout();
            }
        }
        self.hpack.auto_layout();
    }
}

crate::widget_extends!(HGrid, Pack, hpack);

/// A wrapper around a Flex column
#[derive(Debug, Clone)]
pub struct Column {
    p: Flex,
}

impl Default for Column {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, None)
    }
}

impl Column {
    /// Constructs a widget with the size of its parent
    pub fn default_fill() -> Self {
        Self::default().size_of_parent()
    }

    /// Create a new column
    pub fn new<T: Into<Option<&'static str>>>(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        label: T,
    ) -> Column {
        let mut p = Flex::new(x, y, width, height, label);
        p.set_type(FlexType::Column);
        Column { p }
    }

    /// Add a widget to the column with automatic layouting
    pub fn add<W: WidgetExt>(&mut self, w: &W) {
        self.p.add(w);
    }
}

crate::widget_extends!(Column, Flex, p);

/// A wrapper around a Flex row
#[derive(Debug, Clone)]
pub struct Row {
    p: Flex,
}

impl Default for Row {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, None)
    }
}

impl Row {
    /// Constructs a widget with the size of its parent
    pub fn default_fill() -> Self {
        Self::default().size_of_parent().center_of_parent()
    }

    /// Create a new row
    pub fn new<T: Into<Option<&'static str>>>(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        label: T,
    ) -> Row {
        let mut p = Flex::new(x, y, width, height, label);
        p.set_type(FlexType::Row);
        Row { p }
    }

    /// Add a widget to the row with automatic layouting
    pub fn add<W: WidgetExt>(&mut self, w: &W) {
        self.p.add(w);
    }
}

crate::widget_extends!(Row, Flex, p);
