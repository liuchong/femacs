// femacs: port of src/itree.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names.  itree.c has
// one `#if` block, C 486-494's `#ifdef ITREE_TESTING` arm; ITREE_TESTING is not in the
// 4956-macro table and only the itree test program defines it, so the arm is dead text here
// and its body is kept as written (the rawmemchr.c CHERI-arm class).
//
// The `bool_bf` members of itree_node (red, rear_advance, front_advance) are the mirror's
// `bits` byte with the ITREE_* masks (itree.rs 96-104), so `node->red` is read as
// `(*node).bits & ITREE_RED` and written as `(*node).bits |= ITREE_RED` / `&= !ITREE_RED`;
// the token-aliases family `red` keeps the two spellings comparable.
//
// itree.h 166-176's `ITREE_FOREACH` is expanded at its one use site (itree_insert_gap, C
// 1073), because C's expansion declares the iterator at the use site and hands its name to
// the body, which Rust macro hygiene cannot do (P-33); the expansion and the decision are
// recorded in docs/ports/itree.md, and the macro's own name in
// docs/ports/macro-body-notes.tsv.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
// [U1] ../lib/include/math.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] include/itree.rs imported via common

/* C spells `enum itree_order`'s members bare (itree.h 104-110; the switch that reads them is
   itree.c 1238's), and Rust 2024 keeps
   enum variants out of scope, so the port imports them (PORTING.md 0.1).  */
use itree_order::*;


/*
   Intervals of the form [BEGIN, END), are stored as nodes inside a RB
   tree, ordered by BEGIN.  The core operation of this tree (besides
   insert, remove, etc.) is finding all intervals intersecting with
   some given interval.  In order to perform this operation
   efficiently, every node stores a third value called LIMIT. (See
   https://en.wikipedia.org/wiki/Interval_tree#Augmented_tree and its
   source Introduction to Algorithms, Cormen et al. .)

   ==== Finding intervals ====

   If we search for all intervals intersecting with (X, Y], we look at
   some node and test whether

   NODE.BEGIN > Y

   Due to the invariant of the search tree, we know, that we may
   safely prune NODE's right subtree if this test succeeds, since all
   intervals begin strictly after Y.

   But we can not make such an assumptions about the left tree, since
   all we know is that the intervals in this subtree must start before
   or at NODE.BEGIN.  So we can't tell, whether they end before X or
   not.  To solve this problem we add another attribute to each node,
   called LIMIT.

   The LIMIT of a node is the largest END value occurring in the nodes
   subtree (including the node itself).  Thus, we may look at the left
   child of some NODE and test whether

   NODE.left.LIMIT < X

   and this tells us, if all intervals in the left subtree of NODE end
   before X and if they can be pruned.

   Conversely, if this inequality is false, the left subtree must
   contain at least one intersecting interval, giving a resulting time
   complexity of O(K*log(N)) for this operation, where K is the size
   of the result set and N the size of the tree.

   ==== FIXME: bug#58342 some important operations remain slow ===

   The amortized costs of Emacs's previous-overlay-change and
   next-overlay-change functions are O(N) with this data structure.
   The root problem is that we only have an order for the BEG field,
   but not the END.  The previous/next overlay change operations need
   to find the nearest point where there is *either* an interval BEG
   or END point, but there is no efficient way to narrow the search
   space over END positions.

   Consider the case where next-overlay-change is called at POS, all
   interval BEG positions are less than pos POS and all interval END
   positions are after.  These END positions have no order, and so
   *every* interval must be examined.  This is at least O(N).  The
   previous-overlay-change case is similar.  The root issue is that
   the iterative "narrowing" approach is not guaranteed to reduce the
   search space in logarithmic time, since END is not ordered in the
   tree.

   One might argue that the LIMIT value will do this narrowing, but
   this narrowing is O(K*log(N)) where K is the size of the result
   set.  If we are interested in finding the node in a range with the
   smallest END, we might have to examine all K nodes in that range.
   In the case of the *-overlay-change functions, K may well be equal
   to N.

   Ideally, a tree based data structure for overlays would have
   O(log(N)) performance for previous-overlay-change and
   next-overlay-change, as these are called in performance sensitive
   situations such as redisplay.  The only way I can think of
   achieving this is by keeping one ordering by BEG and a separate
   ordering by END, and then performing logic quite similar to the
   current Emacs overlays-before and overlays-after lists.

   ==== Adjusting intervals ====

   Since this data-structure will be used for overlays in an Emacs
   buffer, a second core operation is the ability to insert and delete
   gaps in the tree.  This models the insertion and deletion of text
   in a buffer and the effects it may have on the positions of
   overlays.

   Consider this: Something gets inserted at position P into a buffer
   and assume that all overlays occur strictly after P.  Ordinarily,
   we would have to iterate all overlays and increment their BEGIN and
   END values accordingly (the insertion of text pushes them back).
   In order to avoid this, we introduce yet another node attribute,
   called OFFSET.

   The OFFSET of some subtree, represented by its root, is the
   amount of shift that needs to be applied to its BEGIN, END and
   LIMIT values, in order to get to the actual buffer positions.
   Coming back to the example, all we would need to do in this case,
   is to increment the OFFSET of the tree's root, without any
   traversal of the tree itself.

   As a consequence, the real values of BEGIN, END and LIMIT of some
   NODE need to be computed by incrementing them by the sum of NODE's
   OFFSET and all of its ancestors offsets.  Therefore, we store a
   counter (otick) inside every node and also the tree, by which we
   remember the fact, that a node's path to the root has no offsets
   applied (i.e. its values are up to date).  This is the case if some
   node's value differs from the tree's one, the later of which is
   incremented whenever some node's offset has changed.  */

/* +=======================================================================+
 * | Stack
 * +=======================================================================+ */

/* Simple dynamic array. */
#[repr(C)]
pub struct itree_stack
{
  pub nodes: *mut *mut itree_node,
  pub size: size_t,
  pub length: size_t,
}

/* This is just a simple dynamic array with stack semantics. */

unsafe fn itree_stack_create (initial_size: intmax_t) -> *mut itree_stack
{
  let stack: *mut itree_stack = xmalloc (size_of::<itree_stack> ()) as *mut itree_stack;
  (*stack).size = max (0, initial_size as isize) as size_t;
  (*stack).nodes = xmalloc ((*stack).size * size_of::<*mut itree_node> ()) as *mut *mut itree_node;
  (*stack).length = 0;
  return stack;
}

