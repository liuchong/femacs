// femacs: the <a.out.h> declarations lib/getloadavg.c's live arm needs — a system-header
// mirror in the sense of include/sys_mount.rs (`<sys/mount.h>`) and include/sys_acl.rs.
//
// The frozen target has **no** <a.out.h> (`clang -E` on a file that includes it reports
// "'a.out.h' file not found"), which is also why the reference build never compiles
// lib/getloadavg.c: gnulib only builds the module when the C library lacks `getloadavg`, and
// this target has it (`HAVE_GETLOADAVG 1` in the frozen config.h; `build/emacs-ref/lib/`
// holds `nproc.o` but no `getloadavg.o`).  The file is still ported — the port covers every
// reference file (docs/port-plan.md, P-42) — and `--check` compares it against the frozen
// configuration's live text.
//
// Which shape `struct nlist` has is decided by the C text's own configuration, not by
// <nlist.h>: `NLIST_STRUCT` and `N_NAME_POINTER` are both undefined on this target, so
// `getloadavg.c` 796-797 does `strcpy (name_list[0].n_name, LDAV_SYMBOL)` — for that to
// typecheck `n_name` must be an *array*, which is the 4.3BSD/SunOS <a.out.h>
// `char n_name[NCPLN]` (the modern <nlist.h> the other branch would take spells
// `union { char *n_name; long n_value; } n_un`, a pointer).  Nothing on this target can
// measure the a.out.h layout (`clang` cannot even read the header), so this type is not in
// tools/gen-layout-fields.py's list and the layout gate reports it as pending rather than
// asserting an offset.

/// <a.out.h>: the name array's width (`# define NCPLN 8` in the 4.3BSD/SunOS header this
/// configuration selects).
pub const NCPLN: usize = 8;

/// <a.out.h>'s `struct nlist` — see the note above on why `n_name` is an array here.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct nlist {
  /// `char n_name[NCPLN];`
  pub n_name: [c_char; NCPLN],
  /// `unsigned char n_type;`
  pub n_type: c_uchar,
  /// `char n_other;`
  pub n_other: c_char,
  /// `short n_desc;`
  pub n_desc: c_short,
  /// `unsigned long n_value;` — the kernel symbol's address, which `getloadavg` reads into
  /// `offset`.
  pub n_value: c_ulong,
}

unsafe extern "C" {
  /// <a.out.h>: `int nlist (const char *, struct nlist *);`.
  pub fn nlist (filename: *const c_char, nl: *mut nlist) -> c_int;
}
