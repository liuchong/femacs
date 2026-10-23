# emacs/src/fileio.c (7085 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 55
- [x] find-file-name-handler  (Ffind_file_name_handler, 2/2, doc c345f60bfd27, line 370)
- [x] file-name-directory  (Ffile_name_directory, 1/1, doc 5a662eae0e5d, line 509)
- [x] file-name-nondirectory  (Ffile_name_nondirectory, 1/1, doc 4cdcafd1c0c3, line 534)
- [x] unhandled-file-name-directory  (Funhandled_file_name_directory, 1/1, doc b1efc642ba7e, line 575)
- [x] file-name-as-directory  (Ffile_name_as_directory, 1/1, doc c8b820dd48ac, line 634)
- [x] directory-name-p  (Fdirectory_name_p, 1/1, doc 52853278c183, line 703)
- [x] directory-file-name  (Fdirectory_file_name, 1/1, doc 9ad1fab95041, line 727)
- [x] make-temp-file-internal  (Fmake_temp_file_internal, 4/4, doc 88c289e57e06, line 767)
- [x] make-temp-name  (Fmake_temp_name, 1/1, doc 6d1110a57090, line 830)
- [x] file-name-concat  (Ffile_name_concat, 1/MANY, doc c2e042359a2e, line 846)
- [x] expand-file-name  (Fexpand_file_name, 1/2, doc 0ebf926fd459, line 994)
- [x] substitute-in-file-name  (Fsubstitute_in_file_name, 1/1, doc 0aad5ed9bd86, line 2046)
- [x] copy-file  (Fcopy_file, 2/6, doc 6a6acb20686e, line 2252)
- [x] make-directory-internal  (Fmake_directory_internal, 1/1, doc 75f5f779a811, line 2562)
- [x] delete-directory-internal  (Fdelete_directory_internal, 1/1, doc 88415c6f15d7, line 2583)
- [x] delete-file-internal  (Fdelete_file_internal, 1/1, doc 5b66eb9a5b3d, line 2602)
- [x] file-name-case-insensitive-p  (Ffile_name_case_insensitive_p, 1/1, doc f24e460eec0e, line 2688)
- [x] rename-file  (Frename_file, 2/3, doc 533eb8a76da4, line 2724)
- [x] add-name-to-file  (Fadd_name_to_file, 2/3, doc f7fcc98a7568, line 2851)
- [x] make-symbolic-link  (Fmake_symbolic_link, 2/3, doc 14f070e4a3c2, line 2907)
- [x] file-name-absolute-p  (Ffile_name_absolute_p, 1/1, doc dd0941e76a20, line 2967)
- [x] file-exists-p  (Ffile_exists_p, 1/1, doc 1379985d2cda, line 3010)
- [x] file-executable-p  (Ffile_executable_p, 1/1, doc ba40359abd61, line 3022)
- [x] file-readable-p  (Ffile_readable_p, 1/1, doc 58f19f1a1fc8, line 3032)
- [x] file-writable-p  (Ffile_writable_p, 1/1, doc 70792cf4372d, line 3040)
- [x] access-file  (Faccess_file, 2/2, doc 5655bc8adf0b, line 3078)
- [x] file-symlink-p  (Ffile_symlink_p, 1/1, doc 55443df3d142, line 3162)
- [x] file-directory-p  (Ffile_directory_p, 1/1, doc 6217deeb0b4c, line 3185)
- [x] file-accessible-directory-p  (Ffile_accessible_directory_p, 1/1, doc 49a7897d7a81, line 3255)
- [x] file-regular-p  (Ffile_regular_p, 1/1, doc 839b9be1b7d1, line 3359)
- [x] file-selinux-context  (Ffile_selinux_context, 1/1, doc 2b442d120724, line 3392)
- [x] set-file-selinux-context  (Fset_file_selinux_context, 2/2, doc fbe9eb52b269, line 3446)
- [x] file-acl  (Ffile_acl, 1/1, doc 7087e793b52f, line 3534)
- [x] set-file-acl  (Fset_file_acl, 2/2, doc fb50b9331286, line 3580)
- [x] file-modes  (Ffile_modes, 1/2, doc 33e97596e6fd, line 3644)
- [x] set-file-modes  (Fset_file_modes, 2/3, doc 037afab892ea, line 3666)
- [x] set-default-file-modes  (Fset_default_file_modes, 1/1, doc b8a496d008dd, line 3700)
- [x] default-file-modes  (Fdefault_file_modes, 0/0, doc ad429eaa3b87, line 3730)
- [x] set-file-times  (Fset_file_times, 1/3, doc a4d2c50a605a, line 3741)
- [x] unix-sync  (Funix_sync, 0/0, doc 3d1500a87e55, line 3783)
- [x] file-newer-than-file-p  (Ffile_newer_than_file_p, 2/2, doc f3f1f5f78bb4, line 3793)
- [x] insert-file-contents  (Finsert_file_contents, 1/5, doc 479dff06b9e2, line 4057)
- [x] write-region  (Fwrite_region, 3/7, doc 0f1972265714, line 5461)
- [x] car-less-than-car  (Fcar_less_than_car, 2/2, doc 20a7d5199b11, line 5870)
- [x] verify-visited-file-modtime  (Fverify_visited_file_modtime, 0/1, doc e351270d43ed, line 6121)
- [x] visited-file-modtime  (Fvisited_file_modtime, 0/0, doc 840b89b7983a, line 6165)
- [x] set-visited-file-modtime  (Fset_visited_file_modtime, 0/1, doc b977680607ce, line 6177)
- [x] do-auto-save  (Fdo_auto_save, 0/2, doc efda7f8cf200, line 6315)
- [x] set-buffer-auto-saved  (Fset_buffer_auto_saved, 0/0, doc 1654282f7793, line 6518)
- [x] clear-buffer-auto-save-failure  (Fclear_buffer_auto_save_failure, 0/0, doc 18bca0a31c27, line 6532)
- [x] recent-auto-save-p  (Frecent_auto_save_p, 0/0, doc 05eb0cddd6e9, line 6541)
- [x] next-read-file-uses-dialog-p  (Fnext_read_file_uses_dialog_p, 0/0, doc e992506643a6, line 6556)
- [x] set-binary-mode  (Fset_binary_mode, 2/2, doc 0778ffadfbd3, line 6588)
- [x] file--close-stream  (Ffile__close_stream, 1/1, doc 088cc983bfcc, line 6620)
- [x] file-system-info  (Ffile_system_info, 1/1, doc d128097254b0, line 6659)

