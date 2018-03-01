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
    fn style_at_pos(&self, pos: u32) -> StyleId;
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
    
    fn end_styled(&self) -> u32;
    fn start_styling(&mut self, start: u32);
    fn set_styling(&mut self, len: u32, style: StyleId);
    fn set_styling_ex(&mut self, len: u32, styles: &str);
    fn set_idle_styling(&mut self, style: StyleId);
    fn idle_styling(&self) -> u32;
    fn set_line_state(&mut self, line: u32, state: u32);
    fn line_state(&self, line: u32) -> u32;
    fn max_line_state(&self) -> u32;
    
    fn reset_default_style(&mut self);
    fn clear_all_styles(&mut self);
    fn style_at(&self, id: StyleId) -> &Style;
	fn style_at_mut(&mut self, id: StyleId) -> &mut Style;
    
    fn set_selection_foreground(&mut self, use_setting: bool, color: Color);
    fn set_selection_background(&mut self, use_setting: bool, color: Color);
    fn set_selection_alpha(&mut self, alpha: u8);
    fn selection_alpha(&self) -> u8;
    fn set_selection_eol_filled(&mut self, enabled: bool);
    fn is_selection_eol_filled(&self) -> bool;
    fn set_caret_foreground(&mut self, color: Color);
    fn caret_foreground(&self) -> Color;
    fn set_caret_line_visible(&mut self, enabled: bool);
    fn is_caret_line_visible(&self) -> bool;
    fn set_caret_line_background(&mut self, color: Color);
    fn caret_line_background(&self) -> Color;
    fn set_caret_line_background_alpha(&mut self, alpha: u8);
    fn caret_line_background_alpha(&self) -> u8;
    fn set_caret_line_frame(&mut self, width: u32);
    fn caret_line_frame(&self) -> u32;
    fn set_caret_line_visible_always(&mut self, enabled: bool);
    fn is_caret_line_visible_always(&self) -> bool;
    fn set_caret_period(&mut self, period_ms: u32);
    fn caret_period(&self) -> u32;
    fn set_caret_style(&mut self, style: StyleId);
    fn caret_style(&self) -> StyleId;
    fn set_caret_width(&mut self, style: StyleId);
    fn caret_width(&self) -> u32;
    fn set_hotspot_active_foreground(&mut self, use_setting: bool, color: Color);
    fn hotspot_active_foreground(&self) -> Color;
    fn set_hotspot_active_background(&mut self, use_setting: bool, color: Color);
    fn hotspot_active_background(&self) -> Color;
    fn set_hotspot_active_underline(&mut self, enabled: bool);
    fn is_hotspot_active_underline(&self) -> bool; 
    fn set_hotspot_single_line(&mut self, enabled: bool);
    fn is_hotspot_single_line(&self) -> bool;
    fn set_caret_sticky_behavior(&mut self, behavior: u32);
    fn caret_sticky_behavior(&self) -> u32;
    fn toggle_caret_sticky(&mut self);
    
    fn set_char_representation(&mut self, c: char, repr: &str);
    fn char_representation<'a>(&'a self, c: char) -> Cow<'a, str>;
    fn clear_char_representation(&mut self, c: char);
    fn set_control_char_symbol(&mut self, symbol: u32);
    fn control_char_symbol(&self) -> u32;
    
    fn set_margins(&mut self, margins: u32);
    fn margins(&self) -> u32;
    fn margin_at(&self, id: MarginId) -> Option<&Margin>;
	fn margin_at_mut(&mut self, id: MarginId) -> Option<&mut Margin>;
	fn set_fold_margin_background(&mut self, use_setting: bool, color: Color);
    fn set_fold_margin_foreground(&mut self, use_setting: bool, color: Color);
    fn set_margin_text(&mut self, line: u32, text: &str);
    fn margin_text<'a>(&'a self, line: u32) -> Cow<'a, str>;
    fn set_margin_style(&mut self, line: u32, style: StyleId);
    fn margin_style(&self, line: u32) -> u32;
    fn set_margin_styles(&mut self, line: u32, styles: &str);
    fn margin_styles<'a>(&'a self, line: u32) -> Cow<'a, str>;
    fn clear_all_margin_text(&mut self);
    fn set_margin_style_offset(&mut self, style: StyleId);
    fn margin_style_offset(&self) -> u32;
    fn set_margin_options(&mut self, opts: u32);
    fn margin_options(&self) -> u32;
    
    fn set_annotation_text(&mut self, line: u32, text: &str);
    fn annotation_text<'a>(&'a self, line: u32) -> Cow<'a, str>;
    fn set_annotation_style(&mut self, line: u32, style: StyleId);
    fn annotation_style(&self) -> StyleId;
    fn set_annotation_styles(&mut self, line: u32, styles: &str);
    fn annotation_styles<'a>(&'a self, line: u32) -> Cow<'a, str>;
    fn annotation_lines(&self, line: u32) -> u32;
    fn clear_all_annotations(&mut self);
    fn annotations_visible(&mut self, enabled: bool);
    fn is_annotations_visible(&self) -> bool;
    fn set_annotations_style_offset(&mut self, style: StyleId);
    fn annotations_style_offset(&self) -> u32;
    
    fn set_buffered_draw(&mut self, enabled: bool);
    fn is_buffered_draw(&self) -> bool;
    fn set_draw_phases(&mut self, phases: u32);
    fn draw_phases(&self) -> u32;
    fn set_technology(&mut self, tech: Technology);
    fn technology(&self) -> Technology;
    fn set_font_quality(&mut self, q: FontQuality);
    fn font_quality(&self) -> FontQuality;
    fn set_codepage(&mut self, cp: Codepage);
    fn codepage(&self) -> Codepage;
    fn set_ime_interaction(&mut self, i: ImeInteraction);
    fn ime_interaction(&self) -> ImeInteraction;
    fn grab_focus(&mut self);
    fn set_focused(&mut self, enabled: bool);
    fn is_focused(&self) -> bool;
    
    fn highlight_braces(&mut self, pos_a: u32, pos_b: u32);
    fn highlight_broken_braces(&mut self, pos: u32);
    fn hightlight_braces_indicator(&mut self, use_setting: bool, indicator: u32);
    fn hightlight_broken_braces_indicator(&mut self, use_setting: bool, indicator: u32);
    fn brace_match(&self, max_re_style: StyleId) -> u32;
    
    fn set_tab_width(&mut self, width: u32);
    fn tab_width(&self) -> u32;
    fn clear_tab_stops(&mut self, line: u32);
    fn add_tab_stop(&mut self, line: u32, pos: u32);
    fn set_use_tabs(&mut self, enabled: bool);
    fn is_use_tabs(&self) -> bool;
    fn set_indent_size(&mut self, value: u32);
    fn indent_size(&self) -> u32;
    fn set_tab_indents(&mut self, enabled: bool);
    fn is_tab_indents(&self) -> bool;
    fn set_backspace_unindent(&mut self, enabled: bool);
    fn is_backspace_unindent(&self) -> bool;
    fn set_line_indent_width(&mut self, line: u32, value: u32);
    fn line_indent_width(&self, line: u32) -> u32;
    fn line_indent_position(&self, line: u32) -> u32;
    fn set_indent_guides(&mut self, guides: IndentationGuides);
    fn indent_guides(&self) -> IndentationGuides;
    fn set_highlight_guide(&mut self, column: u32);
    fn highlight_guide(&self) -> u32;
    
    fn marker_at(&mut self, id: MarkerId) -> &Marker;
    fn marker_at_mut(&mut self, id: MarkerId) -> &mut Marker;
    fn set_marker_rgba_image_width(&mut self, value: u32);
    fn set_marker_rgba_image_height(&mut self, value: u32);
    fn set_marker_rgba_image_scale(&mut self, percent: u32);
    fn enable_marker_highlight(&mut self, enabled: bool);
    fn add_marker_set(&mut self, line: u32, markers: u32);
    fn marker_at_line(&self, line: u32) -> &Marker;
    fn marker_at_line_mut(&mut self, line: u32) -> &mut Marker;
    fn marker_next(&self, line_start: u32, marker_mask: u32) -> u32;
    fn marker_prev(&self, line_start: u32, marker_mask: u32) -> u32;
    fn marker_line_from_handle(&self, handle: u32) -> u32;
    fn marker_delete_handle(&mut self, handle: u32);
	
	fn indicator_at(&self, id: IndicatorId) -> &Indicator;
	fn indicator_at_mut(&mut self, id: IndicatorId) -> &mut Indicator;	
	fn current_indicator(&self) -> &CurrentIndicator;
	fn current_indicator_mut(&mut self) -> &mut CurrentIndicator;
	
	fn enabled_indicators(&self, pos: u32) -> u32;
	fn show_find_indicator(&mut self, start: u32, end: u32);
	fn flash_find_indicator(&mut self, start: u32, end: u32);
	fn hide_find_indicator(&mut self);
	
	fn show_autocomplete_separated_str(&mut self, variants: &str);
	//fn show_autocomplete_list<T, S: AsRef<str>>(&mut str, len: u32, variants: T) where T: IntoIterator<Item=S>;
	fn cancel_autocomplete(&mut self);
	fn is_autocomplete_active(&self) -> bool;
	fn autocomplete_start_position(&self) -> u32;
	fn complete_autocomplete(&mut self);
	fn autocomplete_stops(&mut self, charset: &str);
	fn set_autocomplete_separator(&mut self, separator: char);
	fn autocomplete_separator(&self) -> char;
	fn autocomplete_select_matched(&mut self, variant: &str);
	fn current_autocomplete_item_index(&self) -> u32;
	fn current_autocomplete_item_text<'a>(&self) -> Cow<'a, str>;
	fn set_autocomplete_cancel_at_start(&mut self, enabled: bool);
	fn is_autocomplete_cancel_at_start(&self) -> bool;
	fn set_fillups(&mut self, chars: &str);
	fn set_autocomplete_single_select(&mut self, enabled: bool);
	fn is_autocomplete_single_select(&self) -> bool;
	fn set_autocomplete_ignore_case(&mut self, enabled: bool);
	fn is_autocomplete_ignore_case(&self) -> bool;
	fn set_autocomplete_case_insensitive_behavior(&mut self, value: CaseInsensitiveBehavior);
	fn autocomplete_case_insensitive_behavior(&self) -> CaseInsensitiveBehavior;
	fn set_autocomplete_multiselection_mode(&mut self, mode: AutocompleteMultiselectionMode);
	fn autocomplete_multiselection_mode(&self) -> AutocompleteMultiselectionMode;
	fn set_autocomplete_order(&mut self, order: AutocompleteOrder);
	fn autocomplete_order(&self) -> AutocompleteOrder;
	fn set_autocomplete_autohide(&mut self, enabled: bool);
	fn is_autocomplete_autohide(&self) -> bool;
	fn set_autocomplete_drop_rest_of_word(&mut self, enabled: bool);
	fn is_autocomplete_drop_rest_of_word(&self) -> bool;
	fn register_autocomplete_image(&mut self, type_: u32, data: AutocompleteImage);
	fn clear_registered_autocomplete_images(&mut self);
	fn set_autocomplete_type_separator(&mut self, c: char);
	fn autocomplete_type_separator(&self) -> char;
	fn set_autocomplete_max_visible_rows_height(&mut self, value: u32);
	fn autocomplete_max_visible_rows_height(&self) -> u32;
	fn set_autocomplete_max_visible_chars_width(&mut self, value: u32);
	fn autocomplete_max_visible_chars_width(&self) -> u32;
	fn show_autocomplete_user_list(&mut self, type_: u32, variants: &str);
	
	fn show_call_tip(&mut self, message: &str);
	fn cancel_call_tip(&mut self);
	fn is_call_tip_active(&self) -> bool;
	fn call_tip_start_position(&self) -> u32;
	fn set_call_tip_start_position(&mut self, pos: u32);
	fn set_call_tip_highlight(&mut self, start: u32, end: u32);
	fn set_call_tip_foreground(&mut self, color: Color);
	fn set_call_tip_background(&mut self, color: Color);
	fn set_call_tip_highlight_foreground(&mut self, color: Color);
	fn use_call_tip_style(&mut self, tab_size: u32);
	fn set_call_tip_above_text(&mut self, enabled: bool);
	
	fn assign_command_key<Kd: KeyDefinition, Kc: KeyCommand>(&mut self, definition: kd, command: Kc);
	fn clear_command_key<Kd: KeyDefinition>(&mut self, definition: kd);
	fn clear_all_command_keys(&mut self);
	
	
	fn use_popup(&mut self, mode: PopUpMode);
	
	fn start_macro_recording(&mut self);
	fn stop_macro_recording(&mut self);
	
	fn format_range(&mut self, draw: bool, fr: &Sci_RangeToFormat) -> u32;
	fn set_print_magnification(&mut self, m: i32);
	fn print_magnification(&self) -> i32;
	fn set_print_color_mode(&mut self, mode: PrintColorMode);
	fn print_color_mode(&self) -> PrintColorMode;
	fn set_print_wrap_mode(&mut self, mode: PrintWrapMode);
	fn print_wrap_mode(&self) -> PrintWrapMode;
	
	fn new_document(&mut self, size_in_bytes: usize, options: DocumentOptions) -> Box<Document>; // TODO `impl Trait` when available
	fn replace_current_document(&mut self, d: Box<Document>) -> Box<Document>; // TODO `impl Trait` when available
	/*
	1. Use SCI_GETDOCPOINTER to get a pointer to the document, doc.
	2. Use SCI_ADDREFDOCUMENT(0, doc) to increment the reference count.
	3. Use SCI_SETDOCPOINTER(0, docNew) to set a different document or SCI_SETDOCPOINTER(0, 0) to set a new, empty document.
	*/
	
	fn create_loader(&mut self, options: DocumentOptions) -> Result<Box<Loader>, LoaderError>; // TODO `impl Trait` when available
	
	
	/*
SCI_VISIBLEFROMDOCLINE(int docLine) → int
SCI_DOCLINEFROMVISIBLE(int displayLine) → int
SCI_SHOWLINES(int lineStart, int lineEnd)
SCI_HIDELINES(int lineStart, int lineEnd)
SCI_GETLINEVISIBLE(int line) → bool
SCI_GETALLLINESVISIBLE → bool
SCI_SETFOLDLEVEL(int line, int level)
SCI_GETFOLDLEVEL(int line) → int
SCI_SETAUTOMATICFOLD(int automaticFold)
SCI_GETAUTOMATICFOLD → int
SCI_SETFOLDFLAGS(int flags)
SCI_GETLASTCHILD(int line, int level) → int
SCI_GETFOLDPARENT(int line) → int
SCI_SETFOLDEXPANDED(int line, bool expanded)
SCI_GETFOLDEXPANDED(int line) → bool
SCI_CONTRACTEDFOLDNEXT(int lineStart) → int
SCI_TOGGLEFOLD(int line)
SCI_TOGGLEFOLDSHOWTEXT(int line, const char *text)
SCI_FOLDDISPLAYTEXTSETSTYLE(int style)
SCI_FOLDLINE(int line, int action)
SCI_FOLDCHILDREN(int line, int action)
SCI_FOLDALL(int action)
SCI_EXPANDCHILDREN(int line, int level)
SCI_ENSUREVISIBLE(int line)
SCI_ENSUREVISIBLEENFORCEPOLICY(int line)
*/
}
pub struct LoaderError;
pub trait Loader: Drop { // TODO call Release during Drop
	fn add_data(&mut self, data: &[u8]) -> Result<(), LoaderError>;
	fn into_document(self) -> Box<Document>;
}
pub trait Document: Drop {	
	// SCI_RELEASEDOCUMENT(<unused>, document *doc) called here during drop
}
pub enum DocumentOptions {
	Default = scintilla_sys::SC_DOCUMENTOPTION_DEFAULT as isize, // 	0 	Standard behaviour
	NoStyles = scintilla_sys::SC_DOCUMENTOPTION_STYLES_NONE as isize, //	1 	Stop allocation of memory for styles and treat all text as style 0
}
pub enum PrintWrapMode {
	None = scintilla_sys::SC_WRAP_NONE as isize, 
	Word = scintilla_sys::SC_WRAP_WORD as isize,
	Char = scintilla_sys::SC_WRAP_CHAR as isize, 
}
pub enum PrintColorMode {
	Normal = scintilla_sys::SC_PRINT_NORMAL as isize, //	0 	Print using the current screen colours. This is the default.
	InvertLight = scintilla_sys::SC_PRINT_INVERTLIGHT as isize, // 	1 	If you use a dark screen background this saves ink by inverting the light value of all colours and printing on a white background.
	BlackOnWhite = scintilla_sys::SC_PRINT_BLACKONWHITE as isize, //	2 	Print all text as black on a white background.
	ColorOnWhite = scintilla_sys::SC_PRINT_COLOURONWHITE as isize, // 	3 	Everything prints in its own colour on a white background.
	ColorOnWhiteDefaultBackground = scintilla_sys::SC_PRINT_COLOURONWHITEDEFAULTBG as isize, // 	4 	Everything prints in its own colour on a white background except that line numbers use their own background colour.
}
pub enum PopUpMode {
	Never = scintilla_sys::SC_POPUP_NEVER as isize, // 	0 	Never show default editing menu.
	All = scintilla_sys::SC_POPUP_ALL as isize, // 	1 	Show default editing menu if clicking on scintilla.
	Text = scintilla_sys::SC_POPUP_TEXT as isize, // 	2 	Show default editing menu only if clicking on text area.
}
pub enum KeyCode {
	Add = scintilla_sys::SCK_ADD as isize, 
	Back = scintilla_sys::SCK_BACK as isize, 
	Delete = scintilla_sys::SCK_DELETE as isize, 
	Divide = scintilla_sys::SCK_DIVIDE as isize, 
	Down = scintilla_sys::SCK_DOWN as isize, 
	End = scintilla_sys::SCK_END as isize, 
	Escape = scintilla_sys::SCK_ESCAPE as isize, 
	Home = scintilla_sys::SCK_HOME as isize, 
	Insert = scintilla_sys::SCK_INSERT as isize, 
	Left = scintilla_sys::SCK_LEFT as isize, 
	Menu = scintilla_sys::SCK_MENU as isize, 
	Next = scintilla_sys::SCK_NEXT as isize,
	PageDown = scintilla_sys::SCK_NEXT as isize, 
	Prior = scintilla_sys::SCK_PRIOR as isize,
	PageUp = scintilla_sys::SCK_PRIOR as isize, 
	Return = scintilla_sys::SCK_RETURN as isize, 
	Right = scintilla_sys::SCK_RIGHT as isize, 
	RightWin = scintilla_sys::SCK_RWIN as isize, 
	Subtract = scintilla_sys::SCK_SUBTRACT as isize, 
	Tab = scintilla_sys::SCK_TAB as isize, 
	Up = scintilla_sys::SCK_UP as isize, 
	Win = scintilla_sys::SCK_WIN as isize,
}
pub struct KeyModifier(pub(crate) u32);
pub mod key_modifier {
	pub const Alt: super::KeyModifier = KeyModifier(scintilla_sys::SCMOD_ALT as u32);
	pub const Ctrl: super::KeyModifier = KeyModifier(scintilla_sys::SCMOD_CTRL as u32);
	pub const Shift: super::KeyModifier = KeyModifier(scintilla_sys::SCMOD_SHIFT as u32);
	pub const Meta: super::KeyModifier = KeyModifier(scintilla_sys::SCMOD_META as u32);
	pub const Super: super::KeyModifier = KeyModifier(scintilla_sys::SCMOD_SUPER as u32);
	pub const None: super::KeyModifier = KeyModifier(scintilla_sys::SCMOD_NONE as u32);
}