unsafe fn itree_stack_destroy (stack: *mut itree_stack)
{
  if stack.is_null ()
    { return; }
  if !(*stack).nodes.is_null ()
    { xfree ((*stack).nodes as *mut c_void); }
  xfree (stack as *mut c_void);
}

unsafe fn itree_stack_ensure_space (stack: *mut itree_stack, nelements: uintmax_t)
{
  if nelements > (*stack).size as uintmax_t
    {
      (*stack).size = (nelements as size_t + 1) * 2;
      (*stack).nodes = xrealloc ((*stack).nodes as *mut c_void,
				 (*stack).size * size_of::<*mut itree_node> ()) as *mut *mut itree_node;
    }
}

/* Push NODE on the STACK, while settings its visited flag to FLAG. */

unsafe fn itree_stack_push (stack: *mut itree_stack, node: *mut itree_node)
{
  eassert! (!node.is_null ());
  itree_stack_ensure_space (stack, (*stack).length as uintmax_t + 1);

  *(*stack).nodes.offset ((*stack).length as isize) = node;
  (*stack).length += 1;
}

unsafe fn itree_stack_pop (stack: *mut itree_stack) -> *mut itree_node
{
  if (*stack).length == 0
    { return std::ptr::null_mut (); }
  (*stack).length -= 1;
  return *(*stack).nodes.offset ((*stack).length as isize);
}

/* +-----------------------------------------------------------------------+ */

unsafe fn itree_max_height (tree: *const itree_tree) -> c_int
{
  return (2.0 * log (((*tree).size + 1) as c_double) / log (2.0) + 0.5) as c_int;
}

#[repr(C)]
pub struct check_subtree_result
{
  /* Node count of the tree.  */
  pub size: c_int,

  /* Limit of the tree (max END).  */
  pub limit: ptrdiff_t,

  /* Black height of the tree.  */
  pub black_height: c_int,
}

unsafe fn check_subtree (node: *mut itree_node,
			 check_red_black_invariants: bool, tree_otick: uintmax_t,
			 offset: ptrdiff_t, min_begin: ptrdiff_t,
			 max_begin: ptrdiff_t) -> check_subtree_result
{
  let mut result: check_subtree_result = check_subtree_result { size: 0,
					       limit: PTRDIFF_MIN,
					       black_height: 0 };
  if node.is_null ()
    { return result; }

  /* Validate structure.  */
  eassert! ((*node).left.is_null () || (*(*node).left).parent == node);
  eassert! ((*node).right.is_null () || (*(*node).right).parent == node);

  /* Validate otick.  A node's otick must be <= to the tree's otick
     and <= to its parent's otick.

     Note: we cannot assert that (NODE.otick == NODE.parent.otick)
     implies (NODE.offset == 0) because itree_inherit_offset()
     doesn't always update otick.  It could, but it is not clear there
     is a need.  */
  eassert! ((*node).otick <= tree_otick);
  eassert! ((*node).parent.is_null () || (*node).otick <= (*(*node).parent).otick);
  eassert! ((*node).otick != tree_otick || (*node).offset == 0);

  let mut offset: ptrdiff_t = offset;
  offset += (*node).offset;
  let begin: ptrdiff_t = (*node).begin + offset;
  let end: ptrdiff_t = (*node).end + offset;
  let limit: ptrdiff_t = (*node).limit + offset;

  eassert! (min_begin <= max_begin);
  eassert! (min_begin <= begin);
  eassert! (begin <= max_begin);
  eassert! (end <= limit);

  let left_result: check_subtree_result
    = check_subtree ((*node).left, check_red_black_invariants,
		     tree_otick, offset, min_begin, begin);
  let right_result: check_subtree_result
    = check_subtree ((*node).right, check_red_black_invariants,
		     tree_otick, offset, begin, max_begin);

  eassert! (left_result.limit <= limit);
  eassert! (right_result.limit <= limit);
  eassert! (limit == max (end, max (left_result.limit, right_result.limit)));

  if check_red_black_invariants
    {
      eassert! (left_result.black_height == right_result.black_height);
      eassert! ((*node).parent.is_null () || ((*node).bits & ITREE_RED) == 0
		|| (*(*node).parent).bits & ITREE_RED == 0);
    }

  result.size = 1 + left_result.size + right_result.size;
  result.limit = limit;
  result.black_height = (if (*node).bits & ITREE_RED != 0 { 0 } else { 1 })
    + left_result.black_height;
  return result;
}

/* Validate invariants for TREE.  If CHECK_RED_BLACK_INVARIANTS, red
   nodes with red children are considered invalid.

   This runs in constant time when ENABLE_OVERLAY_CHECKING is 0
   (i.e. Emacs is not configured with
   "--enable-checking=yes,overlays").  In this mode it can't check all
   the invariants.  When ENABLE_OVERLAY_CHECKING is 1 it checks the
   entire tree and validates all invariants.
*/

unsafe fn check_tree (tree: *mut itree_tree,
		      check_red_black_invariants: bool) -> bool
{
  eassert! (!tree.is_null ());
  eassert! ((*tree).size >= 0);
  eassert! (((*tree).size == 0) == ((*tree).root.is_null ()));
  if (*tree).root.is_null ()
    { return true; }
  eassert! ((*(*tree).root).parent.is_null ());
  eassert! (!check_red_black_invariants || ((*(*tree).root).bits & ITREE_RED) == 0);

  let node: *mut itree_node = (*tree).root;
  let result: check_subtree_result
    = check_subtree (node, check_red_black_invariants, (*tree).otick,
		     (*node).offset, PTRDIFF_MIN,
		     PTRDIFF_MAX);
  eassert! (result.size as intmax_t == (*tree).size);

  /* The only way this function fails is eassert().  */
  return true;
}

/* +=======================================================================+
 * | Internal Functions
 * +=======================================================================+ */

unsafe fn null_safe_is_red (node: *mut itree_node) -> bool
{
  return !node.is_null () && ((*node).bits & ITREE_RED) != 0;
}

unsafe fn null_safe_is_black (node: *mut itree_node) -> bool
{
  return node.is_null () || ((*node).bits & ITREE_RED) == 0; /* NULL nodes are black */
}

