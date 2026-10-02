# emacs/src/font.h (1034 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 20
- [ ] FONTP  (line 435)
- [ ] FONT_SPEC_P  (line 442)
- [ ] GC_FONT_SPEC_P  (line 449)
- [ ] FONT_ENTITY_P  (line 456)
- [ ] GC_FONT_ENTITY_P  (line 463)
- [ ] FONT_OBJECT_P  (line 470)
- [ ] GC_FONT_OBJECT_P  (line 477)
- [ ] CHECK_FONT  (line 485)
- [ ] CHECK_FONT_SPEC  (line 491)
- [ ] CHECK_FONT_ENTITY  (line 497)
- [ ] CHECK_FONT_OBJECT  (line 503)
- [ ] XFONT_SPEC  (line 511)
- [ ] GC_XFONT_SPEC  (line 518)
- [ ] XFONT_ENTITY  (line 525)
- [ ] GC_XFONT_ENTITY  (line 532)
- [ ] XFONT_OBJECT  (line 539)
- [ ] GC_XFONT_OBJECT  (line 546)
- [ ] CHECK_FONT_GET_OBJECT  (line 555)
- [ ] valid_font_driver  (line 902)
- [ ] font_data_structures_may_be_ill_formed  (line 1021)

## Macros defined — 28
- [ ] EMACS_FONT_H  (line 23, body da39a3ee5e6b)
- [ ] FONT_ADD_LOG  (line 1004, body 9c0a062da66b)
- [ ] FONT_BASE  (line 245, body fa7425ff23ab)
- [ ] FONT_DEBUG  (line 998, body da39a3ee5e6b)
- [ ] FONT_DEFERRED_LOG  (line 1010, body 2be01ccfaba8)
- [ ] FONT_DESCENT  (line 246, body 9ac285903217)
- [ ] FONT_ENCODING_NOT_DECIDED  (line 273, body 3028f51407d8)
- [ ] FONT_HEIGHT  (line 244, body 17ec4152d2d2)
- [ ] FONT_INVALID_CODE  (line 585, body bd700042aab3)
- [ ] FONT_PIXEL_SIZE_QUANTUM  (line 583, body 356a192b7913)
- [ ] FONT_SET_STYLE  (line 236, body c194bafab603)
- [ ] FONT_SLANT_FOR_FACE  (line 217, body 1a82bbad5872)
- [ ] FONT_SLANT_NAME_NUMERIC  (line 227, body 3a529e8dc132)
- [ ] FONT_SLANT_NUMERIC  (line 197, body 518e029c9a6e)
- [ ] FONT_SLANT_SYMBOLIC  (line 208, body b630f2b4c11e)
- [ ] FONT_WEIGHT_FOR_FACE  (line 214, body 1215e8b22009)
- [ ] FONT_WEIGHT_NAME_NUMERIC  (line 224, body 84c695f2ce3e)
- [ ] FONT_WEIGHT_NUMERIC  (line 193, body 77d5c64e797a)
- [ ] FONT_WEIGHT_SYMBOLIC  (line 205, body 61d819e5e43a)
- [ ] FONT_WIDTH  (line 240, body f9796d0cbc37)
- [ ] FONT_WIDTH_FOR_FACE  (line 220, body 4159c49613a0)
- [ ] FONT_WIDTH_NAME_NUMERIC  (line 230, body dadeeb47050a)
- [ ] FONT_WIDTH_NUMERIC  (line 201, body e80f3e7f5190)
- [ ] FONT_WIDTH_SYMBOLIC  (line 211, body d1d27c38efcc)
- [ ] PIXEL_TO_POINT  (line 579, body 2b796ab22aad)
- [ ] POINT_TO_PIXEL  (line 572, body 25504e1cec3d)
- [ ] PT_PER_INCH  (line 563, body cd14d6bdf7d0)
- [ ] XSETFONT  (line 552, body a7abab8225c7)

## Types and their fields — 7
- [ ] font: union vectorlike_header header, Lisp_Object props, int min_width, int max_width, int pixel_size, int height, int space_width, int average_width, int ascent, int descent, struct font_driver const* driver
- [ ] font_bitmap: int bits_per_pixel, int rows, int width, int pitch, unsigned char* buffer, int left, int top, int advance
- [ ] font_driver: Lisp_Object type, bool case_sensitive, Lisp_Object (*) get_cache, Lisp_Object (*) list, Lisp_Object (*) match, Lisp_Object (*) list_family, void (*) free_entity, void (*) close_font, void (*) prepare_face, void (*) done_face, int (*) has_char, unsigned (*) encode_char, int* x, Lisp_Object (*) otf_capability, Lisp_Object gstring_out, Lisp_Object idx, int (*) start_for_frame, int (*) end_for_frame, Lisp_Object (*) shape, int (*) check, int c, void (*) filter_properties, Lisp_Object (*) combining_capability, void (*) drop_xrender_surfaces
- [ ] font_driver_list: bool on, struct font_driver const* driver, struct font_driver_list* next
- [ ] font_entity: union vectorlike_header header, Lisp_Object props
- [ ] font_metrics: short lbearing, short rbearing, short width, short ascent, short descent
- [ ] font_spec: union vectorlike_header header, Lisp_Object props

## File-scope objects — 0
