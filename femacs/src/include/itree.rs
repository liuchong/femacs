// femacs: port of src/itree.h @ fd7afced403.
//
// Header mirror: types, constants and accessors only, plus the `unsafe extern "C"`
// declarations C spells as `extern` (PORTING.md 0.1).  Those declarations sit in
// `itree_externs`, because itree.c's port defines every one of them and a definition
// must shadow the import instead of colliding with it (P-21/P-39, the category.h
// shape).  The two loop macros are deferred (P-33); see
// docs/ports/excluded-macros.txt.  Layouts are asserted from the C compiler's numbers
// by `xtask layout-check`.

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const ITREE_H: bool = true;

/// The tree and node structs are mainly here, so they can be
/// allocated.
///
/// NOTE: The only time where it is safe to modify node.begin and
/// node.end directly, is while the node is not part of any tree.
///
/// NOTE: It is safe to read node.begin and node.end directly, if the
/// node came from an iterator, because it validates the nodes it
/// returns as a side-effect.  See ITREE_FOREACH.
#[repr(C)]
pub struct itree_node
{
  /// The normal parent, left and right links found in binary trees.
  /// See also 'red', below, which completes the Red-Black tree
  /// representation.
  pub parent: *mut itree_node,
  pub left: *mut itree_node,
  pub right: *mut itree_node,

  /// The following five fields comprise the interval abstraction.
  ///
  /// BEGIN, END are buffer positions describing the range.  When a
  /// node is in a tree these fields are read only, written only by
  /// itree functions.
  ///
  /// The LIMIT, OFFSET and OTICK fields should be considered internal
  /// to itree.c and used only by itree functions.
  ///
  /// LIMIT is a buffer position, the maximum of END of this node and
  /// its children.  See itree.c for its use.
  ///
  /// OFFSET is in buffer position units, and will be non-zero only
  /// when the node is dirty.
  ///
  /// OTICK determines whether BEGIN, END, LIMIT and OFFSET are
  /// considered dirty.  A node is clean when its OTICK is equal to
  /// the OTICK of its tree (see struct itree_tree).  Otherwise, it is
  /// dirty.
  ///
  /// In a clean node, BEGIN, END and LIMIT are correct buffer
  /// positions, and OFFSET is zero.  The parent of a clean node is
  /// also clean, recursively.
  ///
  /// In a dirty node, the node's OTICK won't equal its tree's OTICK,
  /// and its OFFSET may be non-zero.  At all times the descendents of
  /// a dirty node are also dirty.  BEGIN, END and LIMIT require
  /// adjustment before use as buffer positions.
  ///
  /// NOTE: BEGIN and END must not be modified while the node is part
  /// of a tree.  Use itree_insert_gap and itree_delete_gap instead.
  ///
  /// NOTE: The interval iterators ensure nodes are clean before
  /// yielding them, so BEGIN and END may be safely used as buffer
  /// positions then.
  pub begin: ptrdiff_t,		/* The beginning of this interval. */
  pub end: ptrdiff_t,		/* The end of the interval. */
  pub limit: ptrdiff_t,		/* The maximum end in this subtree. */
  pub offset: ptrdiff_t,	/* The amount of shift to apply to this subtree. */
  pub otick: uintmax_t,		/* offset modified tick */
  pub data: Lisp_Object,	/* Exclusively used by the client. */
  /// bool_bf red : 1; bool_bf rear_advance : 1; bool_bf front_advance : 1;
  /// `bool_bf` is one byte on the frozen target (`conf_post.h`), so the three bits
  /// share one byte: bits 0, 1 and 2 of this field (clang's record layout says
  /// 72:0, 72:1, 72:2).  PORTING.md 1.3 wants the storage unit recorded; the
  /// offsets around it and `sizeof itree_node` are asserted, but they cannot see
  /// the unit's width (P-35).
  pub bits: u8,
}

/// The three `bool_bf` members of itree_node, in itree.h's declaration order.
/// C names the bits through the bitfields; Rust reads them by mask (PORTING.md 1.3).
pub const ITREE_RED: u8 = 1 << 0;
pub const ITREE_REAR_ADVANCE: u8 = 1 << 1;
pub const ITREE_FRONT_ADVANCE: u8 = 1 << 2;