impl ::std::ops::Add for KeyModifier {
	type Output = KeyModifier;
    fn add(self, other: KeyModifier) -> KeyModifier {
		KeyModifier(self.0 | other.0)
	}
}
impl ::std::ops::Sub for KeyModifier {
	type Output = KeyModifier;
    fn sub(self, other: KeyModifier) -> KeyModifier {
		KeyModifier(self.0 & ^other.0)
	}
}
impl ::std::ops::BitOr for KeyModifier {
	type Output = KeyModifier;
    fn bit_or(self, other: KeyModifier) -> KeyModifier {
		KeyModifier(self.0 | other.0)
	}
}
impl ::std::ops::BitAnd for KeyModifier {
	type Output = KeyModifier;
    fn bit_and(self, other: KeyModifier) -> KeyModifier {
		KeyModifier(self.0 & other.0)
	}
}
impl ::std::ops::BitXor for KeyModifier {
	type Output = KeyModifier;
    fn bit_xor(self, other: KeyModifier) -> KeyModifier {
		KeyModifier(self.0 ^ other.0)
	}
}

impl ::std::ops::AddAssign for KeyModifier {
	fn add_assign(&mut self, other: KeyModifier) {
		self.0 |= other.0;
	}
}
impl ::std::ops::SubAssign for KeyModifier {
	fn sub_assign(&mut self, other: KeyModifier) {
		self.0 &= ^other.0;
	}
}
impl ::std::ops::BitOrAssign for KeyModifier {
	fn bit_or_assign(&mut self, other: KeyModifier) {
		self.0 |= other.0
	}
}
impl ::std::ops::BitAndAssign for KeyModifier {
	fn bit_and_assign(&mut self, other: KeyModifier) {
		self.0 &= other.0;
	}
}
impl ::std::ops::BitXorAssign for KeyModifier {
	fn bit_xor_assign(&mut self, other: KeyModifier) {
		self.0 ^= other.0;
	}
}

