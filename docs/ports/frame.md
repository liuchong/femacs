# emacs/src/frame.c (7907 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 66
- [x] framep  (Fframep, 1/1, doc 4f91c4d2fbba, line 286)
- [x] frame-live-p  (Fframe_live_p, 1/1, doc 11ff775ef2e0, line 326)
- [x] frame-id  (Fframe_id, 0/1, doc f289fcbcb305, line 345)
- [x] window-system  (Fwindow_system, 0/1, doc 6a9b13da901e, line 415)
- [x] frame-windows-min-size  (Fframe_windows_min_size, 4/4, doc 76d779f19812, line 495)
- [x] make-terminal-frame  (Fmake_terminal_frame, 1/1, doc 8bebd66a1678, line 1736)
- [x] select-frame  (Fselect_frame, 1/2, doc 32cf6ee01793, line 2097)
- [x] handle-switch-frame  (Fhandle_switch_frame, 1/1, doc 35cbb1fd733e, line 2126)
- [x] selected-frame  (Fselected_frame, 0/0, doc 3c054f509beb, line 2142)
- [x] old-selected-frame  (Fold_selected_frame, 0/0, doc c56243113513, line 2149)
- [x] frame-list  (Fframe_list, 0/0, doc f76baa4a6a2d, line 2161)
- [x] frame-parent  (Fframe_parent, 0/1, doc a8eac84d5397, line 2180)
- [x] frame-ancestor-p  (Fframe_ancestor_p, 2/2, doc f73cd3411ba9, line 2242)
- [x] frame-root-frame  (Fframe_root_frame, 0/1, doc e5ae09bb56e9, line 2269)
- [x] next-frame  (Fnext_frame, 0/2, doc 9f1420a62c04, line 2415)
- [x] previous-frame  (Fprevious_frame, 0/2, doc 1cf34128b904, line 2441)
- [x] last-nonminibuffer-frame  (Flast_nonminibuf_frame, 0/0, doc 03a98ef0d3ae, line 2467)
- [x] delete-frame  (Fdelete_frame, 0/2, doc 25f1b9319c8e, line 3150)
- [x] mouse-position  (Fmouse_position, 0/0, doc bc805c817668, line 3271)
- [x] mouse-pixel-position  (Fmouse_pixel_position, 0/0, doc 10185049c011, line 3329)
- [x] set-mouse-position  (Fset_mouse_position, 3/3, doc 4de496c9960a, line 3407)
- [x] set-mouse-pixel-position  (Fset_mouse_pixel_position, 3/3, doc 7e21ca01935b, line 3457)
- [x] make-frame-visible  (Fmake_frame_visible, 0/1, doc eda3336bf392, line 3505)
- [x] make-frame-invisible  (Fmake_frame_invisible, 0/2, doc b0f1cef2fec7, line 3549)
- [x] iconify-frame  (Ficonify_frame, 0/1, doc f12199e6dfd4, line 3591)
- [x] frame-visible-p  (Fframe_visible_p, 1/1, doc 06bc2b37fec9, line 3633)
- [x] visible-frame-list  (Fvisible_frame_list, 0/0, doc 070c2a01cb3b, line 3652)
- [x] raise-frame  (Fraise_frame, 0/1, doc bd313f46ee9a, line 3667)
- [x] lower-frame  (Flower_frame, 0/1, doc 722310663b5f, line 3686)
- [x] redirect-frame-focus  (Fredirect_frame_focus, 1/2, doc c4706aeda107, line 3702)
- [x] frame-focus  (Fframe_focus, 0/1, doc f8019955b620, line 3746)
- [x] x-focus-frame  (Fx_focus_frame, 1/2, doc e4a8dad8e20e, line 3756)
- [x] frame-after-make-frame  (Fframe_after_make_frame, 2/2, doc d69a2444ddde, line 3772)
- [x] frame-parameters  (Fframe_parameters, 0/1, doc 5c11ee07a06d, line 4083)
- [x] frame-parameter  (Fframe_parameter, 2/2, doc c631c4276663, line 4172)
- [x] modify-frame-parameters  (Fmodify_frame_parameters, 2/2, doc a21a9785ca31, line 4243)
- [x] frame-char-height  (Fframe_char_height, 0/1, doc cb7b0296d98c, line 4345)
- [x] frame-char-width  (Fframe_char_width, 0/1, doc 4707b526fead, line 4363)
- [x] frame-native-width  (Fframe_native_width, 0/1, doc 6aea9f198b3b, line 4381)
- [x] frame-native-height  (Fframe_native_height, 0/1, doc 8e2052ba07d5, line 4401)
- [x] tool-bar-pixel-width  (Ftool_bar_pixel_width, 0/1, doc 6e27dc322b24, line 4430)
- [x] frame-text-cols  (Fframe_text_cols, 0/1, doc 47fd165280a5, line 4447)
- [x] frame-text-lines  (Fframe_text_lines, 0/1, doc 9e07436df740, line 4454)
- [x] frame-total-cols  (Fframe_total_cols, 0/1, doc 3c7c8e48908d, line 4461)
- [x] frame-total-lines  (Fframe_total_lines, 0/1, doc 11485c022463, line 4468)
- [x] frame-text-width  (Fframe_text_width, 0/1, doc fd99b4e35a90, line 4475)
- [x] frame-text-height  (Fframe_text_height, 0/1, doc 1295808bba9e, line 4482)
- [x] frame-scroll-bar-width  (Fscroll_bar_width, 0/1, doc a3048f67f659, line 4489)
- [x] frame-scroll-bar-height  (Fscroll_bar_height, 0/1, doc 3eb68e8173e2, line 4496)
- [x] frame-fringe-width  (Ffringe_width, 0/1, doc abc24b5a50f1, line 4503)
- [x] frame-child-frame-border-width  (Fframe_child_frame_border_width, 0/1, doc 00fa40692584, line 4510)
- [x] frame-internal-border-width  (Fframe_internal_border_width, 0/1, doc 288a148e2803, line 4524)
- [x] frame-right-divider-width  (Fright_divider_width, 0/1, doc adb7dfe3a0e3, line 4531)
- [x] frame-bottom-divider-width  (Fbottom_divider_width, 0/1, doc ec88c2cbd2cb, line 4538)
- [x] set-frame-height  (Fset_frame_height, 2/4, doc 52ddc411639f, line 4563)
- [x] set-frame-width  (Fset_frame_width, 2/4, doc 88eb7ce23a3a, line 4592)
- [x] set-frame-size  (Fset_frame_size, 3/4, doc a703d54a255e, line 4621)
- [x] frame-position  (Fframe_position, 0/1, doc 38244a590e10, line 4644)
- [x] set-frame-position  (Fset_frame_position, 3/3, doc 28e5d89a9307, line 4662)
- [x] set-frame-size-and-position-pixelwise  (Fset_frame_size_and_position_pixelwise, 5/6, doc cb9035db063c, line 4697)
- [x] frame-window-state-change  (Fframe_window_state_change, 0/1, doc 875aaffcc3bc, line 4760)
- [x] set-frame-window-state-change  (Fset_frame_window_state_change, 0/2, doc 33a3650845a0, line 4775)
- [x] frame-scale-factor  (Fframe_scale_factor, 0/1, doc d2d8b2c743dd, line 4793)
- [x] frame-pointer-visible-p  (Fframe_pointer_visible_p, 0/1, doc 38724019d098, line 7001)
- [x] mouse-position-in-root-frame  (Fmouse_position_in_root_frame, 0/0, doc bbe78b0cf8ed, line 7011)
- [x] frame--set-was-invisible  (Fframe__set_was_invisible, 2/2, doc c8a999cdfcac, line 7042)