unsafe fn itree_newlimit (node: *mut itree_node) -> ptrdiff_t
{
  eassert! (!node.is_null ());
  return max ((*node).end,
	      max (if (*node).left.is_null ()
		     { PTRDIFF_MIN }
		     else
		     { (*(*node).left).limit + (*(*node).left).offset },
		   if (*node).right.is_null ()
		     { PTRDIFF_MIN }
		     else
		     { (*(*node).right).limit + (*(*node).right).offset }));
}

/* Update NODE's limit attribute according to its children. */

unsafe fn itree_update_limit (node: *mut itree_node)
{
  if node.is_null ()
    { return; }

  (*node).limit = itree_newlimit (node);
}

/* Apply NODE's offset to its begin, end and limit values and
   propagate it to its children.

   Does nothing, if NODE is clean, i.e. NODE.otick = tree.otick .
*/

unsafe fn itree_inherit_offset (otick: uintmax_t, node: *mut itree_node)
{
  eassert! ((*node).parent.is_null () || (*(*node).parent).otick >= (*node).otick);
  if (*node).otick == otick
    {
      eassert! ((*node).offset == 0);
      return;
    }

  /* Offsets can be inherited from dirty nodes (with out of date
     otick) during removal, since we do not travel down from the root
     in that case.  In this case rotations are performed on
     potentially "dirty" nodes, where we only need to make sure the
     *local* offsets are zero.  */

  if (*node).offset != 0
    {
      (*node).begin += (*node).offset;
      (*node).end   += (*node).offset;
      (*node).limit += (*node).offset;
      if !(*node).left.is_null ()
	{ (*(*node).left).offset += (*node).offset; }
      if !(*node).right.is_null ()
	{ (*(*node).right).offset += (*node).offset; }
      (*node).offset = 0;
    }
  /* The only thing that matters about 'otick' is whether it's equal to
     that of the tree.  We could also "blindly" inherit from parent->otick,
     but we need the tree's 'otick' anyway for when there's no parent.  */
  if (*node).parent.is_null () || (*(*node).parent).otick == otick
    { (*node).otick = otick; }
}

/* Update limit of NODE and its ancestors.  Stop when it becomes
   stable, i.e. new_limit = old_limit.  */

unsafe fn itree_propagate_limit (node: *mut itree_node)
{
  let mut newlimit: ptrdiff_t;

  if node.is_null ()
    { return; }

  let mut node: *mut itree_node = node;
  loop
    {
      newlimit = itree_newlimit (node);

      if newlimit == (*node).limit
	{ break; }
      (*node).limit = newlimit;
      if (*node).parent.is_null ()
	{ break; }
      node = (*node).parent;
    }
}

unsafe fn itree_validate (tree: *mut itree_tree, node: *mut itree_node)
  -> *mut itree_node
{

  if (*tree).otick == (*node).otick || node.is_null ()
    { return node; }
  if node != (*tree).root
    { itree_validate (tree, (*node).parent); }

  itree_inherit_offset ((*tree).otick, node);
  return node;
}

/* +=======================================================================+
 * | Tree operations
 * +=======================================================================+ */

/* Initialize an allocated node. */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_node_init (node: *mut itree_node,
					  front_advance: bool, rear_advance: bool,
					  data: Lisp_Object)
{
  (*node).parent = std::ptr::null_mut ();
  (*node).left = std::ptr::null_mut ();
  (*node).right = std::ptr::null_mut ();
  (*node).begin = -1;
  (*node).end = -1;
  (*node).bits = if front_advance
    { (*node).bits | ITREE_FRONT_ADVANCE } else { (*node).bits & !ITREE_FRONT_ADVANCE };
  (*node).bits = if rear_advance
    { (*node).bits | ITREE_REAR_ADVANCE } else { (*node).bits & !ITREE_REAR_ADVANCE };
  (*node).data = data;
}

/* Return NODE's begin value, computing it if necessary. */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_node_begin (tree: *mut itree_tree,
					   node: *mut itree_node) -> ptrdiff_t
{
  itree_validate (tree, node);
  return (*node).begin;
}

/* Return NODE's end value, computing it if necessary. */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_node_end (tree: *mut itree_tree,
					 node: *mut itree_node) -> ptrdiff_t
{
  itree_validate (tree, node);
  return (*node).end;
}

/* Allocate an itree_tree.  Free with itree_destroy.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_create () -> *mut itree_tree
{
  let tree: *mut itree_tree = xmalloc (size_of::<itree_tree> ()) as *mut itree_tree;
  itree_clear (tree);
  return tree;
}

/* Reset the tree TREE to its empty state.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_clear (tree: *mut itree_tree)
{
  (*tree).root = std::ptr::null_mut ();
  (*tree).otick = 1;
  (*tree).size = 0;
}

/* C 486-494's `#ifdef ITREE_TESTING` arm: ITREE_TESTING is not in the frozen macro table
   and only the itree test program defines it, so this arm is dead text C-side; its body is
   kept as written (the rawmemchr.c CHERI-arm class).  */
unsafe fn itree_init (tree: *mut itree_tree)
{
  itree_clear (tree);
}

/* Release a tree, freeing its allocated memory.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_destroy (tree: *mut itree_tree)
{
  eassert! ((*tree).root.is_null ());
  xfree (tree as *mut c_void);
}

/* Return the number of nodes in TREE.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_size (tree: *mut itree_tree) -> intmax_t
{
  return (*tree).size;
}

/* Perform the familiar left-rotation on node NODE.  */

unsafe fn itree_rotate_left (tree: *mut itree_tree,
			     node: *mut itree_node)
{
  eassert! (!(*node).right.is_null ());

  let right: *mut itree_node = (*node).right;

  itree_inherit_offset ((*tree).otick, node);
  itree_inherit_offset ((*tree).otick, right);

  /* Turn right's left subtree into node's right subtree.  */
  (*node).right = (*right).left;
  if !(*right).left.is_null ()
    { (*(*right).left).parent = node; }

  /* right's parent was node's parent.  */
  if !right.is_null ()
    { (*right).parent = (*node).parent; }

  /* Get the parent to point to right instead of node.  */
  if node != (*tree).root
    {
      if node == (*(*node).parent).left
	{ (*(*node).parent).left = right; }
      else
	{ (*(*node).parent).right = right; }
    }
  else
    { (*tree).root = right; }

  /* Put node on right's left.  */
  (*right).left = node;
  if !node.is_null ()
    { (*node).parent = right; }

  /* Order matters here.  */
  itree_update_limit (node);
  itree_update_limit (right);
}

