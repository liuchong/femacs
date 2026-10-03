# emacs/src/android.c (7594 lines)

> **This file is not compiled by the frozen configuration** (docs/baseline/frozen-build-sources.txt, docs/status.md 3.31): 195 declaration(s) are deferred to the platform target, and the port is an empty unit plus the note that names the gate.  Nothing in this file is required here.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 10
- [x] android_emacs_cursor: jclass class, jmethodID constructor
- [x] android_emacs_drawable: jclass class, jmethodID get_bitmap
- [x] android_emacs_handle: jclass class, jmethodID destroy_handle, jfieldID handle
- [x] android_emacs_pixmap: jclass class, jmethodID constructor_mutable
- [x] android_emacs_window: jclass class, jmethodID swap_buffers, jmethodID toggle_on_screen_keyboard, jmethodID lookup_string, jmethodID set_fullscreen, jmethodID change_window_background, jmethodID reparent_to, jmethodID map_window, jmethodID unmap_window, jmethodID resize_window, jmethodID move_window, jmethodID move_resize_window, jmethodID make_input_focus, jmethodID raise, jmethodID lower, jmethodID reconfigure, jmethodID get_window_geometry, jmethodID translate_coordinates, jmethodID set_dont_accept_focus, jmethodID set_dont_focus_on_map, jmethodID define_cursor, jmethodID damage_rect, jmethodID recreate_activity, jmethodID clear_window, jmethodID clear_area, jmethodID set_wm_name
- [x] android_event_container: struct android_event_container* next, struct android_event_container* last, union android_event event
- [x] android_event_queue: pthread_mutex_t mutex, pthread_cond_t read_var, struct android_event_container events, struct android_thread_event_queue thread, int num_events
- [x] android_graphics_point: jclass class, jmethodID constructor
- [x] android_key_character_map: jclass class, jmethodID get_dead_char
- [x] android_thread_event_queue: pthread_mutex_t select_mutex, pthread_t select_thread, fd_set* select_readfds, fd_set* select_writefds, fd_set* select_exceptfds, struct timespec* select_timeout, int select_nfds, sem_t start_sem, sem_t select_sem, int select_rc, int select_pipe, int canceled

## File-scope objects — 0

## Decisions

- **The frozen configuration does not compile this file** (docs/baseline/frozen-build-sources.txt,
  generated from `make -n` in `build/emacs-ref/src`): this target's platform object variables
  (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`) are all empty, so the file
  is not in the build's source list at all, and its text needs headers this target has not got.
  The port is therefore the empty unit `femacs/src/android.rs` — C's own license header plus the
  note that names the exclusion and points here — and the declarations above are **deferred to
  the platform target** rather than dropped (docs/status.md §3.31).
