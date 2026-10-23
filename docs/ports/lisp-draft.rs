// DRAFT mirror lines for src/lisp.h, emitted by tools/emit-mirror.py.
// They are NOT part of src/include/lisp.rs: the layout gate rejected the
// block because several of these types map incompletely (for example `handler`
// has fields this tool cannot express yet), so every type here must be completed
// and layout-verified before it is merged into the real mirror.

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Symbol_With_Pos {
    pub header: vectorlike_header,
    pub sym: Lisp_Object,
    pub pos: Lisp_Object,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Bool_Vector {
    pub header: vectorlike_header,
    pub size: EMACS_INT,
    pub data: u8,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Char_Table {
    pub header: vectorlike_header,
    pub defalt: Lisp_Object,
    pub parent: Lisp_Object,
    pub purpose: Lisp_Object,
    pub ascii: Lisp_Object,
    pub contents: Lisp_Object,
    pub extras: Lisp_Object,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Sub_Char_Table {
    pub header: vectorlike_header,
    pub depth: c_int,
    pub min_char: c_int,
    pub contents: Lisp_Object,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Obarray {
    pub header: vectorlike_header,
    pub buckets: *mut Lisp_Object,
    pub size_bits: c_uint,
    pub count: c_uint,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Overlay {
    pub header: vectorlike_header,
    pub plist: Lisp_Object,
    pub buffer: *mut buffer,
    pub interval: *mut itree_node,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Misc_Ptr {
    pub header: vectorlike_header,
    pub pointer: *mut c_void,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Lisp_Finalizer {
    pub header: vectorlike_header,
    pub function: Lisp_Object,
    pub prev: *mut Lisp_Finalizer,
    pub next: *mut Lisp_Finalizer,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct handler {
    pub type: handlertype,
    pub tag_or_ch: Lisp_Object,
    pub nonlocal_exit: nonlocal_exit,
    pub val: Lisp_Object,
    pub next: *mut handler,
    pub nextfree: *mut handler,
    pub bytecode_top: *mut Lisp_Object,
    pub bytecode_dest: c_int,
    pub jmp: jmp_buf,
    pub f_lisp_eval_depth: EMACS_INT,
    pub pdlcount: isize,
    pub act_rec: *mut bc_frame,
    pub poll_suppress_count: c_int,
    pub interrupt_input_blocked: c_int,
    pub x_error_handler_depth: c_int,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct for_each_tail_internal {
    pub tortoise: Lisp_Object,
    pub l: isize,
}

pub const BOOL_VECTOR_BITS_PER_CHAR: isize = 8;
pub const IDEAL_GCALIGNMENT: isize = 8;
pub const GCALIGNMENT: isize = 1;
pub const WRAP_SPECPDL_REF: isize = 1;
pub const FLOAT_TO_STRING_BUFSIZE: isize = 350;
pub const FOR_EACH_TAIL_THRESHOLD: isize = 4096;
pub const Lisp_Type_Unused0: isize = 1;
pub const Lisp_Int0: isize = 2;
pub const Lisp_Vectorlike: isize = 5;

pub type INTERVAL = *mut interval;
pub type KBOARD = kboard;
pub type Lisp_Word_tag = EMACS_UINT;
pub type bits_word = u8;
pub type hash_hash_t = c_uint;
pub type hash_idx_t = int_least32_t;
pub type lispfwd = *mut Lisp_Fwd;
pub type modiff_count = i64;
pub type specpdl_ref = isize;
pub type sys_jmp_buf = jmp_buf;