## Functions defined — 57
- [x] fset_buffer_predicate  (line 87, static)
- [x] fset_minibuffer_window  (line 92, static)
- [x] decode_live_frame  (line 98)
- [x] decode_any_frame  (line 107)
- [x] decode_window_system_frame  (line 124)
- [x] check_window_system  (line 134)
- [x] check_tty  (line 145, static)
- [x] decode_tty_frame  (line 159)
- [x] frame_get  (line 170, static)
- [x] get_frame_param  (line 183)
- [x] frame_inhibit_resize  (line 200)
- [x] set_menu_bar_lines  (line 222, static)
- [x] set_tab_bar_lines  (line 266, static)
- [x] frame_set_id  (line 368)
- [x] frame_set_id_from_params  (line 401)
- [x] frame_redisplay_p  (line 452)
- [x] frame_windows_min_size  (line 528, static)
- [x] keep_ratio  (line 606, static)
- [x] frame_size_history_adjust  (line 698, static)
- [x] frame_size_history_plain  (line 737)
- [x] frame_size_history_extra  (line 753)
- [x] adjust_frame_size  (line 826)
- [x] allocate_frame  (line 1147, static)
- [x] make_frame  (line 1154)
- [x] make_frame_without_minibuffer  (line 1314)
- [x] make_minibuffer_frame  (line 1365)
- [x] frame_next_F_name  (line 1409, static)
- [x] make_initial_frame  (line 1423)
- [x] make_terminal_frame  (line 1480, static)
- [x] get_future_frame_param  (line 1623, static)
- [x] tty_child_pos_param  (line 1645)
- [x] tty_child_size_param  (line 1685)
- [x] tty_child_frame_rect  (line 1725, static)
- [x] do_switch_frame  (line 1922)
- [x] frame_ancestor_p  (line 2211)
- [x] frame_subsumes_p  (line 2229, static)
- [x] root_frame  (line 2261)
- [x] candidate_frame  (line 2299, static)
- [x] next_frame  (line 2342, static)
- [x] prev_frame  (line 2382, static)
- [x] other_frames  (line 2496, static)
- [x] delete_frame  (line 2598)
- [x] frame_internal_border_part  (line 3192)
- [x] mouse_position  (line 3291)
- [x] make_frame_visible_1  (line 3535, static)
- [x] frames_discard_buffer  (line 3795)
- [x] store_in_alist  (line 3812)
- [x] frame_name_fnn_p  (line 3822, static)
- [x] set_term_frame_name  (line 3839, static)
- [x] store_frame_param  (line 3872)
- [x] frame_unspecified_color  (line 4075, static)
- [x] check_frame_pixels  (line 4546, static)
- [x] frame_make_pointer_invisible  (line 6974)
- [x] frame_make_pointer_visible  (line 6989)
- [x] init_frame_once  (line 7198)
- [x] init_frame_once_for_pdumper  (line 7209, static)
- [x] syms_of_frame  (line 7216)

## Macros defined — 0

## Types and their fields — 1
- [x] frame_parm_table: const char* name, int sym

## File-scope objects — 8
- [x] Lisp_Object selected_frame  (line 58)
- [x] Lisp_Object old_selected_frame  (line 61)
- [x] static struct frame last_nonminibuf_frame  (line 66, static)
- [x] bool frame_garbaged  (line 69)
- [x] int frame_default_tab_bar_height  (line 72)
- [x] int frame_default_tool_bar_height  (line 78)
- [x] Lisp_Object Vframe_list  (line 284)
- [x] static intmax_t tty_frame_count  (line 1406, static)
# ----------------------------------------------------------------------
## C line ranges

The 66 subrs, 57 functions and 8 objects above are the frozen configuration's *live* checklist
(`python3 tools/extract-abi.py emacs/src/frame.c`).  This unit now additionally **carries** the
first half of the `#ifdef HAVE_WINDOW_SYSTEM` arm, **C 4889-5919** (through
`gui_set_unsplittable`), which P-08 had left recorded only; C 5921-6970 (from
`gui_set_vertical_scroll_bars` to the `#endif` at C 6971) is the next round's work.  One row per
C block, in C's order; the `arm` cell says what the port does with it.  This table is the
coverage evidence P-86 ② asks for - `python3 tools/check-dead-file-coverage.py` binds a ledger
that carries this heading and requires every non-blank line of the reference to sit in exactly
one ascending row, with the last row reaching the file's final line.

