// femacs: port of src/lcms.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  `HAVE_LCMS2` is `undef` in the frozen configuration
// (docs/baseline/config/darwin-arm64.macros.tsv 1412), so C compiles nothing from this file but
// C 19's `<config.h>`; this is a P-86 unit, and **both arms of every `#if` are carried**, each item
// under the gate that names C's own macro:
//
//   * the file gate (`#ifdef HAVE_LCMS2`, C 21) is `feature = "have_lcms2"`, on every item *and on
//     C 23-26's include graph* - which is why the frozen configuration compiles nothing here;
//   * C 35-90's `#ifdef WINDOWSNT` arm is `feature = "windowsnt"` (nested inside the file gate),
//     the name `src/vm-limit.rs` 95 already uses for that configuration, and C 125-133, 329-337,
//     366-374, 406-414, 447-455, 502-510, 547-555 and 570-584's blocks inside the eight DEFUNs are
//     nested under it too.
//
// C 23's `<lcms2.h>` has no file in the reference tree, so this unit carries the declarations it
// uses itself (PORTING.md P-88; the layout numbers are clang readings, not memory, and
// `docs/ports/lcms.md` records them).  C 24's `<math.h>` does have a mirror
// (`femacs/lib/include/math.rs`), which carries `M_PI`/`hypot`/`cbrt` since this batch landed
// (they were use-site declarations until then; P-26's single declaration point).  C 40-52's
// `DEF_DLL_FN` calls are the macro `emacs/src/w32common.h` 77-79 defines and
// `femacs/src/include/w32common.rs` keeps `deferred` (P-22); this unit is its second consumer and
// writes the expansion in long form, the way `femacs/src/decompress.rs` (the first) does.  C 82-88
// renames seven identifiers with `#define`; an object-like `#define` has no Rust counterpart, so
// the arms spell the call sites the way C's preprocessor leaves them (see the arm's note, and
// `docs/ports/lcms.md`'s `## Deviations`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");

/* C 24's `<math.h>` and C 26's `"lisp.h"`, inside the file gate.  <math.h> has a mirror, included
   here the way `src/floatfns.rs` includes it (that file's own `include!` line).  */
#[cfg (feature = "have_lcms2")]
include!("../lib/include/math.rs");             // C 24: `#include <math.h>`
#[cfg (feature = "have_lcms2")]
include!("include/lisp.rs");                    // C 26: `#include "lisp.h"`

/* C 36-38's `#include <windows.h>`, `"w32common.h"` and `"w32.h"`.  `<windows.h>` has no mirror,
   so `HMODULE` is the pointer PORTING.md 0.2 maps it to (`*mut c_void`, the spelling
   `femacs/src/include/w32common.rs` 137 already uses for `hbfont_init_w32_funcs`).  */
#[cfg (all (feature = "have_lcms2", feature = "windowsnt"))]
include!("include/w32common.rs");               // C 37
#[cfg (all (feature = "have_lcms2", feature = "windowsnt"))]
include!("include/w32.rs");                     // C 38

/* ----------------------------------------------------------------------------------------------
   C 23: `#include <lcms2.h>` - the reference tree has no `lcms2.h` (P-88: a third-party header
   with no file anywhere in the tree), so the declarations this unit uses stand here rather than in
   a new file under `femacs/src/include/`, which is the *reference* headers' mirror directory.
   Every declaration carries the header's own line (P-55).

   Measured with clang against this machine's header (`/opt/homebrew/include/lcms2.h` ->
   `Cellar/little-cms2/2.19.1/include/lcms2.h`, `LCMS_VERSION` 2190):
   `sizeof (cmsCIELab)` **24** (offsets `L` 0, `a` 8, `b` 16), `sizeof (cmsCIEXYZ)` **24** (`X` 0,
   `Y` 8, `Z` 16), `sizeof (cmsJCh)` **24** (`J` 0, `C` 8, `h` 16), `sizeof (cmsCIExyY)` **24**
   (`x` 0, `y` 8, `Y` 16), `sizeof (cmsViewingConditions)` **56** (offsets `whitePoint` 0, `Yb` 24,
   `La` 32, `surround` 40, `D_value` 48); `sizeof (cmsFloat64Number)` **8**, `cmsHANDLE` **8**,
   `cmsContext` **8**, `cmsBool` **4**, `cmsUInt32Number` **4**; `AVG_SURROUND` **1**,
   `DIM_SURROUND` **2**, `DARK_SURROUND` **3**, `CUTSHEET_SURROUND` **4**.  The probe source, its
   command and its output are in docs/ports/lcms.md's `## Evidence`.  */

/// C: lcms2.h 112: `typedef double               cmsFloat64Number;`
#[cfg (feature = "have_lcms2")]
pub type cmsFloat64Number = c_double;
/// C: lcms2.h 190: `typedef int                  cmsBool;`
#[cfg (feature = "have_lcms2")]
pub type cmsBool = c_int;
/// C: lcms2.h 133: `    typedef unsigned int        cmsUInt32Number;`
#[cfg (feature = "have_lcms2")]
pub type cmsUInt32Number = c_uint;
/// C: lcms2.h 686: `typedef void* cmsHANDLE ;              // Generic handle`
#[cfg (feature = "have_lcms2")]
pub type cmsHANDLE = *mut c_void;
/// C: lcms2.h 1109: `typedef struct _cmsContext_struct* cmsContext;` - an incomplete type named
/// only through a pointer (PORTING.md 0.1-2's zero-sized stand-in).
#[cfg (feature = "have_lcms2")]
pub struct _cmsContext_struct { _private: [u8; 0] }
/// C: lcms2.h 1109: the pointer type itself.
#[cfg (feature = "have_lcms2")]
pub type cmsContext = *mut _cmsContext_struct;