pub trait KeyDefinition {
	fn exec(self) -> u32;
}
impl KeyDefinition for (KeyCode, KeyModifier) {
	fn exec(self) -> u32 {
		self.0 as u32 & (self.1 as u32 << 16)
	}
}
impl KeyDefinition for (u8, KeyModifier) {
	fn exec(self) -> u32 {
		self.0 as u32 & (self.1 as u32 << 16)
	}
}

pub trait KeyCommand {
	fn exec(self) -> u32;
}
impl KeyCommand for KeyboardCommand {
	fn exec(self) -> u32 {
		self as u32
	}
}
impl KeyCommand for u32 { // TODO narrow to the unparameterized SCI_* functions list
	fn exec(self) -> u32 {
		self as u32
	}
}

pub struct StyleId(u8);
impl From<u8> for StyleId {
	fn from(a: u8) -> StyleId {
		StyleId(a)
	}
}
impl From<StyleId> for u8 {
	fn from(a: StyleId) -> u8 {
		StyleId.0
	}
}
pub trait Style {
	fn id(&self) -> StyleId;
	fn set_font(&mut self, style: StyleId, name: &str);
    fn font<'a>(&'a self, style: StyleId) -> Cow<'a, str>;
    fn set_size(&mut self, style: StyleId, size: u32);
    fn size(&self, style: StyleId) -> u32;
    fn set_size_fractional(&mut self, style: StyleId, hundreth_points: u32);
    fn size_fractional(&self, style: StyleId) -> u32;
    fn set_bold(&mut self, style: StyleId, enabled: bool);
    fn is_bold(&self, style: StyleId) -> bool;
    fn set_italic(&mut self, style: StyleId, enabled: bool);
    fn is_italic(&self, style: StyleId) -> bool;
    fn set_underline(&mut self, style: StyleId, enabled: bool);
    fn is_underline(&self, style: StyleId) -> bool;
    fn set_weight(&mut self, style: StyleId, weight: u32);
    fn weight(&self, style: StyleId) -> u32;
    fn set_foreground(&mut self, style: StyleId, color: Color);
    fn foreground(&self, style: StyleId) -> Color;
    fn set_background(&mut self, style: StyleId, color: Color);
    fn background(&self, style: StyleId) -> Color;
    fn set_eol_filled(&mut self, style: StyleId, enabled: bool);
    fn is_eol_filled(&self, style: StyleId) -> bool;
    fn set_charset(&mut self, style: StyleId, charset: u32);
    fn charset(&self, style: StyleId) -> u32;
    fn set_case(&mut self, style: StyleId, case: u32);
    fn case(&self, style: StyleId) -> u32;
    fn set_visible(&mut self, style: StyleId, enabled: bool);
    fn is_visible(&self, style: StyleId) -> bool;
    fn set_changeable(&mut self, style: StyleId, enabled: bool);
    fn is_changeable(&self, style: StyleId) -> bool;
    fn set_hotspot(&mut self, style: StyleId, enabled: bool);
    fn is_hotspot(&self, style: StyleId) -> bool;
}

