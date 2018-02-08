extern crate scintilla_sys;

#[macro_use]
extern crate lazy_static;
//#[macro_use]
extern crate plygui_api;

#[cfg(target_os="windows")]
mod lib_win32;
#[macro_use]
#[cfg(target_os="windows")]
extern crate plygui_win32;
#[cfg(target_os="windows")]
extern crate winapi;
#[cfg(target_os="windows")]
pub use lib_win32::Scintilla;

//pub use scintilla_sys::{Sci_TextRange as TextRange, Sci_CharacterRange as CharacterRange, Sci_PositionCR as PositionCr};

use std::borrow::Cow;

pub const MEMBER_ID_SCINTILLA: &str = "Scintilla";

pub trait UiScintilla: plygui_api::traits::UiControl {
    fn text<'a>(&'a self) -> Cow<'a, str>;
    fn set_text(&mut self, text: &str);
    fn set_save_point(&mut self);
    fn line<'a>(&'a self, line: u32) -> Cow<'a, str>;
    fn replace_selection(&mut self, replacement: &str);
    fn set_readonly(&mut self, readonly: bool);
    fn is_readonly(&self) -> bool;
    fn text_range<'a>(&'a self, start: u32, end: u32) -> Cow<'a, str>;
    //fn allocate(&mut self, bytes: u32);
    fn add_text(&mut self, text: &str);
    fn add_styled_text(&mut self, text: &str);
    fn append_text(&mut self, text: &str);
    fn insert_text(&mut self, text: &str);
    fn change_insertion(&mut self, pos: u32, text: &str);
    fn clear_all(&mut self);
    fn delete_range(&mut self, start: u32, len: u32);
    fn clear_document_style(&mut self);
    fn char_at(&self, pos: u32) -> char;
    fn style_at(&self, pos: u32) -> u32;
    fn styled_text<'a>(&'a self, start: u32, end: u32) -> Cow<'a, str>;
    fn release_all_extended_styles(&mut self);
    //fn allocate_extended_styles(&mut self, count: u32) -> bool;
    
    //TODO we operate utf8 anyway - should we export these?
    fn target_as_utf8<'a>(&'a mut self) -> Cow<'a, str>;
    fn encoded_from_utf8(&mut self, utf8_str: &str) -> Vec<u8>;
    
    fn set_target_start(&mut self, start: u32);
    fn target_start(&self) -> u32;
    fn set_target_end(&mut self, end: u32);
    fn target_end(&self) -> u32;
    fn set_target_range(&mut self, start: u32, end: u32);
    fn target_from_selection(&mut self);
    fn target_whole_document(&mut self);
    fn set_search_flags(&mut self, flags: u32);
    fn search_flags(&self) -> u32;
    fn search_in_target(&self, text: &str) -> Option<u32>;
    fn target_text(&self, text: &str) -> u32;
    fn replace_target(&mut self, text: &str);
    fn replace_target_regexp(&mut self, text: &str);
    fn tag(&self, tag_number: u32, tag: &str) -> u32;
    
    fn find_text(&self, flags: u32, text: &str) -> u32;
    fn search_anchor(&mut self);
    fn search_next(&self, text: &str) -> u32;
    fn search_prev(&self, text: &str) -> u32;
    
    fn set_overtype(&mut self, overtype: bool);
    fn is_overtype(&self) -> bool;
    
    fn cut(&mut self);
    fn copy(&self);
    fn paste(&mut self);
    fn is_paste_allowed(&self) -> bool;
    fn copy_range(&self, start: u32, end: u32);
    fn copy_text(&self, text: &str);
    fn copy_line(&self);
    fn set_paste_convert_endings(&mut self, convert: bool);
    fn is_paste_convert_endings(&self) -> bool;
    
    fn set_status(&mut self, status: u32);
    fn status(&self) -> u32;
    
    fn undo(&mut self);
    fn is_undo_allowed(&self) -> bool;
    fn empty_undo_buffer(&mut self);
    fn redo(&mut self);
    fn is_redo_allowed(&self) -> bool;
    fn set_undo_collected(&mut self, collect: bool);
    fn is_undo_collected(&self) -> bool;
    fn begin_undo_action(&mut self);
    fn end_undo_action(&mut self);
    fn add_undo_action(&mut self, token: u32, flags: u32);
    
    fn text_len(&self) -> u32;
    fn len(&self) -> u32;
    fn line_count(&self) -> u32;
    fn lines_on_screen(&self) -> u32;
    fn is_modified(&self) -> bool;
    fn set_selection(&mut self, anchor: Option<u32>, caret: Option<u32>);
    fn go_to_position(&mut self, caret: Option<u32>);
    fn go_to_line(&mut self, line: u32);
    fn set_current_position(&mut self, caret: Option<u32>);
    fn current_position(&self) -> u32;
    fn set_anchor(&mut self, anchor: Option<u32>);
    fn anchor(&self) -> u32;
    fn set_selection_start(&mut self, start: u32);
    fn set_selection_end(&mut self, end: u32);
    fn selection_start(&self) -> u32;
    fn selection_end(&self) -> u32;
    fn clear_selection(&mut self, caret: u32);
    fn select_all(&mut self);
    fn line_from_position(&self, position: u32) -> u32;
    fn position_from_line(&self, line: u32) -> u32;
    fn line_end_position(&self, line: u32) -> u32;
    fn line_len(&self, line: u32) -> u32;
    fn column(&self, position: u32) -> u32;
    fn position_from_line_column(&self, line: u32, column: u32) -> u32;
    fn position_from_point(&self, x: u32, y: u32) -> u32;
    fn position_from_point_close(&self, x: u32, y: u32) -> u32;
    fn char_position_from_point(&self, x: u32, y: u32) -> u32;
    fn char_position_from_point_close(&self, x: u32, y: u32) -> u32;
    fn point_x_from_position(&self, position: u32) -> u32;
    fn point_y_from_position(&self, position: u32) -> u32;
    fn hide_selection(&mut self, hide: bool);
    fn selected_text<'a>(&'a self) -> Cow<'a, str>; 
    fn current_line<'a>(&'a self) -> Cow<'a, str>;
    fn is_rectangle_selection(&self) -> bool;
    fn set_selection_mode(&mut self, mode: u32);
    fn selection_mode(&self) -> u32;
    fn line_selection_start_position(&self, line: u32) -> u32;
    fn line_selection_end_position(&self, line: u32) -> u32;
    fn move_caret_inside_view(&mut self);
    fn position_before(&self, position: u32) -> u32;
    fn position_after(&self, position: u32) -> u32;
    fn position_relative(&self, position: u32, relative: u32) -> u32;
    fn character_count(&self, start: u32, end: u32) -> u32;
    fn text_width(&self, text: &str) -> u32;
    fn text_height(&self, line: u32) -> u32;
    fn choose_caret_x(&mut self);
    fn move_selected_lines_up(&mut self);
    fn move_selected_lines_down(&mut self);
    fn set_rectangular_mouse_selection(&mut self, rectangular: bool);
    fn is_rectangular_mouse_selection(&self) -> bool;
    
    fn set_multiple_selection(&mut self, enabled: bool);
    fn is_multiple_selection(&self) -> bool;
    fn set_additional_selection_typing(&mut self, enabled: bool);
    fn is_additional_selection_typing(&self) -> bool;
    fn set_multi_paste(&mut self, paste: u32);
    fn multi_paste(&self) -> u32;
    fn set_virtual_space_options(&mut self, options: u32);
    fn virtual_space_options(&self) -> u32;
    fn set_rectangular_selection_modifier(&mut self, modifier: u32);
    fn rectangular_selection_modifier(&self) -> u32;
    fn selections(&self) -> u32;
    fn is_selection_empty(&self) -> bool;
    fn clear_selections(&mut self);
    fn set_single_selection(&mut self, caret: u32, anchor: u32);
    fn push_selection(&mut self, caret: u32, anchor: u32);
    fn remove_selection(&mut self, selection: u32);
    fn set_main_selection(&mut self, selection: u32);
    fn main_selection(&self) -> u32;
    fn set_nth_selection_caret(&mut self, selection: u32, caret: u32);
    fn nth_selection_caret(&self, selection: u32) -> u32;
    fn set_nth_selection_caret_virtual_space(&mut self, selection: u32, space: u32);
    fn nth_selection_caret_virtual_space(&self, selection: u32) -> u32;
    fn set_nth_selection_anchor(&mut self, selection: u32, anchor: u32);
    fn nth_selection_anchor(&self, selection: u32) -> u32;
    fn set_nth_selection_anchor_virtual_space(&mut self, selection: u32, anchor: u32);
    fn nth_selection_anchor_virtual_space(&self, selection: u32) -> u32;
    fn set_nth_selection_start(&mut self, selection: u32, anchor: u32);
    fn nth_selection_start(&self, selection: u32) -> u32;
    fn set_nth_selection_end(&mut self, selection: u32, caret: u32);
    fn nth_selection_end(&self, selection: u32) -> u32;
    fn set_rectangular_selection_caret(&mut self, caret: u32);
    fn rectangular_selection_caret(&self) -> u32;
    fn set_rectangular_selection_caret_virtual_space(&mut self, space: u32);
    fn rectangular_selection_caret_virtual_space(&self) -> u32;
    fn set_rectangular_selection_anchor(&mut self, anchor: u32);
    fn rectangular_selection_anchor(&self) -> u32;
    fn set_rectangular_selection_anchor_virtual_space(&mut self, space: u32);
    fn rectangular_selection_anchor_virtual_space(&self) -> u32;
    fn set_additional_selection_alpha(&mut self, alpha: Option<u8>);
    fn additional_selection_alpha(&self) -> Option<u8>;
    fn set_additional_selection_foreground(&mut self, color: Color);
    fn additional_selection_foreground(&self) -> Color;
    fn set_additional_selection_background(&mut self, color: Color);
    fn additional_selection_background(&self) -> Color;
    fn set_additional_caret_foreground(&mut self, color: Color);
    fn additional_caret_foreground(&self) -> Color;
    fn set_additional_caret_blink(&mut self, enabled: bool);
    fn additional_caret_blink(&self) -> bool;
    fn set_additional_caret_visible(&mut self, enabled: bool);
    fn additional_caret_visible(&self) -> bool;
    fn swap_main_anchor_caret(&mut self);
    fn rotate_selection(&mut self);
    fn multiple_selection_add_next(&mut self);
    fn multiple_selection_add_each(&mut self);
    
    fn set_first_visible_line(&mut self, line: u32);
    fn first_visible_line(&self) -> u32;
    fn set_x_offset(&mut self, offset: u32);
    fn x_offset(&self) -> u32;
    fn line_scroll(&mut self, columns: u32, lines: u32);
    fn scroll_caret(&mut self);
    fn scroll_range(&mut self, secondary: u32, primary: u32);
    fn set_x_caret_policy(&mut self, policy: u32, slop: u32);
    fn set_y_caret_policy(&mut self, policy: u32, slop: u32);
    fn set_visible_policy(&mut self, policy: u32, slop: u32);
    fn set_horizontal_scrollbar_visible(&mut self, enabled: bool);
    fn is_horizontal_scrollbar_visible(&self) -> bool;
    fn set_vertical_scrollbar_visible(&mut self, enabled: bool);
    fn is_vertical_scrollbar_visible(&self) -> bool;
    fn set_scroll_width(&mut self, width: u32);
    fn scroll_width(&self) -> u32;
    fn set_scroll_width_tracking(&mut self, enabled: bool);
    fn is_scroll_width_tracking(&self) -> bool;
    fn set_end_at_last_line(&mut self, enabled: bool);
    fn is_end_at_last_line(&self) -> bool;
    
    fn set_view_whitespace(&mut self, ws: u32);
    fn view_whitespace(&self) -> u32;
    fn set_whitespace_foreground(&mut self, use_setting: bool, color: Color);
    fn set_whitespace_background(&mut self, use_setting: bool, color: Color);
    fn set_whitespace_size(&mut self, size: u32);
    fn whitespace_size(&self) -> u32;
    fn set_tab_draw_mode(&mut self, mode: u32);
    fn tab_draw_mode(&self) -> u32;
    fn set_extra_ascent(&mut self, ascent: u32);
    fn extra_ascent(&self) -> u32;
    fn set_extra_descent(&mut self, descent: u32);
    fn extra_descent(&self) -> u32;

    fn set_cursor(&mut self, cursor: u32);
    fn cursor(&self) -> u32;
    
    fn set_mouse_down_captures(&mut self, enabled: bool);
    fn is_mouse_down_captures(&self) -> bool;
    fn set_mouse_wheel_captures(&mut self, enabled: bool);
    fn is_mouse_wheel_captures(&self) -> bool;
    
    fn set_eol_mode(&mut self, mode: u32);
    fn eol_mode(&self) -> u32;
    fn convert_eols(&mut self, new_mode: u32);
    fn set_eol_visible(&mut self, enabled: bool);
    fn is_eol_visible(&self) -> bool;
    fn supported_line_end_types(&self) -> u32;
    fn set_allowed_line_end_types(&mut self, bit_set: u32);
    fn allowed_line_end_types(&self) -> u32;
    fn active_line_end_types(&self) -> u32;
    
    fn word_end_position(&self, position: u32, only_word_chars: bool) -> u32;
    fn word_start_position(&self, position: u32, only_word_chars: bool) -> u32;
    fn is_range_word(&self, start: u32, end: u32) -> bool;
    fn set_word_chars(&mut self, chars: &str);
    fn word_chars<'a>(&'a self) -> Cow<'a, str>; 
    fn set_whitespace_chars(&mut self, chars: &str);
    fn whitespace_chars<'a>(&'a self) -> Cow<'a, str>;
    fn set_punctuation_chars(&mut self, chars: &str);
    fn punctuation_chars<'a>(&'a self) -> Cow<'a, str>;
    fn set_default_chars(&mut self);
    
    /*

    */
    //fn set_margin_width(&mut self, index: usize, width: isize);
}

pub struct Color(u32);
impl Color {
    pub fn from_components(red: u8, green: u8, blue: u8) -> Color {
        Color (
            red as u32 | (green as u32) << 8 | (blue as u32) << 16
        )
    }
}
impl From<(u8, u8, u8)> for Color {
    fn from(arg: (u8, u8, u8)) -> Color {
        Color::from_components(arg.0, arg.1, arg.2)
    }
}
impl From<Color> for (u8, u8, u8) {
    fn from(arg: Color) -> (u8, u8, u8) {
        (
            (arg.0 & 0xFF) as u8,
            (arg.0 & 0xFF00) >> 8 as u8,
            (arg.0 & 0xFF0000) >> 16 as u8 
        )
    }
}
