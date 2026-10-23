// femacs: mirror of oldXMenu/X10.h @ fd7afced403.
//
// The V10 compatibility header the oldXMenu library includes for `Vertex` (the XDraw /
// XDrawFilled helper record) and for the `XAssoc`/`XAssocTable` association tables the menu
// windows are looked up in.  The frozen configuration builds no window system -
// `USE_X_TOOLKIT`, `HAVE_X11` and `HAVE_X_WINDOWS` are all `undef` in
// docs/baseline/config/darwin-arm64.macros.tsv - so no translation unit includes it and the
// mirror is never compiled; the text gate `tools/extract-abi.py --check` is its whole
// acceptance (docs/status.md 3.31).
//
// The X11 names this mirror spells - `Display`, `XID` - come from <X11/Xlib.h>; the port has
// no mirror for that header, so they keep C's spelling and are left for the unit that
// eventually uses this header to supply (PORTING.md P-26/P-72 (3)).

/*
 *	X10.h - Header definition and support file for the C subroutine
 *	interface library for V10 support routines.
 */

// C 24-25: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _X10_H_: bool = true;

/* Used in XDraw and XDrawFilled */

/// C 30-34: `typedef struct { short x, y; unsigned short flags; } Vertex;` (anonymous, so
/// the port names the record after its typedef).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex
{
  pub x: c_short,
  pub y: c_short,
  pub flags: c_ushort,
}

/* The meanings of the flag bits.  If the bit is 1 the predicate is true */

/// C 35: `#define VertexRelative 0x0001` - else absolute
pub const VertexRelative: c_int = 0x0001;
/// C 36: `#define VertexDontDraw 0x0002` - else draw
pub const VertexDontDraw: c_int = 0x0002;
/// C 37: `#define VertexCurved 0x0004` - else straight
pub const VertexCurved: c_int = 0x0004;
/// C 38: `#define VertexStartClosed 0x0008` - else not
pub const VertexStartClosed: c_int = 0x0008;
/// C 39: `#define VertexEndClosed 0x0010` - else not
pub const VertexEndClosed: c_int = 0x0010;
/* C 40: `/*#define VertexDrawLastPoint	0x0020 	*/` - C's own commented-out define.  */

/*
The VertexDrawLastPoint option has not been implemented in XDraw and
XDrawFilled so it shouldn't be defined.
*/

/*
 * XAssoc - Associations used in the XAssocTable data structure.  The
 * associations are used as circular queue entries in the association table
 * which is contains an array of circular queues (buckets).
 */
/// C 52-59: `typedef struct _XAssoc { ... } XAssoc;`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _XAssoc
{
  pub next: *mut _XAssoc,   /* Next object in this bucket. */
  pub prev: *mut _XAssoc,   /* Previous object in this bucket. */
  pub display: *mut Display,/* Display which owns the id. */
  pub x_id: XID,            /* X Window System id. */
  pub data: *mut c_void,    /* Pointer to untyped memory. */
}
/// C 59: `typedef struct _XAssoc { ... } XAssoc;`
pub type XAssoc = _XAssoc;

/*
 * XAssocTable - X Window System id to data structure pointer association
 * table.  An XAssocTable is a hash table whose buckets are circular
 * queues of XAssoc's.  The XAssocTable is constructed from an array of
 * XAssoc's which are the circular queue headers (bucket headers).
 * An XAssocTable consists an XAssoc pointer that points to the first
 * bucket in the bucket array and an integer that indicates the number
 * of buckets in the array.
 */
/// C 66-69: `typedef struct { XAssoc *buckets; int size; } XAssocTable;` (anonymous, so the
/// port names the record after its typedef).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XAssocTable
{
  pub buckets: *mut XAssoc, /* Pointer to first bucket in bucket array.*/
  pub size: c_int,          /* Table size (number of buckets). */
}

unsafe extern "C" {
  /// C 74: `XAssocTable *XCreateAssocTable(int size);`
  pub fn XCreateAssocTable (size: c_int) -> *mut XAssocTable;
  /// C 75: `char *XLookUpAssoc(Display *dpy, XAssocTable *table, XID x_id);`
  pub fn XLookUpAssoc (dpy: *mut Display, table: *mut XAssocTable, x_id: XID) -> *mut c_char;
}