## Functions defined — 57
- [x] check_vfs_filename  (line 190, static)
- [x] file_access_p  (line 227)
- [x] get_file_errno_data  (line 265)
- [x] report_file_errno  (line 286)
- [x] report_file_error  (line 298)
- [x] file_metadata_errno  (line 325, static)
- [x] file_attribute_errno  (line 333)
- [x] close_file_unwind  (line 339)
- [x] close_file_unwind_emacs_fd  (line 345, static)
- [x] fclose_unwind  (line 354)
- [x] restore_point_unwind  (line 363)
- [x] file_name_directory  (line 435, static)
- [x] file_name_as_directory  (line 613, static)
- [x] directory_file_name  (line 682, static)
- [x] expand_cp_target  (line 720, static)
- [x] file_name_absolute_no_tilde_p  (line 960, static)
- [x] user_homedir  (line 972, static)
- [x] splice_dir_file  (line 1932)
- [x] get_homedir  (line 1945)
- [x] search_embedded_absfilename  (line 2033, static)
- [x] expand_and_dir_to_file  (line 2165)
- [x] barf_or_query_if_file_exists  (line 2188, static)
- [x] emacs_full_read  (line 2230, static)
- [x] clone_file  (line 2243, static)
- [x] file_name_case_insensitive_err  (line 2647, static)
- [x] file_name_absolute_p  (line 2980)
- [x] check_file_access  (line 2992, static)
- [x] emacs_readlinkat  (line 3110, static)
- [x] check_emacs_readlinkat  (line 3145)
- [x] file_directory_p  (line 3212)
- [x] file_accessible_directory_p  (line 3304)
- [x] symlink_nofollow_flag  (line 3638, static)
- [x] decide_coding_unwind  (line 3857, static)
- [x] read_non_regular  (line 3924, static)
- [x] read_non_regular_quit  (line 3941, static)
- [x] file_offset  (line 3949, static)
- [x] time_error_value  (line 3973, static)
- [x] get_window_points_and_markers  (line 3982, static)
- [x] restore_window_points  (line 4005, static)
- [x] maybe_move_gap  (line 4031, static)
- [x] xlseek  (line 4046, static)
- [x] build_annotations_unwind  (line 5340, static)
- [x] choose_write_coding_system  (line 5348, static)
- [x] write_region  (line 5511)
- [x] build_annotations  (line 5889, static)
- [x] a_write  (line 5975, static)
- [x] e_write  (line 6025, static)
- [x] buffer_visited_file_modtime  (line 6157)
- [x] auto_save_error  (line 6234, static)
- [x] auto_save_1  (line 6249, static)
- [x] do_auto_save_unwind  (line 6283, static)
- [x] do_auto_save_make_dir  (line 6298, static)
- [x] do_auto_save_eh  (line 6309, static)
- [x] file_for_stream  (line 6576, static)
- [x] blocks_to_bytes  (line 6646, static)
- [x] init_fileio  (line 6711)
- [x] syms_of_fileio  (line 6720)

