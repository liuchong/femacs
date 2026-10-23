// femacs: port of lib/filename.h @ fd7afced403.
//
// The header is two arms: the Windows/Cygwin/OS2/DOS spelling (C 55-91) and the Unix one
// (C 93-104).  The frozen target is darwin-arm64, so the Unix arm is the live text; every
// macro of the other arm except `_IS_DRIVE_LETTER` is defined by both, and the gate does
// not compare a macro that has several definitions (P-24), so only `_IS_DRIVE_LETTER`
// needs its Windows spelling kept - the port keeps it as the no-drive-letters test it is,
// since the frozen target's `HAS_DEVICE` never calls it.
//
// C 106-108's two deprecated aliases are macros over the function-like ones.

/// C 55-91's `# define _IS_DRIVE_LETTER(C) (((C) >= 'A' && (C) <= 'Z') || …)` (the
/// Windows arm's helper; nothing on the frozen target calls it).
macro_rules! _IS_DRIVE_LETTER {
    ($c:expr) => {
        ((($c) >= 'A' as c_char && ($c) <= 'Z' as c_char)
         || (($c) >= 'a' as c_char && ($c) <= 'z' as c_char))
    };
}

/* # define ISSLASH(C) ((C) == '/')  */
macro_rules! ISSLASH {
    ($c:expr) => { (($c) == '/' as c_char) };
}
/* # define HAS_DEVICE(Filename) ((void) (Filename), 0)  */
macro_rules! HAS_DEVICE {
    ($filename:expr) => { { let _ = $filename; 0 } };
}
/* # define FILE_SYSTEM_PREFIX_LEN(Filename) ((void) (Filename), 0)  */
macro_rules! FILE_SYSTEM_PREFIX_LEN {
    ($filename:expr) => { { let _ = $filename; 0 } };
}
/* # define FILE_SYSTEM_DRIVE_PREFIX_CAN_BE_RELATIVE 0  */
// The live arm's value on this target (the Windows arms that set it to 1 are dead).
pub const FILE_SYSTEM_DRIVE_PREFIX_CAN_BE_RELATIVE: c_int = 0;
macro_rules! FILE_SYSTEM_DRIVE_PREFIX_CAN_BE_RELATIVE { () => { 0 }; }
/* # define IS_ABSOLUTE_FILE_NAME(Filename) ISSLASH ((Filename)[0])  */
macro_rules! IS_ABSOLUTE_FILE_NAME {
    ($filename:expr) => { ISSLASH! (*($filename).offset (0)) };
}
/* # define IS_RELATIVE_FILE_NAME(Filename) (! ISSLASH ((Filename)[0]))  */
macro_rules! IS_RELATIVE_FILE_NAME {
    ($filename:expr) => { ! ISSLASH! (*($filename).offset (0)) };
}
/* # define IS_FILE_NAME_WITH_DIR(Filename) (strchr ((Filename), '/') != NULL)  */
macro_rules! IS_FILE_NAME_WITH_DIR {
    ($filename:expr) => {
        (strchr ($filename, '/' as c_int) != std::ptr::null_mut ())
    };
}

/* Deprecated macros.  For backward compatibility with old users of the
   'filename' module.  */
/* #define IS_ABSOLUTE_PATH IS_ABSOLUTE_FILE_NAME  */
macro_rules! IS_ABSOLUTE_PATH {
    ($filename:expr) => { IS_ABSOLUTE_FILE_NAME! ($filename) };
}
/* #define IS_PATH_WITH_DIR IS_FILE_NAME_WITH_DIR  */
macro_rules! IS_PATH_WITH_DIR {
    ($filename:expr) => { IS_FILE_NAME_WITH_DIR! ($filename) };
}

/* The include guard has no Rust counterpart (PORTING.md 0.1).  */
pub const _FILENAME_H: bool = true;