/* Perform the familiar right-rotation on node NODE.  */

unsafe fn itree_rotate_right (tree: *mut itree_tree,
			      node: *mut itree_node)
{
  eassert! (!tree.is_null () && !node.is_null () && !(*node).left.is_null ());

  let left: *mut itree_node = (*node).left;

  itree_inherit_offset ((*tree).otick, node);
  itree_inherit_offset ((*tree).otick, left);

  (*node).left = (*left).right;
  if !(*left).right.is_null ()
    { (*(*left).right).parent = node; }

  if !left.is_null ()
    { (*left).parent = (*node).parent; }
  if node != (*tree).root
    {
      if node == (*(*node).parent).right
	{ (*(*node).parent).right = left; }
      else
	{ (*(*node).parent).left = left; }
    }
  else
    { (*tree).root = left; }

  (*left).right = node;
  if !node.is_null ()
    { (*node).parent = left; }

  itree_update_limit (left);
  itree_update_limit (node);
}

/* Repair the tree after an insertion.
   The new NODE was added as red, so we may have 2 reds in a row.
   Rebalance the parents as needed to re-establish the RB invariants.  */

unsafe fn itree_insert_fix (tree: *mut itree_tree,
			    node: *mut itree_node)
{
  eassert! (((*(*tree).root).bits & ITREE_RED) == 0);

  let mut node: *mut itree_node = node;
  while null_safe_is_red ((*node).parent)
    {
      /* NODE is red and its parent is red.  This is a violation of
	 red-black tree property #3.  */
      eassert! (((*node).bits & ITREE_RED) != 0);

      if (*node).parent == (*(*(*node).parent).parent).left
	{
	  /* We're on the left side of our grandparent, and OTHER is
	     our "uncle".  */
	  let uncle: *mut itree_node = (*(*(*node).parent).parent).right;

	  if null_safe_is_red (uncle) /* case 1.a */
	    {
	      /* Uncle and parent are red but should be black because
		 NODE is red.  Change the colors accordingly and
		 proceed with the grandparent.  */
	      (*(*node).parent).bits &= !ITREE_RED;
	      (*uncle).bits &= !ITREE_RED;
	      (*(*(*node).parent).parent).bits |= ITREE_RED;
	      node = (*(*node).parent).parent;
	    }
	  else
	    {
	      /* Parent and uncle have different colors; parent is
		 red, uncle is black.  */
	      if node == (*(*node).parent).right /* case 2.a */
		{
		  node = (*node).parent;
		  itree_rotate_left (tree, node);
		}
	      /* case 3.a */
	      (*(*node).parent).bits &= !ITREE_RED;
	      (*(*(*node).parent).parent).bits |= ITREE_RED;
	      itree_rotate_right (tree, (*(*node).parent).parent);
	    }
	}
      else
	{
	  /* This is the symmetrical case of above.  */
	  let uncle: *mut itree_node = (*(*(*node).parent).parent).left;

	  if null_safe_is_red (uncle) /* case 1.b */
	    {
	      (*(*node).parent).bits &= !ITREE_RED;
	      (*uncle).bits &= !ITREE_RED;
	      (*(*(*node).parent).parent).bits |= ITREE_RED;
	      node = (*(*node).parent).parent;
	    }
	  else
	    {
	      if node == (*(*node).parent).left /* case 2.b */
		{
		  node = (*node).parent;
		  itree_rotate_right (tree, node);
		}
	      /* case 3.b */
	      (*(*node).parent).bits &= !ITREE_RED;
	      (*(*(*node).parent).parent).bits |= ITREE_RED;
	      itree_rotate_left (tree, (*(*node).parent).parent);
	    }
	}
    }

  /* The root may have been changed to red due to the algorithm.
     Set it to black so that property #5 is satisfied.  */
  (*(*tree).root).bits &= !ITREE_RED;
  eassert! (check_tree (tree, true)); /* FIXME: Too expensive.  */
}

/* Insert a NODE into the TREE.
   Note, that inserting a node twice results in undefined behavior.  */