## Macros defined — 7
- [x] emacs_fd_close  (line 126, body 3dbfda739be6)
- [x] emacs_fd_fstat  (line 129, body 77a2e71c53d9)
- [x] emacs_fd_lseek  (line 128, body 0d5930e232f1)
- [x] emacs_fd_open  (line 125, body a2e752273417)
- [x] emacs_fd_read  (line 127, body 6f0f5a8a7f7f)
- [x] emacs_fd_to_int  (line 135, body 4a1bb42abbbc)
- [x] emacs_fd_valid_p  (line 130, body 3e4ff0024176)

## Types and their fields — 2
- [x] auto_save_unwind: FILE* stream, bool auto_raise
- [x] read_non_regular: emacs_fd fd, char* buf, ptrdiff_t inserted, ptrdiff_t bufsize, <aggregate> s

## File-scope objects — 8
- [x] static bool auto_saving  (line 153, static)
- [x] static mode_t realmask  (line 156, static)
- [x] static mode_t auto_saving_dir_umask  (line 159, static)
- [x] static mode_t auto_save_mode_bits  (line 163, static)
- [x] static bool auto_save_error_occurred  (line 166, static)
- [x] static bool valid_timestamp_file_system  (line 170, static)
- [x] static dev_t timestamp_file_system  (line 171, static)
- [x] static Lisp_Object Vwrite_region_annotation_buffers  (line 175, static)

# ----------------------------------------------------------------------
## Decisions

`emacs/src/fileio.c` @ `fd7afced403`; port `src/fileio.rs` (5,987 lines vs C's 7,085 total /
6,927 live - **-13.6% against the live count**), declared in `docs/ports/line-drift-tolerance.tsv` with
the `live` basis and the reason: this unit's dead region is *recorded* (P-08), and the 55 `DEFUN`
docstrings are one-line string literals, which saves C's 8-30 line `doc: /* ... */` comments
(~700 lines).  The per-function token check is the real guard and it is `OK`.

