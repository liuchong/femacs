# emacs/src/androidvfs.c (7950 lines)

> **This file is not compiled by the frozen configuration** (docs/baseline/frozen-build-sources.txt, docs/status.md 3.31): 222 declaration(s) are deferred to the platform target, and the port is an empty unit plus the note that names the gate.  Nothing in this file is required here.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 20
- [x] android_afs_open_fd: struct android_afs_open_fd* next, int fd, struct stat statb
- [x] android_afs_vdir: struct android_vdir vdir, struct android_afs_vdir* next, char* asset_dir, char* asset_limit, char* asset_file, int fd
- [x] android_afs_vnode: struct android_vnode vnode, size_t name_length, char* name
- [x] android_authority_vnode: struct android_vnode vnode, char* uri
- [x] android_content_vdir: struct android_vdir vdir, struct android_content_vdir* next, const char** next_name, int fd
- [x] android_cursor_class: jclass class, jmethodID close
- [x] android_fd_or_asset: struct android_fd_or_asset fd, AAsset* asset, int rc, struct android_vnode* vp
- [x] android_parcel_fd: struct android_parcel_fd* next, jobject descriptor, struct timespec mtime, int fd
- [x] android_root_vdir: struct android_vdir vdir, DIR* directory, int index
- [x] android_saf_root_vdir: struct android_vdir vdir, struct android_saf_root_vdir* next, jobjectArray array, char* authority, jsize length, jsize i, int fd
- [x] android_saf_root_vnode: struct android_vnode vnode, char* authority
- [x] android_saf_tree_vdir: struct android_vdir vdir, struct android_saf_tree_vdir* next, char* name, jobject cursor, int fd
- [x] android_saf_tree_vnode: struct android_vnode vnode, const char* tree_uri, char* tree_id, char* document_id, char* name
- [x] android_special_vnode: const char* name, size_t length, struct android_vnode * (*) initial, Lisp_Object special_coding_system
- [x] android_unix_vdir: struct android_vdir vdir, DIR* directory
- [x] android_unix_vnode: struct android_vnode vnode, size_t name_length, char* name
- [x] android_vdir: struct dirent * (*) readdir, void (*) closedir, int (*) dirfd
- [x] android_vnode: struct android_vops* ops, short type, short flags
- [x] android_vops: struct android_vnode * (*) name, int (*) open, void (*) close, int (*) unlink, int (*) symlink, int (*) rmdir, int (*) rename, int (*) stat, int (*) access, int (*) mkdir, int (*) chmod, ssize_t (*) readlink, struct android_vdir * (*) opendir
- [x] emacs_directory_entry_class: jclass class, jfieldID d_type, jfieldID d_name

## File-scope objects — 0

## Decisions

- **The frozen configuration does not compile this file** (docs/baseline/frozen-build-sources.txt,
  generated from `make -n` in `build/emacs-ref/src`): this target's platform object variables
  (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`) are all empty, so the file
  is not in the build's source list at all, and its text needs headers this target has not got.
  The port is therefore the empty unit `femacs/src/androidvfs.rs` — C's own license header plus the
  note that names the exclusion and points here — and the declarations above are **deferred to
  the platform target** rather than dropped (docs/status.md §3.31).