| C | block | arm |
| --- | --- | --- |
| 1-85 | head / includes | live, with dead arms inside |
| 86-90 | fset_buffer_predicate | live |
| 91-96 | fset_minibuffer_window | live |
| 97-105 | decode_live_frame | live |
| 106-115 | decode_any_frame | live |
| 116-122 | display_available | dead, carried this round (`have_window_system`, C 4889-5919) |
| 123-132 | decode_window_system_frame | live, with dead arms inside |
| 133-143 | check_window_system | live, with dead arms inside |
| 144-157 | check_tty | live |
| 158-168 | decode_tty_frame | live |
| 169-181 | frame_get | live |
| 182-198 | get_frame_param | live |
| 199-220 | frame_inhibit_resize | live, with dead arms inside |
| 221-264 | set_menu_bar_lines | live |
| 265-285 | set_tab_bar_lines | live |
| 286-325 | framep | live |
| 326-344 | frame-live-p | live |
| 345-366 | frame-id | live |
| 367-399 | frame_set_id | live |
| 400-414 | frame_set_id_from_params | live |
| 415-450 | window-system | live |
| 451-494 | frame_redisplay_p | live, with dead arms inside |
| 495-526 | frame-windows-min-size | live |
| 527-604 | frame_windows_min_size | live |
| 605-696 | keep_ratio | live |
| 697-735 | frame_size_history_adjust | live |
| 736-751 | frame_size_history_plain | live |
| 752-824 | frame_size_history_extra | live |
| 825-1145 | adjust_frame_size | live, with dead arms inside |
| 1146-1152 | allocate_frame | live |
| 1153-1312 | make_frame | live, with dead arms inside |
| 1313-1363 | make_frame_without_minibuffer | live |
| 1364-1407 | make_minibuffer_frame | live |
| 1408-1421 | frame_next_F_name | live |
| 1422-1478 | make_initial_frame | live, with dead arms inside |
| 1479-1621 | make_terminal_frame | live, with dead arms inside |
| 1622-1643 | get_future_frame_param | live |
| 1644-1683 | tty_child_pos_param | live |
| 1684-1723 | tty_child_size_param | live |
| 1724-1735 | tty_child_frame_rect | live |
| 1736-1920 | make-terminal-frame | live, with dead arms inside |
| 1921-2096 | do_switch_frame | live, with dead arms inside |
| 2097-2125 | select-frame | live |
| 2126-2141 | handle-switch-frame | live |
| 2142-2148 | selected-frame | live |
| 2149-2160 | old-selected-frame | live |
| 2161-2179 | frame-list | live, with dead arms inside |
| 2180-2209 | frame-parent | live |
| 2210-2227 | frame_ancestor_p | live |
| 2228-2241 | frame_subsumes_p | live |
| 2242-2259 | frame-ancestor-p | live |
| 2260-2268 | root_frame | live |
| 2269-2297 | frame-root-frame | live |
| 2298-2340 | candidate_frame | live |
| 2341-2380 | next_frame | live |
| 2381-2414 | prev_frame | live |
| 2415-2440 | next-frame | live |
| 2441-2466 | previous-frame | live |
| 2467-2494 | last-nonminibuffer-frame | live |
| 2495-2596 | other_frames | live, with dead arms inside |
| 2597-3149 | delete_frame | live, with dead arms inside |
| 3150-3190 | delete-frame | live |
| 3191-3270 | frame_internal_border_part | live |
| 3271-3289 | mouse-position | live |
| 3290-3328 | mouse_position | live |
| 3329-3375 | mouse-pixel-position | live, with dead arms inside |
| 3376-3395 | frame_char_to_pixel_position | dead, carried this round (`have_window_system`, C 4889-5919) |
| 3396-3406 | frame_set_mouse_position | dead, carried this round (`have_window_system`, C 4889-5919) |
| 3407-3456 | set-mouse-position | live, with dead arms inside |
| 3457-3504 | set-mouse-pixel-position | live, with dead arms inside |
| 3505-3533 | make-frame-visible | live |
| 3534-3548 | make_frame_visible_1 | live |
| 3549-3590 | make-frame-invisible | live |
| 3591-3632 | iconify-frame | live |
| 3633-3651 | frame-visible-p | live |
| 3652-3666 | visible-frame-list | live |
| 3667-3685 | raise-frame | live |
| 3686-3701 | lower-frame | live |
| 3702-3745 | redirect-frame-focus | live |
| 3746-3755 | frame-focus | live |
| 3756-3771 | x-focus-frame | live, with dead arms inside |
| 3772-3793 | frame-after-make-frame | live |
| 3794-3810 | frames_discard_buffer | live |
| 3811-3820 | store_in_alist | live |
| 3821-3837 | frame_name_fnn_p | live |
| 3838-3870 | set_term_frame_name | live |
| 3871-4073 | store_frame_param | live |
| 4074-4082 | frame_unspecified_color | live |
| 4083-4171 | frame-parameters | live, with dead arms inside |
| 4172-4242 | frame-parameter | live, with dead arms inside |
| 4243-4344 | modify-frame-parameters | live, with dead arms inside |
| 4345-4362 | frame-char-height | live, with dead arms inside |
| 4363-4380 | frame-char-width | live, with dead arms inside |
| 4381-4400 | frame-native-width | live, with dead arms inside |
| 4401-4429 | frame-native-height | live, with dead arms inside |
| 4430-4446 | tool-bar-pixel-width | live, with dead arms inside |
| 4447-4453 | frame-text-cols | live |
| 4454-4460 | frame-text-lines | live |
| 4461-4467 | frame-total-cols | live |
| 4468-4474 | frame-total-lines | live |
| 4475-4481 | frame-text-width | live |
| 4482-4488 | frame-text-height | live |
| 4489-4495 | frame-scroll-bar-width | live |
| 4496-4502 | frame-scroll-bar-height | live |
| 4503-4509 | frame-fringe-width | live |
| 4510-4523 | frame-child-frame-border-width | live |
| 4524-4530 | frame-internal-border-width | live |
| 4531-4537 | frame-right-divider-width | live |
| 4538-4544 | frame-bottom-divider-width | live |
| 4545-4562 | check_frame_pixels | live |
| 4563-4591 | set-frame-height | live |
| 4592-4620 | set-frame-width | live |
| 4621-4643 | set-frame-size | live |
| 4644-4661 | frame-position | live |
| 4662-4696 | set-frame-position | live, with dead arms inside |
| 4697-4759 | set-frame-size-and-position-pixelwise | live |
| 4760-4774 | frame-window-state-change | live |
| 4775-4792 | set-frame-window-state-change | live |
| 4793-4925 | frame-scale-factor | live, with dead arms inside |
| 4926-5078 | frame_float | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5079-5107 | handle_frame_param | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5108-5395 | gui_set_frame_parameters_1 | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5396-5407 | gui_set_frame_parameters | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5408-5493 | gui_report_frame_params | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5494-5515 | gui_set_fullscreen | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5516-5583 | gui_set_line_spacing | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5584-5606 | gui_set_screen_gamma | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5607-5741 | gui_set_font | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5742-5798 | gui_set_font_backend | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5799-5822 | gui_set_left_fringe | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5823-5846 | gui_set_right_fringe | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5847-5860 | gui_set_border_width | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5861-5874 | gui_set_right_divider_width | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5875-5888 | gui_set_bottom_divider_width | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5889-5902 | gui_set_visibility | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5903-5908 | gui_set_autoraise | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5909-5914 | gui_set_autolower | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5915-5920 | gui_set_unsplittable | dead, carried this round (`have_window_system`, C 4889-5919) |
| 5921-5952 | gui_set_vertical_scroll_bars | dead, next round (`have_window_system`, C 5921-6970) |
| 5953-5973 | gui_set_horizontal_scroll_bars | dead, next round (`have_window_system`, C 5921-6970) |
| 5974-6007 | gui_set_scroll_bar_width | dead, next round (`have_window_system`, C 5921-6970) |
| 6008-6045 | gui_set_scroll_bar_height | dead, next round (`have_window_system`, C 5921-6970) |
| 6046-6096 | gui_set_alpha | dead, next round (`have_window_system`, C 5921-6970) |
| 6097-6125 | gui_set_alpha_background | dead, next round (`have_window_system`, C 5921-6970) |
| 6126-6139 | gui_set_borders_respect_alpha_background | dead, next round (`have_window_system`, C 5921-6970) |
| 6140-6150 | gui_set_no_special_glyphs | dead, next round (`have_window_system`, C 5921-6970) |
| 6151-6163 | gui_mouse_grabbed | dead, next round (`have_window_system`, C 5921-6970) |
| 6164-6178 | gui_redo_mouse_highlight | dead, next round (`have_window_system`, C 5921-6970) |
| 6179-6247 | validate_x_resource_name | dead, next round (`have_window_system`, C 5921-6970) |
| 6248-6321 | gui_display_get_resource | dead, next round (`have_window_system`, C 5921-6970) |
| 6322-6343 | x-get-resource | dead, next round (`have_window_system`, C 5921-6970) |
| 6344-6380 | x_get_resource_string | dead, next round (`have_window_system`, C 5921-6970) |
| 6381-6479 | gui_display_get_arg | dead, next round (`have_window_system`, C 5921-6970) |
| 6480-6490 | gui_frame_get_arg | dead, next round (`have_window_system`, C 5921-6970) |
| 6491-6513 | gui_frame_get_and_record_arg | dead, next round (`have_window_system`, C 5921-6970) |
| 6514-6551 | gui_default_parameter | dead, next round (`have_window_system`, C 5921-6970) |
| 6552-6636 | XParseGeometry | dead, next round (`have_window_system`, C 5921-6970) |
| 6637-6707 | x-parse-geometry | dead, next round (`have_window_system`, C 5921-6970) |
| 6708-6972 | gui_figure_window_size | dead, next round (`have_window_system`, C 5921-6970) |
| 6973-6987 | frame_make_pointer_invisible | live |
| 6988-7000 | frame_make_pointer_visible | live |
| 7001-7010 | frame-pointer-visible-p | live |
| 7011-7041 | mouse-position-in-root-frame | live |
| 7042-7056 | frame--set-was-invisible | live |
| 7057-7123 | reconsider-frame-fonts | live, with dead arms inside |
| 7124-7133 | free_monitors | dead, next round (`have_window_system`, C 5921-6970) |
| 7134-7196 | make_monitor_attribute_list | live, with dead arms inside |
| 7197-7207 | init_frame_once | live |
| 7208-7214 | init_frame_once_for_pdumper | live |
| 7215-7907 | syms_of_frame | live, with dead arms inside |