- **Dead arms (P-08: `src/fileio.c` *is* in `docs/baseline/config/cond-eval-verified.txt` -
  `python3 tools/check-cond-eval.py src/fileio.c` = `122 conditions, 0 unknown,
  0 disagreements [VERIFIED]` - so they are recorded here, not transcribed).**  The evaluator
  the gate itself uses marks **86 runs** dead; the ones this unit's comments name, in C order:
  the `#include`s of `selinux/selinux.h`/`selinux/context.h` (HAVE_LIBSELINUX undef), `<sys/acl.h>`
  (USE_ACL is 0), `android.h` (HAVE_ANDROID undef), `<sys/ioctl.h>`/`<linux/fs.h>`
  (HAVE_LINUX_FS_H undef), `<windows.h>`/`<sys/file.h>`/`w32.h` (WINDOWSNT undef),
  `msdos.h`/`<sys/param.h>` (MSDOS undef), the DOS_NT `IS_DRIVE`/`DRIVE_LETTER` block and
  `<netio.h>` (HPUX undef) at C 38-120; the `struct android_fd_or_asset` arm of `emacs_fd`
  (C 137-149); `selinux_enabled_p` (C 194-208, HAVE_LIBSELINUX - the whole function, so it is
  *not* in this file's function list at all); `file_access_p`'s MSDOS and CYGWIN arms (C 229-251);
  `report_file_notify_error` (C 303-317, USE_FILE_NOTIFY undef - likewise a whole function);
  the DOS_NT halves of `file_name_directory` (C 441-503), `file_name_nondirectory` (C 563-567),
  `file_name_as_directory` (WINDOWSNT, C 664-666), `directory_file_name` (C 697-698) and
  `Ffile_name_as_directory` (C 755-757); `Fexpand_file_name`'s DOS_NT locals (C 1038-1041),
  its `emacs_root_dir ()` root (C 1067-1076), the seven DOS_NT/WINDOWSNT clauses of its two
  nested conditions (C 1143-1155, 1258-1295, 1303-1307), the drive-specifier parsing and
  UNC handling (C 1342-1365, 1450-1484, 1513-1606, 1619-1642), the `/..` DOS_NT clause
  (C 1691-1742) and its DOS_NT result construction (C 1718-1742); the `#if 0` old
  `expand-file-name` (C 1767-1925, a `DEAFUN` - the one large recorded region);
  `get_homedir`'s WINDOWSNT/Android/DOS_NT arms (C 1949-1958, 1980-2013);
  `search_embedded_absfilename`'s `DOUBLE_SLASH_IS_DISTINCT_ROOT` arm (C 2036-2038);
  `Fsubstitute_in_file_name`'s DOS_NT and WINDOWSNT arms (C 2092-2153); `Fcopy_file`'s
  WINDOWSNT arm, its `HAVE_LIBSELINUX` body, its MSDOS `new_mask`/`#else`s and its MSDOS tail
  (C 2321-2366, 2377-2378, 2499-2552); `internal_delete_file_1`/`internal_delete_file`
  (C 2620-2640, `HAVE_NATIVE_COMP && WINDOWSNT`); `file_name_case_insensitive_err`'s
  `_PC_CASE_INSENSITIVE` arm and its `CYGWIN || DOS_NT` tail (C 2668-2682);
  `Frename_file`'s `CYGWIN || DOS_NT` case-only-rename arm (C 2748-2754); `Ffile_writable_p`'s
  MSDOS and WINDOWSNT arms (C 3063-3072); `emacs_readlinkat`'s `android_readlinkat` argument
  and `check_emacs_readlinkat`'s CYGWIN arm (C 3125-3155); `file_directory_p`'s DOS_NT arm and
  its `O_PATH` arm (C 3214-3234, `O_PATH` is undefined on this target);
  `file_accessible_directory_p`'s DOS_NT arm (C 3306-3317); `Ffile_regular_p`'s two WINDOWSNT
  arms (C 3377-3380); the `HAVE_LIBSELINUX` bodies of `Ffile_selinux_context` /
  `Fset_file_selinux_context` (C 3385-3470, 3481-3528) and the `USE_ACL` bodies of `Ffile_acl` /
  `Fset_file_acl` (C 3543-3631); `Fset_file_times`'s MSDOS arm (C 3771-3774);
  `Fcopy_file`'s `WINDOWSNT` `EBADF` clause, `Fwrite_region`'s DOS_NT `mode` (C 5642-5643) and
  the Android `st_ino` clause (C 5766-5770); `Fnext_read_file_uses_dialog_p`'s whole
  `USE_GTK || USE_MOTIF || HAVE_NS || HAVE_NTGUI || HAVE_HAIKU` arm (C 6563-6569); and
  `Ffile_system_info`'s Android special-directory arm plus the `#else return Qnil` of its
  `STAT_*` condition (C 6703-6704).
- **The one P-51 exception.**  `check_vfs_filename` (C 190-205) is a *live* function whose whole
  body C compiles out; an empty body is the one thing `extract-abi.py` rejects outright
  ("stub or empty body", because the `#if` line's text makes the C body non-empty for
  `body_is_empty`), so the arm's own text is kept behind `#[cfg (feature = "have_android")]`
  - a feature named for the *non-frozen* side, the §3.49/§3.9 shape, which `xtask dead-arms`
  does not flip (it is not a `config.rs` key).  More C text, not less.
- **Use-site declarations (PORTING.md 0.1-2 / P-72 rule 3): 52 names**, each with C's own
  prototype in a `/// C:` line, because the header that declares it has no mirror in this
  unit's include graph: `<pwd.h>` (`struct passwd`, measured with the frozen clang -
  `sizeof` 72, `pw_name`@0, `pw_uid`@16, `pw_dir`@48, `pw_shell`@56, `pw_expire`@64, pinned by
  `const _`, plus `getpwuid`/`getpwnam`); `<limits.h>`'s `SSIZE_MAX` (the limits mirror's own
  note says it is deliberately not carried); `<unistd.h>`'s `_PC_CASE_SENSITIVE` (11),
  `pathconf`, `sync`, `copy_file_range`; `<errno.h>`'s `EXDEV` (18); `<config.h>`'s `DIRECTORY_SEP`, `IS_DIRECTORY_SEP`, `IS_ANY_SEP` and
  `IS_DEVICE_SEP` (not in `config.rs`, the buffer.rs/callproc.rs/dired.rs shape); and the
  `lisp.h`/`coding.h` entries the mirrors do not carry yet - `sys_fstat`, `sys_faccessat`,
  `emacs_fstatat`, `emacs_fchmodat`, `emacs_strerror`, `emacs_wd`, `buffer_overflow`, `quit`,
  `make_specified_string`, `string_ascii_p`, `do_yes_or_no_p`, `FUNCTIONP`,
  `message_with_string`, `merge`, `record_unwind_protect_void`, `clear_unwind_protect`,
  `internal_condition_case`, `internal_condition_case_1`, `save_restriction_save`/`_restore`,
  `adjust_markers_for_delete`, `adjust_overlays_for_delete`, `push_message`,
  `pop_message_unwind`, `restore_message`, `sit_for`, `safe_run_hooks`, `record_auto_save`,
  `labeled_restrictions_remove_in_current_buffer`, `memory_full`, `ring_bell`, `emacs_fopen`,
  `code_conversion_save`, `setup_coding_system`, `raw_text_coding_system`,
  `coding_inherit_eol_type`, `detect_coding_system`, `insert_1_both`, `insert_from_buffer`,
  `del_range_byte`, `emacs_renameat_noreplace`, `emacs_rename`, `emacs_symlink`, `emacs_rmdir`,
  `emacs_mkdir`, `emacs_unlink`, `emacs_write_quit`.  This is the list of *mirror gaps* this
  unit reports and does not touch (P-26): a later unit that needs one of these from a mirror
  should move the declaration there.
