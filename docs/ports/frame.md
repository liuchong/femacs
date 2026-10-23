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
(`python3 tools/extract-abi.py emacs/src/frame.c`).  This unit additionally **carries the whole
`#ifdef HAVE_WINDOW_SYSTEM` arm, C 4889-6970**, which P-08 had left recorded only: the first half
(C 4889-5919, through `gui_set_unsplittable`) landed in the round before this one and the second
half (C 5921-6970, from `gui_set_vertical_scroll_bars` to `gui_figure_window_size`) in this one;
C 6971 is the `#endif` and closes the run.  One row per C block, in C's order; the `arm` cell says
what the port does with it, and `carried this round` marks this round's half.  This table is the
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
| 116-122 | display_available | dead, carried (`have_window_system`, C 4889-5919) |
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
| 3376-3395 | frame_char_to_pixel_position | dead, carried (`have_window_system`, C 4889-5919) |
| 3396-3406 | frame_set_mouse_position | dead, carried (`have_window_system`, C 4889-5919) |
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
| 4926-5078 | frame_float | dead, carried (`have_window_system`, C 4889-5919) |
| 5079-5107 | handle_frame_param | dead, carried (`have_window_system`, C 4889-5919) |
| 5108-5395 | gui_set_frame_parameters_1 | dead, carried (`have_window_system`, C 4889-5919) |
| 5396-5407 | gui_set_frame_parameters | dead, carried (`have_window_system`, C 4889-5919) |
| 5408-5493 | gui_report_frame_params | dead, carried (`have_window_system`, C 4889-5919) |
| 5494-5515 | gui_set_fullscreen | dead, carried (`have_window_system`, C 4889-5919) |
| 5516-5583 | gui_set_line_spacing | dead, carried (`have_window_system`, C 4889-5919) |
| 5584-5606 | gui_set_screen_gamma | dead, carried (`have_window_system`, C 4889-5919) |
| 5607-5741 | gui_set_font | dead, carried (`have_window_system`, C 4889-5919) |
| 5742-5798 | gui_set_font_backend | dead, carried (`have_window_system`, C 4889-5919) |
| 5799-5822 | gui_set_left_fringe | dead, carried (`have_window_system`, C 4889-5919) |
| 5823-5846 | gui_set_right_fringe | dead, carried (`have_window_system`, C 4889-5919) |
| 5847-5860 | gui_set_border_width | dead, carried (`have_window_system`, C 4889-5919) |
| 5861-5874 | gui_set_right_divider_width | dead, carried (`have_window_system`, C 4889-5919) |
| 5875-5888 | gui_set_bottom_divider_width | dead, carried (`have_window_system`, C 4889-5919) |
| 5889-5902 | gui_set_visibility | dead, carried (`have_window_system`, C 4889-5919) |
| 5903-5908 | gui_set_autoraise | dead, carried (`have_window_system`, C 4889-5919) |
| 5909-5914 | gui_set_autolower | dead, carried (`have_window_system`, C 4889-5919) |
| 5915-5920 | gui_set_unsplittable | dead, carried (`have_window_system`, C 4889-5919) |
| 5921-5952 | gui_set_vertical_scroll_bars | dead, carried this round (`have_window_system`, C 5921-6970) |
| 5953-5973 | gui_set_horizontal_scroll_bars | dead, carried this round (`have_window_system`, C 5921-6970) |
| 5974-6007 | gui_set_scroll_bar_width | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6008-6045 | gui_set_scroll_bar_height | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6046-6096 | gui_set_alpha | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6097-6125 | gui_set_alpha_background | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6126-6139 | gui_set_borders_respect_alpha_background | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6140-6150 | gui_set_no_special_glyphs | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6151-6163 | gui_mouse_grabbed | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6164-6178 | gui_redo_mouse_highlight | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6179-6247 | validate_x_resource_name | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6248-6321 | gui_display_get_resource | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6322-6343 | x-get-resource | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6344-6380 | x_get_resource_string | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6381-6479 | gui_display_get_arg | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6480-6490 | gui_frame_get_arg | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6491-6513 | gui_frame_get_and_record_arg | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6514-6551 | gui_default_parameter | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6552-6636 | XParseGeometry | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6637-6707 | x-parse-geometry | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6708-6972 | gui_figure_window_size | dead, carried this round (`have_window_system`, C 5921-6970) |
| 6973-6987 | frame_make_pointer_invisible | live |
| 6988-7000 | frame_make_pointer_visible | live |
| 7001-7010 | frame-pointer-visible-p | live |
| 7011-7041 | mouse-position-in-root-frame | live |
| 7042-7056 | frame--set-was-invisible | live |
| 7057-7123 | reconsider-frame-fonts | live, with dead arms inside |
| 7124-7133 | free_monitors | dead, recorded (P-08; a separate `#ifdef HAVE_WINDOW_SYSTEM` block whose own `# if (defined USE_GTK \|\| defined HAVE_PGTK \|\| defined HAVE_NS \|\| defined HAVE_XINERAMA \|\| defined HAVE_XRANDR)` sits at C 7120-7123 - outside the C 5921-6970 range this round carries) |
| 7134-7196 | make_monitor_attribute_list | live, with dead arms inside |
| 7197-7207 | init_frame_once | live |
| 7208-7214 | init_frame_once_for_pdumper | live |
| 7215-7907 | syms_of_frame | live, with dead arms inside |

