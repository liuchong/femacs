# emacs/src/intervals.h (314 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [ ] set_interval_object  (line 147)
- [ ] set_interval_parent  (line 155)
- [ ] set_interval_plist  (line 162)

## Macros defined — 29
- [ ] AM_LEFT_CHILD  (line 80, body 75d6df7dd56d)
- [ ] AM_RIGHT_CHILD  (line 84, body 56bab2704d44)
- [ ] BOTH_KIDS_P  (line 97, body 6b221477fe70)
- [ ] COPY_INTERVAL_CACHE  (line 186, body 18798c8aa850)
- [ ] DEFAULT_INTERVAL_P  (line 125, body e8b91f269760)
- [ ] EMACS_INTERVALS_H  (line 20, body da39a3ee5e6b)
- [ ] GET_INTERVAL_OBJECT  (line 141, body 69772cfb6a7f)
- [ ] INTERVAL_EXPRESSLY_WRITABLE_P  (line 217, body 4540c3bb9a63)
- [ ] INTERVAL_GENERALLY_WRITABLE_P  (line 210, body 92572abd891d)
- [ ] INTERVAL_HAS_OBJECT  (line 130, body 981f6cceda30)
- [ ] INTERVAL_HAS_PARENT  (line 129, body cabc692f9ddb)
- [ ] INTERVAL_LAST_POS  (line 113, body 868ca50fcc0a)
- [ ] INTERVAL_PARENT  (line 138, body 39d13c7b15c1)
- [ ] INTERVAL_PARENT_OR_NULL  (line 170, body 181041307952)
- [ ] INTERVAL_VISIBLE_P  (line 204, body 562734ea6040)
- [ ] LEAF_INTERVAL_P  (line 88, body cabb59d0af0f)
- [ ] LEFT_TOTAL_LENGTH  (line 116, body 09f829681350)
- [ ] LENGTH  (line 107, body 969f813c8e72)
- [ ] MERGE_INTERVAL_CACHE  (line 195, body 1db639537fae)
- [ ] NULL_LEFT_CHILD  (line 74, body 848c105e73a1)
- [ ] NULL_PARENT  (line 77, body d16419e8df35)
- [ ] NULL_RIGHT_CHILD  (line 71, body 9e64d25dae14)
- [ ] ONLY_INTERVAL_P  (line 94, body 1e30108e7530)
- [ ] RESET_INTERVAL  (line 174, body a4fecf6e2a59)
- [ ] RIGHT_TOTAL_LENGTH  (line 119, body 1c9ec5f5ec5f)
- [ ] ROOT_INTERVAL_P  (line 91, body b6b61c702fc1)
- [ ] TEXT_PROP_MEANS_INVISIBLE  (line 240, body 56e3b8b48cb7)
- [ ] TOTAL_LENGTH  (line 101, body de2d8da351d2)
- [ ] TOTAL_LENGTH0  (line 104, body 220a8a398f9d)

## Types and their fields — 1
- [ ] interval: ptrdiff_t total_length, ptrdiff_t position, struct interval* left, struct interval* right, struct interval* interval, Lisp_Object obj, <aggregate> up, bool_bf up_obj : 1, bool_bf gcmarkbit : 1, bool_bf write_protect : 1, bool_bf visible : 1, bool_bf front_sticky : 1, bool_bf rear_sticky : 1, Lisp_Object plist

## File-scope objects — 0
