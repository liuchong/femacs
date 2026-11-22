// femacs: port of src/intervals.h @ fd7afced403.
//
// Header mirror: the type, the macros, the inline setters and the `unsafe extern "C"`
// declarations C spells as `extern` (PORTING.md 0.1).  The macros are the same statements
// in the same order as C's.
//
// Every `extern` prototype of intervals.h's "Defined in textprop.c" block lives in
// `pub mod textprop_decls` + glob import, because `src/textprop.rs` defines all nine
// of those names (PORTING.md 0.1-10/P-21); the three prototypes of the "Declared in
// intervals.c" block that textprop.c also defines are in `intervals_decls`, the same way.
//
// Why it exists now: src/syntax.c is the first unit that reads interval fields
// (`i->position`, `i->plist`) and C's interval arithmetic (`LENGTH (i)`,
// `INTERVAL_LAST_POS (i)`), so the header that defines them has to have a home before that
// unit can compile at all.  C's own inclusion graph (intervals.h 21-22 include buffer.h and
// lisp.h) is reproduced below; `xtask materialize` de-duplicates the shared mirrors (P-61).
//
// The one shape that cannot be copied literally is `INTERVAL` itself: lisp.h 1423 declares
// `typedef struct interval *INTERVAL;` while `struct interval` is incomplete there, so
// lisp.rs keeps `pub type INTERVAL = *mut c_void` (P-72 rule 3) and this mirror keeps the
// same opaque ABI.  Where C writes `i->field` the mirror casts once, `((i) as *mut interval)`
// -- P-72 rule 4's shape: a unit that needs the complete type carries this mirror.
//
// `#if false` at C 227-234 (FRONT_STICKY_P / END_NONSTICKY_P / FRONT_NONSTICKY_P) is
// literal-false text, not a configuration arm, so P-51's feature gate does not apply and the
// three macros are not mirrored (the cm.rs and font.rs precedents record the same case).
//
// The layout of `struct interval` is measured, not guessed: clang, with the spikes/run.sh
// flags, reports sizeof 56 / align 8 and the offsets total_length 0, position 8, left 16,
// right 24, up 32, plist 48, with the six `bool_bf : 1` members as one byte at 40 (`bool_bf`
// is `bool` on this configuration, conf_post.h 61).  The mirror stores that byte as
// `bits: u8` with the `INTERVAL_*_BIT` masks (PORTING.md P-80, the cm.rs `bits_*` shape).
//
// The mirror is complete as of the commit that lands `src/intervals.rs` (the
// composite.rs/ccl.rs precedent): the type, the 29 macros, the three INLINE setters, the
// `intervals_decls` block above and the three remaining extern groups.  Its ledger is
// docs/ports/include-intervals.md, whose Decisions section records what each shape costs and
// the three token rows the gate cannot express (the `do { … } while (false)` spelling of
// COPY_INTERVAL_CACHE/MERGE_INTERVAL_CACHE, the `1` of `i->up_obj = 1`, and the `0` of the
// three null tests, all in docs/ports/macro-body-notes.tsv).

// [U1-MODULE-MIRROR]
use super::*;
// [U1] lisp.rs imported via super
// [U1] buffer.rs imported via super

// C's include guard `EMACS_INTERVALS_H` has no Rust counterpart; the constant below exists
// only so the mirror's name set matches the reference header's (PORTING.md 0.1, P-24).
pub const EMACS_INTERVALS_H: bool = true;

/* Basic data type for use of intervals.  */

/// C: intervals.h 29: `struct interval`.  The `bool_bf : 1` members are one storage byte
/// (bit 0 is C's first member, `up_obj`); use `interval_bits!` to read or write them.
#[repr(C)]
pub struct interval
{
  /* The first group of entries deal with the tree structure.  */
  pub total_length: ptrdiff_t,		/* Length of myself and both children.  */
  pub position: ptrdiff_t,		/* Cache of interval's character position.  */
  pub left: *mut interval,		/* Intervals which precede me.  */
  pub right: *mut interval,		/* Intervals which succeed me.  */

  /* Parent in the tree, or the Lisp_Object containing this interval tree.  */
  pub up: interval_up,
  /// C: intervals.h 44-52: the bitfields `up_obj : 1`, `gcmarkbit : 1`,
  /// `write_protect : 1`, `visible : 1`, `front_sticky : 1`, `rear_sticky : 1`.
  pub bits: u8,
  pub plist: Lisp_Object,		/* Other properties.  */
}