## Evidence (this round)

**① The frozen configuration (`build-tu`).**
```
$ cd femacs && CARGO_TARGET_DIR=/tmp/frame-gui-8f/target cargo run -q -p xtask -- build-tu src/frame.c
built src/frame.rs -> ../build/femacs/frame.o          # exit 0, 3 warnings (all pre-existing mirror warnings)
```
`build/femacs/frame.o` = **413,320 bytes**, md5 `1509205b1f37c6a097c04d0b7d337827`,
`nm | wc -l` = **1,523**, and
`nm build/femacs/frame.o | grep -c 'gui_set\|gui_display\|gui_default\|validate_x_resource\|x_get_resource\|XParseGeometry\|gui_mouse_grabbed\|gui_redo_mouse_highlight\|gui_figure_window_size\|Fx_get_resource\|Fx_parse_geometry'` = **0**.
Every item of C 5921-6970 sits behind `feature = "have_window_system"` (or a nested feature), which
no configuration this repository builds enables (`docs/baseline/feature-coverage.tsv`:
`have_window_system` sites 107 / files 10 / **enabled by `-`**), so the frozen object's size, symbol
count and whole symbol set are **identical to the previous round's reading** (413,320 / 1,523 / 0) -
the only proof this run can give about the new text is that it is *parsed and then stripped*.  It is
not a compile reading of the arm (P-86 ③⑵, and see ③ below).
Two consecutive builds of the same source are **byte-identical** (`cmp`), so the hash is a real
reading of *this* artifact; it is not a shape reading across rounds - between the source snapshot
this round started from and the final one the hash moved (1509205b… -> 93043164…) while size and
`nm` did not, because rustc's `--emit=obj` carries a metadata section that names the expanded
source.  The frozen-configuration readings that pin the arm's shape are the **size, the `nm`
count and the blocked-symbol count**, and all three are the previous round's exactly.

**② `extract-abi --check` reads OK.**
```
$ python3 tools/extract-abi.py --check emacs/src/frame.c src/frame.rs | tail -3
conditions: 7306 macros from docs/baseline/config/darwin-arm64.macros.tsv (6866 defined), applied to
the 424 files in docs/baseline/config/cond-eval-verified.txt (tools/check-cond-eval.py verifies
them; elsewhere the table is not consulted, since it cannot decide a gnulib template's
`@PLACEHOLDER@` conditions - P-66)
OK emacs/src/frame.c -> src/frame.rs
```
No `- ` row.  This round adds **19 more non-`static` definitions** of the same class the previous
round's 17 belonged to (`gui_set_vertical_scroll_bars`, `gui_set_horizontal_scroll_bars`,
`gui_set_scroll_bar_width`, `gui_set_scroll_bar_height`, `gui_set_alpha`,
`gui_set_alpha_background`, `gui_set_borders_respect_alpha_background`,
`gui_set_no_special_glyphs`, `gui_mouse_grabbed`, `gui_redo_mouse_highlight`,
`validate_x_resource_name`, `gui_display_get_resource`, `gui_display_get_arg`,
`gui_frame_get_and_record_arg`, `gui_default_parameter`, `gui_figure_window_size`,
`x_get_resource_string`, and the two `DEFUN`s `Fx_get_resource`/`Fx_parse_geometry`; C's other two
functions of the range, `gui_frame_get_arg` and `XParseGeometry`, are `static` and stay plain
`unsafe fn`s).  The row is covered by the extractor's `whole_reference ("function")` subtraction
(3.389), not by an allowance list, and the line row stays green because the tolerance row is
`total` now (Owed 4) and the port is 7,857 lines against the file's 7,907 (**0.63%**, inside 0.18).