/// C: lcms2.h 991-996: `typedef struct {` / `        cmsFloat64Number X;` / `        cmsFloat64Number
/// Y;` / `        cmsFloat64Number Z;` / `    } cmsCIEXYZ;`  `Copy` is C's trivial copyability
/// (PORTING.md 1.3), which the two DEFUNs' `xyzw = illuminant_d65;` needs.
#[cfg (feature = "have_lcms2")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cmsCIEXYZ
{
  pub X: cmsFloat64Number,
  pub Y: cmsFloat64Number,
  pub Z: cmsFloat64Number,
}
/// C: lcms2.h 998-1003: `typedef struct {` / `        cmsFloat64Number x;` / `        cmsFloat64Number
/// y;` / `        cmsFloat64Number Y;` / `    } cmsCIExyY;`
#[cfg (feature = "have_lcms2")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cmsCIExyY
{
  pub x: cmsFloat64Number,
  pub y: cmsFloat64Number,
  pub Y: cmsFloat64Number,
}
/// C: lcms2.h 1005-1010: `typedef struct {` / `        cmsFloat64Number L;` / `        cmsFloat64Number
/// a;` / `        cmsFloat64Number b;` / `    } cmsCIELab;`
#[cfg (feature = "have_lcms2")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cmsCIELab
{
  pub L: cmsFloat64Number,
  pub a: cmsFloat64Number,
  pub b: cmsFloat64Number,
}
/// C: lcms2.h 1019-1024: `typedef struct {` / `        cmsFloat64Number J;` / `        cmsFloat64Number
/// C;` / `        cmsFloat64Number h;` / `    } cmsJCh;`
#[cfg (feature = "have_lcms2")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cmsJCh
{
  pub J: cmsFloat64Number,
  pub C: cmsFloat64Number,
  pub h: cmsFloat64Number,
}
/// C: lcms2.h 1212-1219: `typedef struct {` / `    cmsCIEXYZ        whitePoint;` / `    cmsFloat64Number
/// Yb;` / `    cmsFloat64Number La;` / `    cmsUInt32Number  surround;` / `    cmsFloat64Number
/// D_value;` / `    } cmsViewingConditions;`
#[cfg (feature = "have_lcms2")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cmsViewingConditions
{
  pub whitePoint: cmsCIEXYZ,
  pub Yb: cmsFloat64Number,
  pub La: cmsFloat64Number,
  pub surround: cmsUInt32Number,
  pub D_value: cmsFloat64Number,
}

/// C: lcms2.h 1205: `#define AVG_SURROUND       1`
#[cfg (feature = "have_lcms2")]
pub const AVG_SURROUND: c_int = 1;
/// C: lcms2.h 1206: `#define DIM_SURROUND       2`
#[cfg (feature = "have_lcms2")]
pub const DIM_SURROUND: c_int = 2;
/// C: lcms2.h 1207: `#define DARK_SURROUND      3`
#[cfg (feature = "have_lcms2")]
pub const DARK_SURROUND: c_int = 3;
/// C: lcms2.h 1208: `#define CUTSHEET_SURROUND  4`
#[cfg (feature = "have_lcms2")]
pub const CUTSHEET_SURROUND: c_int = 4;

// C: lcms2.h 1168: `CMSAPI void              CMSEXPORT cmsxyY2XYZ(cmsCIEXYZ* Dest, const cmsCIExyY* Source);`
// C: lcms2.h 1187: `CMSAPI cmsFloat64Number  CMSEXPORT cmsCIE2000DeltaE(const cmsCIELab* Lab1, const cmsCIELab* Lab2, cmsFloat64Number Kl, cmsFloat64Number Kc, cmsFloat64Number Kh);`
// C: lcms2.h 1190: `CMSAPI cmsBool           CMSEXPORT cmsWhitePointFromTemp(cmsCIExyY* WhitePoint, cmsFloat64Number  TempK);`
// C: lcms2.h 1221: `CMSAPI cmsHANDLE         CMSEXPORT cmsCIECAM02Init(cmsContext ContextID, const cmsViewingConditions* pVC);`
// C: lcms2.h 1222: `CMSAPI void              CMSEXPORT cmsCIECAM02Done(cmsHANDLE hModel);`
// C: lcms2.h 1223: `CMSAPI void              CMSEXPORT cmsCIECAM02Forward(cmsHANDLE hModel, const cmsCIEXYZ* pIn, cmsJCh* pOut);`
// C: lcms2.h 1224: `CMSAPI void              CMSEXPORT cmsCIECAM02Reverse(cmsHANDLE hModel, const cmsJCh* pIn,    cmsCIEXYZ* pOut);`
#[cfg (feature = "have_lcms2")]
unsafe extern "C" {
  pub fn cmsxyY2XYZ (Dest: *mut cmsCIEXYZ, Source: *const cmsCIExyY);
  pub fn cmsCIE2000DeltaE (Lab1: *const cmsCIELab, Lab2: *const cmsCIELab,
                           Kl: cmsFloat64Number, Kc: cmsFloat64Number, Kh: cmsFloat64Number)
                           -> cmsFloat64Number;
  pub fn cmsWhitePointFromTemp (WhitePoint: *mut cmsCIExyY, TempK: cmsFloat64Number) -> cmsBool;
  pub fn cmsCIECAM02Init (ContextID: cmsContext, pVC: *const cmsViewingConditions) -> cmsHANDLE;
  pub fn cmsCIECAM02Done (hModel: cmsHANDLE);
  pub fn cmsCIECAM02Forward (hModel: cmsHANDLE, pIn: *const cmsCIEXYZ, pOut: *mut cmsJCh);
  pub fn cmsCIECAM02Reverse (hModel: cmsHANDLE, pIn: *const cmsJCh, pOut: *mut cmsCIEXYZ);
}

/* C 24's `<math.h>` reaches this unit through `femacs/lib/include/math.rs`; `M_PI` (math.h 751),
   `cbrt` (455) and `hypot` (459) were declared at this use site (P-72 rule 4) until the integrator
   moved them into that mirror in the same commit - the single-declaration-point rule (P-26) is why
   the local copies are gone.  */

/// C 28-33: `typedef struct { double J; double a; double b; } lcmsJab_t;` - the file's own type,
/// not the header's.
#[cfg (feature = "have_lcms2")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct lcmsJab_t
{
  pub J: c_double,
  pub a: c_double,
  pub b: c_double,
}