## Evidence (this round)

**① The frozen configuration (`build-tu`).**
```
$ cd femacs && CARGO_TARGET_DIR=/tmp/frame-gui-7f/target cargo run -q -p xtask -- build-tu src/frame.c
built src/frame.rs -> ../build/femacs/frame.o          # exit 0, 3 warnings (all pre-existing mirror warnings)
```
`build/femacs/frame.o` = **413,320 bytes**, `nm | wc -l` = **1,523**, and
`nm build/femacs/frame.o | grep -c 'gui_set\|frame_float\|handle_frame_param'` = **0**.  Every item
of C 4889-5919 sits behind `feature = "have_window_system"` (or a nested feature), which no
configuration this repository builds enables (`docs/baseline/feature-coverage.tsv`: `have_window_system`
sites 107 / files 10 / **enabled by `-`**), so the frozen object's shape is unchanged from before this
round.  This reading shows the new text is *parsed and then stripped*; it is not a compile reading of
the arm (P-86 ③⑵, and see ③ below).

**② `extract-abi --check` stops being vacuous, exactly as P-86 ⑩ predicts.**
```
$ python3 tools/extract-abi.py --check emacs/src/frame.c femacs/src/frame.rs | tail -3
- fn gui_set_visibility carries #[unsafe(no_mangle)] but the reference does not define a non-static
  function of that name (invented exported symbol)
- line count drifted 24.7%: C 5243 live (of 7907) vs Rust 6536
FAIL emacs/src/frame.c -> femacs/src/frame.rs
```
18 rows: **17 `invented exported symbol`** - one per non-`static` C 4889-5919 definition the port now
carries with C's linkage (`gui_set_frame_parameters_1`, `gui_set_frame_parameters`,
`gui_report_frame_params`, `gui_set_fullscreen`, `gui_set_line_spacing`, `gui_set_screen_gamma`,
`gui_set_font`, `gui_set_font_backend`, `gui_set_left_fringe`, `gui_set_right_fringe`,
`gui_set_border_width`, `gui_set_right_divider_width`, `gui_set_bottom_divider_width`,
`gui_set_visibility`, `gui_set_autoraise`, `gui_set_autolower`, `gui_set_unsplittable`) - plus the
**line-drift** row (the check's basis for this unit is `live`, `docs/ports/line-drift-tolerance.tsv`
196, and the port is now 6,536 lines against 5,243 live C lines).  Neither row is a defect *of this
unit*: the first is the documented consequence of carrying a dead arm in a `cond-eval-verified` file
with the C linkage the arm really has, and the second is the row the ledger asks the integrator to
move (see **Owed**).  No row says a live function is missing or unnamed.

**③ The flip is text-only here (P-86 ③⑵).**
This machine has no X11/NS/PGTK/W32/Haiku headers the mirror set needs, so a
`rustc --cfg 'feature="have_window_system"'` flip of `build/femacs/units/frame/_root.rs` is not run
and is not claimed.  What is measured instead: the file **parses** in both readings - the frozen
`build-tu` of ①, and a direct compile that skips xtask's include-graph dedupe:
```
$ cd femacs && rustc --edition 2024 --crate-type lib --emit=metadata -C panic=abort \
    -o /tmp/frame-gui-7f/parse.rmeta src/frame.rs
error: aborting due to 7494 previous errors        # E0428 / E0659 / E0252 only, no parse error
```
(The 7,494 are `defined multiple times`/`ambiguous` from the include guard xtask supplies; this same
reading is what caught two `match` arms whose bodies were unbraced `return` statements before they
reached the frozen build.)  A flip would also fail for a reason the mirror states itself:
`include/frame.rs` 250-268 records that a window-system configuration "fails to compile at the first
accessor" because the flag-run numbering for those arms is not written out yet.

**④ The other gates, one line each.**
```
$ python3 tools/check-dead-file-coverage.py
info docs/ports/frame.md: 170 range row(s) cover src/frame.c (7907 lines; 0 blank line(s) between blocks need no row)
info docs/ports/frame.md: 170 range row(s) cover src/frame.h (1956 lines; 0 blank line(s) between blocks need no row)
4 ledger(s) bound (a C range table), 0 failure(s)
   # the bound-ledger count moves as other lanes add `## C line ranges` sections (it was 5 on a
   # second run of the same command); the two rows naming this ledger are the ones quoted here.
   # note: port-map pairs src/frame.c with femacs/src/frame.rs (439) and src/frame.h with
   # femacs/src/frame.rs's include mirror (440); the ledger rule resolves both to this file, so
   # this table - whose last row reaches C 7,907 - is also what the gate reads for the 1,956-line
   # header (docs/ports/include-frame.md is not consulted by this rule).  See Owed 4.

$ python3 tools/check-port-lines.py
92 file(s) under ten percent, 243,405 reference line(s); 95 ratchet row(s), 0 new, 3 stale
   # this unit's row is not among the three stale ones; the tree moves under this reading because
   # other lanes are landing work in the same checkout (see below)

$ python3 tools/check-citations.py
check-citations: 675 file(s) scanned, 1 drift, 0 range, 456 external, 33 allowed, 0 stale
   # the one drift finding is femacs/src/androidselect.rs (another lane); frame.rs contributes
   # none: `python3 tools/check-citations.py | grep -c femacs/src/frame.rs` = 0

$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures

$ python3 tools/check-cond-eval.py src/frame.c
105 conditions, 0 unknown, 0 disagreements [VERIFIED]

$ python3 tools/check-globals.py            # no new DEFVAR_* carrier in this arm
check-globals: 284 units, 0 DEFVAR_* variables without a definition in their own unit

$ python3 tools/check-undef.py              # frame.o unchanged; the 23 NEW rows are all xdisp.o
check-undef: 23 NEW undefined reference(s) that nothing can satisfy   # every one "referenced by xdisp.o"
```
The stale rows, the `check-citations.py` finding and `check-undef.py`'s NEW rows are read in a
**shared worktree** (`git status --short` also shows `femacs/src/decompress.rs` and
`femacs/src/xdisp.rs` modified by other lanes while this round ran, and the counts moved between two
runs of the same command); none of them names `frame.rs`, and `check-port-lines.py`'s **`0 new`** is
the reading that concerns this unit.

**⑤ Line count.**  C 4889-5919 is 1,031 C lines (all of them carried, comments included); the port
grew 5,251 -> 6,536 lines (+1,285, i.e. 1.25 port lines per C line - +24.5% against the unit's own
previous size).  The excess over the C text is the per-item `#[cfg]` attribute (the run's condition
sits on every item, P-86 ②), the 26 use-site items (2 macros, 8 constants, the `fullscreen_type`
alias, 14 prototypes, the `frame_parm_handler` typedef), the `match`/`if`-expression shapes Rust
needs for C's `switch` and `?:`, and the file-scope statics `frame_float`'s function statics become.
`extract-abi --check`'s line row measures the same change against the **live** C baseline (5,243)
and reports 24.7%; against the file's 7,907 total lines the port is 17.3% short, which is the number
the moved tolerance row needs (Owed 1).

## Deviations (this round)

**The `#if`s of C 4889-5919, one by one** (P-86 ②: each arm is carried and each condition is a
lower-case feature; `HAVE_WINDOW_SYSTEM` is `undef` at `darwin-arm64.macros.tsv` 1677,
`HAVE_X_WINDOWS` at 1719, `USE_X_TOOLKIT` at 3872):