/// C: intervals.h 39-43: the anonymous `union { struct interval *interval;
/// Lisp_Object obj; } up` (PORTING.md P-67: an anonymous union is named `<union>_<member>`).
#[repr(C)]
#[derive(Clone, Copy)]
pub union interval_up
{
  pub interval: *mut interval,
  pub obj: Lisp_Object,
}

/* The six `bool_bf : 1` members of `struct interval` share one storage byte in C.  */
pub const INTERVAL_UP_OBJ_BIT: u32 = 1 << 0;
pub const INTERVAL_GCMARKBIT_BIT: u32 = 1 << 1;
pub const INTERVAL_WRITE_PROTECT_BIT: u32 = 1 << 2;
pub const INTERVAL_VISIBLE_BIT: u32 = 1 << 3;
pub const INTERVAL_FRONT_STICKY_BIT: u32 = 1 << 4;
pub const INTERVAL_REAR_STICKY_BIT: u32 = 1 << 5;

#[inline(always)]
pub fn interval_bit_p (i: *const interval, bit: u32) -> bool
{
  return (/* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*i).bits } as u32 & bit) != 0;
}

#[inline(always)]
pub fn interval_bit_set (i: *mut interval, bit: u32, value: bool)
{
  if value { /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*i).bits |= bit as u8 } }
  else { unsafe { (*i).bits &= !(bit as u8) } }
}

/// C: intervals.h 44-56: the bitfield reads and writes, named as C names them, so the port
/// text carries the member name the token gate looks for (PORTING.md P-80).
macro_rules! interval_bits {
  (up_obj, $i:expr) => { interval_bit_p (($i) as *const interval, INTERVAL_UP_OBJ_BIT) };
  (up_obj, $i:expr, $value:expr) =>
    { interval_bit_set (($i) as *mut interval, INTERVAL_UP_OBJ_BIT, $value) };
  (gcmarkbit, $i:expr) => { interval_bit_p (($i) as *const interval, INTERVAL_GCMARKBIT_BIT) };
  (gcmarkbit, $i:expr, $value:expr) =>
    { interval_bit_set (($i) as *mut interval, INTERVAL_GCMARKBIT_BIT, $value) };
  (write_protect, $i:expr) =>
    { interval_bit_p (($i) as *const interval, INTERVAL_WRITE_PROTECT_BIT) };
  (write_protect, $i:expr, $value:expr) =>
    { interval_bit_set (($i) as *mut interval, INTERVAL_WRITE_PROTECT_BIT, $value) };
  (visible, $i:expr) => { interval_bit_p (($i) as *const interval, INTERVAL_VISIBLE_BIT) };
  (visible, $i:expr, $value:expr) =>
    { interval_bit_set (($i) as *mut interval, INTERVAL_VISIBLE_BIT, $value) };
  (front_sticky, $i:expr) =>
    { interval_bit_p (($i) as *const interval, INTERVAL_FRONT_STICKY_BIT) };
  (front_sticky, $i:expr, $value:expr) =>
    { interval_bit_set (($i) as *mut interval, INTERVAL_FRONT_STICKY_BIT, $value) };
  (rear_sticky, $i:expr) =>
    { interval_bit_p (($i) as *const interval, INTERVAL_REAR_STICKY_BIT) };
  (rear_sticky, $i:expr, $value:expr) =>
    { interval_bit_set (($i) as *mut interval, INTERVAL_REAR_STICKY_BIT, $value) };
}

/* These are macros for dealing with the interval tree.  */

/* True if this interval has no right child.  */
macro_rules! NULL_RIGHT_CHILD {
  ($i:expr) => { (/* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(($i) as *mut interval)).right == std::ptr::null_mut () }) };
}

/* True if this interval has no left child.  */
macro_rules! NULL_LEFT_CHILD {
  ($i:expr) => { (/* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(($i) as *mut interval)).left == std::ptr::null_mut () }) };
}

/* True if this interval has no parent.  */
macro_rules! NULL_PARENT {
  ($i:expr) => { (interval_bits! (up_obj, $i)
			 || /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(($i) as *mut interval)).up.interval } == 0 as *mut interval) };
}

/* True if this interval is the left child of some other interval.  */
macro_rules! AM_LEFT_CHILD {
  ($i:expr) => { (! NULL_PARENT! ($i)
		  && /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*INTERVAL_PARENT! ($i)).left } == (($i) as *mut interval)) };
}

/* True if this interval is the right child of some other interval.  */
macro_rules! AM_RIGHT_CHILD {
  ($i:expr) => { (! NULL_PARENT! ($i)
		  && /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*INTERVAL_PARENT! ($i)).right } == (($i) as *mut interval)) };
}

/* True if this interval has no children.  */
macro_rules! LEAF_INTERVAL_P {
  ($i:expr) => { (/* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(($i) as *mut interval)).left } == std::ptr::null_mut ()
		  && unsafe { (*(($i) as *mut interval)).right } == std::ptr::null_mut ()) };
}

/* True if this interval has no parent and is therefore the root.  */
macro_rules! ROOT_INTERVAL_P { ($i:expr) => { NULL_PARENT! ($i) }; }

/* True if this interval is the only interval in the interval tree.  */
macro_rules! ONLY_INTERVAL_P {
  ($i:expr) => { (ROOT_INTERVAL_P! ($i) && LEAF_INTERVAL_P! ($i)) };
}

/* True if this interval has both left and right children.  */
macro_rules! BOTH_KIDS_P {
  ($i:expr) => { (/* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(($i) as *mut interval)).left } != std::ptr::null_mut ()
		  && unsafe { (*(($i) as *mut interval)).right } != std::ptr::null_mut ()) };
}

/* The total size of all text represented by the nonnull interval I
   and all its children in the tree.  */
macro_rules! TOTAL_LENGTH { ($i:expr) => { /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(($i) as *mut interval)).total_length } }; }

/* Likewise, but also defined to be zero if I is null.  */
macro_rules! TOTAL_LENGTH0 {
  ($i:expr) => { (if !($i).is_null () { TOTAL_LENGTH! ($i) } else { 0 }) };
}

/* The size of text represented by this interval alone.  */
macro_rules! LENGTH {
  ($i:expr) => { (TOTAL_LENGTH! ($i) - RIGHT_TOTAL_LENGTH! ($i) - LEFT_TOTAL_LENGTH! ($i)) };
}

/* The position of the character just past the end of I.  Note that
   the position cache i->position must be valid for this to work.  */
macro_rules! INTERVAL_LAST_POS {
  ($i:expr) => { (/* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(($i) as *mut interval)).position } + LENGTH! ($i)) };
}

/* The total size of the left subtree of this interval.  */
macro_rules! LEFT_TOTAL_LENGTH {
  ($i:expr) => { TOTAL_LENGTH0! (/* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(($i) as *mut interval)).left }) };
}

/* The total size of the right subtree of this interval.  */
macro_rules! RIGHT_TOTAL_LENGTH {
  ($i:expr) => { TOTAL_LENGTH0! (/* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(($i) as *mut interval)).right }) };
}

/* These macros are for dealing with the interval properties.  */

/* True if this is a default interval, which is the same as being null
   or having no properties.  */
macro_rules! DEFAULT_INTERVAL_P {
  ($i:expr) => { ((($i) as *mut interval).is_null ()
		  || NILP (/* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(($i) as *mut interval)).plist })) };
}

/* Test what type of parent we have.  Three possibilities: another
   interval, a buffer or string object, or NULL.  */
macro_rules! INTERVAL_HAS_PARENT {
  ($i:expr) => { (! interval_bits! (up_obj, $i)
		  && /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(($i) as *mut interval)).up.interval } != std::ptr::null_mut ()) };
}
macro_rules! INTERVAL_HAS_OBJECT { ($i:expr) => { (interval_bits! (up_obj, $i)) }; }

/* Use these macros to get parent of an interval.

   The choice of macros is dependent on the type needed.  Don't add
   casts to get around this, it will break some development work in
   progress.  */

macro_rules! INTERVAL_PARENT {
  ($i:expr) => { { eassert! (!(($i) as *mut interval).is_null () && ! interval_bits! (up_obj, $i));
		   /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(($i) as *mut interval)).up.interval } } };
}

macro_rules! GET_INTERVAL_OBJECT {
  ($d:expr, $s:expr) => { { eassert! (interval_bits! (up_obj, $s));
			    $d = /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(($s) as *mut interval)).up.obj }; } };
}

/* Use these functions to set Lisp_Object
   or pointer slots of struct interval.  */

#[inline(always)]
pub fn set_interval_object (i: INTERVAL, obj: Lisp_Object)
{
  eassert! (BUFFERP (obj) || STRINGP (obj));
  interval_bits! (up_obj, i, true);
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(i as *mut interval)).up.obj = obj; }
}

#[inline(always)]
pub fn set_interval_parent (i: INTERVAL, parent: INTERVAL)
{
  interval_bits! (up_obj, i, false);
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(i as *mut interval)).up.interval = parent as *mut interval; }
}

#[inline(always)]
pub fn set_interval_plist (i: INTERVAL, plist: Lisp_Object)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(i as *mut interval)).plist = plist; }
}

/* Get the parent interval, if any, otherwise a null pointer.  Useful
   for walking up to the root in a "for" loop; use this to get the
   "next" value, and test the result to see if it's NULL.  */
macro_rules! INTERVAL_PARENT_OR_NULL {
  ($i:expr) => { (if INTERVAL_HAS_PARENT! ($i) { INTERVAL_PARENT! ($i) as INTERVAL }
		  else { std::ptr::null_mut () }) };
}

/* Reset this interval to its vanilla, or no-property state.  */
macro_rules! RESET_INTERVAL {
  ($i:expr) => { {
    /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*(($i) as *mut interval)).total_length = 0; }
    unsafe { (*(($i) as *mut interval)).position = 0; }
    unsafe { (*(($i) as *mut interval)).left = std::ptr::null_mut (); }
    unsafe { (*(($i) as *mut interval)).right = std::ptr::null_mut (); }
    set_interval_parent ($i as INTERVAL, std::ptr::null_mut ());
    interval_bits! (write_protect, $i, false);
    interval_bits! (visible, $i, false);
    interval_bits! (front_sticky, $i, false);
    interval_bits! (rear_sticky, $i, false);
    set_interval_plist ($i as INTERVAL, Qnil ());
  } };
}

/* Copy the cached property values of interval FROM to interval TO.  */
macro_rules! COPY_INTERVAL_CACHE {
  ($from:expr, $to:expr) => { {
    interval_bits! (write_protect, $to, interval_bits! (write_protect, $from));
    interval_bits! (visible, $to, interval_bits! (visible, $from));
    interval_bits! (front_sticky, $to, interval_bits! (front_sticky, $from));
    interval_bits! (rear_sticky, $to, interval_bits! (rear_sticky, $from));
  } };
}

/* Copy only the set bits of FROM's cache.  */
macro_rules! MERGE_INTERVAL_CACHE {
  ($from:expr, $to:expr) => { {
    if interval_bits! (write_protect, $from) { interval_bits! (write_protect, $to, true); }
    if interval_bits! (visible, $from) { interval_bits! (visible, $to, true); }
    if interval_bits! (front_sticky, $from) { interval_bits! (front_sticky, $to, true); }
    if interval_bits! (rear_sticky, $from) { interval_bits! (rear_sticky, $to, true); }
  } };
}

/* Is this interval visible?  Replace later with cache access.  */
macro_rules! INTERVAL_VISIBLE_P {
  ($i:expr) => { (!(($i) as *mut interval).is_null ()
		  && NILP (/* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { textget (unsafe { (*(($i) as *mut interval)).plist },
					      Qinvisible ()) })) };
}

/* Is this interval writable by virtue of not being marked read-only
   by the 'read-only' property (passed via RO), or due to the general
   value of Vinhibit_read_only?  Replace later with cache access.  */
macro_rules! INTERVAL_GENERALLY_WRITABLE_P {
  ($i:expr, $ro:expr) => { (NILP ($ro)
			    || (! NILP (/* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*Vinhibit_read_only.as_mut_ptr ()) })
				&& ! CONSP (unsafe { (*Vinhibit_read_only.as_mut_ptr ()) }))) };
}

/* Is this interval writable by virtue of its explicit
   'inhibit-read-only' property, or due to the presence of its
   'read-only' property (passed via RO) in Vinhibit_read_only list?  */
macro_rules! INTERVAL_EXPRESSLY_WRITABLE_P {
  ($i:expr, $ro:expr) => { (! NILP (/* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { textget (unsafe { (*(($i) as *mut interval)).plist },
						     Qinhibit_read_only ()) })
			    || (! NILP ($ro)
				&& CONSP (unsafe { (*Vinhibit_read_only.as_mut_ptr ()) })
				&& ! NILP (Fmemq ($ro, unsafe { (*Vinhibit_read_only.as_mut_ptr ()) })))) };
}

/* If PROP is the `invisible' property of a character,
   this is 1 if the character should be treated as invisible,
   and 2 if it is invisible but with an ellipsis.  */

// The first arm used to read `(prop)`, a *literal* token matcher: only a call site that
// spelled the token `prop` could match it, and every other caller (`indent.c`'s
// `TEXT_PROP_MEANS_INVISIBLE (prop)`, `TEXT_PROP_MEANS_INVISIBLE (propval)`) fell through
// with "cannot find value `prop`" — Rust's `macro_rules!` has no C-style implicit
// parameter.  The arm is deleted; the metavariable arm below is the macro (status §3.56's
// integrator-edit class, recorded in docs/ports/indent.md).
macro_rules! TEXT_PROP_MEANS_INVISIBLE {
  ($prop:expr) => { (if EQ (/* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { BVAR! ((current_buffer! ()), invisibility_spec_) }, Qt ()) { (! NILP ($prop)) as c_int }
		     else { unsafe { invisible_prop ($prop, unsafe { BVAR! ((current_buffer! ()), invisibility_spec_) }) } }) };
}


// Every `extern` prototype of intervals.h's "Declared in intervals.c" block, in C's own
// order.  The unit defines 24 of them and one glob import per mirror is what keeps the
// definition from being an `E0428` against its own declaration (PORTING.md 0.1-10/P-21, the
// composite.rs/tempname.rs shape); the other three of the block (`verify_interval_modification`,
// `validate_interval_range`, `interval_of`) are defined in textprop.c and are declared here
// because no mirror carries textprop.h yet.  They stay in this module so the block keeps C's
// declaration order; the glob import makes them visible to the units that call them today
// (and src/textprop.c's definition shadows it, 0.1-10).
pub mod intervals_decls {
  use super::*;

  /* intervals.h 247's `make_interval` is defined in *alloc.c*, not in intervals.c, which is
     why it stood at crate root in a block of its own - and why `src/alloc.rs` had to
     wrap its definition in a module instead of shadowing the declaration (status 3.82).  It
     belongs with its siblings here: the glob import is what lets the owner's definition win
     (PORTING.md 0.1-10/P-21).  */
  unsafe extern "C-unwind" {
    /// C: intervals.h 247: `extern INTERVAL make_interval (void) ATTRIBUTE_RETURNS_NONNULL;`
    pub fn make_interval () -> INTERVAL;
  }


  unsafe extern "C-unwind" {
    /// C: intervals.h 251: `extern INTERVAL create_root_interval (Lisp_Object);`
    pub fn create_root_interval (object: Lisp_Object) -> INTERVAL;
    /// C: intervals.h 252: `extern void copy_properties (INTERVAL, INTERVAL);`
    pub fn copy_properties (source: INTERVAL, target: INTERVAL);
    /// C: intervals.h 253: `extern bool intervals_equal (INTERVAL, INTERVAL);`
    pub fn intervals_equal (i0: INTERVAL, i1: INTERVAL) -> bool;
    /// C: intervals.h 248-250: `extern void traverse_intervals (INTERVAL, ptrdiff_t,
    /// void (*) (INTERVAL, void *), void *);`
    pub fn traverse_intervals (tree: INTERVAL, position: ptrdiff_t,
  			     function: Option<unsafe extern "C-unwind" fn (INTERVAL, *mut c_void)>,
  			     arg: *mut c_void);
    /// C: intervals.h 251-252: `extern void traverse_intervals_noorder (INTERVAL,
    /// void (*) (INTERVAL, void *), void *);`
    pub fn traverse_intervals_noorder (tree: INTERVAL,
  				     function: Option<unsafe extern "C-unwind" fn (INTERVAL, *mut c_void)>,
  				     arg: *mut c_void);
    /// C: intervals.h 253-254: `extern INTERVAL split_interval_right (INTERVAL, ptrdiff_t)
    /// ATTRIBUTE_RETURNS_NONNULL;`
    pub fn split_interval_right (interval: INTERVAL, offset: ptrdiff_t) -> INTERVAL;
    /// C: intervals.h 255: `extern INTERVAL split_interval_left (INTERVAL, ptrdiff_t)
    /// ATTRIBUTE_RETURNS_NONNULL;`
    pub fn split_interval_left (interval: INTERVAL, offset: ptrdiff_t) -> INTERVAL;
    /// C: intervals.h 262: `extern INTERVAL find_interval (INTERVAL, ptrdiff_t);`
    pub fn find_interval (tree: INTERVAL, position: ptrdiff_t) -> INTERVAL;
    /// C: intervals.h 263: `extern INTERVAL next_interval (INTERVAL);`
    pub fn next_interval (interval: INTERVAL) -> INTERVAL;
    /// C: intervals.h 264: `extern INTERVAL previous_interval (INTERVAL);`
    pub fn previous_interval (interval: INTERVAL) -> INTERVAL;
    /// C: intervals.h 265: `extern INTERVAL merge_interval_left (INTERVAL);`
    pub fn merge_interval_left (interval: INTERVAL) -> INTERVAL;
    /// C: intervals.h 266: `extern void offset_intervals (struct buffer *, ptrdiff_t, ptrdiff_t);`
    pub fn offset_intervals (buffer: *mut buffer, start: ptrdiff_t, length: ptrdiff_t);
    /// C: intervals.h 267-268: `extern void graft_intervals_into_buffer (INTERVAL, ptrdiff_t,
    /// ptrdiff_t, struct buffer *, bool);`
    pub fn graft_intervals_into_buffer (source: INTERVAL, position: ptrdiff_t,
  				      length: ptrdiff_t, buffer: *mut buffer, inherit: bool);
    /// C: intervals.h 269-270: `extern void verify_interval_modification (struct buffer *,
    /// ptrdiff_t, ptrdiff_t);`
    pub fn verify_interval_modification (buffer: *mut buffer, start: ptrdiff_t, end: ptrdiff_t);
    /// C: intervals.h 271: `extern INTERVAL balance_intervals (INTERVAL);`
    pub fn balance_intervals (tree: INTERVAL) -> INTERVAL;
    /// C: intervals.h 272-273: `extern void copy_intervals_to_string (Lisp_Object,
    /// struct buffer *, ptrdiff_t, ptrdiff_t);`
    pub fn copy_intervals_to_string (string: Lisp_Object, buffer: *mut buffer,
  				   position: ptrdiff_t, length: ptrdiff_t);
    /// C: intervals.h 274: `extern INTERVAL copy_intervals (INTERVAL, ptrdiff_t, ptrdiff_t);`
    pub fn copy_intervals (tree: INTERVAL, start: ptrdiff_t, length: ptrdiff_t) -> INTERVAL;
    /// C: intervals.h 275: `extern bool compare_string_intervals (Lisp_Object, Lisp_Object);`
    pub fn compare_string_intervals (s1: Lisp_Object, s2: Lisp_Object) -> bool;
    /// C: intervals.h 276: `extern Lisp_Object textget (Lisp_Object, Lisp_Object);`
    pub fn textget (prop: Lisp_Object, object: Lisp_Object) -> Lisp_Object;
    /// C: intervals.h 277: `extern Lisp_Object lookup_char_property (Lisp_Object, Lisp_Object,
    /// bool);`
    pub fn lookup_char_property (plist: Lisp_Object, prop: Lisp_Object, textprop: bool) -> Lisp_Object;
    /// C: intervals.h 278: `extern void move_if_not_intangible (ptrdiff_t);`
    pub fn move_if_not_intangible (position: ptrdiff_t);
    /// C: intervals.h 279-280: `extern bool get_property_and_range (ptrdiff_t, Lisp_Object,
    /// Lisp_Object *, ptrdiff_t *, ptrdiff_t *, Lisp_Object);`
    pub fn get_property_and_range (pos: ptrdiff_t, prop: Lisp_Object, val: *mut Lisp_Object,
  				 start: *mut ptrdiff_t, end: *mut ptrdiff_t,
  				 object: Lisp_Object) -> bool;
    /// C: intervals.h 281: `extern Lisp_Object get_local_map (ptrdiff_t, struct buffer *,
    /// Lisp_Object);`
    pub fn get_local_map (pos: ptrdiff_t, buffer: *mut buffer, prop: Lisp_Object) -> Lisp_Object;
    /// C: intervals.h 282: `extern INTERVAL update_interval (INTERVAL, ptrdiff_t);`
    pub fn update_interval (interval: INTERVAL, pos: ptrdiff_t) -> INTERVAL;
    /// C: intervals.h 283: `extern void set_intervals_multibyte (bool);`
    pub fn set_intervals_multibyte (multibyte: bool);
    /// C: intervals.h 284-285: `extern INTERVAL validate_interval_range (Lisp_Object,
    /// Lisp_Object *, Lisp_Object *, bool);`
    pub fn validate_interval_range (object: Lisp_Object, begin: *mut Lisp_Object,
  				  end: *mut Lisp_Object, force: bool) -> INTERVAL;
    /// C: intervals.h 286: `extern INTERVAL interval_of (ptrdiff_t, Lisp_Object);`
    pub fn interval_of (pos: ptrdiff_t, object: Lisp_Object) -> INTERVAL;
  }
}
pub use self::intervals_decls::*;

/* Defined in xdisp.c.  */
unsafe extern "C-unwind" {
  /// C: intervals.h 289: `extern int invisible_prop (Lisp_Object, Lisp_Object);`
  pub fn invisible_prop (prop: Lisp_Object, lookup_vector: Lisp_Object) -> c_int;
}

// The `Defined in textprop.c` block is in a module + glob import: src/textprop.c defines
// every one of these names, and a same-named `unsafe extern "C"` item at this scope would
// be `E0428` against the definition (PORTING.md 0.1-10/P-21, the buffer.rs
// `buffer_point_decls`/charset.rs `charset_decls` shape).  docs/ports/intervals.md and
// docs/ports/textprop.md record the move.
pub mod textprop_decls {
  use super::*;

  /* Defined in textprop.c.  */
  unsafe extern "C-unwind" {
    /// C: intervals.h 292-294: `extern Lisp_Object copy_text_properties (Lisp_Object,
    /// Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object);`
    pub fn copy_text_properties (start: Lisp_Object, end: Lisp_Object, src: Lisp_Object,
  			       pos: Lisp_Object, dest: Lisp_Object, prop: Lisp_Object) -> Lisp_Object;
    /// C: intervals.h 295-297: `extern Lisp_Object set_text_properties (Lisp_Object,
    /// Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object);`
    pub fn set_text_properties (start: Lisp_Object, end: Lisp_Object, properties: Lisp_Object,
  			      object: Lisp_Object, coherent_change: Lisp_Object) -> Lisp_Object;
    /// C: intervals.h 298-299: `extern void set_text_properties_1 (Lisp_Object, Lisp_Object,
    /// Lisp_Object, Lisp_Object, INTERVAL);`
    pub fn set_text_properties_1 (start: Lisp_Object, end: Lisp_Object, properties: Lisp_Object,
  				object: Lisp_Object, i: INTERVAL);
    /// C: intervals.h 301-302: `Lisp_Object text_property_list (Lisp_Object, Lisp_Object,
    /// Lisp_Object, Lisp_Object);`
    pub fn text_property_list (object: Lisp_Object, start: Lisp_Object, end: Lisp_Object,
  			     prop: Lisp_Object) -> Lisp_Object;
    /// C: intervals.h 303: `void add_text_properties_from_list (Lisp_Object, Lisp_Object,
    /// Lisp_Object);`
    pub fn add_text_properties_from_list (object: Lisp_Object, list: Lisp_Object, prop: Lisp_Object);
    /// C: intervals.h 304: `Lisp_Object extend_property_ranges (Lisp_Object, Lisp_Object,
    /// Lisp_Object);`
    pub fn extend_property_ranges (list: Lisp_Object, old: Lisp_Object, new: Lisp_Object) -> Lisp_Object;
    /// C: intervals.h 305-306: `Lisp_Object get_char_property_and_overlay (Lisp_Object,
    /// Lisp_Object, Lisp_Object, Lisp_Object *);`
    pub fn get_char_property_and_overlay (position: Lisp_Object, prop: Lisp_Object,
  					object: Lisp_Object, overlay: *mut Lisp_Object) -> Lisp_Object;
    /// C: intervals.h 307-308: `extern int text_property_stickiness (Lisp_Object prop,
    /// Lisp_Object pos, Lisp_Object buffer);`
    pub fn text_property_stickiness (prop: Lisp_Object, pos: Lisp_Object, buffer: Lisp_Object) -> c_int;
    /// C: intervals.h 310: `extern void syms_of_textprop (void);`
    pub fn syms_of_textprop ();
  }
}
pub use self::textprop_decls::*;

// INLINE_HEADER_END has no Rust counterpart (PORTING.md 0.1).