/* ----------------------------------------------------------------------------------------------
   C 35-90: the `#ifdef WINDOWSNT` arm - the lazy loading of the seven lcms2 entry points from the
   DLL.  `WINDOWSNT` is `undef` here, so no line below is compiled on this target (P-51 names the
   non-frozen side), and this machine has no Windows cross toolchain (no `*-w64-mingw32-gcc`), so
   this arm's evidence is `text-only` (P-86 clause 3 (2)): there is no compile reading of it.  */

/// C: w32common.h 77-79: `#define DEF_DLL_FN(type, func, ...)  typedef type (CDECL *W32_PFN_##func)`
/// `__VA_ARGS__; static W32_PFN_##func fn_##func` - the macro
/// `femacs/src/include/w32common.rs` keeps `deferred` (P-22).  `femacs/src/decompress.rs` (the
/// first consumer) writes the expansion in long form and this unit repeats it, because a unit is
/// its own crate; a third consumer is what should move it into a shared prelude.  C pastes
/// `W32_PFN_##func` and `fn_##func` together, which Rust cannot do, so both generated names are
/// parameters here and C's `__VA_ARGS__` (the parameter list, with any trailing attributes) is the
/// `$args` fragment.
#[cfg (all (feature = "have_lcms2", feature = "windowsnt"))]
macro_rules! DEF_DLL_FN {
  ($type:ty, $func:ident, $pfn:ident, $fnvar:ident, ($($arg:ty),* $(,)?)) => {
    pub type $pfn = Option<unsafe extern "C" fn ($($arg),*) -> $type>;
    static mut $fnvar: $pfn = None;
  };
}
/// C: w32common.h 82-89: `#define LOAD_DLL_FN(lib, func)       do { fn_##func = (W32_PFN_##func)`
/// `get_proc_addr (lib, #func); if (!fn_##func) return false; } while (false)`.  `do { … } while
/// (false)` is a block expression (PORTING.md 2.1); C's `#func` is the stringified name, which is
/// also a parameter here for the same reason as above; the assignment's cast between two function
/// pointer types is C's, and Rust has no `as` cast to an `Option<fn>` type, so it is the
/// `transmute` that the equal widths make the same operation.  `get_proc_addr` is the mirror's own
/// inline definition (`femacs/src/include/w32common.rs` 128-136).
#[cfg (all (feature = "have_lcms2", feature = "windowsnt"))]
macro_rules! LOAD_DLL_FN {
  ($lib:expr, $func:ident, $pfn:ident, $fnvar:ident, $name:expr) => {
    {
      $fnvar = std::mem::transmute::<VOIDFNPTR, $pfn> (get_proc_addr ($lib, $name.as_ptr ()));
      if $fnvar.is_none () { return false; }
    }
  };
}

/* C 40-52: the seven `DEF_DLL_FN` uses, each expanding to the two names C's macro would paste.
   C's `CDECL` is the `__cdecl` attribute, which is empty on this target's ABI, so the pointer type
   is the plain `extern "C"` one.  */
#[cfg (all (feature = "have_lcms2", feature = "windowsnt"))]
DEF_DLL_FN! (cmsFloat64Number, cmsCIE2000DeltaE, W32_PFN_cmsCIE2000DeltaE, fn_cmsCIE2000DeltaE,
             (*const cmsCIELab, *const cmsCIELab, cmsFloat64Number, cmsFloat64Number,
              cmsFloat64Number));
#[cfg (all (feature = "have_lcms2", feature = "windowsnt"))]
DEF_DLL_FN! (cmsHANDLE, cmsCIECAM02Init, W32_PFN_cmsCIECAM02Init, fn_cmsCIECAM02Init,
             (cmsContext, *const cmsViewingConditions));
#[cfg (all (feature = "have_lcms2", feature = "windowsnt"))]
DEF_DLL_FN! ((), cmsCIECAM02Forward, W32_PFN_cmsCIECAM02Forward, fn_cmsCIECAM02Forward,
             (cmsHANDLE, *const cmsCIEXYZ, *mut cmsJCh));
#[cfg (all (feature = "have_lcms2", feature = "windowsnt"))]
DEF_DLL_FN! ((), cmsCIECAM02Reverse, W32_PFN_cmsCIECAM02Reverse, fn_cmsCIECAM02Reverse,
             (cmsHANDLE, *const cmsJCh, *mut cmsCIEXYZ));
#[cfg (all (feature = "have_lcms2", feature = "windowsnt"))]
DEF_DLL_FN! ((), cmsCIECAM02Done, W32_PFN_cmsCIECAM02Done, fn_cmsCIECAM02Done, (cmsHANDLE));
#[cfg (all (feature = "have_lcms2", feature = "windowsnt"))]
DEF_DLL_FN! (cmsBool, cmsWhitePointFromTemp, W32_PFN_cmsWhitePointFromTemp,
             fn_cmsWhitePointFromTemp, (*mut cmsCIExyY, cmsFloat64Number));
#[cfg (all (feature = "have_lcms2", feature = "windowsnt"))]
DEF_DLL_FN! ((), cmsxyY2XYZ, W32_PFN_cmsxyY2XYZ, fn_cmsxyY2XYZ,
             (*mut cmsCIEXYZ, *const cmsCIExyY));

/// C 54: `static bool lcms_initialized;` - C's tentative definition, i.e. zero (PORTING.md 0.1-13).
#[cfg (all (feature = "have_lcms2", feature = "windowsnt"))]
static mut lcms_initialized: bool = false;