| C `#if` | verdict | arm the port writes |
| --- | --- | --- |
| 4889 `#ifdef HAVE_WINDOW_SYSTEM` (closes at 6971) | false (1677) | no single counterpart: the condition sits on **every item** of the run (P-86 ②) as `#[cfg (feature = "have_window_system")]`, and the run's start/end are comments |
| 4935-4937 `static` locals of `frame_float` | n/a (not an `#if`) | moved to file scope (`static mut` with C's names, zero-initialised) - Rust has no function-scope `static` |
| 5133-5135 `#ifdef HAVE_X_WINDOWS` (`icon_left_no_change`/`icon_top_no_change` declarations) | false (1719) | `#[cfg (feature = "have_x_windows")] let …` statement-attributes |
| 5270-5272 / 5279-5281 `#ifdef HAVE_X_WINDOWS` (the two `…_no_change = 1` assignments) | false | `#[cfg (feature = "have_x_windows")] { … }` statement blocks |
| 5387-5391 `#ifdef HAVE_X_WINDOWS` (`x_wm_set_icon_position`) | false | `#[cfg (feature = "have_x_windows")] { … }`; the prototype is declared at the use site (`frame.h` 1886, the mirror lacks it) |
| 31-33 `#ifdef HAVE_WINDOW_SYSTEM` + `#include TERM_HEADER` | false | six **gated `include!`s**, one per platform mirror, behind that platform's own feature (`have_x_windows`, `have_ns`, `have_pgtk`, `have_ntgui`, `have_haiku`, `have_android`) - C picks the one header at `configure.ac` 7180 |
| 44-46 `#ifdef HAVE_WINDOW_SYSTEM` + `#include "fontset.h"` | false | `#[cfg (feature = "have_window_system")] include!("include/fontset.rs")` |
| 52-54 `#ifdef USE_X_TOOLKIT` + `#include "widget.h"` | false (3872) | `#[cfg (feature = "use_x_toolkit")] include!("include/widget.rs")` |
| 5464-5472 `#ifdef HAVE_X_WINDOWS` (+ nested 5465-5468 `#ifdef USE_X_TOOLKIT`) | false | outer `#[cfg (feature = "have_x_windows")] { … }`; inner: `#[cfg (feature = "use_x_toolkit")] if … { w = … }` with the `#[cfg (not (feature = "use_x_toolkit"))] { w = … }` arm (C's guard is a statement guard, so the port keeps it as one) |

**Deviations in the carried text, each with its reason:**

1. **Names with no mirror, carried at the use site** (P-72 rule 3).  Macros:
   `WINDOW_HANDLE_UINTPTR` (`dispextern.h` 189 and the six `*gui.h`); `PRIuMAX` (the
   `dispnew.rs` 69-72 reading, `"ju"` on this target).  Constants: `fullscreen_type` and its five
   members (`frame.h` 49-60 - itself inside `#ifdef HAVE_WINDOW_SYSTEM`; the `HAVE_NTGUI`
   `FULLSCREEN_WAIT` is that arm's own and is not carried); `XNegative`/`YNegative` (the six
   `*gui.h`: 0x0010/0x0020); `NorthWestGravity` (the five non-X GUI headers define it as `1`;
   C's X11 build takes it from `<X11/Xlib.h>`, which this tree does not have).  Functions, with the
   header prototype on each `/// C:` line: `make_formatted_string` (lisp.h 4542),
   `check_int_nonnegative` (lisp.h 607), `redraw_frame` (dispextern.h 3896), `clear_face_cache`
   (3802), `clear_current_matrices` (3907), `recompute_basic_faces` (3823), `clear_image_cache`
   (3729), `share_image_cache` (3819), `free_all_realized_faces` (3833),
   `font_open_by_name` (font.h 883), `font_spec_from_name` (865), `font_match_p` (860),
   `font_update_drivers` (907), `x_wm_set_icon_position` (frame.h 1886).  Each belongs in the
   mirror that declares it; this unit's copies must be deleted when those land (the `search.rs`
   precedent the `## Decisions` section already records for the ~40 geometry macros).
2. **The platform `FRAME_*` macros come from the gated mirrors**, not from hand-written copies:
   `FRAME_NATIVE_WINDOW`/`FRAME_OUTPUT_DATA`/`FRAME_FONT`/`FRAME_FONTSET`/`FRAME_DISPLAY_INFO`/
   `FRAME_X_OUTPUT` are the ones `include/xterm.rs`/`nsterm.rs`/`pgtkterm.rs`/`w32term.rs`/
   `haikuterm.rs`/`androidterm.rs` already define, and the six `have_window_system`-and-platform
   `include!`s of row 31-33 above are what makes them visible.  This is new to *this* unit's include
   graph; it is gated off in every configuration this repository builds (evidence ①: the object and
   its symbol count are unchanged).
3. **`frame_float`'s function statics are file-scope statics** (`parent_width`, `parent_height`,
   `parent_left`, `parent_top`, `outer_minus_text_width`, `outer_minus_text_height`): Rust has no
   function-scope `static`.  C's storage duration is static and zero-initialised; the port's
   initialisers say so, and the names are C's.
4. **`want_fullscreen` has no storage in the mirror** (`include/frame.rs` 250-268: the
   window-system flag run's numbering is measured (`x11` 316:0-3, `ns_cocoa` 276:0-3,
   `docs/baseline/frame-layout.tsv` 219/374) but not written out, and the mirror's reader takes
   `have_window_system` as uncompilable on purpose).  The arm therefore keeps C's own write
   (`(*f).want_fullscreen = FULLSCREEN_…`) rather than inventing an offset, and a GUI flip fails
   there - the mirror's documented behaviour, not a silent wrong read.  Closing it is mirror work
   (Owed 3).
5. **`FRAME_OUTPUT_DATA (f)->parent_desc`** (C 5480/5483): no platform macro names that member, so
   the arm reaches it through one gated binding per `TERM_HEADER` arm (`x_output`/`ns_output`/
   `pgtk_output`/`w32_output`/`haiku_output`/`android_output`); `FRAME_DISPLAY_INFO (f)->root_window`
   and `->name_list_element` go through the platform mirror's own macro (C's `Display_Info` is the
   platform's display record, `include/xterm.rs` 318).
6. **`FRAME_IMAGE_CACHE (f)->…`** (C 5685-5705): the mirror keeps the `frame` member as
   `*mut c_void` (P-72 rule 3), so the arm casts to `*mut image_cache` (the gated record in
   `include/dispextern.rs`), and the one write to the macro's lvalue goes through the struct member
   (`(*f).image_cache = …`), as the mirror's own note at 937-945 requires.
7. **Small C-to-Rust shapes inside the carried bodies**, none changing a value or an order:
   `switch` -> `match` with a `_ => emacs_abort ()` arm; C's `?:` -> Rust `if`-expressions;
   `top = left = Qunbound;` -> two statements; `int`/`unsigned` mixes get explicit `as`
   casts (`XFIXNUM`/`XFIXNAT` are `EMACS_INT` in the port); `do_pending_window_change (0)` and
   `adjust_frame_size (…, 3, 0, …)` -> the port's `bool` parameters (`false`), the declarations the
   live part of this unit already uses; `UNINIT` -> zero-initialised (`Qnil ()`), PORTING 1.5;
   the port's macro-call spelling for C's macros (`XSETFRAME!`, `XSETINT!`, `SAVE_*`, `eabs!`,
   `countof!`, `SET_FRAME_GARBAGED!`, `FRAME_*!`) - the token check ignores the `!`.