- **`stdin`/`stdout`/`stderr` are macros on this target.**  `<stdio.h>` spells them
  `#define stdin __stdinp` (and `__stdoutp`/`__stderrp`), so C's object references
  `___stdinp`; a port that declares a symbol named `stdin` is textually faithful and cannot
  link.  `check-undef.py` caught exactly this while this unit carried such a declaration (its
  own worked example is `emacs_fclose`), and the `stdio.rs` mirror has since grown the three
  objects with the `link_name` spelling in its glob-imported `stdio_objs` module
  (docs/status.md 3.79), so this unit now uses them directly and declares none of them.
- **The labels and `goto`s of `Finsert_file_contents`.**  C's `notfound:`/`handled:` labels and
  its five `goto`s are the port's labelled blocks `'notfound`/`'chunk_a` plus an outer
  `'handled: loop` that carries the `handled` name: `goto notfound` is `break 'chunk_a` (the
  block of C's text before the label) and `goto handled` is `break 'notfound` (which lands on
  the `handled:` text), the PORTING.md 2.2 shape.  The outer label's own `break` is never
  used; it exists so C's label name is visible in the port and so the `'handled`/`'notfound`
  labels are far enough apart that `body_token_seq`'s character-literal scanner cannot read
  `'handled: { '` as a literal (it drops up to twelve characters after an unmatched `'`).
- **`Fdo_auto_save`'s `FOR_EACH_LIVE_BUFFER`**, the open P-22 loop macro
  (`docs/ports/excluded-macros.txt`), is written as `FOR_EACH_ALIST_VALUE`'s own loop
  (`tail = Vbuffer_alist; while CONSP (tail) && { buf = XCDR (XCAR (tail)); true }`, the
  buffer.rs `unlock_all_files` shape); C's two `continue`s become the enclosing `if`s and its
  shrink case becomes the same `if`'s `else`, because the loop's increment runs after the body
  on every path.  `build_annotations`' `goto loop_over_p` is `continue 'loop_over_p` on a
  `loop` that re-runs the `FOR_EACH_TAIL` (whose expansion is written out at the use site, as
  §3.7 prescribes for `ITREE_FOREACH`, because the macro re-binds its loop variable and so
  cannot be re-entered with a new `p`).
- **Bitfields.**  `coding->mode`/`->src_multibyte`/`->dst_multibyte`/`->raw_destination` and
  `window->start_at_line_beg` are written through the mirrors' `bits` word and their
  `*_SHIFT` constants (PORTING.md 1.3, the callproc.rs/pdumper.rs shape), and
  `buffer->clip_changed` through buffer.rs's `BUFFER_CLIP_CHANGED`.  Each write carries C's own
  spelling in a comment and a `docs/ports/macro-body-notes.tsv` row names the field token the
  token check can no longer see.
- **`docs/ports/macro-body-notes.tsv` gained 17 rows** for this unit: the C tokens the port
  legitimately cannot spell - `continue` for C's `for (...; ...; ...) continue;` loops
  (`user_homedir`, `emacs_full_read`, `Fexpand_file_name`), C's `bool = 0`/`= 1` assignments
  (`auto_save_error`, `do_auto_save_unwind`, `choose_write_coding_system`, `syms_of_fileio`,
  `delete_by_moving_to_trash`), the `UNINIT` macro (`write_region`, `Frename_file`,
  `Finsert_file_contents`), `switch`/`case`/`break`/`goto` in bodies the port writes as
  `match`/labelled blocks (`Fcopy_file`, `Frename_file`, `build_annotations`,
  `Finsert_file_contents`), the `doc:` label of a `DEFVAR_*` invocation whose docstring the
  mirror takes positionally (`syms_of_fileio`, the `syms_of_data` precedent), the bitfield
  field names above, `MAYBE_UNUSED` (`Fcopy_file`), `memory_full` (`emacs_readlinkat`), the
  `FOR_EACH_LIVE_BUFFER`/`FOR_EACH_TAIL` macro names, `true` for C's `while (true)`
  (`Ffile_name_case_insensitive_p`, where Rust's `while true` does not diverge - measured - so
  the port writes `loop`), and the five other `STAT_STATFS*` names that appear only on the
  continuation lines of `Ffile_system_info`'s `#if`.
- **`enum { INSERT_READ_SIZE_MAX = min (1024 * 1024, SYS_BUFSIZE_MAX) };`, `E_WRITE_MAX`,
  `file_name_as_directory_slop` and the function-local enums** (`INSERT_READ_SIZE_MAX`,
  `growth_factor`) are `const` items at the same position; `min` is not const in Rust, so the
  value is written as the winning arm with the other named in the expression
  (`if 1024 * 1024 < SYS_BUFSIZE_MAX { … } else { … }`), which keeps every token.
- **`ckd_sub`/`ckd_mul`** use the `overflowing_*` pair (P-15): `inserted_estimate
  .overflowing_sub (GAP_SIZE - 1)` and `blocksize.overflowing_mul (blocks)` keep C's operands
  and the "the target is not written on overflow" semantics; the wrapped value still lands in
  the C variable, as `ckd_*` does.
- **C's `MAYBE_UNUSED off_t newsize`** is `let mut newsize: off_t = 0;` (the attribute is an
  optimisation hint, recorded in the notes row), and the four `UNINIT` locals
  (`rename_errno`, `count1`, `endpos`) are zero-initialised (PORTING.md 1.5).
