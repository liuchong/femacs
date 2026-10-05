// femacs: port of src/w32heap.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The header is the NT heap layer's interface: C 22-23 is the
// include guard, C 25 the one system header it includes, C 31-45 the heap entry points,
// C 50-56 the `struct file_data` record the memory-mapped-file helpers fill, and C 58-67 the
// five routines that work on it.  No `#if` anywhere
// (`tools/check-cond-eval.py src/w32heap.h` -> `1 conditions, 0 unknown, 0 disagreements
// [VERIFIED]`, the guard), so every line below is live text.
//
// Nothing on this target compiles it: the frozen configuration's `WINDOW_SYSTEM_OBJ` is empty
// (docs/baseline/frozen-build-sources.txt), so `src/w32heap.c`'s port is the shape
// `femacs/src/w32heap.rs` carries and no unit `include!`s this mirror.  The gateway declarations
// are deferred to the Windows target with their definitions.
//
// **The platform's own typedefs are spelled as the port's primitives, with C's text kept in a
// `/// C:` line.**  The header gets `HANDLE`, `IMAGE_SECTION_HEADER` and `DWORD_PTR` from
// <windows.h> (C 25); the port has no windows.h mirror, so each name below is written the way
// PORTING.md's type table spells it - `HANDLE` is `void *`, the two `IMAGE_*` names are only
// ever pointers to records no mirror owns, and `DWORD_PTR` is C's pointer-sized unsigned
// (`uintptr_t`'s width, PORTING.md 1.1).  Arity, pointer depth, `const`-ness and the
// `int`/`void` returns are C's; the `/// C:` line above each declaration is the reference text
// (PORTING.md P-55), which is what keeps the substitution auditable.
//
// `struct file_data`'s layout is **not** asserted here: the assertions `xtask layout-check`
// compares come from a C compiler, and no compiler on this target has a <windows.h>.  A Windows
// target that freezes this header must add the record to `tools/gen-layout-fields.py` and
// measure it there.

// C 22-23: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const NTHEAP_H_: bool = true;

/*
 * Heap related stuff.
 */

/// C 31-42: the heap's edges, its one size knob, and the three mmap-shaped allocators
/// (`sbrk` is C's emulation of the Unix call, C 42's `ptrdiff_t` is the port's `isize`).
/// C: extern unsigned char *get_data_start (void);
/// C: extern unsigned char *get_data_end (void);
/// C: extern size_t         reserved_heap_size;
/// C: extern void *mmap_realloc (void **, size_t);
/// C: extern void  mmap_free (void **);
/// C: extern void *mmap_alloc (void **, size_t);
unsafe extern "C" {
  pub fn get_data_start () -> *mut u8;
  pub fn get_data_end () -> *mut u8;
  pub static mut reserved_heap_size: usize;

  /// C 39: `report_temacs_memory_usage` prints the heap's state for the `temacs` build.
  /// C: extern void report_temacs_memory_usage (void);
  pub fn report_temacs_memory_usage ();

}

/* C 36-38/41-45's five entry points `src/w32heap.c` **defines** (C 300-310's `sbrk`, C 100-151's
   `init_heap` and the three mmap-shaped allocators) are declared in a module the mirror
   glob-imports (P-39): a copy at the top level is `E0428` in that unit - measured: five of them
   on `src/w32heap.rs`'s flip (status 3.504).  A glob import is shadowed by the including unit's
   own definition, which is the property this class needs.  */
#[cfg (feature = "windowsnt")]
pub mod w32heap_decls {
  use super::*;

  unsafe extern "C" {
    /// C: extern void *mmap_realloc (void **, size_t);
    pub fn mmap_realloc (block: *mut *mut c_void, size: usize) -> *mut c_void;
    /// C: extern void  mmap_free (void **);
    pub fn mmap_free (block: *mut *mut c_void);
    /// C: extern void *mmap_alloc (void **, size_t);
    pub fn mmap_alloc (block: *mut *mut c_void, size: usize) -> *mut c_void;
    /// C: extern void *sbrk (ptrdiff_t size);
    pub fn sbrk (size: isize) -> *mut c_void;
    /// C: extern void init_heap (void);
    pub fn init_heap ();
  }
}
#[cfg (feature = "windowsnt")]
use w32heap_decls::*;

/* ----------------------------------------------------------------- */
/* Useful routines for manipulating memory-mapped files.  */

/// C 50-56: `typedef struct file_data { … } file_data;`.  `HANDLE` is the platform's opaque
/// handle, which is `void *` at the ABI; `unsigned long` is PORTING.md 1.1's `c_ulong`.
/// C: typedef struct file_data {
/// C:     char          *name;
/// C:     unsigned long  size;
/// C:     HANDLE         file;
/// C:     HANDLE         file_mapping;
/// C:     unsigned char *file_base;
/// C: } file_data;
#[repr(C)]
pub struct file_data
{
  pub name: *mut c_char,
  pub size: c_ulong,

  /// C: `HANDLE         file;`
  pub file: *mut c_void,

  /// C: `HANDLE         file_mapping;`
  pub file_mapping: *mut c_void,

  pub file_base: *mut u8,
}

/// C 58-60: the three entry points the memory-mapped-file helpers share.
/// C: int open_input_file (file_data *p_file, char *name);
/// C: int open_output_file (file_data *p_file, char *name, unsigned long size);
/// C: void close_file_data (file_data *p_file);
unsafe extern "C" {
  pub fn open_input_file (p_file: *mut file_data, name: *mut c_char) -> c_int;
  pub fn open_output_file (p_file: *mut file_data, name: *mut c_char, size: c_ulong) -> c_int;
  pub fn close_file_data (p_file: *mut file_data);
}

/* Return pointer to section header for named section.  */
/// C 63: `IMAGE_SECTION_HEADER * find_section (const char *, IMAGE_NT_HEADERS *);`
/// Both `IMAGE_SECTION_HEADER` and `IMAGE_NT_HEADERS` are <windows.h>'s records, only ever
/// pointers here, and neither has an owner mirror - hence `*mut c_void` (PORTING.md P-55).
/// C: IMAGE_SECTION_HEADER * find_section (const char *, IMAGE_NT_HEADERS *);
unsafe extern "C" {
  pub fn find_section (name: *const c_char, nt_headers: *mut c_void) -> *mut c_void;
}

/* Return pointer to section header for section containing the given
   relative virtual address.  */
/// C 67: `IMAGE_SECTION_HEADER * rva_to_section (DWORD_PTR, IMAGE_NT_HEADERS *);` - the first
/// argument is the RVA, which `DWORD_PTR` makes pointer-sized.
/// C: IMAGE_SECTION_HEADER * rva_to_section (DWORD_PTR, IMAGE_NT_HEADERS *);
unsafe extern "C" {
  pub fn rva_to_section (rva: usize, nt_headers: *mut c_void) -> *mut c_void;
}