**③ The flip is text-only here (P-86 ③⑵).**
This machine has no X11/NS/PGTK/W32/Haiku headers the mirror set needs, so a
`rustc --cfg 'feature="have_window_system"'` flip of `build/femacs/units/frame/_root.rs` is not run
and is not claimed.  What is measured instead: the file **parses** in both readings - the frozen
`build-tu` of ①, and a direct compile that skips xtask's include-graph dedupe:
```
$ cd femacs && rustc --edition 2024 --crate-type lib --emit=metadata -C panic=abort \
    -o /tmp/frame-gui-8f/parse.rmeta src/frame.rs
error: aborting due to 7494 previous errors        # E0252 / E0428 / E0659 only, no parse error
```
(The 7,494 are `defined multiple times`/`ambiguous` from the include guard xtask supplies; the count
is the previous round's exactly, because every item this round adds is inside a stripped `#[cfg]`.)
A flip would also fail for a reason each mirror states itself: `include/frame.rs` 250-268 records
that a window-system configuration "fails to compile at the first accessor" because the flag-run
numbering for those arms is not written out yet, and this round's use-site macros keep C's own field
text (`(*f).vertical_scroll_bar_type`, `(*f).horizontal_scroll_bars`, `(*f).config_scroll_bar_*`)
for that same reason - a GUI flip fails *there*, not by reading another configuration's bits.

**④ The other gates, one line each.**
```
$ python3 tools/check-dead-file-coverage.py
info docs/ports/frame.md: 170 range row(s) cover src/frame.c (7907 lines; 0 blank line(s) between blocks need no row)
info docs/ports/frame.md: 170 range row(s) cover src/frame.h (1956 lines; 0 blank line(s) between blocks need no row)
7 ledger(s) bound (a C range table), 0 failure(s)
   # the bound-ledger count moves as other lanes add `## C line ranges` sections (it was 6 earlier
   # in this same round and 5 on a second run of the previous one); the two rows naming this ledger
   # are the ones quoted.
   # note: port-map pairs src/frame.c with src/frame.rs (439) and src/frame.h with
   # src/frame.rs's include mirror (440); the ledger rule resolves both to this file, so
   # this table - whose last row reaches C 7,907 - is also what the gate reads for the 1,956-line
   # header (docs/ports/include-frame.md is not consulted by this rule).  See Owed 3.

$ python3 tools/check-port-lines.py
90 file(s) under ten percent, 229,815 reference line(s); 92 ratchet row(s), 0 new, 2 stale
   # the stale rows are other lanes' units (no longer under the threshold); this unit's row is not
   # among them and `0 new` is the reading that concerns this round.  Every number on this line
   # moves between runs because other lanes are landing work in the same checkout (it read
   # `91 / 242,303 / 1 stale` earlier in this same round).

$ python3 tools/check-citations.py
check-citations: 675 file(s) scanned, 0 drift, 0 range, 457 external, 33 allowed, 0 stale
   # this round's 24 `/// C:` citations were checked against the reference (frame.h 1107, 1108,
   # 1217-1223, 1277-1278, 1286-1301, 1840; dispextern.h 115, 136, 145, 152, 159, 165, 3688, 3983,
   # 3993; xterm.h 1744; lisp.h 4717; nsgui.h 116; and the prose citation of frame.c 6191).  The
   # first run found three (a `w32gui.h 120-124` range past that 115-line file's end, and two stale
   # line numbers) and all three are fixed, not allowlisted.

$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures

$ python3 tools/check-carried.py
11 carried file(s), 11 baseline row(s), 0 failure(s)

$ python3 tools/check-cond-eval.py src/frame.c
105 conditions, 0 unknown, 0 disagreements [VERIFIED]

$ python3 tools/check-globals.py            # no new DEFVAR_* carrier in this arm
check-globals: 284 units, 0 DEFVAR_* variables without a definition in their own unit

$ python3 tools/check-linkage.py
check-linkage: 2 unit object(s) compared against the non-static objects of the C sources the frozen build links, 0 with 0 missing symbol(s)   # src/frame is one of the two (its 6 non-static objects are unchanged)

$ python3 tools/check-undef.py              # frame.o contributes no row
check-undef: 22 NEW undefined reference(s) that nothing can satisfy   # every one "referenced by xdisp.o" (another lane's in-flight unit)
check-undef: 3 ratchet entries are stale (now satisfied): _Vcomp_eln_to_el_h _redisplay_internal _try_window

$ python3 tools/check-verbatim.py
6 trees, 4604 files, 0 failures

$ grep -c '^\- \[ \]' docs/ports/frame.md
0
```
Every exit status above is 0, and the `check-undef.py` rows name `xdisp.o`, not `frame.o` (this
round changed no live text and the object's linkage surface is the previous round's).

**⑤ Line count.**  C 5921-6970 is 1,050 C lines (all of them carried, comments included); the port
grew 6,536 -> **7,857** lines (+1,321, i.e. 1.26 port lines per C line - +20.2% against the unit's
own previous size).  The excess over the C text is the per-item `#[cfg]` attribute (the run's
condition sits on every item, P-86 ②), this half's 41 use-site items (11 macros,
7 type aliases, 19 constants, 4 prototypes), the `match`/`if`-expression shapes Rust
needs for C's `switch` and `?:`, the split postfix `*p++ = c` and chained assignments, and the
explicit `as` casts the C promotions hide.  Against the file's 7,907 total lines the port is
**0.63%** short (was 17.3%), which is what the `frame 0.18 total` row now reads (Owed 4).

