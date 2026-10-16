// Every macro the ledgers tick must actually *expand*: nothing in the repository used to
// expand AUTO_*/DEFVAR_* (review C's F2: 12 macros could not be used at all, with ~1,070
// call sites waiting in the reference tree), and `extract-abi.py` can only see names.
// The mirrors are included flat, exactly as a translation unit includes them, and the
// probe compiles metadata-only so the unresolved externs (Fcons, defvar_lisp, …) need no
// definition.  `macro_rules!` paths resolve at the *call* site, so this also documents
// which names a caller must have in scope.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs)]
include!("../femacs/src/include/config.rs");
include!("../femacs/lib/include/stdio.rs");
include!("../femacs/src/include/lisp.rs");
include!("../femacs/src/include/itree.rs");
include!("../femacs/src/include/character.rs");
include!("../femacs/src/include/buffer.rs");

// a translation unit's own storage
static mut V_INT: EMACS_INT = 0;
static mut V_LISP: Lisp_Object = lisp_h_Qnil ();
static mut V_BOOL: bool = false;

// The builtin symbol table the mirror references; a nested module keeps the definition
// from colliding with globals.rs's `extern` declaration in this same crate.
#[repr(C, align(8))]
pub struct Arena (pub [u8; 101664]);
mod arena {
    #[unsafe(no_mangle)]
    pub static mut lispsym: super::Arena = super::Arena ([0; 101664]);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn probe_macros (a: Lisp_Object, b: Lisp_Object) -> Lisp_Object {
  unsafe {
    AUTO_CONS! (cons1, a, b);
    AUTO_LIST1! (lst1, cons1);
    AUTO_LIST2! (lst2, a, b);
    AUTO_LIST3! (lst3, a, b, cons1);
    AUTO_LIST4! (lst4, a, b, cons1, lst1);
    AUTO_STRING! (str1, c"probe".as_ptr ());
    AUTO_STRING_WITH_LEN! (str2, c"probe".as_ptr (), 5);
    /* C's expression forms have no Rust counterpart - a compound literal's storage lives in the
       enclosing block, which an expression macro cannot reach (PORTING.md P-113 ②) - so this
       probe exercises the statement forms, exactly as every call site does.  */
    AUTO_STRING! (str3, c"probe".as_ptr ());
    AUTO_STRING_WITH_LEN! (str4, c"probe".as_ptr (), 5);
    let mut target: Lisp_Object = lisp_h_Qnil ();
    XSETINT! (target, 7);
    XSETCONS! (target, cons1);
    XSETSTRING! (target, str2);
    XSETSYMBOL! (target, unsafe { XBARE_SYMBOL (Qnil ()) });
    USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
    /* The scope's own height is the third name (PORTING.md P-117 ⑧); `sa_count` stays an
       expression here, the shape the probe has always exercised.  */
    SAFE_FREE! (SPECPDL_INDEX (), sa_mark);
    let mut buf: *mut Lisp_Object = std::ptr::null_mut ();
    let _p = SAFE_ALLOCA! (sa_avail, 16);
    SAFE_ALLOCA_LISP! (sa_avail, buf, 4);
    SAFE_NALLOCA! (sa_avail, buf, 1, 4);
    let _all = ALLOCATE_PLAIN_PSEUDOVECTOR! (Lisp_Vector, pvec_type::PVEC_NORMAL_VECTOR);
    println! ("{} {:p} {:p} {:p} {:p} {:p} {:p}", lst1 == Qnil (), target, str3, str4, cons1, a, b);
    lst4
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn probe_defvars () {
  unsafe {
    DEFVAR_LISP! (c"probe-lisp", V_LISP, "");
    DEFVAR_LISP_NOPRO! (c"probe-lisp-nopro", V_LISP, "");
    DEFVAR_INT! (c"probe-int", V_INT, "");
    DEFVAR_BOOL! (c"probe-bool", V_BOOL, "");
    // The offset argument reaches a `static` initializer (`$vname as c_int`), so it has to be
    // a constant expression - the shape every unit's `offset_of! (kboard, …)` has (status 3.617's
    // name-argument rule is the same call site).
    DEFVAR_KBOARD! (c"probe-kboard", 0, "");
  }
}

fn main () {}