- **The two multi-line `intspec`s** are single-line literals with C's *spliced* whitespace
  spelled out, which is why `Fset_file_modes`' literal carries four tabs and two spaces
  between `)))` and `(list …` (C's `\`+newline deletes the newline but keeps the next line's
  indentation; Rust's line continuation would delete it too - the editfns.rs note).
- **`make-temp-file-internal`'s `static char const kind_message[][32]`** keeps C's element type
  (`[[c_char; 32]; 3]`) through the port's `c_str_32` const fn; `report_file_errno` /
  `report_file_error` are `-> !` because C declares them `AVOID` (`_Noreturn`), which is also
  what lets `file_metadata_errno`'s body end on its `report_file_errno (…)` statement, exactly
  as C's does.
- **Line count relation.**  C 7,085 total / 6,927 live vs the port's 5,987: the row in
  `docs/ports/line-drift-tolerance.tsv` declares the `live` basis and 0.15 (measured 13.6%,
  the §3.58 "measured + 0.01" convention).  The deficits are the recorded dead arms, the
  one-line docstrings and the single-line `DEFUN!` invocations; the excesses are the 52
  use-site declarations, the `'handled`/`'notfound` blocks and the 15 `DEFVAR_*` storage
  definitions (status 3.29).
- **C's forward declarations are dropped** (PORTING.md 0.1-12): C 177-181's
  `emacs_readlinkat`/`a_write`/`e_write` and C 5338's `build_annotations` prototype.

### Call-relationship differences (D7)

The two "identifier followed by `(`" sets differ, and every difference is one of four
mechanical classes - there is **no live call whose callee or argument order changed**:

1. **C's own macros**, which the port calls as `NAME! (…)` (the `!` is invisible to the token
   check): `AUTO_STRING`, `AUTO_STRING_WITH_LEN`, `BVAR`, `CALLN`, `SAFE_ALLOCA`,
   `SAFE_ALLOCA_STRING`, `SAFE_FREE`, `BUF_Z`/`BUF_GPT_BYTE`/`BUF_BEG_ADDR`/`BUF_MODIFF`/
   `BUF_SAVE_MODIFF`/`BUF_AUTOSAVE_MODIFF`, `FETCH_BYTE`, `XSETBUFFER`, `XSETFASTINT`,
   `XSETINT`, `CAR_SAFE`/`CDR_SAFE` (dead), `IS_ABSOLUTE_FILE_NAME`, `IS_ANY_SEP`,
   `S_ISDIR`/`S_ISLNK`/`S_ISREG`, `S_TYPEISSHM`/`S_TYPEISTMO`, `TYPE_MAXIMUM`, `countof`,
   `eassert`, `static_assert`, `TEMP_SET_PT_BOTH`, the seven `emacs_fd_*` file macros,
   `emacs_fclose`, `DEFSYM`/`DEFVAR_LISP`/`DEFVAR_BOOL`/`DEFUN`, `GCALIGNED`,
   `FOR_EACH_LIVE_BUFFER`/`FOR_EACH_TAIL` (expanded at the use site, P-22/§3.7).