/// C 56-72: `init_lcms_functions`.  `HMODULE` is a pointer; `w32_delayed_load` is the declaration
/// `femacs/src/include/w32.rs` 313-314 carries for `w32.h` 197.
#[cfg (all (feature = "have_lcms2", feature = "windowsnt"))]
unsafe fn init_lcms_functions () -> bool
{
  let library: *mut c_void = w32_delayed_load (Qlcms2 ());

  if library.is_null ()
    { return false; }

  LOAD_DLL_FN! (library, cmsCIE2000DeltaE, W32_PFN_cmsCIE2000DeltaE, fn_cmsCIE2000DeltaE,
                c"cmsCIE2000DeltaE");
  LOAD_DLL_FN! (library, cmsCIECAM02Init, W32_PFN_cmsCIECAM02Init, fn_cmsCIECAM02Init,
                c"cmsCIECAM02Init");
  LOAD_DLL_FN! (library, cmsCIECAM02Forward, W32_PFN_cmsCIECAM02Forward, fn_cmsCIECAM02Forward,
                c"cmsCIECAM02Forward");
  LOAD_DLL_FN! (library, cmsCIECAM02Reverse, W32_PFN_cmsCIECAM02Reverse, fn_cmsCIECAM02Reverse,
                c"cmsCIECAM02Reverse");
  LOAD_DLL_FN! (library, cmsCIECAM02Done, W32_PFN_cmsCIECAM02Done, fn_cmsCIECAM02Done,
                c"cmsCIECAM02Done");
  LOAD_DLL_FN! (library, cmsWhitePointFromTemp, W32_PFN_cmsWhitePointFromTemp,
                fn_cmsWhitePointFromTemp, c"cmsWhitePointFromTemp");
  LOAD_DLL_FN! (library, cmsxyY2XYZ, W32_PFN_cmsxyY2XYZ, fn_cmsxyY2XYZ, c"cmsxyY2XYZ");
  return true;
}

/* C 74-88: the seven `#undef`s and the seven `#define` renames.  The `#undef`s have no
   counterpart - the `W32_PFN_`/`fn_` names above are already the names C's macros make - and an
   object-like `#define` has none in Rust either: C 82-88 rename the seven `cms*` identifiers, so
   the arms spell the call sites the way C's preprocessor leaves them (`fn_cms… (...)` under this
   gate, `cms… (...)` otherwise; the `ccl.rs` "C's preprocessed text" rule), while the pointer
   objects keep C's own names.  */

/* ----------------------------------------------------------------------------------------------
   C 92-604: the rest of the file, under the file gate alone.  */

/// C 92-109: `parse_lab_list`.  C's `PARSE_LAB_LIST_FIELD` is a `#define` inside the function body;
/// the port keeps C's name as a `macro_rules!` defined in the same body (the `androidmenu.rs`
/// `FIND_METHOD` shape), and Rust's `macro_rules!` scope ends with that block, which is what C's
/// never-`#undef`ed `#define` does not do - the macro is used nowhere else.
#[cfg (feature = "have_lcms2")]
unsafe fn parse_lab_list (lab_list: Lisp_Object, color: *mut cmsCIELab) -> bool
{
  /* C's macro assigns the parameter, so the port rebinds it mutable before the
     `macro_rules!` that captures it (a macro's names resolve where it is defined).  */
  let mut lab_list = lab_list;

  macro_rules! PARSE_LAB_LIST_FIELD {
    ($field:ident) => {
      if CONSP (lab_list) && NUMBERP (XCAR (lab_list))
	{
	  (*color).$field = XFLOATINT (XCAR (lab_list));
	  lab_list = XCDR (lab_list);
	}
      else
	{ return false; }
    };
  }

  PARSE_LAB_LIST_FIELD! (L);
  PARSE_LAB_LIST_FIELD! (a);
  PARSE_LAB_LIST_FIELD! (b);

  return true;
}

/* http://www.ece.rochester.edu/~gsharma/ciede2000/ciede2000noteCRNA.pdf */

#[cfg (feature = "have_lcms2")]
DEFUN! ("lcms-cie-de2000", Flcms_cie_de2000, Slcms_cie_de2000, 2, 5, 0,
       doc: "Compute CIEDE2000 metric distance between COLOR1 and COLOR2.\nEach color is a list of L*a*b* coordinates, where the L* channel ranges from\n0 to 100, and the a* and b* channels range from -128 to 128.\nOptional arguments KL, KC, KH are weighting parameters for lightness,\nchroma, and hue, respectively. The parameters each default to 1.",
       (color1: Lisp_Object, color2: Lisp_Object,
	kL: Lisp_Object, kC: Lisp_Object, kH: Lisp_Object),
       {
  let mut Lab1: cmsCIELab = std::mem::zeroed ();
  let mut Lab2: cmsCIELab = std::mem::zeroed ();
  /* C declares `cmsFloat64Number Kl, Kc, Kh;` uninitialized and lets the branches below assign
     them; the port zeroes them first (PORTING.md 1.5/0.1-13).  */
  let mut Kl: cmsFloat64Number = 0.0;
  let mut Kc: cmsFloat64Number = 0.0;
  let mut Kh: cmsFloat64Number = 0.0;

  #[cfg (feature = "windowsnt")]
  {
    if !lcms_initialized
      { lcms_initialized = init_lcms_functions (); }
    if !lcms_initialized
      {
        message1 (c"lcms2 library not found".as_ptr ());
        return Qnil ();
      }
  }

  if !(CONSP (color1) && parse_lab_list (color1, &raw mut Lab1))
    { signal_error (c"Invalid color".as_ptr (), color1); }
  if !(CONSP (color2) && parse_lab_list (color2, &raw mut Lab2))
    { signal_error (c"Invalid color".as_ptr (), color1); }
  /* C 139-150: `else if (!(NUMBERP (kL) && (Kl = XFLOATINT(kL))))` - Rust has no assignment
     expression, so the assignment and C's truth test on its value are one block, evaluated in
     C's order (the `NUMBERP` test first, the assignment only if it held).  C 147 tests `kL` for
     `Kh`; that is C's text and the port keeps it.  */
  if NILP (kL)
    { Kl = 1.0f64; }
  else if !(NUMBERP (kL) && { Kl = XFLOATINT (kL); Kl != 0.0 })
    { wrong_type_argument (Qnumberp (), kL); }
  if NILP (kC)
    { Kc = 1.0f64; }
  else if !(NUMBERP (kC) && { Kc = XFLOATINT (kC); Kc != 0.0 })
    { wrong_type_argument (Qnumberp (), kC); }
  if NILP (kL)
    { Kh = 1.0f64; }
  else if !(NUMBERP (kH) && { Kh = XFLOATINT (kH); Kh != 0.0 })
    { wrong_type_argument (Qnumberp (), kH); }

  return make_float (cmsCIE2000DeltaE (&raw const Lab1, &raw const Lab2, Kl, Kc, Kh));
});