unsafe fn itree_insert_node (tree: *mut itree_tree, node: *mut itree_node)
{
  eassert! (!node.is_null () && (*node).begin <= (*node).end);
  eassert! ((*node).left.is_null () && (*node).right.is_null ()
	    && (*node).parent.is_null ());
  eassert! (check_tree (tree, true)); /* FIXME: Too expensive.  */

  let mut parent: *mut itree_node = std::ptr::null_mut ();
  let mut child: *mut itree_node = (*tree).root;
  let otick: uintmax_t = (*tree).otick;
  /* It's the responsibility of the caller to set 'otick' on the node,
     to "confirm" that the begin/end fields are up to date.  */
  eassert! ((*node).otick == otick);

  /* Find the insertion point, accumulate node's offset and update
     ancestors limit values.  */
  while !child.is_null ()
    {
      itree_inherit_offset (otick, child);
      parent = child;
      eassert! ((*child).offset == 0);
      (*child).limit = max ((*child).limit, (*node).end);
      /* This suggests that nodes in the right subtree are strictly
	 greater.  But this is not true due to later rotations.  */
      child = if (*node).begin <= (*child).begin { (*child).left } else { (*child).right };
    }

  /* Insert the node */
  if parent.is_null ()
    { (*tree).root = node; }
  else if (*node).begin <= (*parent).begin
    { (*parent).left = node; }
  else
    { (*parent).right = node; }

  /* Init the node */
  (*node).parent = parent;
  (*node).left = std::ptr::null_mut ();
  (*node).right = std::ptr::null_mut ();
  (*node).offset = 0;
  (*node).limit = (*node).end;
  eassert! ((*node).parent.is_null () || (*(*node).parent).otick >= (*node).otick);

  /* Fix/update the tree */
  (*tree).size += 1;
  if node == (*tree).root
    { (*node).bits &= !ITREE_RED; }
  else
    {
      (*node).bits |= ITREE_RED;
      eassert! (check_tree (tree, false)); /* FIXME: Too expensive.  */
      itree_insert_fix (tree, node);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_insert (tree: *mut itree_tree, node: *mut itree_node,
				       begin: ptrdiff_t, end: ptrdiff_t)
{
  (*node).begin = begin;
  (*node).end = end;
  (*node).otick = (*tree).otick;
  itree_insert_node (tree, node);
}

/* Safely modify a node's interval. */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_node_set_region (tree: *mut itree_tree,
						node: *mut itree_node,
						begin: ptrdiff_t, end: ptrdiff_t)
{
  itree_validate (tree, node);
  if begin != (*node).begin
    {
      itree_remove (tree, node);
      (*node).begin = min (begin, PTRDIFF_MAX - 1);
      (*node).end = max ((*node).begin, end);
      itree_insert_node (tree, node);
    }
  else if end != (*node).end
    {
      (*node).end = max ((*node).begin, end);
      eassert! (!node.is_null ());
      itree_propagate_limit (node);
    }
}

/* Return true, if NODE is a member of TREE. */

unsafe fn itree_contains (tree: *mut itree_tree, node: *mut itree_node) -> bool
{
  eassert! (!node.is_null ());
  let mut node: *mut itree_node = node;
  let root: *mut itree_node = (*tree).root;
  while node != root
    {
      let parent: *mut itree_node = (*node).parent;
      if parent.is_null ()
	{ return false; }
      eassert! ((*parent).left == node || (*parent).right == node);
      node = parent;
    }
  return true;
}

unsafe fn itree_limit_is_stable (node: *mut itree_node) -> bool
{
  if node.is_null ()
    { return true; }
  let newlimit: ptrdiff_t = itree_newlimit (node);
  return (newlimit == (*node).limit);
}

unsafe fn itree_subtree_min (otick: uintmax_t, node: *mut itree_node)
  -> *mut itree_node
{
  let mut node: *mut itree_node = node;
  if node.is_null ()
    { return node; }
  while ({ itree_inherit_offset (otick, node); !(*node).left.is_null () })
    { node = (*node).left; }
  return node;
}

/* Repair the tree after a deletion.
   The black-depth of NODE is one less than that of its sibling,
   so re-balance the parents to re-establish the RB invariants.  */

unsafe fn itree_remove_fix (tree: *mut itree_tree,
			    node: *mut itree_node,
			    parent: *mut itree_node)
{
  if parent.is_null ()
    { eassert! (node == (*tree).root); }
  else
    { eassert! (node.is_null () || (*node).parent == parent); }

  let mut node: *mut itree_node = node;
  let mut parent: *mut itree_node = parent;
  while !parent.is_null () && null_safe_is_black (node)
    {
      eassert! (node == (*parent).left || node == (*parent).right);

      if node == (*parent).left
	{
	  let mut other: *mut itree_node = (*parent).right;

	  if ((*other).bits & ITREE_RED) != 0 /* case 1.a */
	    {
	      (*other).bits &= !ITREE_RED;
	      (*parent).bits |= ITREE_RED;
	      itree_rotate_left (tree, parent);
	      other = (*parent).right;
	    }

	  if (null_safe_is_black ((*other).left) /* 2.a */
	      && null_safe_is_black ((*other).right))
	    {
	      (*other).bits |= ITREE_RED;
	      node = parent;
	      eassert! (!node.is_null ());
	      parent = (*node).parent;
	    }
	  else
	    {
	      if null_safe_is_black ((*other).right) /* 3.a */
		{
		  (*(*other).left).bits &= !ITREE_RED;
		  (*other).bits |= ITREE_RED;
		  itree_rotate_right (tree, other);
		  other = (*parent).right;
		}
	      (*other).bits = if ((*parent).bits & ITREE_RED) != 0
		{ (*other).bits | ITREE_RED } else { (*other).bits & !ITREE_RED }; /* 4.a */
	      (*parent).bits &= !ITREE_RED;
	      (*(*other).right).bits &= !ITREE_RED;
	      itree_rotate_left (tree, parent);
	      node = (*tree).root;
	      parent = std::ptr::null_mut ();
	    }
	}
      else
	{
	  let mut other: *mut itree_node = (*parent).left;

	  if ((*other).bits & ITREE_RED) != 0 /* 1.b */
	    {
	      (*other).bits &= !ITREE_RED;
	      (*parent).bits |= ITREE_RED;
	      itree_rotate_right (tree, parent);
	      other = (*parent).left;
	    }

	  if (null_safe_is_black ((*other).right) /* 2.b */
	      && null_safe_is_black ((*other).left))
	    {
	      (*other).bits |= ITREE_RED;
	      node = parent;
	      eassert! (!node.is_null ());
	      parent = (*node).parent;
	    }
	  else
	    {
	      if null_safe_is_black ((*other).left) /* 3.b */
		{
		  (*(*other).right).bits &= !ITREE_RED;
		  (*other).bits |= ITREE_RED;
		  itree_rotate_left (tree, other);
		  other = (*parent).left;
		}

	      (*other).bits = if ((*parent).bits & ITREE_RED) != 0
		{ (*other).bits | ITREE_RED } else { (*other).bits & !ITREE_RED }; /* 4.b */
	      (*parent).bits &= !ITREE_RED;
	      (*(*other).left).bits &= !ITREE_RED;
	      itree_rotate_right (tree, parent);
	      node = (*tree).root;
	      parent = std::ptr::null_mut ();
	    }
	}
    }

  if !node.is_null ()
    { (*node).bits &= !ITREE_RED; }
}

/* Return accumulated offsets of NODE's parents.  */
unsafe fn itree_total_offset (node: *mut itree_node) -> ptrdiff_t
{
  eassert! (!node.is_null ());
  let mut offset: ptrdiff_t = 0;
  let mut node: *mut itree_node = node;
  while !(*node).parent.is_null ()
    {
      node = (*node).parent;
      offset += (*node).offset;
    }
  return offset;
}

/* Replace DEST with SOURCE as a child of DEST's parent.  Adjusts
   *only* the parent linkage of SOURCE and either the parent's child
   link the tree root.

   Warning: DEST is left unmodified.  SOURCE's child links are
   unchanged.  Caller is responsible for recalculation of 'limit'.
   Requires both nodes to be using the same effective 'offset'.  */
unsafe fn itree_replace_child (tree: *mut itree_tree,
			       source: *mut itree_node,
			       dest: *mut itree_node)
{
  eassert! (!tree.is_null () && !dest.is_null ());
  eassert! (source.is_null ()
	    || itree_total_offset (source) == itree_total_offset (dest));

  if dest == (*tree).root
    { (*tree).root = source; }
  else if dest == (*(*dest).parent).left
    { (*(*dest).parent).left = source; }
  else
    { (*(*dest).parent).right = source; }

  if !source.is_null ()
    { (*source).parent = (*dest).parent; }
}
/* Replace DEST with SOURCE in the tree.  Copies the following fields
   from DEST to SOURCE: red, parent, left, right.  Also updates
   parent, left and right in surrounding nodes to point to SOURCE.

   Warning: DEST is left unmodified.  Caller is responsible for
   recalculation of 'limit'.  Requires both nodes to be using the same
   effective 'offset'. */
unsafe fn itree_transplant (tree: *mut itree_tree,
			    source: *mut itree_node,
			    dest: *mut itree_node)
{
  itree_replace_child (tree, source, dest);
  (*source).left = (*dest).left;
  if !(*source).left.is_null ()
    { (*(*source).left).parent = source; }
  (*source).right = (*dest).right;
  if !(*source).right.is_null ()
    { (*(*source).right).parent = source; }
  (*source).bits = if ((*dest).bits & ITREE_RED) != 0
    { (*source).bits | ITREE_RED } else { (*source).bits & !ITREE_RED };
}

/* Remove NODE from TREE and return it.  NODE must exist in TREE.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_remove (tree: *mut itree_tree, node: *mut itree_node)
  -> *mut itree_node
{
  eassert! (itree_contains (tree, node));
  eassert! (check_tree (tree, true)); /* FIXME: Too expensive.  */

  /* We can get here straight from, say, 'move-overlay', so NODE may be dirty.
     Strictly speaking, we could propagate NODE's offset to its children
     locally (and thus leave it dirty if there are pending offsets higher
     up the tree), but it's not clear it's worth the added complexity
     in the resulting invariants.  */
  itree_validate (tree, node);
  /* Find 'splice', the leaf node to splice out of the tree.  When
     'node' has at most one child this is 'node' itself.  Otherwise,
     it is the in order successor of 'node'.  */
  let splice: *mut itree_node
    = if (*node).left.is_null () || (*node).right.is_null ()
	{ node }
	else
	{ itree_subtree_min ((*tree).otick, (*node).right) };

  /* Find 'subtree', the only child of 'splice' (may be NULL).  Note:
     'subtree' will not be modified other than changing its parent to
     'splice'.  */
  eassert! ((*splice).left.is_null () || (*splice).right.is_null ());
  let subtree: *mut itree_node
    = if !(*splice).left.is_null () { (*splice).left } else { (*splice).right };

  /* Save a pointer to the parent of where 'subtree' will eventually
     be in 'subtree_parent'.  */
  let subtree_parent: *mut itree_node
    = if (*splice).parent != node { (*splice).parent } else { splice };

  /* If 'splice' is black removing it may violate Red-Black
     invariants, so note this for later.  */

  /* Replace 'splice' with 'subtree' under subtree's parent.  If
     'splice' is black, this creates a red-red violation, so remember
     this now as the field can be overwritten when splice is
     transplanted below.  */
  itree_replace_child (tree, subtree, splice);
  let removed_black: bool = ((*splice).bits & ITREE_RED) == 0;

  /* Replace 'node' with 'splice' in the tree and propagate limit
     upwards, if necessary.  Note: Limit propagation can stabilize at
     any point, so we must call from bottom to top for every node that
     has a new child.  */
  if splice != node
    {
      itree_transplant (tree, splice, node);
      itree_propagate_limit (subtree_parent);
      if splice != subtree_parent
	{ itree_update_limit (splice); }
    }
  itree_propagate_limit ((*splice).parent);

  (*tree).size -= 1;

  /* Fix any black height violation caused by removing a black node.  */
  if removed_black
    { itree_remove_fix (tree, subtree, subtree_parent); }

  eassert! (((*tree).size == 0) == ((*tree).root.is_null ()));
  eassert! (check_tree (tree, true)); /* FIXME: Too expensive.  */

  /* Clear fields related to the tree for sanity while debugging.  */
  (*node).bits &= !ITREE_RED;
  (*node).parent = std::ptr::null_mut ();
  (*node).left = std::ptr::null_mut ();
  (*node).right = std::ptr::null_mut ();
  (*node).limit = 0;

  /* Must be clean (all offsets applied).  Also, some callers rely on
     node's otick being the tree's otick.  */
  eassert! ((*node).otick == (*tree).otick);
  eassert! ((*node).offset == 0);

  return node;
}

/* +=======================================================================+
 * | Insert/Delete Gaps
 * +=======================================================================+ */

/* Insert a gap at POS of length LENGTH expanding all intervals
   intersecting it, while respecting their rear_advance and
   front_advance setting.

   If BEFORE_MARKERS is non-zero, all overlays beginning/ending at POS
   are treated as if their front_advance/rear_advance was true. */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_insert_gap (tree: *mut itree_tree,
					   pos: ptrdiff_t, length: ptrdiff_t,
					   before_markers: bool)
{
  if tree.is_null () || length <= 0 || (*tree).root.is_null ()
    { return; }
  let ootick: uintmax_t = (*tree).otick;

  /* FIXME: Don't allocate iterator/stack anew every time. */

  /* Nodes with front_advance starting at pos may mess up the tree
     order, so we need to remove them first.  This doesn't apply for
     'before_markers' since in that case, all positions move identically
     regardless of 'front_advance' or 'rear_advance'.  */
  let saved: *mut itree_stack = itree_stack_create (0);
  let mut node: *mut itree_node = std::ptr::null_mut ();
  if !before_markers
    {
      /* Actually any order would do.  */
      /* itree.h 166-176's `ITREE_FOREACH (node, tree, pos, pos + 1, PRE_ORDER)` is
	 expanded here: C's expansion declares the iterator and drives it with
	 `for (…; ((n = itree_iterator_next (itree_iter_)));)`, so the port writes that
	 loop out, with the iterator C declares at the expansion site (P-33).  */
      if tree.is_null ()
	{ }
      else
	{
	  let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
	  let itree_iter_: *mut itree_iterator
	    = itree_iterator_start (&raw mut itree_local_iter_,
				    tree, pos, pos + 1, ITREE_PRE_ORDER);
	  while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	    {
	      if (*node).begin == pos && ((*node).bits & ITREE_FRONT_ADVANCE) != 0
		  /* If we have front_advance and !rear_advance and
		     the overlay is empty, make sure we don't move
		     begin past end by pretending it's !front_advance.  */
		  && ((*node).begin != (*node).end
		      || ((*node).bits & ITREE_REAR_ADVANCE) != 0)
		{ itree_stack_push (saved, node); }
	    }
	}
    }
  let mut i: size_t = 0;
  while i < (*saved).length
    {
      itree_remove (tree, *(*saved).nodes.offset (i as isize));
      i += 1;
    }

  /* We can't use an iterator here, because we can't effectively
     narrow AND shift some subtree at the same time.  */
  if !(*tree).root.is_null ()
    {
      let size: c_int = itree_max_height (tree) + 1;
      let stack: *mut itree_stack = itree_stack_create (size as intmax_t);
      itree_stack_push (stack, (*tree).root);
      loop
	{
	  node = itree_stack_pop (stack);
	  if node.is_null () { break; }
	  /* Process in pre-order. */
	  itree_inherit_offset ((*tree).otick, node);
	  if pos > (*node).limit
	    { continue; }
	  if !(*node).right.is_null ()
	    {
	      if (*node).begin > pos
		{
		  /* All nodes in this subtree are shifted by length.  */
		  (*(*node).right).offset += length;
		  (*tree).otick += 1;
		}
	      else
		{ itree_stack_push (stack, (*node).right); }
	    }
	  if !(*node).left.is_null ()
	    { itree_stack_push (stack, (*node).left); }

	  if if before_markers
	       { (*node).begin >= pos }
	     else
	       { (*node).begin > pos } /* node->begin == pos => !front-advance  */
	    { (*node).begin += length; }
	  if (*node).end > pos
	      || ((*node).end == pos && (before_markers || ((*node).bits & ITREE_REAR_ADVANCE) != 0))
	    {
	      (*node).end += length;
	      eassert! (!node.is_null ());
	      itree_propagate_limit (node);
	    }
	}
      itree_stack_destroy (stack);
    }

  /* Reinsert nodes starting at POS having front-advance.  */
  let notick: uintmax_t = (*tree).otick;
  loop
    {
      node = itree_stack_pop (saved);
      if node.is_null () { break; }
      eassert! ((*node).otick == ootick);
      eassert! ((*node).begin == pos);
      eassert! ((*node).end > pos || ((*node).bits & ITREE_REAR_ADVANCE) != 0);
      (*node).begin += length;
      (*node).end += length;
      (*node).otick = notick;
      itree_insert_node (tree, node);
    }

  itree_stack_destroy (saved);
}

/* Delete a gap at POS of length LENGTH, contracting all intervals
   intersecting it.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_delete_gap (tree: *mut itree_tree,
					   pos: ptrdiff_t, length: ptrdiff_t)
{
  if tree.is_null () || length <= 0 || (*tree).root.is_null ()
    { return; }

  /* FIXME: Don't allocate stack anew every time.  */

  /* Can't use the iterator here, because by decrementing begin, we
     might unintentionally bring shifted nodes back into our search space.  */
  let size: c_int = itree_max_height (tree) + 1;
  let stack: *mut itree_stack = itree_stack_create (size as intmax_t);
  let mut node: *mut itree_node;

  itree_stack_push (stack, (*tree).root);
  loop
    {
      node = itree_stack_pop (stack);
      if node.is_null () { break; }
      itree_inherit_offset ((*tree).otick, node);
      if pos > (*node).limit
	{ continue; }
      if !(*node).right.is_null ()
	{
	  if (*node).begin > pos + length
	    {
	      /* Shift right subtree to the left. */
	      (*(*node).right).offset -= length;
	      (*tree).otick += 1;
	    }
	  else
	    { itree_stack_push (stack, (*node).right); }
	}
      if !(*node).left.is_null ()
	{ itree_stack_push (stack, (*node).left); }

      if pos < (*node).begin
	{ (*node).begin = max (pos, (*node).begin - length); }
      if (*node).end > pos
	{
	  (*node).end = max (pos , (*node).end - length);
	  eassert! (!node.is_null ());
	  itree_propagate_limit (node);
	}
    }
  itree_stack_destroy (stack);
}