2. **C's `Q` symbols**, constants that the port spells as the globals.rs per-symbol inline
   functions (`Qfile_error` in C, `Qfile_error ()` in the port - P-17), 77 names.
3. **Callees that exist only in C's dead arms** (P-08): `Fdowncase`, `SELECTED_FRAME`,
   `DEAFUN`, `alloca`, `chmod`, `getcwd`, `strcat`, `strchr`, `strncmp`, `memcmp`,
   `emacs_abort`, `make_string`, `concat2`, `ioctl`, `android_get_home_directory`,
   `getdefdir`, `dostounix_filename`, the eight `acl_*`/`context_*`/`freecon`/`*filecon`
   SELinux entries, `is_selinux_enabled`, `acl_errno_valid`, `report_file_notify_error`,
   `selinux_enabled_p`, `internal_delete_file(_1)`, `emacs_openat`, `emacs_root_dir`,
   `w32_accessible_directory_p`, `w32_copy_file`, `window_system_available`,
   `filename_from_ansi`, `Ffile_name_case_insensitive_p`, `c_isalpha`/`c_tolower`/`c_toupper`.
4. **Rust spellings of C constructs** - not call-relationship changes: pointer arithmetic and
   null tests (`offset`, `offset_from`, `as_ptr`, `as_mut_ptr`, `is_null`, `null`, `null_mut`),
   `std::mem::{zeroed, size_of_val}`, the two `ckd_*` operators (`overflowing_sub`/
   `overflowing_mul`), the `likely`/`POWER_OF_2`/`circular_list` helpers the mirror's
   `FOR_EACH_TAIL` expansion uses, `Some (…)` for C's bare function-pointer arguments
   (`pop_message_unwind`, `save_restriction_restore`, `read_non_regular`, `auto_save_1`,
   `close_file_unwind`, …), and the two names this unit adds with no C counterpart:
   `c_str_32` (the `[c_char; 32]` initializer) and `memory_full_wrapper` (the `-> !`
   `memory_full` cannot be a `fn (size_t)` field - measured).