/// C 155-159: `deg2rad`.
#[cfg (feature = "have_lcms2")]
unsafe fn deg2rad (degrees: c_double) -> c_double
{
  return M_PI * degrees / 180.0;
}

/// C 161-165: `rad2deg`.
#[cfg (feature = "have_lcms2")]
unsafe fn rad2deg (radians: c_double) -> c_double
{
  return 180.0 * radians / M_PI;
}

/// C 167: `static cmsCIEXYZ illuminant_d65 = { .X = 95.0455, .Y = 100.0, .Z = 108.8753 };` - C's
/// designated initializer is the struct literal.
#[cfg (feature = "have_lcms2")]
static mut illuminant_d65: cmsCIEXYZ
  = cmsCIEXYZ { X: 95.0455, Y: 100.0, Z: 108.8753 };

/// C 169-179: `default_viewing_conditions`.  C's `vc->Yb = 20;`, `vc->La = 100;` and
/// `vc->D_value = 1.0;` widen integer literals to `cmsFloat64Number`; the port writes the widened
/// values.
#[cfg (feature = "have_lcms2")]
unsafe fn default_viewing_conditions (wp: *const cmsCIEXYZ, vc: *mut cmsViewingConditions)
{
  (*vc).whitePoint.X = (*wp).X;
  (*vc).whitePoint.Y = (*wp).Y;
  (*vc).whitePoint.Z = (*wp).Z;
  (*vc).Yb = 20.0;
  (*vc).La = 100.0;
  (*vc).surround = AVG_SURROUND as cmsUInt32Number;
  (*vc).D_value = 1.0;
}

/* FIXME: code duplication */

/// C 183-200: `parse_xyz_list`.
#[cfg (feature = "have_lcms2")]
unsafe fn parse_xyz_list (xyz_list: Lisp_Object, color: *mut cmsCIEXYZ) -> bool
{
  /* C's macro assigns the parameter, so the port rebinds it mutable before the
     `macro_rules!` that captures it (a macro's names resolve where it is defined).  */
  let mut xyz_list = xyz_list;

  macro_rules! PARSE_XYZ_LIST_FIELD {
    ($field:ident) => {
      if CONSP (xyz_list) && NUMBERP (XCAR (xyz_list))
	{
	  (*color).$field = 100.0 * XFLOATINT (XCAR (xyz_list));
	  xyz_list = XCDR (xyz_list);
	}
      else
	{ return false; }
    };
  }

  PARSE_XYZ_LIST_FIELD! (X);
  PARSE_XYZ_LIST_FIELD! (Y);
  PARSE_XYZ_LIST_FIELD! (Z);

  return true;
}

/// C 202-221: `parse_jch_list`.
#[cfg (feature = "have_lcms2")]
unsafe fn parse_jch_list (jch_list: Lisp_Object, color: *mut cmsJCh) -> bool
{
  /* C's macro assigns the parameter, so the port rebinds it mutable before the
     `macro_rules!` that captures it (a macro's names resolve where it is defined).  */
  let mut jch_list = jch_list;

  macro_rules! PARSE_JCH_LIST_FIELD {
    ($field:ident) => {
      if CONSP (jch_list) && NUMBERP (XCAR (jch_list))
	{
	  (*color).$field = XFLOATINT (XCAR (jch_list));
	  jch_list = XCDR (jch_list);
	}
      else
	{ return false; }
    };
  }

  PARSE_JCH_LIST_FIELD! (J);
  PARSE_JCH_LIST_FIELD! (C);
  PARSE_JCH_LIST_FIELD! (h);

  if ! NILP (jch_list)
    { return false; }
  return true;
}

/// C 223-240: `parse_jab_list`.
#[cfg (feature = "have_lcms2")]
unsafe fn parse_jab_list (jab_list: Lisp_Object, color: *mut lcmsJab_t) -> bool
{
  /* C's macro assigns the parameter, so the port rebinds it mutable before the
     `macro_rules!` that captures it (a macro's names resolve where it is defined).  */
  let mut jab_list = jab_list;

  macro_rules! PARSE_JAB_LIST_FIELD {
    ($field:ident) => {
      if CONSP (jab_list) && NUMBERP (XCAR (jab_list))
	{
	  (*color).$field = XFLOATINT (XCAR (jab_list));
	  jab_list = XCDR (jab_list);
	}
      else
	{ return false; }
    };
  }

  PARSE_JAB_LIST_FIELD! (J);
  PARSE_JAB_LIST_FIELD! (a);
  PARSE_JAB_LIST_FIELD! (b);

  return true;
}

/// C 242-275: `parse_viewing_conditions` - the second of its two macros takes an integer field:
/// C's `check_integer_range` returns `intmax_t` and the member is `cmsUInt32Number`, so the port
/// spells the implicit conversion.
#[cfg (feature = "have_lcms2")]
unsafe fn parse_viewing_conditions (view: Lisp_Object, wp: *const cmsCIEXYZ,
                                    vc: *mut cmsViewingConditions) -> bool
{
  /* C's macro assigns the parameter, so the port rebinds it mutable before the
     `macro_rules!` that captures it (a macro's names resolve where it is defined).  */
  let mut view = view;

  macro_rules! PARSE_VIEW_CONDITION_FLOAT {
    ($field:ident) => {
      if CONSP (view) && NUMBERP (XCAR (view))
	{
	  (*vc).$field = XFLOATINT (XCAR (view));
	  view = XCDR (view);
	}
      else
	{ return false; }
    };
  }
  macro_rules! PARSE_VIEW_CONDITION_INT {
    ($field:ident) => {
      if CONSP (view) && FIXNATP (XCAR (view))
	{
	  (*vc).$field = check_integer_range (XCAR (view), 1, 4) as cmsUInt32Number;
	  view = XCDR (view);
	}
      else
	{ return false; }
    };
  }

  PARSE_VIEW_CONDITION_FLOAT! (Yb);
  PARSE_VIEW_CONDITION_FLOAT! (La);
  PARSE_VIEW_CONDITION_INT! (surround);
  PARSE_VIEW_CONDITION_FLOAT! (D_value);

  if ! NILP (view)
    { return false; }

  (*vc).whitePoint.X = (*wp).X;
  (*vc).whitePoint.Y = (*wp).Y;
  (*vc).whitePoint.Z = (*wp).Z;
  return true;
}