pub struct MarginId(u8);
impl From<u8> for MarginId {
	//TODO sanity check 
	fn from(a: u8) -> MarginId {
		MarginId(a)
	}
}
impl From<MarginId> for u8 {
	fn from(a: MarginId) -> u8 {
		MarginId.0
	}
}
pub trait Margin {
	fn id(&self) -> StyleId;
	fn set_type(&mut self, type_: u32);
    fn type_(&self) -> u32;
    fn set_width(&mut self, width: u32);
    fn width(&self) -> u32;
    fn set_mask(&mut self, mask: u32);
    fn mask(&self) -> u32;
    fn set_sensitive(&mut self, enabled: bool);
    fn is_sensitive(&self) -> bool;
    fn set_cursor(&mut self, cursor: u32);
    fn cursor(&self) -> u32;
    fn set_background(&mut self, color: Color);
    fn background(&self) -> Color;
    fn set_left(&mut self, width: u32);
    fn left(&self) -> u32;
    fn set_right(&mut self, width: u32);
    fn right(&self) -> u32;
}

pub struct MarkerId(u8);
impl From<u8> for MarkerId {
	//TODO sanity check
	fn from(a: u8) -> MarkerId {
		MarkerId(a)
	}
}
impl From<MarkerId> for u8 {
	fn from(a: MarkerId) -> u8 {
		MarkerId.0
	}
}
pub struct MarkerError;
pub trait Marker {
	fn id(&self) -> MarkerId;
	fn define_with_mark<T>(&mut self, mark: T) where T: Mark;
    fn define_with_char(&mut self, c: char);
    fn define_with_pixmap(&mut self, number: u32, pixmap: &str);    
    