### What no gate can see

(a) The `#[cfg (feature = "have_android")]` arm in `check_vfs_filename` is never compiled by
any gate this unit runs (`build-tu` and `enable_checking` both leave it out), so a typo inside
it would only surface if a future Android configuration is checked in.  (b) The six bitfield
writes (above) are invisible to the token check except through the notes rows: a wrong
`*_SHIFT` would still mention the C field's token, and only the C-side layout probe pins the
shift.  (c) The value of the two multi-line intspecs and of the 55 docstrings is compared by
the *decoded* value; nothing checks that C's spliced whitespace in `Fset_file_modes`'
`(let ((file (read-file-name "File: ")))` prefix (three tabs) plus the continuation line's
tab+two spaces is what the port's literal holds except the digest comparison itself.
(d) `Fdo_auto_save`'s restructuring turns two `continue`s and one shrink-case `goto`-free
branch into `if`/`else` conditions; the token check compares token *sets*, so a swapped
condition (e.g. `<=` for `>`) is only caught because numbers count and the arms' tokens differ
- a semantically identical but differently-grouped condition would pass.  (e) `'handled`'s
label is never broken to, so if someone deleted the outer loop the port would still compile
(the DEFUN's body would then end on the `return`), and no gate would notice that the C
`handled:` name had stopped being visible.  (f) `kind_message`'s element type is C's
`[c_char; 32]` but the C text's `static` storage duration vs the port's in-function `static`
are equivalent only because the port's `c_str_32` pads with NULs - `report_file_error` reads
it as a C string, so a missing NUL would be a runtime over-read no gate sees.  (g) The port
calls `report_file_error`/`report_file_errno` as `-> !` and relies on C's `AVOID`; if a future
mirror declared them `-> ()` the definitions would still compile but the diverging-statement
trick in `file_metadata_errno` would break loudly.  (h) `build_annotations`' `goto loop_over_p`
is written as an inner `loop`; the recursion order of the two `FOR_EACH_TAIL` passes is what C
executes and no gate compares the *order* of those two loops' bodies beyond their tokens.

### `check-undef.py` - what this unit owes to units that are not ported yet

`nm -u build/femacs/fileio.o` references twelve names that no configuration can satisfy yet,
all of them definitions in C files that are not ported - `_Fsubstring_no_properties` (fns.c),
`_Vselect_safe_coding_system_function`, `_coding_system_require_warning`,
`_decode_coding_gap`, `_detect_coding_system`, `_string_ascii_p` (coding.c), `_merge` (fns.c),
`_push_message`, `_pop_message_unwind`, `_restore_message` (xdisp.c), `_sit_for` (dispnew.c)
and `_record_auto_save` (keyboard.c).  They are the natural "the unit that defines it is not
ported yet" entries and belong in the `docs/baseline/undef-pending.txt` ratchet with
`python3 tools/check-undef.py --write-pending` (the integrator re-takes the ratchet in an
isolated worktree; this lane did not edit `docs/baseline/`).  The same run also reports nine
ratchet rows as **stale (now satisfied)** - `_Vauto_save_list_file_name`,
`_Vdefault_file_name_coding_system`, `_Vfile_name_coding_system`, `_Vface_font_rescale_alist`,
`_Vface_ignored_fonts`, `_Vgc_cons_percentage`, `_Vpost_gc_hook`, `_Vpurify_flag` and one more
- because this unit now *defines* the fifteen `DEFVAR_*` carriers it registers (status 3.29),
so the pending entries for the extern declarations of those names are satisfied and can be
dropped.