/// C 277-285: `xyz_to_jch`.  C's `cmsCIECAM02Init (0, vc)` passes a null `cmsContext`.
#[cfg (feature = "have_lcms2")]
unsafe fn xyz_to_jch (xyz: *const cmsCIEXYZ, jch: *mut cmsJCh, vc: *const cmsViewingConditions)
{
  let mut h: cmsHANDLE;

  h = cmsCIECAM02Init (std::ptr::null_mut (), vc);
  cmsCIECAM02Forward (h, xyz, jch);
  cmsCIECAM02Done (h);
}

/// C 287-295: `jch_to_xyz`.
#[cfg (feature = "have_lcms2")]
unsafe fn jch_to_xyz (jch: *const cmsJCh, xyz: *mut cmsCIEXYZ, vc: *const cmsViewingConditions)
{
  let mut h: cmsHANDLE;

  h = cmsCIECAM02Init (std::ptr::null_mut (), vc);
  cmsCIECAM02Reverse (h, jch, xyz);
  cmsCIECAM02Done (h);
}

/// C 297-304: `jch_to_jab`.
#[cfg (feature = "have_lcms2")]
unsafe fn jch_to_jab (jch: *const cmsJCh, jab: *mut lcmsJab_t, FL: c_double, c1: c_double,
                      c2: c_double)
{
  let Mp: c_double = 43.86 * log (1.0 + c2 * ((*jch).C * sqrt (sqrt (FL))));
  (*jab).J = 1.7 * (*jch).J / (1.0 + (c1 * (*jch).J));
  (*jab).a = Mp * cos (deg2rad ((*jch).h));
  (*jab).b = Mp * sin (deg2rad ((*jch).h));
}

/// C 306-316: `jab_to_jch`.
#[cfg (feature = "have_lcms2")]
unsafe fn jab_to_jch (jab: *const lcmsJab_t, jch: *mut cmsJCh, FL: c_double, c1: c_double,
                      c2: c_double)
{
  (*jch).J = (*jab).J / (1.0 + c1 * (100.0 - (*jab).J));
  (*jch).h = atan2 ((*jab).b, (*jab).a);
  let Mp: c_double = hypot ((*jab).a, (*jab).b);
  (*jch).h = rad2deg ((*jch).h);
  if (*jch).h < 0.0
    { (*jch).h += 360.0; }
  (*jch).C = (exp (c2 * Mp) - 1.0) / (c2 * sqrt (sqrt (FL)));
}

#[cfg (feature = "have_lcms2")]
DEFUN! ("lcms-xyz->jch", Flcms_xyz_to_jch, Slcms_xyz_to_jch, 1, 3, 0,
       doc: "Convert CIE XYZ to CIE CAM02 JCh.\nCOLOR is a list (X Y Z), with Y scaled about unity.\nOptional arguments WHITEPOINT and VIEW are the same as in `lcms-cam02-ucs',\nwhich see.",
       (color: Lisp_Object, whitepoint: Lisp_Object, view: Lisp_Object),
       {
  let mut vc: cmsViewingConditions = std::mem::zeroed ();
  let mut jch: cmsJCh = std::mem::zeroed ();
  let mut xyz: cmsCIEXYZ = std::mem::zeroed ();
  let mut xyzw: cmsCIEXYZ = std::mem::zeroed ();

  #[cfg (feature = "windowsnt")]
  {
    if !lcms_initialized
      { lcms_initialized = init_lcms_functions (); }
    if !lcms_initialized
      {
        message1 (c"lcms2 library not found".as_ptr ());
        return Qnil ();
      }
  }

  if !(CONSP (color) && parse_xyz_list (color, &raw mut xyz))
    { signal_error (c"Invalid color".as_ptr (), color); }
  if NILP (whitepoint)
    { xyzw = illuminant_d65; }
  else if !(CONSP (whitepoint) && parse_xyz_list (whitepoint, &raw mut xyzw))
    { signal_error (c"Invalid white point".as_ptr (), whitepoint); }
  if NILP (view)
    { default_viewing_conditions (&raw const xyzw, &raw mut vc); }
  else if !(CONSP (view) && parse_viewing_conditions (view, &raw const xyzw, &raw mut vc))
    { signal_error (c"Invalid viewing conditions".as_ptr (), view); }

  xyz_to_jch (&raw const xyz, &raw mut jch, &raw const vc);
  return list3 (make_float (jch.J), make_float (jch.C), make_float (jch.h));
});

#[cfg (feature = "have_lcms2")]
DEFUN! ("lcms-jch->xyz", Flcms_jch_to_xyz, Slcms_jch_to_xyz, 1, 3, 0,
       doc: "Convert CIE CAM02 JCh to CIE XYZ.\nCOLOR is a list (J C h), where lightness of white is equal to 100, and hue\nis given in degrees.\nOptional arguments WHITEPOINT and VIEW are the same as in `lcms-cam02-ucs',\nwhich see.",
       (color: Lisp_Object, whitepoint: Lisp_Object, view: Lisp_Object),
       {
  let mut vc: cmsViewingConditions = std::mem::zeroed ();
  let mut jch: cmsJCh = std::mem::zeroed ();
  let mut xyz: cmsCIEXYZ = std::mem::zeroed ();
  let mut xyzw: cmsCIEXYZ = std::mem::zeroed ();

  #[cfg (feature = "windowsnt")]
  {
    if !lcms_initialized
      { lcms_initialized = init_lcms_functions (); }
    if !lcms_initialized
      {
        message1 (c"lcms2 library not found".as_ptr ());
        return Qnil ();
      }
  }

  if !(CONSP (color) && parse_jch_list (color, &raw mut jch))
    { signal_error (c"Invalid color".as_ptr (), color); }
  if NILP (whitepoint)
    { xyzw = illuminant_d65; }
  else if !(CONSP (whitepoint) && parse_xyz_list (whitepoint, &raw mut xyzw))
    { signal_error (c"Invalid white point".as_ptr (), whitepoint); }
  if NILP (view)
    { default_viewing_conditions (&raw const xyzw, &raw mut vc); }
  else if !(CONSP (view) && parse_viewing_conditions (view, &raw const xyzw, &raw mut vc))
    { signal_error (c"Invalid viewing conditions".as_ptr (), view); }

  jch_to_xyz (&raw const jch, &raw mut xyz, &raw const vc);
  return list3 (make_float (xyz.X / 100.0),
                make_float (xyz.Y / 100.0),
                make_float (xyz.Z / 100.0));
});