## Deviations (this round)

**The `#if`s of C 5921-6970, one by one** (P-86 ②: each arm is carried and each condition is a
lower-case feature; `HAVE_WINDOW_SYSTEM` is `undef` at `darwin-arm64.macros.tsv` 1677,
`HAVE_X_WINDOWS` at 1719, `USE_X_TOOLKIT` at 3872, `HAVE_ANDROID` at 1239):

| C `#if` | verdict | arm the port writes |
| --- | --- | --- |
| 5921 (the run's own `#ifdef HAVE_WINDOW_SYSTEM`, closes at 6971) | false (1677) | no single counterpart: the condition sits on **every item** of the run as `#[cfg (feature = "have_window_system")]` - including on the two `DEFUN!` invocations (`Fx_get_resource`, `Fx_parse_geometry`), which are macro invocations and take the attribute the `androidselect.rs` units give theirs |
| 5956-5971 / 6011-6041 `#if USE_HORIZONTAL_SCROLL_BARS` | false (window.h 903's `# define USE_HORIZONTAL_SCROLL_BARS false`, which `include/window.rs` 495 mirrors as a `false` const) | `#[cfg (feature = "use_horizontal_scroll_bars")] { … }` statement blocks |
| 6044-6045 `#if (defined HAVE_PGTK \|\| defined HAVE_NTGUI \|\| defined HAVE_HAIKU \|\| defined HAVE_NS)` | false (all four undef) | `#[cfg (any (feature = "have_pgtk", feature = "have_ntgui", feature = "have_haiku", feature = "have_ns"))]` on `gui_set_alpha` |
| 6302-6318 `#ifndef HAVE_ANDROID` / `#else` | HAVE_ANDROID undef (1239), so C's *live* arm of this pair is the `#ifndef` one; **both** are carried | `#[cfg (not (feature = "have_android"))] { … }`, and the counterpart `#[cfg (feature = "have_android")] { SAFE_FREE! (sa_count); return Qnil (); }` |
| 6341 `#if defined HAVE_X_WINDOWS && !defined USE_X_TOOLKIT && !defined USE_GTK` | false (1719) | `#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"), not (feature = "use_gtk")))]` on `x_get_resource_string` |
| 6437-6439 / 6455-6457 / 6461-6463 `#ifdef HAVE_NS` (three, inside `gui_display_get_arg`'s `switch`) | false (1480) | `#[cfg (feature = "have_ns")] { strcmp (…) == 0 }` with the `#[cfg (not (feature = "have_ns"))] { false }` counterpart, spliced into C's `\|\|` chain |
| 6538-6626 `#if !defined (HAVE_X_WINDOWS) && defined (NoValue)` | false: `HAVE_X_WINDOWS` undef, and `defined (NoValue)` is false - the evaluator answers false for it exactly as clang does (it is not in `macros.tsv` at all) | `#[cfg (all (not (feature = "have_x_windows"), feature = "no_value"))]` on `XParseGeometry`; `no_value` is C's own macro name lower-cased (P-86 ②), nothing enables it, and the ledger records it here rather than leaving the block unexplained |
| 6657-6660 `#ifdef HAVE_NS` | false | `#[cfg (feature = "have_ns")] { … }` statement block |
| 6971 `#endif /* HAVE_WINDOW_SYSTEM */` | the run's end | text only (a comment) |

**Deviations in the carried text, each with its reason:**

1. **Names with no mirror, carried at the use site** (P-72 rule 3).  Types: the six `Display_Info`
   typedefs (`dispextern.h` 115/136/145/152/159/165 - one per GUI, though C's `#if` there is inside
   `#ifdef HAVE_WINDOW_SYSTEM` and the port keeps each behind its platform's feature) and
   `enum resource_types` (`dispextern.h` 3983-3992, a `c_int` alias with its six members).
   Constants: `EMACS_CLASS` (`frame.h` 1840, a `&str`; see 3 below);
   `NorthEastGravity`/`SouthWestGravity`/`SouthEastGravity` (the five non-X `*gui.h`, all 3/7/9 -
   `NorthWestGravity` was already defined for the first half); `USPosition`/`USSize`/`PPosition`/
   `PSize` (`(1L << 0…3)`, `c_long`, the same five headers); the `NoValue` family
   (`NoValue`/`XValue`/`YValue`/`WidthValue`/`HeightValue`, the same five).  Functions, with the
   header prototype on each `/// C:` line: `check_x_display_info` (`dispextern.h` 3993),
   `note_mouse_highlight` (`dispextern.h` 3688), `x_get_string_resource` (`xterm.h` 1744),
   `esprintf` (`lisp.h` 4717).  Each belongs in the mirror that declares it; this unit's copies must
   be deleted when those land (the `search.rs` precedent the `## Decisions` section records for the
   first half's ~40 geometry macros).
2. **The window-system arm of the scroll-bar accessors is at the use site too.**  `include/frame.rs`
   carries the `#else` arm's readers (`FRAME_VERTICAL_SCROLL_BAR_TYPE`, `FRAME_HAS_VERTICAL_SCROLL_BARS`,
   `…_ON_LEFT`, `…_ON_RIGHT`, `FRAME_HAS_HORIZONTAL_SCROLL_BARS`) gated with
   `not (have_window_system)` / `not (use_horizontal_scroll_bars)`, and the four
   `FRAME_CONFIG_SCROLL_BAR_*` and the two `FRAME_TOOL_BAR_*` as **readers**, while C's text is an
   l-value macro (C 5985-5986, 6020-6021, 6734, 6766, 6783-6784 write them).  The port therefore
   defines the window-system readers and all six writable slots here under C's own names; the
   readers return `bool` where C's `!=`/`==` yields `int` (the `int`/`bool` mix the live text already
   spells with explicit comparisons).
3. **`EMACS_CLASS` is a `&str`.**  C's three uses are `build_string (EMACS_CLASS)`,
   `sizeof (EMACS_CLASS) - 1` and a `%s` argument; the port writes `build_string (EMACS_CLASS.as_ptr ()
   as *const c_char)`, `EMACS_CLASS.len ()` (5, the same character count) and
   `EMACS_CLASS.as_ptr () as *const c_char`.  The one magic number C hides behind `sizeof` at C 6223
   (`MAX_ALLOCA - sizeof ".customization" < len`) is written out as `MAX_ALLOCA - 16` with C's
   `sizeof` text in the comment (15 bytes + NUL).
4. **`horizontal_scroll_bars` has no storage in the mirror** (the window-system flag run's numbering
   is not written out - the same deferral `want_fullscreen` records above).  C 5960 therefore keeps
   C's own field write (`(*f).horizontal_scroll_bars = …`), and the C 5929-5940 write to
   `FRAME_VERTICAL_SCROLL_BAR_TYPE` is the use-site l-value macro's `(*f).vertical_scroll_bar_type`.
   A GUI flip fails at those fields (E0609), which is the mirror's documented behaviour, not a silent
   wrong read.
5. **`dpyinfo->rdb` is untyped at the hook call.**  `termhooks.h` 846's `get_string_resource_hook`
   takes `void *rdb`, and the record's member is `XrmDatabase` (X), `Lisp_Object` (NS/Haiku),
   `*const c_char` (W32) or nothing (Android); the port passes
   `&raw mut (*dpyinfo).rdb as *mut c_void`, C's own `&dpyinfo->rdb`.  The call itself is the port's
   `if let Some (hook) = … { … } else { std::ptr::null () }` shape (the `Option` fn-pointer field has
   no C counterpart; the same family the live text uses for `set_window_size_hook`).
6. **Small C-to-Rust shapes inside the carried bodies**, none changing a value or an order:
   `switch` -> `match` with a `_ => { emacs_abort (); }` arm; C's `?:` -> Rust `if`-expressions;
   C's postfix `*cz++ = '.'` / `*nz++` / `string++` (C 6287, 6295, 6299, 6566, 6580) -> the store
   followed by `cz = cz.offset (1)` / `&raw mut`-free pointer steps, and C's chained assignments (`Vx_resource_name = new = Fcopy_sequence (…)`
   at C 6232, `f->new_width = f->new_height = -1` at C 6790) -> one statement each (the established
   split); `min (…)`, `clip_to_bounds (…)`, `check_integer_range (…)` and the `XFIXNUM`-into-`int`
   stores get the explicit `as` casts C's promotions hide, and `window_prompting` (C's `long`) mixes
   with the `int` sign bits as `window_prompting |= XNegative as c_long`; `UNINIT` and the
   otherwise-indeterminate locals -> zero-initialised (PORTING 1.5): `XParseGeometry`'s
   `tempWidth`/`tempHeight`/`tempX`/`tempY`/`nextCharacter`, `Fx_parse_geometry`'s `x`/`y`,
   `gui_set_alpha`'s `item`; `frame_float`'s `parent_done`/`outer_done` are passed as
   `&raw mut …` exactly as C's `&parent_done`; the port's macro-call spelling for C's macros
   (`FRAME_*!`, `SAFE_*!`, `AUTO_STRING!`, `AUTO_FRAME_ARG!`, `calln!`; `XSETCAR` is a plain function
   here) - the token check ignores the `!`.
7. **`gui_set_alpha`'s `#if` is an item gate, not a statement gate.**  C's guard wraps the *whole
   function*, so the port puts both features on the definition (two `#[cfg]` attributes) rather than
   around the body; C has no `#else`, so there is no counterpart item.
8. **The two `DEFUN`s carry C's own docstring text** (C 6323-6331, 6638-6644), escaped the way the
   other 66 do; they are not in the live inventory (`extract-abi.py`'s subr list filters by the
   frozen arms, so they appear in `## Subrs` nowhere - they are inside the dead run), which is why
   the checklist at the top of this file still reads 66.

## Owed by the integrator (this round; nothing here is claimed as done)

1. **Mirror work this arm needs before any GUI configuration can compile.**
   `include/frame.rs` 250-268 records the window-system `want_fullscreen` numbering and flag run
   (the previous round's item, still open; the readings are in `docs/baseline/frame-layout.tsv`);
   this round adds `horizontal_scroll_bars`, `vertical_scroll_bar_type` and the six scroll-bar
   slots to the same deferral, because C's text for them is the field itself and the mirror numbers
   only the frozen arm.  Also missing from mirrors, and therefore written at this unit's use site:
   the six `Display_Info` typedefs (`dispextern.h` 115-165), `enum resource_types`
   (`dispextern.h` 3983-3992), the four prototypes of Deviations 1, `EMACS_CLASS` (`frame.h` 1840)
   and the `*gui.h` gravity/`US*`/`PPosition`/`NoValue` constants.  The `*gui.h` mirrors carry the
   constants already, but this unit reaches them only through a platform `TERM_HEADER` include,
   which is a GUI configuration; when the typedefs and the enum land in `include/dispextern.rs`
   this unit's copies must go in the same commit (the `search.rs` precedent).
2. **C 7877-7894's dead `#ifdef HAVE_WINDOW_SYSTEM` block is still recorded, not carried**
   (`defsubr (&Sx_get_resource)`, `defsubr (&Sx_parse_geometry)`,
   `defsubr (&Sreconsider_frame_fonts)`, the `Qmove_toolbar` `DEFSYM`/`Fprovide`).  It is outside
   this round's range (C 5921-6970), but it is the only text that would register the two `DEFUN`s
   this round defines - so under a GUI flip they would exist and be unreachable.  Whoever carries
   C 6972-7907 closes it; until then this is a text-level gap, not a symbol-level one (the frozen
   object has neither).
3. **`docs/ports/frame.md` is a two-target ledger** (the previous round's item, still open):
   `docs/port-map.md` 439 pairs `src/frame.c` with `src/frame.rs` and 440 pairs `src/frame.h`
   with that unit's include mirror, and `check-dead-file-coverage.py`'s ledger rule resolves both to
   this file.  Today the one table satisfies both because its last row reaches C 7,907; if the header
   ever needs its own ranges, the fix is a `src-frame.md` for it or a rule change - the header's
   coverage should not keep riding on the unit's table.  (`free_monitors`, C 7124-7133, is a row
   this round corrected: it was labelled "next round (`have_window_system`, C 5921-6970)" and is
   neither - it is a separate `#ifdef HAVE_WINDOW_SYSTEM` block with its own nested
   `# if (defined USE_GTK || defined HAVE_PGTK || defined HAVE_NS || defined HAVE_XINERAMA
   || defined HAVE_XRANDR)` at C 7120-7123, recorded under P-08, still not carried.)
4. **`docs/ports/line-drift-tolerance.tsv` 196's `frame 0.18 total` row**: the previous round's
   item 1 was done by the integrator (the row moved from `live`/0.05 to `total`/0.18), and it is
   what makes evidence ② green at this round's 0.63%.  It stays `total` for good: the whole
   `#ifdef HAVE_WINDOW_SYSTEM` run is carried, both arms.
5. **The extractor's invented-exported-symbol rule**: the previous round's item 2 is resolved by
   3.389's `whole_reference ("function")` subtraction (not by an allowance list), and this round's
   17 more non-`static` definitions of that class read OK.  Nothing is owed here; recorded so the
   next reader does not re-open it.

# ----------------------------------------------------------------------
## Decisions

**STATE: COMPLETE for the frozen configuration's checklist; the `#ifdef HAVE_WINDOW_SYSTEM` arm
is now carried in full (C 4889-6970).**
66/66 subrs, 57/57 functions, the `frame_parm_table` type, all 8 file-scope objects and the
51-entry `frame_parms` table are ported, and every box above is ticked.  What the boxes cannot see
is the dead region this file also has: **C 4889-6970 is carried in full** under
`feature = "have_window_system"` and its nested features - the first half (C 4889-5919) in the
round before this one and the second (C 5921-6970) in this one - which replaces P-08's "recorded,
not transcribed" for the run, turns `extract-abi --check` non-vacuous and keeps it green
(evidence ②).  1,050 C lines landed this round; the port is 7,857 lines against the file's 7,907
(0.63%).  What is *not* carried, and is owed rather than claimed, is in `## Owed by the
integrator`.  The gates below are the pre-round readings, kept as the record of the live
checklist; the current readings are in `## Evidence`.  The gates, with the last line each
produced:

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-frame cargo run -q -p xtask -- build-tu src/frame.c
built src/frame.rs -> ../build/femacs/frame.o

$ python3 tools/extract-abi.py --check emacs/src/frame.c src/frame.rs
OK emacs/src/frame.c -> src/frame.rs

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
    7877-7894.  The one large arm of this family the port **does** carry is the whole GUI
    frame-geometry implementation at C 4889-6970 (P-86 ①: both arms, one `#[cfg]` per item;
    `## C line ranges` above is its coverage table and `## Deviations` its `#if` inventory),
    which this bullet recorded until the two rounds that landed it; C 7877-7894's `defsubr`
    block is the part of it still recorded (Owed 2).
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

This round's text (C 5921-6970) is inside the dead arm, so it is outside the C side's live set by
construction; what was measured instead is that it names nothing C does not.  Every identifier
followed by `(` in the added region was extracted the same way (comments and literals stripped,
macro `!` ignored): **129 identifiers, 96 of them in C's text too, and the 33 that are not are all
in families the list above already carries** - the 21 `Q*` Lisp-symbol functions, the pointer/mirror
spellings (`as_ptr`, `is_null`, `is_some`, `len`, `null`, `null_mut`, `offset`) and the `cfg`
attribute vocabulary (`cfg`, `all`, `any`, `not`, `mut`).  No new C-only name and no new Rust-only
family.

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

**Added by the C 5921-6970 round** (all behind `have_window_system` or a nested feature, all named
in `## Deviations` above): the prototypes `check_x_display_info` (dispextern.h 3993),
`note_mouse_highlight` (dispextern.h 3688), `x_get_string_resource` (xterm.h 1744) and `esprintf`
(lisp.h 4717); the type `resource_types` (dispextern.h 3983-3992) and the six `Display_Info`
typedefs (dispextern.h 115/136/145/152/159/165, one per GUI); the constants `EMACS_CLASS`
(frame.h 1840), `NorthEastGravity`/`SouthWestGravity`/`SouthEastGravity`,
`USPosition`/`USSize`/`PPosition`/`PSize` and `NoValue`/`XValue`/`YValue`/`WidthValue`/`HeightValue`
(the five non-X `*gui.h`); and the eleven macros the mirror gated away or mirrors as readers -
`FRAME_VERTICAL_SCROLL_BAR_TYPE`, `FRAME_HAS_VERTICAL_SCROLL_BARS{,_ON_LEFT,_ON_RIGHT}`,
`FRAME_HAS_HORIZONTAL_SCROLL_BARS`, `FRAME_CONFIG_SCROLL_BAR_{WIDTH,HEIGHT,COLS,LINES}` and
`FRAME_TOOL_BAR_{LINES,HEIGHT}`.

### Integrator note (mirror gaps this unit reports and does not touch)

1. **`include/frame.rs` (frame.h's mirror) is read-only WIP**: it carries the readers as
   functions and the flag run as `bits1`/`bits2` masks but no l-value macros, which is why
   §3 above defines ~40 macros at the use site (and the C 5921-6970 round eleven more, including
   the window-system arm of the scroll-bar accessors the mirror gates with
   `not (have_window_system)`).  When the mirror grows them, this unit's copies must be deleted in
   the same commit (the search.rs precedent).
2. **`include/frame.rs`'s crate-root declarations** of the seven file-scope objects and of
   `frame_make_pointer_invisible` are why `frame_globals`/`frame_pointer_fns` exist; moving
   those declarations into a `pub mod … ; use …::*;` (P-21) would let the definitions return
   to the crate root.
3. **`include/window.rs`** says `WINDOW_XFRAME` (window.h 1358) "lands with frame.rs"; it is
   defined here at the use site, and `fset_redisplay`'s mirror declaration takes
   `*mut c_void` where frame.h's prototype takes `struct frame *` (the port casts).
4. **Missing mirrors**: `dispextern.h`'s six `Display_Info` typedefs (115-165), `enum
   resource_types` (3983-3992) and the `check_x_display_info`/`note_mouse_highlight` prototypes
   (3993/3688), `xterm.h`'s `x_get_string_resource` (1744), `lisp.h`'s `esprintf` (4717) and
   `frame.h`'s `EMACS_CLASS` (1840) - all written at this unit's use site for the C 5921-6970
   round; `systty.h`/`<stdio.h>`'s `fileno`, `frame.h`'s `CHECK_FRAME`/
   `CHECK_LIVE_FRAME`, `SET_FRAME_*` writers, `FRAME_INNER_*`, the pixel/text conversions,
   `FRAME_TEXT_COLS_TO_PIXEL_WIDTH`/`FRAME_PIXEL_HEIGHT_TO_TEXT_LINES`,
   `WINDOW_FRAME`/`WINDOW_XFRAME`, `lisp.h`'s `hashtest_eq`, `xfaces.c`'s `unspecified_fg`/
   `unspecified_bg` arrays, `dispextern.h`'s `internal_border_part` enum and
   `FACE_TTY_DEFAULT_*`.
5. **`docs/ports/line-drift-tolerance.tsv`** now carries the `frame 0.18 total` row (the
   integrator moved it from `live`/0.05 when the first half landed, and it is what makes
   `extract-abi --check` green at this round's 0.63%);
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