8. **`handle_frame_param`'s handler read**: the mirror keeps
   `redisplay_interface.frame_parm_handlers` as `*mut c_void` (P-80's untyped-pointer rule), so the
   arm reads the slot at C's index and casts it to the `frame_parm_handler` typedef spelled at the
   use site (`dispextern.h` 3063).

## Owed by the integrator (this round; nothing here is claimed as done)

1. **`docs/ports/line-drift-tolerance.tsv`**: the `frame` row (line 196) asks for the **`live`**
   basis with tolerance `0.05`, which is the right reading for a unit that *records* its dead arms;
   this round's change makes the unit carry C 4889-5919, so the row must move to **`total`** basis.
   Measured: C 7,907 total vs the port's 6,536 = **17.3%** deficit (the arm is carried both-arms, and
   its per-item `#[cfg]`s and use-site declarations cost more lines than C's text), so the row wants
   `frame 0.18 total` with this section's reason.  Until then `extract-abi --check` reports the
   line-drift row of evidence ②.
2. **`tools/extract-abi.py`'s invented-exported-symbol rule** has no baseline mechanism
   (`PORT_ONLY_EXPORTS` is the only allowance, status 3.113), so the 17 rows of evidence ② stay red
   for every future `--all` run of a `cond-eval-verified` file whose dead arm is carried with C
   linkage.  The 17 names are listed in evidence ②; the fix is a tool/integrator decision (an
   explicit allowance with the P-86 reason, or a rule that reads the `#[cfg]` gate), **not** dropping
   `#[unsafe(no_mangle)]` - C defines all 17 non-`static`, and the GUI arm needs those symbols.
3. **Mirror work this arm needs before any GUI configuration can compile**: the window-system
   `want_fullscreen` numbering and flag run (`include/frame.rs` 250-268; the readings are already in
   `docs/baseline/frame-layout.tsv`), and the 14 function prototypes / 2 macros / 5 constants of
   Deviations 1 into the headers that declare them.  `include/frame.rs`'s own note names
   `struct frame`'s window-system arms as unmeasured; this unit does not guess them.
4. **`docs/ports/frame.md` is now a P-86 ② ledger** (it carries `## C line ranges`), so
   `check-dead-file-coverage.py` binds it.  The bind is **two targets, not one**: `docs/port-map.md`
   439 pairs `src/frame.c` with `femacs/src/frame.rs` and 440 pairs `src/frame.h` with that unit's
   include mirror, and the gate's ledger rule resolves both to this file (the 1,956-line header has
   no `src-frame.md` of its own; `docs/ports/include-frame.md` is not what the rule consults).
   Today the one table satisfies both because its last row reaches C 7,907; if the include mirror
   ever needs its own ranges, the fix is a `src-frame.md` for it or a rule change - the header's
   coverage should not silently ride on the unit's table.  (The blank lines the table does not cover
   are the file's own empty lines; this run leaves none uncovered.)

# ----------------------------------------------------------------------
## Decisions

**STATE: COMPLETE for the frozen configuration's checklist; the GUI arm is now being carried.**
66/66 subrs, 57/57 functions, the `frame_parm_table` type, all 8 file-scope objects and the
51-entry `frame_parms` table are ported, and every box above is ticked.  What the boxes cannot see
is the dead region this file also has: **C 4889-5919 (the first half of the `#ifdef
HAVE_WINDOW_SYSTEM` arm) is now carried in full** under `feature = "have_window_system"` and its
nested features - see `## C line ranges`, `## Evidence` and `## Deviations` above, which replace
P-08's "recorded, not transcribed" for this run and turn `extract-abi --check` non-vacuous (evidence
②).  C 5921-6970 is the next round.  The gates below are the pre-round readings, kept as the
record of the live checklist; the current readings are in `## Evidence`.  The gates, with the last
line each produced:

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-frame cargo run -q -p xtask -- build-tu src/frame.c
built src/frame.rs -> ../build/femacs/frame.o

$ python3 tools/extract-abi.py --check emacs/src/frame.c femacs/src/frame.rs
OK emacs/src/frame.c -> femacs/src/frame.rs

$ python3 tools/check-globals.py
check-globals: 265 units, 0 DEFVAR_* variables without a definition in their own unit

$ python3 tools/check-undef.py
check-undef: 515 unit object(s), 151 reference object(s)/archive(s): 151 pending, 0 new
  (this unit's dependencies on unported units were added to
  `docs/baseline/undef-pending.txt` by hand; a later `--write-pending` from another lane pruned
  the entries whose owners' reference objects have meanwhile landed, and all 39 of frame.o's
  orphans are in the ratchet)

$ python3 tools/check-linkage.py
check-linkage: 3 unit object(s) compared against the non-static objects of the C sources the frozen build links, 0 with 0 missing symbol(s)   # src/frame is one of the three (its 6 non-static objects: _Vframe_list, _frame_garbaged, _frame_default_tab_bar_height, _frame_default_tool_bar_height, _old_selected_frame, _selected_frame)

$ cd femacs && rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort -C codegen-units=1 -C opt-level=0 --cfg enable_checking -o /tmp/frame-ec.o ../build/femacs/units/frame/_root.rs
exit 0   (two warnings, both from the mirrors: lisp.rs's `/// C:` doc comment on an extern block and frame.rs's doubled `#[inline(always)]`)

$ python3 tools/check-declarations.py
8 declaration sites (0 forbidden, 8 allowed literals), 0 failures

$ python3 tools/check-verbatim.py
6 trees, 4607 files, 0 failures