#[repr(C)]
pub struct itree_tree
{
  pub root: *mut itree_node,
  pub otick: uintmax_t,		/* offset tick, compared with node's otick. */
  pub size: intmax_t,		/* Number of nodes in the tree. */
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum itree_order
  {
    ITREE_ASCENDING,
    ITREE_DESCENDING,
    ITREE_PRE_ORDER,
    ITREE_POST_ORDER,
  }

/* The definitions are itree.c's; they are declared in a module so the unit that defines
   them shadows the import instead of colliding with it (P-21/P-39).  */
pub mod itree_externs {
  use super::*;

  unsafe extern "C" {
    // Parameter names come from the definitions in itree.c (C's prototypes have none).
    pub fn itree_node_init (node: *mut itree_node, front_advance: bool, rear_advance: bool,
                            data: Lisp_Object);
    pub fn itree_node_begin (tree: *mut itree_tree, node: *mut itree_node) -> ptrdiff_t;
    pub fn itree_node_end (tree: *mut itree_tree, node: *mut itree_node) -> ptrdiff_t;
    pub fn itree_node_set_region (tree: *mut itree_tree, node: *mut itree_node,
                                  begin: ptrdiff_t, end: ptrdiff_t);
    pub fn itree_create () -> *mut itree_tree;
    pub fn itree_destroy (tree: *mut itree_tree);
    pub fn itree_size (tree: *mut itree_tree) -> intmax_t;
    pub fn itree_clear (tree: *mut itree_tree);
    pub fn itree_insert (tree: *mut itree_tree, node: *mut itree_node,
                         begin: ptrdiff_t, end: ptrdiff_t);
    pub fn itree_remove (tree: *mut itree_tree, node: *mut itree_node) -> *mut itree_node;
    pub fn itree_insert_gap (tree: *mut itree_tree, pos: ptrdiff_t, length: ptrdiff_t,
                             before_markers: bool);
    pub fn itree_delete_gap (tree: *mut itree_tree, pos: ptrdiff_t, length: ptrdiff_t);

    /* Iteration functions.  Almost all code should use ITREE_FOREACH
       instead.  */
    pub fn itree_iterator_start (iter: *mut itree_iterator, tree: *mut itree_tree,
                                 begin: ptrdiff_t, end: ptrdiff_t,
                                 order: itree_order) -> *mut itree_iterator;
    pub fn itree_iterator_narrow (g: *mut itree_iterator, begin: ptrdiff_t,
                                  end: ptrdiff_t);
    pub fn itree_iterator_next (iterator: *mut itree_iterator) -> *mut itree_node;
  }
}

use itree_externs::*;

#[inline(always)]
pub fn itree_empty_p (tree: *mut itree_tree) -> bool
{
  // `||` short-circuits exactly as C's `!tree || !tree->root` does, so the
  // dereference only happens for a non-null tree (hence the unsafe block).
  return tree.is_null () || unsafe { (*tree).root.is_null () };
}

/// State used when iterating interval.
#[repr(C)]
pub struct itree_iterator
  {
    pub node: *mut itree_node,
    pub begin: ptrdiff_t,
    pub end: ptrdiff_t,
    pub otick: uintmax_t,	/* A copy of the tree's 'otick'.  */
    pub order: itree_order,
  }

/* Iterate over the intervals between BEG and END in the tree T.
   N will hold successive nodes.  ORDER can be one of : 'ASCENDING',
   'DESCENDING', 'POST_ORDER', or 'PRE_ORDER'.
   It should be used as:

      ITREE_FOREACH (n, t, beg, end, order)
        {
          .. do the thing with n ..
        }

   BEWARE:
   - The expression T may be evaluated more than once, so make sure
     it is cheap and pure.
   - Don't modify the tree during the iteration.
 */
// The macro itself is deferred (P-33): C declares its iterator at the expansion
// site and ITREE_FOREACH_NARROW reads it, which Rust macro hygiene forbids.
// Recorded in docs/ports/excluded-macros.txt; the five users (buffer.c,
// itree.c, editfns.c, textprop.c, xdisp.c) wait for that decision.