/* +=======================================================================+
 * | Iterator
 * +=======================================================================+ */

/* Return true, if NODE's interval intersects with [BEGIN, END).
   Note: We always include empty nodes at BEGIN (and not at END),
   but if BEGIN==END, then we don't include non-empty nodes starting
   at BEGIN or ending at END.  This seems to match the behavior of the
   old overlays code but it's not clear if it's The Right Thing
   (e.g. it breaks the expectation that if NODE1 is included, then
   a NODE2 strictly bigger than NODE1 should also be included).  */

unsafe fn itree_node_intersects (node: *const itree_node,
				 begin: ptrdiff_t, end: ptrdiff_t) -> bool
{
  return (begin < (*node).end && (*node).begin < end)
    || ((*node).begin == (*node).end && begin == (*node).begin);
}

/* Return the "next" node in the current traversal order.

   Note that this should return all the nodes that we need to traverse
   in order to traverse the nodes selected by the current narrowing (i.e.
   'ITER->begin..ITER->end') so it will also return some nodes which aren't in
   that narrowing simply because they may have children which are.

   The code itself is very unsatisfactory because the code of each one
   of the supported traversals seems completely different from the others.
   If someone knows how to make it more uniform and "obviously correct",
   please make yourself heard.  */

unsafe fn itree_iter_next_in_subtree (node: *mut itree_node,
				      iter: *mut itree_iterator) -> *mut itree_node
{
  /* FIXME: Like in the previous version of the iterator, we
     prune based on 'limit' only when moving to a left child,
     but 'limit' can also get smaller when moving to a right child
     It's actually fairly common, so maybe it would be worthwhile
     to prune a bit more aggressively here.  */
  let mut node: *mut itree_node = node;
  let mut next: *mut itree_node;
  match (*iter).order
    {
    ITREE_ASCENDING =>
      {
	next = (*node).right;
	if next.is_null ()
	  {
	    loop
	      {
		next = (*node).parent;
		if next.is_null () || (*next).right != node
		  { break; }
		node = next;
	      }
	    if next.is_null ()
	      { return std::ptr::null_mut (); }   /* No more nodes to visit. */
	    node = next;
	  }
	else
	  {
	    node = next;
	    itree_inherit_offset ((*iter).otick, node);
	    loop
	      {
		next = (*node).left;
		if next.is_null ()
		  { break; }
		itree_inherit_offset ((*iter).otick, next);
		if !((*iter).begin <= (*next).limit)
		  { break; }
		node = next;
	      }
	  }
	if (*node).begin > (*iter).end
	  { return std::ptr::null_mut (); }  /* No more nodes within begin..end.  */
	return node;
      }

    ITREE_DESCENDING =>
      {
	next = (*node).left;
	if next.is_null ()
	    || { itree_inherit_offset ((*iter).otick, next);
		 (*next).limit < (*iter).begin }
	  {
	    loop
	      {
		next = (*node).parent;
		if next.is_null () || (*next).left != node
		  { break; }
		node = next;
	      }
	    if next.is_null ()
	      { return std::ptr::null_mut (); }   /* No more nodes to visit. */
	    node = next;
	  }
	else
	  {
	    node = next;
	    loop
	      {
		if !((*node).begin <= (*iter).end)
		  { break; }
		next = (*node).right;
		if next.is_null ()
		  { break; }
		itree_inherit_offset ((*iter).otick, next);
		node = next;
	      }
	  }
	return node;
      }

    ITREE_PRE_ORDER =>
      {
	next = (*node).left;
	if !next.is_null ()
	  {
	    itree_inherit_offset ((*iter).otick, next);
	    if !((*next).limit < (*iter).begin)
	      { return next; }
	  }
	next = (*node).right;
	if (*node).begin <= (*iter).end && !next.is_null ()
	  {
	    itree_inherit_offset ((*iter).otick, next);
	    return next;
	  }
	loop
	  {
	    next = (*node).parent;
	    if next.is_null ()
	      { break; }
	    if (*next).right == node
	      { node = next; }
	    else
	      {
		eassert! ((*next).left == node);
		node = next;
		next = (*node).right;
		if (*node).begin <= (*iter).end && !next.is_null ()
		  {
		    itree_inherit_offset ((*iter).otick, next);
		    return next;
		  }
	      }
	  }
	return std::ptr::null_mut ();
      }

    ITREE_POST_ORDER =>
      {
	next = (*node).parent;
	if next.is_null () || (*next).right == node
	  { return next; }
	eassert! ((*next).left == node);
	node = next;
	next = (*node).right;
	if !((*node).begin <= (*iter).end && !next.is_null ())
	  { return node; }
	node = next;
	itree_inherit_offset ((*iter).otick, node);
	loop
	  {
	    let mut again: bool = false;
	    next = (*node).left;
	    if !next.is_null ()
	      {
		itree_inherit_offset ((*iter).otick, next);
		again = (*iter).begin <= (*next).limit;
	      }
	    if !again
	      {
		if (*node).begin <= (*iter).end
		  {
		    next = (*node).right;
		    if !next.is_null ()
		      {
			itree_inherit_offset ((*iter).otick, next);
			again = true;
		      }
		  }
	      }
	    if !again
	      { break; }
	    node = next;
	  }
	return node;
      }

    _ =>
    { emacs_abort (); }
    }
}