	fn define_rgba_image(&mut self, pixels: &str);
    fn symbol_defined(&self) -> u32;
    fn set_foreground(&mut self, color: Color);
    fn set_background(&mut self, color: Color);
    fn set_background_selected(&mut self, color: Color);
    fn set_alpha(&mut self, alpha: u8);
    fn add_to_line(&mut self, line: u32) -> Result<(), MarkerError>;
    fn delete_from_line(&mut self, line: u32);
    fn delete_from_all(&mut self);
}
pub trait Mark {
	fn into_arg(self) -> u32;
}
pub enum SymbolMark {
	Circle = scintilla_sys::SC_MARK_CIRCLE as isize, 
	RoundedRectangle = scintilla_sys::SC_MARK_ROUNDRECT as isize, 
	Arrow = scintilla_sys::SC_MARK_ARROW as isize, 
	SmallRectangle = scintilla_sys::SC_MARK_SMALLRECT as isize, 
	ShortArrow = scintilla_sys::SC_MARK_SHORTARROW as isize, 
	Empty = scintilla_sys::SC_MARK_EMPTY as isize, 
	ArrowDown = scintilla_sys::SC_MARK_ARROWDOWN as isize, 
	Minus = scintilla_sys::SC_MARK_MINUS as isize, 
	Plus = scintilla_sys::SC_MARK_PLUS as isize, 
	Arrows = scintilla_sys::SC_MARK_ARROWS as isize, 
	ThreeDots = scintilla_sys::SC_MARK_DOTDOTDOT as isize, 
	BackgroundColor = scintilla_sys::SC_MARK_BACKGROUND as isize, 
	LeftRectangle = scintilla_sys::SC_MARK_LEFTRECT as isize, 
	FullRectangle = scintilla_sys::SC_MARK_FULLRECT as isize, 
	Bookmark = scintilla_sys::SC_MARK_BOOKMARK as isize, 
	Underline = scintilla_sys::SC_MARK_UNDERLINE as isize,
	