#[cfg (feature = "have_lcms2")]
DEFUN! ("lcms-jch->jab", Flcms_jch_to_jab, Slcms_jch_to_jab, 1, 3, 0,
       doc: "Convert CIE CAM02 JCh to CAM02-UCS J'a'b'.\nCOLOR is a list (J C h) as described in `lcms-jch->xyz', which see.\nOptional arguments WHITEPOINT and VIEW are the same as in `lcms-cam02-ucs',\nwhich see.",
       (color: Lisp_Object, whitepoint: Lisp_Object, view: Lisp_Object),
       {
  let mut vc: cmsViewingConditions = std::mem::zeroed ();
  let mut jab: lcmsJab_t = std::mem::zeroed ();
  let mut jch: cmsJCh = std::mem::zeroed ();
  let mut xyzw: cmsCIEXYZ = std::mem::zeroed ();
  /* C declares `double FL, k, k4;` uninitialized.  */
  let mut FL: c_double = 0.0;
  let mut k: c_double = 0.0;
  let mut k4: c_double = 0.0;

  #[cfg (feature = "windowsnt")]
  {
    if !lcms_initialized
      { lcms_initialized = init_lcms_functions (); }
    if !lcms_initialized
      {
        message1 (c"lcms2 library not found".as_ptr ());
        return Qnil ();
      }
  }

  if !(CONSP (color) && parse_jch_list (color, &raw mut jch))
    { signal_error (c"Invalid color".as_ptr (), color); }
  if NILP (whitepoint)
    { xyzw = illuminant_d65; }
  else if !(CONSP (whitepoint) && parse_xyz_list (whitepoint, &raw mut xyzw))
    { signal_error (c"Invalid white point".as_ptr (), whitepoint); }
  if NILP (view)
    { default_viewing_conditions (&raw const xyzw, &raw mut vc); }
  else if !(CONSP (view) && parse_viewing_conditions (view, &raw const xyzw, &raw mut vc))
    { signal_error (c"Invalid viewing conditions".as_ptr (), view); }

  k = 1.0 / (1.0 + (5.0 * vc.La));
  k4 = k * k * k * k;
  FL = vc.La * k4 + 0.1 * (1.0 - k4) * (1.0 - k4) * cbrt (5.0 * vc.La);
  jch_to_jab (&raw const jch, &raw mut jab, FL, 0.007, 0.0228);
  return list3 (make_float (jab.J), make_float (jab.a), make_float (jab.b));
});

#[cfg (feature = "have_lcms2")]
DEFUN! ("lcms-jab->jch", Flcms_jab_to_jch, Slcms_jab_to_jch, 1, 3, 0,
       doc: "Convert CAM02-UCS J'a'b' to CIE CAM02 JCh.\nCOLOR is a list (J' a' b'), where white corresponds to lightness J equal to 100.\nOptional arguments WHITEPOINT and VIEW are the same as in `lcms-cam02-ucs',\nwhich see.",
       (color: Lisp_Object, whitepoint: Lisp_Object, view: Lisp_Object),
       {
  let mut vc: cmsViewingConditions = std::mem::zeroed ();
  let mut jch: cmsJCh = std::mem::zeroed ();
  let mut jab: lcmsJab_t = std::mem::zeroed ();
  let mut xyzw: cmsCIEXYZ = std::mem::zeroed ();
  let mut FL: c_double = 0.0;
  let mut k: c_double = 0.0;
  let mut k4: c_double = 0.0;

  #[cfg (feature = "windowsnt")]
  {
    if !lcms_initialized
      { lcms_initialized = init_lcms_functions (); }
    if !lcms_initialized
      {
        message1 (c"lcms2 library not found".as_ptr ());
        return Qnil ();
      }
  }

  if !(CONSP (color) && parse_jab_list (color, &raw mut jab))
    { signal_error (c"Invalid color".as_ptr (), color); }
  if NILP (whitepoint)
    { xyzw = illuminant_d65; }
  else if !(CONSP (whitepoint) && parse_xyz_list (whitepoint, &raw mut xyzw))
    { signal_error (c"Invalid white point".as_ptr (), whitepoint); }
  if NILP (view)
    { default_viewing_conditions (&raw const xyzw, &raw mut vc); }
  else if !(CONSP (view) && parse_viewing_conditions (view, &raw const xyzw, &raw mut vc))
    { signal_error (c"Invalid viewing conditions".as_ptr (), view); }

  k = 1.0 / (1.0 + (5.0 * vc.La));
  k4 = k * k * k * k;
  FL = vc.La * k4 + 0.1 * (1.0 - k4) * (1.0 - k4) * cbrt (5.0 * vc.La);
  jab_to_jch (&raw const jab, &raw mut jch, FL, 0.007, 0.0228);
  return list3 (make_float (jch.J), make_float (jch.C), make_float (jch.h));
});

/* References:
   Li, Luo et al. "The CRI-CAM02UCS colour rendering index." COLOR research
   and application, 37 No.3, 2012.
   Luo et al. "Uniform colour spaces based on CIECAM02 colour appearance
   model." COLOR research and application, 31 No.4, 2006. */