unsafe fn itree_iterator_first_node (tree: *mut itree_tree,
				     iter: *mut itree_iterator) -> *mut itree_node
{
  let mut node: *mut itree_node = (*tree).root;
  if !node.is_null ()
    {
      let mut dummy: itree_node = std::mem::zeroed ();
      dummy.left = std::ptr::null_mut ();
      dummy.parent = std::ptr::null_mut ();
      dummy.right = std::ptr::null_mut ();
      itree_inherit_offset ((*iter).otick, node);
      match (*iter).order
	{
	ITREE_ASCENDING =>
	  {
	    dummy.right = node;
	    dummy.begin = PTRDIFF_MIN;
	    node = itree_iter_next_in_subtree (&raw mut dummy, iter);
	  }

	ITREE_DESCENDING =>
	  {
	    dummy.left = node;
	    node = itree_iter_next_in_subtree (&raw mut dummy, iter);
	  }

	ITREE_PRE_ORDER =>
	  { }

	ITREE_POST_ORDER =>
	  {
	    dummy.parent = &raw mut dummy;
	    dummy.left = &raw mut dummy;
	    dummy.right = node;
	    dummy.begin = PTRDIFF_MIN;
	    node = itree_iter_next_in_subtree (&raw mut dummy, iter);
	  }
	_ =>
	  { emacs_abort (); }
	}
    }
  return node;
}