	BoxMinus = scintilla_sys::SC_MARK_BOXMINUS as isize, 
	BoxMinusConnected = scintilla_sys::SC_MARK_BOXMINUSCONNECTED as isize, 
	CircleMinus = scintilla_sys::SC_MARK_BOXPLUS as isize, 
	BoxPlusConnected = scintilla_sys::SC_MARK_BOXPLUSCONNECTED as isize, 
	CircleMinus = scintilla_sys::SC_MARK_CIRCLEMINUS as isize, 
	CircleMinusConnected = scintilla_sys::SC_MARK_CIRCLEMINUSCONNECTED as isize, 
	CirclePlus = scintilla_sys::SC_MARK_CIRCLEPLUS as isize, 
	CirclePlusConnected = scintilla_sys::SC_MARK_CIRCLEPLUSCONNECTED as isize, 
	LCorner = scintilla_sys::SC_MARK_LCORNER as isize, 
	LCornerCurve = scintilla_sys::SC_MARK_LCORNERCURVE as isize, 
	TCorner = scintilla_sys::SC_MARK_TCORNER as isize, 
	TCornerCurve = scintilla_sys::SC_MARK_TCORNERCURVE as isize, 
	VLine = scintilla_sys::SC_MARK_VLINE as isize
}
impl Mark for SymbolMark {
	fn into_arg(self) -> u32 {
		self as u32
	}
}
impl Mark for char {
	fn into_arg(self) -> u32 {
		(scintilla_sys::SC_MARK_CHARACTER + self as u32)
	}
}

pub type IndicatorId = u32;
pub trait CurrentIndicator: Indicator {
	fn set_value(&mut self, value: u32);
	fn value(&self) -> u32;
	fn fill_range(&mut self, start: u32, len: u32);
	fn clear_range(&mut self, start: u32, len: u32);	
}
pub trait Indicator {
	fn id(&self) -> IndicatorId;
	fn set_current(&mut self) -> Option<&mut CurrentIndicator>;
	fn is_current(&self) -> Option<&CurrentIndicator>;
	fn is_current_mut(&mut self) -> Option<&mut CurrentIndicator>;
	fn set_style(&mut self, style: StyleId);
	fn style(&self) -> StyleId;
	fn set_foreground(&mut self, color: Color);
	fn foreground(&self) -> Color;
	fn set_alpha(&mut self, alpha: u8);
	fn alpha(&self) -> u8;
	fn set_outline_alpha(&mut self, alpha: u8);
	fn outline_alpha(&self) -> u8;
	fn set_underlined(&mut self, enabled: bool);
	fn is_underlined(&self) -> bool;
	fn set_hover_style(&mut self, style: StyleId);
	fn hover_style(&self) -> StyleId;
	fn set_hover_foreground(&mut self, color: Color);
	fn hover_foreground(&self) -> Color;
	fn set_flags(&mut self, flags: u32);
	fn flags(&self) -> u32;
	fn value(&self, pos: u32) -> u32;
	fn start(&self, pos: u32) -> u32;
	fn end(&self, pos: u32) -> u32;
}