#[cfg (feature = "have_lcms2")]
DEFUN! ("lcms-cam02-ucs", Flcms_cam02_ucs, Slcms_cam02_ucs, 2, 4, 0,
       doc: "Compute CAM02-UCS metric distance between COLOR1 and COLOR2.\nEach color is a list of XYZ tristimulus values, with Y scaled about unity.\nOptional argument WHITEPOINT is the XYZ white point, which defaults to\nilluminant D65.\nOptional argument VIEW is a list containing the viewing conditions, and\nis of the form (YB LA SURROUND DVALUE) where SURROUND corresponds to\n  1   AVG_SURROUND\n  2   DIM_SURROUND\n  3   DARK_SURROUND\n  4   CUTSHEET_SURROUND\nThe default viewing conditions are (20 100 1 1).",
       (color1: Lisp_Object, color2: Lisp_Object, whitepoint: Lisp_Object,
	view: Lisp_Object),
       {
  let mut vc: cmsViewingConditions = std::mem::zeroed ();
  let mut jch1: cmsJCh = std::mem::zeroed ();
  let mut jch2: cmsJCh = std::mem::zeroed ();
  let mut xyz1: cmsCIEXYZ = std::mem::zeroed ();
  let mut xyz2: cmsCIEXYZ = std::mem::zeroed ();
  let mut xyzw: cmsCIEXYZ = std::mem::zeroed ();
  let mut jab1: lcmsJab_t = std::mem::zeroed ();
  let mut jab2: lcmsJab_t = std::mem::zeroed ();
  let mut FL: c_double = 0.0;
  let mut k: c_double = 0.0;
  let mut k4: c_double = 0.0;

  #[cfg (feature = "windowsnt")]
  {
    if !lcms_initialized
      { lcms_initialized = init_lcms_functions (); }
    if !lcms_initialized
      {
        message1 (c"lcms2 library not found".as_ptr ());
        return Qnil ();
      }
  }

  if !(CONSP (color1) && parse_xyz_list (color1, &raw mut xyz1))
    { signal_error (c"Invalid color".as_ptr (), color1); }
  if !(CONSP (color2) && parse_xyz_list (color2, &raw mut xyz2))
    { signal_error (c"Invalid color".as_ptr (), color2); }
  if NILP (whitepoint)
    { xyzw = illuminant_d65; }
  else if !(CONSP (whitepoint) && parse_xyz_list (whitepoint, &raw mut xyzw))
    { signal_error (c"Invalid white point".as_ptr (), whitepoint); }
  if NILP (view)
    { default_viewing_conditions (&raw const xyzw, &raw mut vc); }
  else if !(CONSP (view) && parse_viewing_conditions (view, &raw const xyzw, &raw mut vc))
    { signal_error (c"Invalid view conditions".as_ptr (), view); }

  xyz_to_jch (&raw const xyz1, &raw mut jch1, &raw const vc);
  xyz_to_jch (&raw const xyz2, &raw mut jch2, &raw const vc);

  k = 1.0 / (1.0 + (5.0 * vc.La));
  k4 = k * k * k * k;
  FL = vc.La * k4 + 0.1 * (1.0 - k4) * (1.0 - k4) * cbrt (5.0 * vc.La);
  jch_to_jab (&raw const jch1, &raw mut jab1, FL, 0.007, 0.0228);
  jch_to_jab (&raw const jch2, &raw mut jab2, FL, 0.007, 0.0228);

  return make_float (hypot (jab2.J - jab1.J,
                            hypot (jab2.a - jab1.a, jab2.b - jab1.b)));
});

#[cfg (feature = "have_lcms2")]
DEFUN! ("lcms-temp->white-point", Flcms_temp_to_white_point, Slcms_temp_to_white_point, 1, 1, 0,
       doc: "Return XYZ black body chromaticity from TEMPERATURE given in K.\nValid range of TEMPERATURE is from 4000K to 25000K.",
       (temperature: Lisp_Object),
       {
  let mut tempK: cmsFloat64Number = 0.0;
  let mut whitepoint: cmsCIExyY = std::mem::zeroed ();
  let mut wp: cmsCIEXYZ = std::mem::zeroed ();

  #[cfg (feature = "windowsnt")]
  {
    if !lcms_initialized
      { lcms_initialized = init_lcms_functions (); }
    if !lcms_initialized
      {
        message1 (c"lcms2 library not found".as_ptr ());
        return Qnil ();
      }
  }

  CHECK_NUMBER (temperature);

  tempK = XFLOATINT (temperature);
  if cmsWhitePointFromTemp (&raw mut whitepoint, tempK) == 0
    { signal_error (c"Invalid temperature".as_ptr (), temperature); }
  cmsxyY2XYZ (&raw mut wp, &raw const whitepoint);
  return list3 (make_float (wp.X), make_float (wp.Y), make_float (wp.Z));
});

#[cfg (feature = "have_lcms2")]
DEFUN! ("lcms2-available-p", Flcms2_available_p, Slcms2_available_p, 0, 0, 0,
       doc: "Return t if lcms2 color calculations are available in this instance of Emacs.",
       (),
       {
  #[cfg (feature = "windowsnt")]
  {
    let mut found: Lisp_Object = Fassq (Qlcms2 (), Vlibrary_cache);
    if CONSP (found)
      { return XCDR (found); }
    else
      {
        let mut status: Lisp_Object;
        lcms_initialized = init_lcms_functions ();
        status = if lcms_initialized { Qt () } else { Qnil () };
        Vlibrary_cache = Fcons (Fcons (Qlcms2 (), status), Vlibrary_cache);
        return status;
      }
  }
  #[cfg (not (feature = "windowsnt"))]
  { return Qt (); }
});

/* C 587-602: `syms_of_lcms2`.  */

/// C 589-602: the file's one non-`static` definition.
#[cfg (feature = "have_lcms2")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_lcms2 ()
{
  defsubr (&raw mut Slcms_cie_de2000);
  defsubr (&raw mut Slcms_xyz_to_jch);
  defsubr (&raw mut Slcms_jch_to_xyz);
  defsubr (&raw mut Slcms_jch_to_jab);
  defsubr (&raw mut Slcms_jab_to_jch);
  defsubr (&raw mut Slcms_cam02_ucs);
  defsubr (&raw mut Slcms2_available_p);
  defsubr (&raw mut Slcms_temp_to_white_point);

  Fprovide (intern_c_string (c"lcms2".as_ptr ()), Qnil ());
}