$ grep -c '^\- \[ \]' docs/ports/frame.md
0
```

### The dead arms (P-08: recorded here, not transcribed)

`python3 tools/check-cond-eval.py src/frame.c` = `105 conditions, 0 unknown, 0 disagreements
[VERIFIED]`.  The frozen configuration's undefined macros are (macros.tsv): `HAVE_WINDOW_SYSTEM`
1677, `HAVE_X_WINDOWS` 1719, `HAVE_ANDROID` 1239, `MSDOS` 2848, `USE_X_TOOLKIT` 3872,
`USE_MOTIF` 3864, `USE_GTK` 3861, `HAVE_NS` 1480, `HAVE_NTGUI` 1481, `NS_IMPL_COCOA` 2995,
`HAVE_PGTK` 1488, `HAVE_HAIKU` 1387, `HAVE_GPM` 1381, `HAVE_TEXT_CONVERSION` 1644, `WINDOWSNT`
4009, plus `HAVE_EXT_TOOL_BAR`, `FRAME_TOOLBAR_WIDTH` and `HAVE_XCB`, which are not in the table
at all — the evaluator answers `defined (NAME)` false for every one of them, exactly as clang
does for the frozen build.  Arms the port therefore does not carry:

  * **`#ifdef HAVE_WINDOW_SYSTEM`** — C 31-33 (`#include TERM_HEADER`), 44-46 (`fontset.h`),
    81-83 (`gui_report_frame_params`'s prototype), 115-121 (`display_available`), 128-130
    (`decode_window_system_frame`'s `return f;`), 136-139 (`check_window_system`'s
    `if (window_system_available (f)) return;`), 1187-1213 (`make_frame`'s flag run),
    1452-1455 (`make_initial_frame`), 1593-1596 (`make_terminal_frame`), 2167-2175
    (`Fframe_list`'s X arm and its `#else` counterpart is the live one), 2535-2547 (a comment
    block inside `other_frames`'s `#ifdef HAVE_XCB`), 2885-2888 (`delete_frame`),
    3371-3405 (the X mouse movers), 3764-3768 (`Fx_focus_frame`), 4156-4160 (`Fframe_parameters`),
    4189-4208 (`Fframe_parameter`'s window-system arms), 4261-4265, 4352-4358, 4371-4377,
    4393-4397, 4422-4426 (`Fframe_char_height`/`_width`/`native_width`/`native_height`),
    4438-4443 (`#ifdef FRAME_TOOLBAR_WIDTH`), 4680-4683 (`Fset_frame_position`), 4880-4886
    (the two `frame_parms` entries), 7055-7112 and 7120-7187 (the pointer and `Display_Info`
    machinery), 7251-7253, 7285-7287, 7316-7322, 7396-7426, 7455-7462, 7551-7552, 7685-7691,
    7877-7894 — and **the whole GUI frame-geometry implementation at C 4889-6970**.
  * **`#ifdef MSDOS`** — C 48-51, 1030-1043, 1568-1576, 1762-1764, 1782-1788, 3436-3442,
    3481-3487 (each of the last two keeps its `#else` text, which is live).
  * **`#ifdef HAVE_X_WINDOWS`** — C 482-491 (keeps the `#ifndef` arm's
    `return FRAME_VISIBLE_P (f);`), 943-945 (`USE_MOTIF` too), 1953-1979, 2607-2609, 2622-2626,
    2716-2719, 2852-2866, 2928-2933, 2950-2957, 7766-7768.
  * **`#ifndef HAVE_ANDROID`** — C 1477/1722 open *live* regions (HAVE_ANDROID is undef); C
    1755-1757, 7551's neighbour `#ifdef HAVE_ANDROID`, 2868-2877 and 7551-7552 are dead.
  * **`#ifdef HAVE_GPM`** — C 3446-3451 and 3491-3496 keep the `#else` text (`(void) xval;`).
  * **`#if 0`** — C 2558-2565 (inside `other_frames`) and 7828-7831 (`defsubr` arms).
  * **`#ifdef HAVE_TEXT_CONVERSION`** — C 1218-1226, 2963-2966.
  * **`#ifdef HAVE_EXT_TOOL_BAR`** — C 75-77, so the live arm is the `#else`'s
    `int frame_default_tool_bar_height;` (C 78), which is what the ledger's file-scope object
    is.

### Deviations (each one is also in `docs/ports/macro-body-notes.tsv` where it is a body token)

1. **Chained assignments are split.**  C's `A = B = C = v;` is one statement whose value is
   `v`; Rust's assignment is not an expression.  Five sites keep C's effects in C's order as
   two statements: `set_menu_bar_lines` (C 234, 238-239), `set_tab_bar_lines` (C 278),
   `make_frame` (C 1252-1256, four chains, each expanded to set every place to the same value)
   and `x = y = Qnil;` (C 3298, 3347).  `set-frame-window-state-change` (C 4790) keeps C's
   assign-then-test inside a block expression.
2. **`check_window_system` is `-> !`.**  C declares it `void`, but in this configuration its
   only live tail is `error (…)` (its `if (window_system_available (f)) return;` is in the
   dead `HAVE_WINDOW_SYSTEM` arm, C 136-139), so the port returns `!`; that is what lets
   `decode_window_system_frame` (C 123-131) keep C's live text, whose only `return f;` is in
   its own dead arm.
3. **`l-value` header macros are defined at the use site.**  `frame.h`'s accessors are macros
   in C; `include/frame.rs` mirrors the readers as functions and packs the `bool_bf` run into
   `bits1`/`bits2` (PORTING.md 1.3), so every macro this file *writes* to — `SET_FRAME_VISIBLE`,
   `SET_FRAME_GARBAGED`, `FRAME_COLS`, `FRAME_LINES`, `FRAME_TOTAL_COLS`, `FRAME_TOTAL_LINES`,
   `FRAME_TEXT_WIDTH`, `FRAME_TEXT_HEIGHT`, `FRAME_PIXEL_WIDTH`, `FRAME_PIXEL_HEIGHT`,
   `FRAME_MENU_BAR_LINES`/`_HEIGHT`, `FRAME_TAB_BAR_LINES`/`_HEIGHT`,
   `FRAME_FOREGROUND_PIXEL`/`FRAME_BACKGROUND_PIXEL`, `FRAME_NO_SPECIAL_GLYPHS`,
   `FRAME_WINDOW_STATE_CHANGE`, the geometry conversions (`FRAME_TEXT_TO_PIXEL_*`,
   `FRAME_PIXEL_TO_TEXT_*`, `FRAME_INNER_*`, `FRAME_PIXEL_WIDTH_TO_TEXT_COLS`, …) — plus the
   field-name readers/writers for that flag run (`visible!`, `resized_p!`, `wants_modeline!`,
   `mini!`, `output_method!`, …) and `WINDOW_FRAME!`/`WINDOW_XFRAME` are defined in this unit
   under C's own name, so the body keeps C's token and the write goes through the mirror's
   mask.  `NAME! (…)` is the Rust macro call; the token check ignores the `!`.
4. **Two modules exist for `E0428` reasons.**  `include/frame.rs` declares `selected_frame`,
   `old_selected_frame`, `frame_garbaged`, `frame_default_tab_bar_height`,
   `frame_default_tool_bar_height`, `frame_next_id` and `Vframe_list` at the crate root, and
   `frame_make_pointer_invisible` too, so the definitions live in `frame_globals` and
   `frame_pointer_fns`, which are glob-imported (PORTING.md P-21's mechanism from the other
   side; the terminal.rs/term.rs `*_globals` shape).
5. **`(void) x;` is `let _ = x;`** (C 3449-3450, 3494-3495, 4684-4685; the dired.rs shape) and
   C's `1`/`0` passed to a Rust `bool` parameter/field is `true`/`false` (C 1983's
   `resize_mini_window (…, 1)`, `make_frame`'s flag writes).
6. **`switch` → `match`** in `Fframep` (C 302-323) with C's arm order and `default:` as `_ =>`;
   **`goto next_name`** in `frame_next_F_name` (C 1418) is a labelled `loop`/`continue`
   (PORTING.md 2.2) because the jump target is inside the `FOR_EACH_FRAME` body.
7. **Token-pasting macros.**  `SYMBOL_INDEX (Qx)` in the `frame_parms` table is the pasted
   `iQx` constant (the P-17 generated names), and `KVAR (kb, Vdefault_minibuffer_frame)` is
   `(*kb).Vdefault_minibuffer_frame_` (keyboard.rs's field; `excluded-macros.txt`'s KVAR row).
8. **`UNINIT`** (`make_frame`'s `mw`, `delete_frame`'s `frame1`) is zero-initialized
   (PORTING.md 1.5), and `PRIdMAX` is written as the `intmax_t` conversion it expands to
   (`c"F%jd"`, the print.rs row).
9. **`ckd_mul (&pixel_size, sz, item_size)`** (C 4556) is `sz.overflowing_mul (item_size)` with
   the wrapped value stored and the `int`-fit check made explicit, because C's `ckd_mul`
   multiplies in the *destination's* type (`docs/ports/token-aliases.tsv`'s ckd_mul family,
   PORTING.md P-15).
10. **`attributes: const`** on `Fframe_windows_min_size` (C 498) has no counterpart in the
    mirror's `DEFUN!` and only affects optimization, so it is dropped (PORTING.md P-25's
    class); `--check` compares the `noreturn` flag only.
11. **Line drift.**  The port is **5,302 lines against C's 7,907**, i.e. 1.13 % against the
    **5,243 lines this configuration compiles**; the row
    `frame	0.05	…	live` in `docs/ports/line-drift-tolerance.tsv` selects the `live` basis
    (the dead region above is recorded here, not transcribed).  The port is *longer* than the
    live text by the `// femacs:` note, the `#![allow]`/include blocks, the use-site
    declarations (one `/// C:` line each), the 51-entry `frame_parms` table and the 26
    `DEFVAR_LISP`/`DEFVAR_BOOL` carriers, and *shorter* by the 66 docstrings.

### D7 call-relationship check

Both sides' "identifier followed by `(`" sets (comments and literals stripped, the C side
restricted to the live arms, macro `!` ignored) were subtracted.  **C-only: 8**, every one a
recorded spelling: `ckd_mul` (the alias family's canonical name for `overflowing_mul`),
`kvar` and `symbol_index` (the two token-pasting macros, `excluded-macros.txt`), `switch`,
`for`, `while`, `else` (the keywords `match` and the labelled/explicit loops replace) and
`defined` (a preprocessor directive inside the live stream, not a call).  **Rust-only: 140**,
all of them counterparts: the `Q*` Lisp-symbol *functions* (`P-17`: C's `Qname` is a constant,
the port's is `Qname ()`), the flag-mask macros and their `set_frame_flag1!`/`set_frame_flag2!`/
`set_output_method!` helpers (§3 above, C writes `f->field` or the `FRAME_*` macro), the hook
values the port calls through `if let Some (hook) = … { hook (…) }` where C writes
`(*FRAME_TERMINAL (f)->hook) (f)` (the field name is not followed by `(` in C's text), and
Rust/mirror spellings (`let`, `unsafe`, `include`, `as_ptr`, `is_null`, `is_some`, `null_mut`,
`addr_of`, `offset_of`, `zeroed`, `lisp_h_Qnil`, `check_type`, `is_daemon`, `fset_redisplay`,
`Some`).  No C callee is missing and the port adds no callee C does not name.

### Use-site declarations (headers without a mirror; each keeps C's prototype in `/// C:`)

`is_tty_frame`, `is_tty_child_frame`, `is_tty_root_frame`, `change_frame_size`,
`set_window_buffer`, `make_window`, `other_buffer_safely`, `get_minibuffer`,
`adjust_frame_glyphs`, `calculate_costs`, `resize_frame_windows`, `create_tty_output`,
`init_tty`, `get_tty_size`, `max_child_z_order`, `init_frame_faces`, `init_initial_terminal`,
`decode_live_terminal`, `store_in_alist` (this file's own; declared before its definition),
`window_box`, `pixel_to_glyph_coords`, `tty_raise_lower_frame`, `update_face_from_frame_parameter`,
`tty_color_name`, `mru_rooted_frame`, `free_glyphs`, `delete_all_child_windows`,
`delq_no_quit`, `free_font_driver_list`, `swallow_events`, `not_single_kboard_state`,
`resize_mini_window`, `move_minibuffers_onto_frame`, `fileno`, `hashtest_eq`,
`unspecified_fg`/`unspecified_bg`, and the `frame_parm_table`/`internal_border_part` types and
the `FACE_TTY_DEFAULT_*` constants (dispextern.h 1932-1940).

### Integrator note (mirror gaps this unit reports and does not touch)

1. **`include/frame.rs` (frame.h's mirror) is read-only WIP**: it carries the readers as
   functions and the flag run as `bits1`/`bits2` masks but no l-value macros, which is why
   §3 above defines ~40 macros at the use site.  When the mirror grows them, this unit's
   copies must be deleted in the same commit (the search.rs precedent).
2. **`include/frame.rs`'s crate-root declarations** of the seven file-scope objects and of
   `frame_make_pointer_invisible` are why `frame_globals`/`frame_pointer_fns` exist; moving
   those declarations into a `pub mod … ; use …::*;` (P-21) would let the definitions return
   to the crate root.
3. **`include/window.rs`** says `WINDOW_XFRAME` (window.h 1358) "lands with frame.rs"; it is
   defined here at the use site, and `fset_redisplay`'s mirror declaration takes
   `*mut c_void` where frame.h's prototype takes `struct frame *` (the port casts).
4. **Missing mirrors**: `systty.h`/`<stdio.h>`'s `fileno`, `frame.h`'s `CHECK_FRAME`/
   `CHECK_LIVE_FRAME`, `SET_FRAME_*` writers, `FRAME_INNER_*`, the pixel/text conversions,
   `FRAME_TEXT_COLS_TO_PIXEL_WIDTH`/`FRAME_PIXEL_HEIGHT_TO_TEXT_LINES`,
   `WINDOW_FRAME`/`WINDOW_XFRAME`, `lisp.h`'s `hashtest_eq`, `xfaces.c`'s `unspecified_fg`/
   `unspecified_bg` arrays, `dispextern.h`'s `internal_border_part` enum and
   `FACE_TTY_DEFAULT_*`.
5. **`docs/ports/line-drift-tolerance.tsv`** now carries the `frame … live` row;
   `docs/baseline/undef-pending.txt` gained this unit's 19 dependencies (dispnew.c, window.c,
   fns.c, keyboard.c, xfaces.c — all in `frozen-build-sources.txt`, none ported yet), of which
   two (`_is_tty_child_frame`, `_mru_rooted_frame`) a later `--write-pending` from another lane
   pruned once the reference objects defining them were built.
6. An intermediate run of `check-undef.py`, taken while this unit's `frame.o` was fresh but
   `font.o` (another lane's in-flight unit) still referenced nine names, reported `9 NEW`
   (`_Vdisplay_pixels_per_inch`, the three `_Vface_*`, `_assoc_no_quit`,
   `_face_at_buffer_position`, `_face_at_string_position`, `_lookup_basic_face`,
   `_recompute_basic_faces`) — none referenced by `frame.o`, and they are gone from the
   final measurement above; this unit did not absorb them into the ratchet.
7. **`FRAME_FACE_CACHE` / `FACE_FROM_ID` / `FACE_FROM_ID_OR_NULL` landed (3.230), so this header's
   mirror is complete.** 3.211's ruling deferred them until `struct face` / `struct face_cache`
   existed in a shared mirror; `include/dispextern.rs` now carries both behind
   `dispextern_face_records`, so the three are in `include/frame.rs` as a **module**
   (`frame_face_accessors`) this mirror does not glob-import, behind the same feature: C's text is
   present for the gates while the units keep their live per-unit copies (`macro_rules!
   FRAME_FACE_CACHE` in alloc.rs 3467 / font.rs 545 / xfaces.rs 445 / xdisp.rs 1101 /
   window.rs 920, `fn` forms in term.rs 364-377).  Evidence: `extract-abi --check frame.h` ->
   **OK** (was 3 rows); `build-tu src/{frame,term,alloc,xfaces,category,cm,data}.c` all built;
   flip build (`rustc --cfg 'feature="dispextern_face_records"' build/femacs/units/<stem>/_root.rs`)
   exit 0 for frame, category, cm, data, doc, eval, ccl.  **Known gap**: the flip arm is `E0428`
   for `face`/`face_cache` in term.rs, alloc.rs and xfaces.rs (their typed copies are
   unconditional), plus the two in-flight units; closing it is one
   `#[cfg (not (feature = "dispextern_face_records"))]` per copy, deferred until `xdisp.rs` and
   `font.rs` land so it can be done in one move.