/* Start an iterator enumerating all intervals in [BEGIN,END) in the
   given ORDER.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_iterator_start (iter: *mut itree_iterator,
					       tree: *mut itree_tree,
					       begin: ptrdiff_t, end: ptrdiff_t,
					       order: itree_order) -> *mut itree_iterator
{
  eassert! (!iter.is_null ());
  (*iter).begin = begin;
  (*iter).end = end;
  (*iter).otick = (*tree).otick;
  (*iter).order = order;
  /* Beware: the 'node' field always holds "the next" node to consider.
     so it's always "one node ahead" of what the iterator loop sees.
     In most respects this makes no difference, but we depend on this
     detail in 'delete_all_overlays' where this allows us to modify
     the current node knowing that the iterator will not need it to
     find the next.  */
  (*iter).node = itree_iterator_first_node (tree, iter);
  return iter;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_iterator_next (iter: *mut itree_iterator) -> *mut itree_node
{
  let mut node: *mut itree_node = (*iter).node;
  while !node.is_null ()
      && !itree_node_intersects (node, (*iter).begin, (*iter).end)
    {
      node = itree_iter_next_in_subtree (node, iter);
      eassert! (itree_limit_is_stable (node));
    }
  (*iter).node = if !node.is_null ()
    { itree_iter_next_in_subtree (node, iter) } else { std::ptr::null_mut () };
  return node;
}

/* Limit G to the new interval [BEGIN, END), which must be a subset of
   the current one.  I.E. it can't grow on either side. */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_iterator_narrow (g: *mut itree_iterator,
						begin: ptrdiff_t, end: ptrdiff_t)
{
  eassert! (!g.is_null ());
  eassert! (begin >= (*g).begin);
  eassert! (end <= (*g).end);
  (*g).begin = max (begin, (*g).begin);
  (*g).end = min (end, (*g).end);
}