pub enum KeyboardCommand {
	LineDown = scintilla_sys::SCI_LINEDOWN as isize,
 	LineDownExtend = scintilla_sys::SCI_LINEDOWNEXTEND as isize,
 	LineDownRectExtend = scintilla_sys::SCI_LINEDOWNRECTEXTEND as isize,
 	LineScrollDown = scintilla_sys::SCI_LINESCROLLDOWN as isize,
	LineUp = scintilla_sys::SCI_LINEUP as isize,
 	LineUpExtend = scintilla_sys::SCI_LINEUPEXTEND as isize,
 	LineUpRectExtend = scintilla_sys::SCI_LINEUPRECTEXTEND as isize,
 	LineScrollUp = scintilla_sys::SCI_LINESCROLLUP as isize,
	ParagraphDown = scintilla_sys::SCI_PARADOWN as isize,
 	ParagraphDownExtend = scintilla_sys::SCI_PARADOWNEXTEND as isize,
 	ParagraphUp = scintilla_sys::SCI_PARAUP as isize,
 	ParagraphUpExtend = scintilla_sys::SCI_PARAUPEXTEND as isize,
	CharLeft = scintilla_sys::SCI_CHARLEFT as isize,
 	CharLeftExtend = scintilla_sys::SCI_CHARLEFTEXTEND as isize,
 	CharLeftRectExtend = scintilla_sys::SCI_CHARLEFTRECTEXTEND as isize,
	CharRight = scintilla_sys::SCI_CHARRIGHT as isize,
 	CharRightExtend = scintilla_sys::SCI_CHARRIGHTEXTEND as isize,
 	CharRightRectExtend = scintilla_sys::SCI_CHARRIGHTRECTEXTEND as isize,
	WordLeft = scintilla_sys::SCI_WORDLEFT as isize,
 	WordLeftExtend = scintilla_sys::SCI_WORDLEFTEXTEND as isize,
 	WordRight = scintilla_sys::SCI_WORDRIGHT as isize,
 	WordRightExtend = scintilla_sys::SCI_WORDRIGHTEXTEND as isize,
	WordLeftEnd = scintilla_sys::SCI_WORDLEFTEND as isize,
	WordLeftEndExtend = scintilla_sys::SCI_WORDLEFTENDEXTEND as isize,
 	WordRightEnd = scintilla_sys::SCI_WORDRIGHTEND as isize,
 	WordRightEndExtend = scintilla_sys::SCI_WORDRIGHTENDEXTEND as isize,
	WordPartLeft = scintilla_sys::SCI_WORDPARTLEFT as isize,
 	WordPartLeftExtend = scintilla_sys::SCI_WORDPARTLEFTEXTEND as isize,	WordPartRight = scintilla_sys::SCI_WORDPARTRIGHT as isize,
 	WordPartRightExtend = scintilla_sys::SCI_WORDPARTRIGHTEXTEND as isize,
	Home = scintilla_sys::SCI_HOME as isize,
 	HomeExtend = scintilla_sys::SCI_HOMEEXTEND as isize,	HomeRectExtend = scintilla_sys::SCI_HOMERECTEXTEND as isize,
	HomeDisplay = scintilla_sys::SCI_HOMEDISPLAY as isize,
 	HomeDisplayExtend = scintilla_sys::SCI_HOMEDISPLAYEXTEND as isize,
 	HomeWrap = scintilla_sys::SCI_HOMEWRAP as isize,
 	HomeWrapExtend = scintilla_sys::SCI_HOMEWRAPEXTEND as isize,
	VcHome = scintilla_sys::SCI_VCHOME as isize,
 	VcHomeExtend = scintilla_sys::SCI_VCHOMEEXTEND as isize,
 	VcHomeRectExtend = scintilla_sys::SCI_VCHOMERECTEXTEND as isize,
	VcHomeWrap = scintilla_sys::SCI_VCHOMEWRAP as isize,
 	VcHomeWrapExtend = scintilla_sys::SCI_VCHOMEWRAPEXTEND as isize,
 	VcHomeDisplay = scintilla_sys::SCI_VCHOMEDISPLAY as isize,
 	VcHomeDisplayExtend = scintilla_sys::SCI_VCHOMEDISPLAYEXTEND as isize,
	LineEnd = scintilla_sys::SCI_LINEEND as isize,
 	LineEndExtend = scintilla_sys::SCI_LINEENDEXTEND as isize,
 	LineEndRectExtend = scintilla_sys::SCI_LINEENDRECTEXTEND as isize,
	LineEndDisplay = scintilla_sys::SCI_LINEENDDISPLAY as isize,
 	LineEndDisplayExtend = scintilla_sys::SCI_LINEENDDISPLAYEXTEND as isize,
 	LineEndWrap = scintilla_sys::SCI_LINEENDWRAP as isize,
 	LineEndWrapExtend = scintilla_sys::SCI_LINEENDWRAPEXTEND as isize,
	DocumentStart = scintilla_sys::SCI_DOCUMENTSTART as isize,
 	DocumentStartExtend = scintilla_sys::SCI_DOCUMENTSTARTEXTEND as isize,
 	DocumentEnd = scintilla_sys::SCI_DOCUMENTEND as isize,
 	DocumentEndExtend = scintilla_sys::SCI_DOCUMENTENDEXTEND as isize,
	PageUp = scintilla_sys::SCI_PAGEUP as isize,
 	PageUpExtend = scintilla_sys::SCI_PAGEUPEXTEND as isize,
 	PageUpRectExtend = scintilla_sys::SCI_PAGEUPRECTEXTEND as isize,
	PageDown = scintilla_sys::SCI_PAGEDOWN as isize,
 	PageDownExtend = scintilla_sys::SCI_PAGEDOWNEXTEND as isize,
 	PageDownRectExtend = scintilla_sys::SCI_PAGEDOWNRECTEXTEND as isize,
	StutteredPageUp = scintilla_sys::SCI_STUTTEREDPAGEUP as isize,
 	StutteredPageUpExtend = scintilla_sys::SCI_STUTTEREDPAGEUPEXTEND as isize,
	StutteredPageDown = scintilla_sys::SCI_STUTTEREDPAGEDOWN as isize,
 	StutteredPageDownExtend = scintilla_sys::SCI_STUTTEREDPAGEDOWNEXTEND as isize,
	DeleteBack = scintilla_sys::SCI_DELETEBACK as isize,
 	DeleteBackNotLine = scintilla_sys::SCI_DELETEBACKNOTLINE as isize,
	DeleteWordLeft = scintilla_sys::SCI_DELWORDLEFT as isize,
 	DeleteWordRight = scintilla_sys::SCI_DELWORDRIGHT as isize,
 	DeleteWordRightEnd = scintilla_sys::SCI_DELWORDRIGHTEND as isize,
	DeleteLineLeft = scintilla_sys::SCI_DELLINELEFT as isize,
 	DeleteLineRight = scintilla_sys::SCI_DELLINERIGHT as isize,
 	LineDelete = scintilla_sys::SCI_LINEDELETE as isize,
	LineCut = scintilla_sys::SCI_LINECUT as isize,
	LineCopy = scintilla_sys::SCI_LINECOPY as isize,
 	LineTranspose = scintilla_sys::SCI_LINETRANSPOSE as isize,
 	LineReverse = scintilla_sys::SCI_LINEREVERSE as isize,
 	LineDuplicate = scintilla_sys::SCI_LINEDUPLICATE as isize,
	LowerCase = scintilla_sys::SCI_LOWERCASE as isize,
 	UpperCase = scintilla_sys::SCI_UPPERCASE as isize,
 	Cancel = scintilla_sys::SCI_CANCEL as isize,
 	EditToggleOverType = scintilla_sys::SCI_EDITTOGGLEOVERTYPE as isize,
	NewLine = scintilla_sys::SCI_NEWLINE as isize,
 	FormFeed = scintilla_sys::SCI_FORMFEED as isize,
 	Tab = scintilla_sys::SCI_TAB as isize,
 	BackTab = scintilla_sys::SCI_BACKTAB as isize,
	SelectionDuplicate = scintilla_sys::SCI_SELECTIONDUPLICATE as isize,
 	VerticalCenterCaret = scintilla_sys::SCI_VERTICALCENTRECARET as isize,
	MoveSelectedLinesUp = scintilla_sys::SCI_MOVESELECTEDLINESUP as isize,
 	MoveSelectedLinesDown = scintilla_sys::SCI_MOVESELECTEDLINESDOWN as isize,
	ScrollToStart = scintilla_sys::SCI_SCROLLTOSTART as isize,
 	ScrollToEnd = scintilla_sys::SCI_SCROLLTOEND as isize,
}

pub enum AutocompleteImage<'a> {
	XPM(&'a [u8]),
	RGBA(&'a [u8]),
}

pub enum AutocompleteOrder {
	PreSorted = scintilla_sys::SC_ORDER_PRESORTED as isize,
	PerformSort = scintilla_sys::SC_ORDER_PERFORMSORT as isize,
	Custom = scintilla_sys::SC_ORDER_CUSTOM as isize,
}

pub enum AutocompleteMultiselectionMode {
	Once = scintilla_sys::SC_MULTIAUTOC_ONCE as isize,
	Each = scintilla_sys::SC_MULTIAUTOC_EACH as isize,
}

pub enum IndentationGuides {
    None = scintilla_sys::SC_IV_NONE as isize,
    Real = scintilla_sys::SC_IV_REAL as isize,
    LookForward = scintilla_sys::SC_IV_LOOKFORWARD as isize,
    LookBoth = scintilla_sys::SC_IV_LOOKBOTH as isize,
}

pub enum ImeInteraction {
    Windowed = scintilla_sys::SC_IME_WINDOWED as isize,
    Inline = scintilla_sys::SC_IME_INLINE as isize,
}

pub enum Technology {
    SystemDefault = scintilla_sys::SC_TECHNOLOGY_DEFAULT as isize,
    DirectWrite = scintilla_sys::SC_TECHNOLOGY_DIRECTWRITE as isize,
    DirectWriteFrameRetain = scintilla_sys::SC_TECHNOLOGY_DIRECTWRITERETAIN as isize,
    DirectWriteGdi = scintilla_sys::SC_TECHNOLOGY_DIRECTWRITEDC as isize,    
}

pub enum FontQuality {
    SystemDefault = scintilla_sys::SC_EFF_QUALITY_DEFAULT as isize, 
    NonAntialiased = scintilla_sys::SC_EFF_QUALITY_NON_ANTIALIASED as isize, 
    Antialiased = scintilla_sys::SC_EFF_QUALITY_ANTIALIASED as isize, 
    LcdOptimized = scintilla_sys::SC_EFF_QUALITY_LCD_OPTIMIZED as isize,
}

pub enum Codepage {
    Ascii = 0isize,
    Utf8 = 65001isize, 
    ShiftJis = 932isize, 
    ChineseSimplifiedGbk = 936isize, 
    KoreanUnifiedHangul = 949isize, 
    ChineseTraditionalBig5 = 950isize, 
    KoreanJohab = 1361isize
}

pub enum CaseInsensitiveBehavior {
	Respect = scintilla_sys::SC_CASEINSENSITIVEBEHAVIOUR_RESPECTCASE as isize,
	Ignore = scintilla_sys::SC_CASEINSENSITIVEBEHAVIOUR_IGNORECASE as isize,
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

// TODO move off to mod
trait ScintillaUnsafe {
	fn marker_define(&mut self, number: u32, symbol: u32);
    fn marker_define_pixmap(&mut self, number: u32, pixmap: &str);
    fn rgba_image_set_width(&mut self, value: u32);
    fn rgba_image_set_height(&mut self, value: u32);
    fn rgba_image_set_scale(&mut self, percent: u32);
    fn marker_define_rgba_image(&mut self, number: u32, pixels: &str);
    fn marker_symbol_defined(&self, number: u32) -> u32;
    fn marker_set_foreground(&mut self, number: u32, color: Color);
    fn marker_set_background(&mut self, number: u32, color: Color);
    fn marker_set_background_selected(&mut self, number: u32, color: Color);
    fn enable_marker_highlight(&mut self, enabled: bool);
    fn marker_set_alpha(&mut self, number: u32, alpha: u8);
    fn marker_add(&mut self, line: u32, number: u32) -> u32;
    fn marker_add_set(&mut self, line: u32, set: u32);
    fn marker_delete(&mut self, line: u32, number: u32);
    fn marker_delete_all(&mut self, number: u32);
    fn marker(&self, line: u32) -> u32;
    fn marker_next(&self, line_start: u32, marker_mask: u32) -> u32;
    fn marker_prev(&self, line_start: u32, marker_mask: u32) -> u32;
    fn marker_line_from_handle(&self, handle: u32) -> u32;
    fn marker_delete_handle(&mut self, handle: u32);
	
	fn indicator_set_style(&mut self, indicator: u32, style: StyleId);
	fn indicator_style(&self, indicator: u32) -> u32;
	fn indicator_set_foreground(&mut self, indicator: u32, color: Color);
	fn indicator_foreground(&self, indicator: u32) -> Color;
	fn indicator_set_alpha(&mut self, indicator: u32, alpha: u8);
	fn indicator_alpha(&self, indicator: u32) -> u8;
	fn indicator_set_outline_alpha(&mut self, indicator: u32, alpha: u8);
	fn indicator_outline_alpha(&self, indicator: u32) -> u8;
	fn indicator_set_underline(&mut self, indicator: u32, enabled: bool);
	fn indicator_is_underline(&self, indicator: u32) -> bool;
	fn indicator_set_hover_style(&mut self, indicator: u32, style: StyleId);
	fn indicator_hover_style(&self, indicator: u32) -> u32;
	fn indicator_set_hover_foreground(&mut self, indicator: u32, color: Color);
	fn indicator_hover_foreground(&self, indicator: u32) -> Color;
	fn indicator_set_flags(&mut self, indicator: u32, flags: u32);
	fn indicator_flags(&self, indicator: u32) -> u32;
	
	fn set_current_indicator(&mut self, indicator: u32);
	fn current_indicator(&self) -> u32;
	fn set_indicator_value(&mut self, value: u32);
	fn indicator_value(&self) -> u32;
	fn fill_indicator_range(&mut self, start: u32, len: u32);
	fn clear_indicator_range(&mut self, start: u32, len: u32);
	fn all_indicator_on_for(&mut self, pos: u32);
	fn indicator_value(&self, indicator: u32, pos: u32) -> u32;
	fn indicator_start(&self, indicator: u32, pos: u32) -> u32;
	fn indicator_end(&self, indicator: u32, pos: u32) -> u32;
	fn find_indicator_show(&mut self, start: u32, end: u32);
	fn find_indicator_flash(&mut self, start: u32, end: u32);
	fn hide_indicators(&mut self);
}