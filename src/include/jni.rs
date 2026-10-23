// femacs: mirror of the Android NDK's <jni.h> (system header of another target's toolchain, PORTING.md P-26/P-49).

// `<jni.h>` is the NDK's own header - it is not part of the reference tree, and the frozen
// target's toolchain has no such header - so it has no C file to be paired with and this mirror
// is the port's declaration of the ABI the Android backend is written against (the `stdio.rs`/
// `signal.rs` class of system-header mirror; `check-target-names.py` accepts the first line
// above as the declaration that this file is not a port of a reference file).
//
// Measured with the NDK's own clang against the installed header
// (`~/Library/Android/sdk/ndk/29.0.14206865/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/include/jni.h`,
// compiled `--target=aarch64-linux-android24`): every `offset_of!`/`size_of` assertion below was
// emitted from a C probe that `_Static_assert`s the same numbers, so a wrong field order or a
// missing entry fails to compile on the *target's* ABI, not merely in this port.  The header's
// entries are append-only by the JNI specification, so the offsets hold for other NDK releases.
//
// This is the header's **C branch** (`__cplusplus` undefined): `jobject` is `void *` and
// `JNIEnv`/`JavaVM` are pointers to the two interface tables.  The C++ branch (`_jobject`,
// `_JNIEnv`, `_JavaVM` and the inline C++ wrappers), the invocation API's records
// (`JavaVMOption`, `JavaVMInitArgs`, `JavaVMAttachArgs`) and the three exported functions
// (`JNI_GetDefaultJavaVMInitArgs`, `JNI_CreateJavaVM`, `JNI_GetCreatedJavaVMs`) are **not**
// carried: no file of the port names them yet (measured over `src/android*.c`,
// `src/android*.h` and `src/sfntfont-android.c`), and the ledger says so.

/* jni.h 30-40: the scalar typedefs (`jboolean` is `uint8_t`, `jsize` is `jint`).  */
pub type jboolean = u8;
pub type jbyte = i8;
pub type jchar = u16;
pub type jshort = i16;
pub type jint = i32;
pub type jlong = i64;
pub type jfloat = f32;
pub type jdouble = f64;
pub type jsize = jint;

const _: () = core::assert! (core::mem::size_of::<jboolean> () == 1);
const _: () = core::assert! (core::mem::size_of::<jbyte> () == 1);
const _: () = core::assert! (core::mem::size_of::<jchar> () == 2);
const _: () = core::assert! (core::mem::size_of::<jshort> () == 2);
const _: () = core::assert! (core::mem::size_of::<jint> () == 4);
const _: () = core::assert! (core::mem::size_of::<jlong> () == 8);
const _: () = core::assert! (core::mem::size_of::<jfloat> () == 4);
const _: () = core::assert! (core::mem::size_of::<jdouble> () == 8);
const _: () = core::assert! (core::mem::size_of::<jsize> () == 4);

/* jni.h 61-99, the C branch: `typedef void *jobject;` and the five names that alias it.  */
pub struct _jfieldID;
pub struct _jmethodID;

pub type jobject = *mut c_void;
pub type jclass = jobject;
pub type jstring = jobject;
pub type jarray = jobject;
pub type jobjectArray = jarray;
pub type jbooleanArray = jarray;
pub type jbyteArray = jarray;
pub type jcharArray = jarray;
pub type jshortArray = jarray;
pub type jintArray = jarray;
pub type jlongArray = jarray;
pub type jfloatArray = jarray;
pub type jdoubleArray = jarray;
pub type jthrowable = jobject;
pub type jweak = jobject;
pub type jfieldID = *mut _jfieldID;
pub type jmethodID = *mut _jmethodID;

const _: () = core::assert! (core::mem::size_of::<jobject> () == 8);
const _: () = core::assert! (core::mem::size_of::<jmethodID> () == 8);
const _: () = core::assert! (core::mem::size_of::<jfieldID> () == 8);

/* jni.h 109-119: `union jvalue`, the argument vector the `…V`/`…A` entry points take.  Its
   members all live at offset 0 and the widest (`jlong`, `jdouble`, `jobject`) is 8 bytes.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub union jvalue
{
  pub z: jboolean,
  pub b: jbyte,
  pub c: jchar,
  pub s: jshort,
  pub i: jint,
  pub j: jlong,
  pub f: jfloat,
  pub d: jdouble,
  pub l: jobject,
}

const _: () = core::assert! (core::mem::size_of::<jvalue> () == 8);
const _: () = core::assert! (core::mem::align_of::<jvalue> () == 8);

/* jni.h 121-126: `enum jobjectRefType` (C's enum ABI is `int`).  */
pub type jobjectRefType = c_int;
pub const JNIInvalidRefType: jobjectRefType = 0;
pub const JNILocalRefType: jobjectRefType = 1;
pub const JNIGlobalRefType: jobjectRefType = 2;
pub const JNIWeakGlobalRefType: jobjectRefType = 3;

/* jni.h 128-132: `struct JNINativeMethod`, the descriptor array `RegisterNatives` takes.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JNINativeMethod
{
  pub name: *const c_char,
  pub signature: *const c_char,
  pub fnPtr: *mut c_void,
}

const _: () = core::assert! (core::mem::size_of::<JNINativeMethod> () == 24);
const _: () = core::assert! (core::mem::offset_of! (JNINativeMethod, name) == 0);
const _: () = core::assert! (core::mem::offset_of! (JNINativeMethod, signature) == 8);
const _: () = core::assert! (core::mem::offset_of! (JNINativeMethod, fnPtr) == 16);

/* jni.h 134-143: the two interface tables' typedefs.  `JNIInvokeInterface` stays opaque: no
   file of the port reads a `JavaVM` entry yet, and `JavaVM` is only ever a pointer.  */
pub struct JNIInvokeInterface;

pub type C_JNIEnv = *const JNINativeInterface;
pub type JNIEnv = *const JNINativeInterface;
pub type JavaVM = *const JNIInvokeInterface;

const _: () = core::assert! (core::mem::size_of::<JNIEnv> () == 8);
const _: () = core::assert! (core::mem::size_of::<JavaVM> () == 8);

/* jni.h 1123-1140: the constants the port names (`JNI_COMMIT`/`JNI_ABORT` are the `mode`
   arguments of `Release…ArrayElements`); the version and error codes are not carried yet.  */
pub const JNI_COMMIT: c_int = 1;   /* copy content, do not free buffer */
pub const JNI_ABORT: c_int = 2;    /* free buffer w/o copying back */

/* jni.h 149-481: the 233 entries of `JNINativeInterface`, in the header's order.  The four
   `reserved` slots are C's `void *`; every other field is a function pointer, so the table is
   233 eight-byte slots (measured: `sizeof` 1864) and each entry's offset is its index times 8.
   The five `…V` entries take a `va_list` and the five unsuffixed varargs entries are C-variadic
   (PORTING.md 2.4: the port's toolchain pin compiles `c_variadic`).  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JNINativeInterface
{
  pub reserved0: *mut c_void,
  pub reserved1: *mut c_void,
  pub reserved2: *mut c_void,
  pub reserved3: *mut c_void,
  pub GetVersion: Option<unsafe extern "C" fn (*mut JNIEnv) -> jint>,
  pub DefineClass: Option<unsafe extern "C" fn (*mut JNIEnv, *const c_char, jobject, *const jbyte, jsize) -> jclass>,
  pub FindClass: Option<unsafe extern "C" fn (*mut JNIEnv, *const c_char) -> jclass>,
  pub FromReflectedMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject) -> jmethodID>,
  pub FromReflectedField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject) -> jfieldID>,
  pub ToReflectedMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, jboolean) -> jobject>,
  pub GetSuperclass: Option<unsafe extern "C" fn (*mut JNIEnv, jclass) -> jclass>,
  pub IsAssignableFrom: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jclass) -> jboolean>,
  pub ToReflectedField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID, jboolean) -> jobject>,
  pub Throw: Option<unsafe extern "C" fn (*mut JNIEnv, jthrowable) -> jint>,
  pub ThrowNew: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, *const c_char) -> jint>,
  pub ExceptionOccurred: Option<unsafe extern "C" fn (*mut JNIEnv) -> jthrowable>,
  pub ExceptionDescribe: Option<unsafe extern "C" fn (*mut JNIEnv)>,
  pub ExceptionClear: Option<unsafe extern "C" fn (*mut JNIEnv)>,
  pub FatalError: Option<unsafe extern "C" fn (*mut JNIEnv, *const c_char)>,
  pub PushLocalFrame: Option<unsafe extern "C" fn (*mut JNIEnv, jint) -> jint>,
  pub PopLocalFrame: Option<unsafe extern "C" fn (*mut JNIEnv, jobject) -> jobject>,
  pub NewGlobalRef: Option<unsafe extern "C" fn (*mut JNIEnv, jobject) -> jobject>,
  pub DeleteGlobalRef: Option<unsafe extern "C" fn (*mut JNIEnv, jobject)>,
  pub DeleteLocalRef: Option<unsafe extern "C" fn (*mut JNIEnv, jobject)>,
  pub IsSameObject: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jobject) -> jboolean>,
  pub NewLocalRef: Option<unsafe extern "C" fn (*mut JNIEnv, jobject) -> jobject>,
  pub EnsureLocalCapacity: Option<unsafe extern "C" fn (*mut JNIEnv, jint) -> jint>,
  pub AllocObject: Option<unsafe extern "C" fn (*mut JNIEnv, jclass) -> jobject>,
  pub NewObject: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, ...) -> jobject>,
  pub NewObjectV: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, core::ffi::VaList<'static>) -> jobject>,
  pub NewObjectA: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jobject>,
  pub GetObjectClass: Option<unsafe extern "C" fn (*mut JNIEnv, jobject) -> jclass>,
  pub IsInstanceOf: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass) -> jboolean>,
  pub GetMethodID: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, *const c_char, *const c_char) -> jmethodID>,
  pub CallObjectMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, ...) -> jobject>,
  pub CallObjectMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, core::ffi::VaList<'static>) -> jobject>,
  pub CallObjectMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jobject>,
  pub CallBooleanMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, ...) -> jboolean>,
  pub CallBooleanMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, core::ffi::VaList<'static>) -> jboolean>,
  pub CallBooleanMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jboolean>,
  pub CallByteMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, ...) -> jbyte>,
  pub CallByteMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, core::ffi::VaList<'static>) -> jbyte>,
  pub CallByteMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jbyte>,
  pub CallCharMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, ...) -> jchar>,
  pub CallCharMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, core::ffi::VaList<'static>) -> jchar>,
  pub CallCharMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jchar>,
  pub CallShortMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, ...) -> jshort>,
  pub CallShortMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, core::ffi::VaList<'static>) -> jshort>,
  pub CallShortMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jshort>,
  pub CallIntMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, ...) -> jint>,
  pub CallIntMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, core::ffi::VaList<'static>) -> jint>,
  pub CallIntMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jint>,
  pub CallLongMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, ...) -> jlong>,
  pub CallLongMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, core::ffi::VaList<'static>) -> jlong>,
  pub CallLongMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jlong>,
  pub CallFloatMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, ...) -> jfloat>,
  pub CallFloatMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, core::ffi::VaList<'static>) -> jfloat>,
  pub CallFloatMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jfloat>,
  pub CallDoubleMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, ...) -> jdouble>,
  pub CallDoubleMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, core::ffi::VaList<'static>) -> jdouble>,
  pub CallDoubleMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jdouble>,
  pub CallVoidMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, ...)>,
  pub CallVoidMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, core::ffi::VaList<'static>)>,
  pub CallVoidMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jmethodID, *const jvalue)>,
  pub CallNonvirtualObjectMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jobject>,
  pub CallNonvirtualObjectMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, core::ffi::VaList<'static>) -> jobject>,
  pub CallNonvirtualObjectMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jobject>,
  pub CallNonvirtualBooleanMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jboolean>,
  pub CallNonvirtualBooleanMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, core::ffi::VaList<'static>) -> jboolean>,
  pub CallNonvirtualBooleanMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jboolean>,
  pub CallNonvirtualByteMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jbyte>,
  pub CallNonvirtualByteMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, core::ffi::VaList<'static>) -> jbyte>,
  pub CallNonvirtualByteMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jbyte>,
  pub CallNonvirtualCharMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jchar>,
  pub CallNonvirtualCharMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, core::ffi::VaList<'static>) -> jchar>,
  pub CallNonvirtualCharMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jchar>,
  pub CallNonvirtualShortMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jshort>,
  pub CallNonvirtualShortMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, core::ffi::VaList<'static>) -> jshort>,
  pub CallNonvirtualShortMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jshort>,
  pub CallNonvirtualIntMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jint>,
  pub CallNonvirtualIntMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, core::ffi::VaList<'static>) -> jint>,
  pub CallNonvirtualIntMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jint>,
  pub CallNonvirtualLongMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jlong>,
  pub CallNonvirtualLongMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, core::ffi::VaList<'static>) -> jlong>,
  pub CallNonvirtualLongMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jlong>,
  pub CallNonvirtualFloatMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jfloat>,
  pub CallNonvirtualFloatMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, core::ffi::VaList<'static>) -> jfloat>,
  pub CallNonvirtualFloatMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jfloat>,
  pub CallNonvirtualDoubleMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jdouble>,
  pub CallNonvirtualDoubleMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, core::ffi::VaList<'static>) -> jdouble>,
  pub CallNonvirtualDoubleMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jdouble>,
  pub CallNonvirtualVoidMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, ...)>,
  pub CallNonvirtualVoidMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, core::ffi::VaList<'static>)>,
  pub CallNonvirtualVoidMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue)>,
  pub GetFieldID: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, *const c_char, *const c_char) -> jfieldID>,
  pub GetObjectField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID) -> jobject>,
  pub GetBooleanField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID) -> jboolean>,
  pub GetByteField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID) -> jbyte>,
  pub GetCharField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID) -> jchar>,
  pub GetShortField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID) -> jshort>,
  pub GetIntField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID) -> jint>,
  pub GetLongField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID) -> jlong>,
  pub GetFloatField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID) -> jfloat>,
  pub GetDoubleField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID) -> jdouble>,
  pub SetObjectField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID, jobject)>,
  pub SetBooleanField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID, jboolean)>,
  pub SetByteField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID, jbyte)>,
  pub SetCharField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID, jchar)>,
  pub SetShortField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID, jshort)>,
  pub SetIntField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID, jint)>,
  pub SetLongField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID, jlong)>,
  pub SetFloatField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID, jfloat)>,
  pub SetDoubleField: Option<unsafe extern "C" fn (*mut JNIEnv, jobject, jfieldID, jdouble)>,
  pub GetStaticMethodID: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, *const c_char, *const c_char) -> jmethodID>,
  pub CallStaticObjectMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, ...) -> jobject>,
  pub CallStaticObjectMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, core::ffi::VaList<'static>) -> jobject>,
  pub CallStaticObjectMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jobject>,
  pub CallStaticBooleanMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, ...) -> jboolean>,
  pub CallStaticBooleanMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, core::ffi::VaList<'static>) -> jboolean>,
  pub CallStaticBooleanMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jboolean>,
  pub CallStaticByteMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, ...) -> jbyte>,
  pub CallStaticByteMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, core::ffi::VaList<'static>) -> jbyte>,
  pub CallStaticByteMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jbyte>,
  pub CallStaticCharMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, ...) -> jchar>,
  pub CallStaticCharMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, core::ffi::VaList<'static>) -> jchar>,
  pub CallStaticCharMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jchar>,
  pub CallStaticShortMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, ...) -> jshort>,
  pub CallStaticShortMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, core::ffi::VaList<'static>) -> jshort>,
  pub CallStaticShortMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jshort>,
  pub CallStaticIntMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, ...) -> jint>,
  pub CallStaticIntMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, core::ffi::VaList<'static>) -> jint>,
  pub CallStaticIntMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jint>,
  pub CallStaticLongMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, ...) -> jlong>,
  pub CallStaticLongMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, core::ffi::VaList<'static>) -> jlong>,
  pub CallStaticLongMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jlong>,
  pub CallStaticFloatMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, ...) -> jfloat>,
  pub CallStaticFloatMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, core::ffi::VaList<'static>) -> jfloat>,
  pub CallStaticFloatMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jfloat>,
  pub CallStaticDoubleMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, ...) -> jdouble>,
  pub CallStaticDoubleMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, core::ffi::VaList<'static>) -> jdouble>,
  pub CallStaticDoubleMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jdouble>,
  pub CallStaticVoidMethod: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, ...)>,
  pub CallStaticVoidMethodV: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, core::ffi::VaList<'static>)>,
  pub CallStaticVoidMethodA: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jmethodID, *const jvalue)>,
  pub GetStaticFieldID: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, *const c_char, *const c_char) -> jfieldID>,
  pub GetStaticObjectField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID) -> jobject>,
  pub GetStaticBooleanField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID) -> jboolean>,
  pub GetStaticByteField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID) -> jbyte>,
  pub GetStaticCharField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID) -> jchar>,
  pub GetStaticShortField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID) -> jshort>,
  pub GetStaticIntField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID) -> jint>,
  pub GetStaticLongField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID) -> jlong>,
  pub GetStaticFloatField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID) -> jfloat>,
  pub GetStaticDoubleField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID) -> jdouble>,
  pub SetStaticObjectField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID, jobject)>,
  pub SetStaticBooleanField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID, jboolean)>,
  pub SetStaticByteField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID, jbyte)>,
  pub SetStaticCharField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID, jchar)>,
  pub SetStaticShortField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID, jshort)>,
  pub SetStaticIntField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID, jint)>,
  pub SetStaticLongField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID, jlong)>,
  pub SetStaticFloatField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID, jfloat)>,
  pub SetStaticDoubleField: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, jfieldID, jdouble)>,
  pub NewString: Option<unsafe extern "C" fn (*mut JNIEnv, *const jchar, jsize) -> jstring>,
  pub GetStringLength: Option<unsafe extern "C" fn (*mut JNIEnv, jstring) -> jsize>,
  pub GetStringChars: Option<unsafe extern "C" fn (*mut JNIEnv, jstring, *mut jboolean) -> *const jchar>,
  pub ReleaseStringChars: Option<unsafe extern "C" fn (*mut JNIEnv, jstring, *const jchar)>,
  pub NewStringUTF: Option<unsafe extern "C" fn (*mut JNIEnv, *const c_char) -> jstring>,
  pub GetStringUTFLength: Option<unsafe extern "C" fn (*mut JNIEnv, jstring) -> jsize>,
  pub GetStringUTFChars: Option<unsafe extern "C" fn (*mut JNIEnv, jstring, *mut jboolean) -> *const c_char>,
  pub ReleaseStringUTFChars: Option<unsafe extern "C" fn (*mut JNIEnv, jstring, *const c_char)>,
  pub GetArrayLength: Option<unsafe extern "C" fn (*mut JNIEnv, jarray) -> jsize>,
  pub NewObjectArray: Option<unsafe extern "C" fn (*mut JNIEnv, jsize, jclass, jobject) -> jobjectArray>,
  pub GetObjectArrayElement: Option<unsafe extern "C" fn (*mut JNIEnv, jobjectArray, jsize) -> jobject>,
  pub SetObjectArrayElement: Option<unsafe extern "C" fn (*mut JNIEnv, jobjectArray, jsize, jobject)>,
  pub NewBooleanArray: Option<unsafe extern "C" fn (*mut JNIEnv, jsize) -> jbooleanArray>,
  pub NewByteArray: Option<unsafe extern "C" fn (*mut JNIEnv, jsize) -> jbyteArray>,
  pub NewCharArray: Option<unsafe extern "C" fn (*mut JNIEnv, jsize) -> jcharArray>,
  pub NewShortArray: Option<unsafe extern "C" fn (*mut JNIEnv, jsize) -> jshortArray>,
  pub NewIntArray: Option<unsafe extern "C" fn (*mut JNIEnv, jsize) -> jintArray>,
  pub NewLongArray: Option<unsafe extern "C" fn (*mut JNIEnv, jsize) -> jlongArray>,
  pub NewFloatArray: Option<unsafe extern "C" fn (*mut JNIEnv, jsize) -> jfloatArray>,
  pub NewDoubleArray: Option<unsafe extern "C" fn (*mut JNIEnv, jsize) -> jdoubleArray>,
  pub GetBooleanArrayElements: Option<unsafe extern "C" fn (*mut JNIEnv, jbooleanArray, *mut jboolean) -> *mut jboolean>,
  pub GetByteArrayElements: Option<unsafe extern "C" fn (*mut JNIEnv, jbyteArray, *mut jboolean) -> *mut jbyte>,
  pub GetCharArrayElements: Option<unsafe extern "C" fn (*mut JNIEnv, jcharArray, *mut jboolean) -> *mut jchar>,
  pub GetShortArrayElements: Option<unsafe extern "C" fn (*mut JNIEnv, jshortArray, *mut jboolean) -> *mut jshort>,
  pub GetIntArrayElements: Option<unsafe extern "C" fn (*mut JNIEnv, jintArray, *mut jboolean) -> *mut jint>,
  pub GetLongArrayElements: Option<unsafe extern "C" fn (*mut JNIEnv, jlongArray, *mut jboolean) -> *mut jlong>,
  pub GetFloatArrayElements: Option<unsafe extern "C" fn (*mut JNIEnv, jfloatArray, *mut jboolean) -> *mut jfloat>,
  pub GetDoubleArrayElements: Option<unsafe extern "C" fn (*mut JNIEnv, jdoubleArray, *mut jboolean) -> *mut jdouble>,
  pub ReleaseBooleanArrayElements: Option<unsafe extern "C" fn (*mut JNIEnv, jbooleanArray, *mut jboolean, jint)>,
  pub ReleaseByteArrayElements: Option<unsafe extern "C" fn (*mut JNIEnv, jbyteArray, *mut jbyte, jint)>,
  pub ReleaseCharArrayElements: Option<unsafe extern "C" fn (*mut JNIEnv, jcharArray, *mut jchar, jint)>,
  pub ReleaseShortArrayElements: Option<unsafe extern "C" fn (*mut JNIEnv, jshortArray, *mut jshort, jint)>,
  pub ReleaseIntArrayElements: Option<unsafe extern "C" fn (*mut JNIEnv, jintArray, *mut jint, jint)>,
  pub ReleaseLongArrayElements: Option<unsafe extern "C" fn (*mut JNIEnv, jlongArray, *mut jlong, jint)>,
  pub ReleaseFloatArrayElements: Option<unsafe extern "C" fn (*mut JNIEnv, jfloatArray, *mut jfloat, jint)>,
  pub ReleaseDoubleArrayElements: Option<unsafe extern "C" fn (*mut JNIEnv, jdoubleArray, *mut jdouble, jint)>,
  pub GetBooleanArrayRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jbooleanArray, jsize, jsize, *mut jboolean)>,
  pub GetByteArrayRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jbyteArray, jsize, jsize, *mut jbyte)>,
  pub GetCharArrayRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jcharArray, jsize, jsize, *mut jchar)>,
  pub GetShortArrayRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jshortArray, jsize, jsize, *mut jshort)>,
  pub GetIntArrayRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jintArray, jsize, jsize, *mut jint)>,
  pub GetLongArrayRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jlongArray, jsize, jsize, *mut jlong)>,
  pub GetFloatArrayRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jfloatArray, jsize, jsize, *mut jfloat)>,
  pub GetDoubleArrayRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jdoubleArray, jsize, jsize, *mut jdouble)>,
  pub SetBooleanArrayRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jbooleanArray, jsize, jsize, *const jboolean)>,
  pub SetByteArrayRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jbyteArray, jsize, jsize, *const jbyte)>,
  pub SetCharArrayRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jcharArray, jsize, jsize, *const jchar)>,
  pub SetShortArrayRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jshortArray, jsize, jsize, *const jshort)>,
  pub SetIntArrayRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jintArray, jsize, jsize, *const jint)>,
  pub SetLongArrayRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jlongArray, jsize, jsize, *const jlong)>,
  pub SetFloatArrayRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jfloatArray, jsize, jsize, *const jfloat)>,
  pub SetDoubleArrayRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jdoubleArray, jsize, jsize, *const jdouble)>,
  pub RegisterNatives: Option<unsafe extern "C" fn (*mut JNIEnv, jclass, *const JNINativeMethod, jint) -> jint>,
  pub UnregisterNatives: Option<unsafe extern "C" fn (*mut JNIEnv, jclass) -> jint>,
  pub MonitorEnter: Option<unsafe extern "C" fn (*mut JNIEnv, jobject) -> jint>,
  pub MonitorExit: Option<unsafe extern "C" fn (*mut JNIEnv, jobject) -> jint>,
  pub GetJavaVM: Option<unsafe extern "C" fn (*mut JNIEnv, *mut *mut JavaVM) -> jint>,
  pub GetStringRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jstring, jsize, jsize, *mut jchar)>,
  pub GetStringUTFRegion: Option<unsafe extern "C" fn (*mut JNIEnv, jstring, jsize, jsize, *mut c_char)>,
  pub GetPrimitiveArrayCritical: Option<unsafe extern "C" fn (*mut JNIEnv, jarray, *mut jboolean) -> *mut c_void>,
  pub ReleasePrimitiveArrayCritical: Option<unsafe extern "C" fn (*mut JNIEnv, jarray, *mut c_void, jint)>,
  pub GetStringCritical: Option<unsafe extern "C" fn (*mut JNIEnv, jstring, *mut jboolean) -> *const jchar>,
  pub ReleaseStringCritical: Option<unsafe extern "C" fn (*mut JNIEnv, jstring, *const jchar)>,
  pub NewWeakGlobalRef: Option<unsafe extern "C" fn (*mut JNIEnv, jobject) -> jweak>,
  pub DeleteWeakGlobalRef: Option<unsafe extern "C" fn (*mut JNIEnv, jweak)>,
  pub ExceptionCheck: Option<unsafe extern "C" fn (*mut JNIEnv) -> jboolean>,
  pub NewDirectByteBuffer: Option<unsafe extern "C" fn (*mut JNIEnv, *mut c_void, jlong) -> jobject>,
  pub GetDirectBufferAddress: Option<unsafe extern "C" fn (*mut JNIEnv, jobject) -> *mut c_void>,
  pub GetDirectBufferCapacity: Option<unsafe extern "C" fn (*mut JNIEnv, jobject) -> jlong>,
  pub GetObjectRefType: Option<unsafe extern "C" fn (*mut JNIEnv, jobject) -> jobjectRefType>,
}

const _: () = core::assert! (core::mem::size_of::<JNINativeInterface> () == 1864);const _: () = core::assert! (core::mem::size_of::<JNINativeInterface> () == 1864);const _: () = core::assert! (core::mem::size_of::<JNINativeInterface> () == 1864);const _: () = core::assert! (core::mem::size_of::<JNINativeInterface> () == 1864);const _: () = core::assert! (core::mem::size_of::<JNINativeInterface> () == 1864);
const _: () = core::assert! (core::mem::align_of::<JNINativeInterface> () == 8);

const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, reserved0) == 0);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, reserved1) == 8);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, reserved2) == 16);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, reserved3) == 24);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetVersion) == 32);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, DefineClass) == 40);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, FindClass) == 48);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, FromReflectedMethod) == 56);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, FromReflectedField) == 64);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ToReflectedMethod) == 72);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetSuperclass) == 80);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, IsAssignableFrom) == 88);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ToReflectedField) == 96);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, Throw) == 104);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ThrowNew) == 112);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ExceptionOccurred) == 120);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ExceptionDescribe) == 128);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ExceptionClear) == 136);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, FatalError) == 144);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, PushLocalFrame) == 152);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, PopLocalFrame) == 160);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewGlobalRef) == 168);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, DeleteGlobalRef) == 176);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, DeleteLocalRef) == 184);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, IsSameObject) == 192);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewLocalRef) == 200);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, EnsureLocalCapacity) == 208);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, AllocObject) == 216);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewObject) == 224);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewObjectV) == 232);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewObjectA) == 240);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetObjectClass) == 248);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, IsInstanceOf) == 256);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetMethodID) == 264);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallObjectMethod) == 272);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallObjectMethodV) == 280);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallObjectMethodA) == 288);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallBooleanMethod) == 296);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallBooleanMethodV) == 304);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallBooleanMethodA) == 312);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallByteMethod) == 320);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallByteMethodV) == 328);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallByteMethodA) == 336);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallCharMethod) == 344);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallCharMethodV) == 352);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallCharMethodA) == 360);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallShortMethod) == 368);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallShortMethodV) == 376);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallShortMethodA) == 384);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallIntMethod) == 392);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallIntMethodV) == 400);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallIntMethodA) == 408);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallLongMethod) == 416);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallLongMethodV) == 424);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallLongMethodA) == 432);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallFloatMethod) == 440);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallFloatMethodV) == 448);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallFloatMethodA) == 456);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallDoubleMethod) == 464);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallDoubleMethodV) == 472);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallDoubleMethodA) == 480);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallVoidMethod) == 488);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallVoidMethodV) == 496);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallVoidMethodA) == 504);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualObjectMethod) == 512);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualObjectMethodV) == 520);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualObjectMethodA) == 528);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualBooleanMethod) == 536);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualBooleanMethodV) == 544);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualBooleanMethodA) == 552);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualByteMethod) == 560);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualByteMethodV) == 568);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualByteMethodA) == 576);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualCharMethod) == 584);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualCharMethodV) == 592);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualCharMethodA) == 600);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualShortMethod) == 608);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualShortMethodV) == 616);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualShortMethodA) == 624);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualIntMethod) == 632);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualIntMethodV) == 640);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualIntMethodA) == 648);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualLongMethod) == 656);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualLongMethodV) == 664);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualLongMethodA) == 672);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualFloatMethod) == 680);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualFloatMethodV) == 688);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualFloatMethodA) == 696);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualDoubleMethod) == 704);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualDoubleMethodV) == 712);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualDoubleMethodA) == 720);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualVoidMethod) == 728);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualVoidMethodV) == 736);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallNonvirtualVoidMethodA) == 744);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetFieldID) == 752);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetObjectField) == 760);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetBooleanField) == 768);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetByteField) == 776);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetCharField) == 784);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetShortField) == 792);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetIntField) == 800);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetLongField) == 808);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetFloatField) == 816);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetDoubleField) == 824);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetObjectField) == 832);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetBooleanField) == 840);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetByteField) == 848);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetCharField) == 856);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetShortField) == 864);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetIntField) == 872);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetLongField) == 880);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetFloatField) == 888);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetDoubleField) == 896);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStaticMethodID) == 904);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticObjectMethod) == 912);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticObjectMethodV) == 920);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticObjectMethodA) == 928);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticBooleanMethod) == 936);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticBooleanMethodV) == 944);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticBooleanMethodA) == 952);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticByteMethod) == 960);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticByteMethodV) == 968);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticByteMethodA) == 976);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticCharMethod) == 984);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticCharMethodV) == 992);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticCharMethodA) == 1000);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticShortMethod) == 1008);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticShortMethodV) == 1016);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticShortMethodA) == 1024);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticIntMethod) == 1032);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticIntMethodV) == 1040);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticIntMethodA) == 1048);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticLongMethod) == 1056);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticLongMethodV) == 1064);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticLongMethodA) == 1072);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticFloatMethod) == 1080);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticFloatMethodV) == 1088);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticFloatMethodA) == 1096);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticDoubleMethod) == 1104);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticDoubleMethodV) == 1112);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticDoubleMethodA) == 1120);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticVoidMethod) == 1128);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticVoidMethodV) == 1136);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, CallStaticVoidMethodA) == 1144);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStaticFieldID) == 1152);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStaticObjectField) == 1160);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStaticBooleanField) == 1168);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStaticByteField) == 1176);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStaticCharField) == 1184);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStaticShortField) == 1192);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStaticIntField) == 1200);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStaticLongField) == 1208);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStaticFloatField) == 1216);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStaticDoubleField) == 1224);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetStaticObjectField) == 1232);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetStaticBooleanField) == 1240);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetStaticByteField) == 1248);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetStaticCharField) == 1256);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetStaticShortField) == 1264);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetStaticIntField) == 1272);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetStaticLongField) == 1280);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetStaticFloatField) == 1288);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetStaticDoubleField) == 1296);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewString) == 1304);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStringLength) == 1312);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStringChars) == 1320);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ReleaseStringChars) == 1328);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewStringUTF) == 1336);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStringUTFLength) == 1344);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStringUTFChars) == 1352);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ReleaseStringUTFChars) == 1360);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetArrayLength) == 1368);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewObjectArray) == 1376);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetObjectArrayElement) == 1384);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetObjectArrayElement) == 1392);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewBooleanArray) == 1400);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewByteArray) == 1408);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewCharArray) == 1416);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewShortArray) == 1424);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewIntArray) == 1432);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewLongArray) == 1440);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewFloatArray) == 1448);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewDoubleArray) == 1456);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetBooleanArrayElements) == 1464);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetByteArrayElements) == 1472);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetCharArrayElements) == 1480);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetShortArrayElements) == 1488);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetIntArrayElements) == 1496);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetLongArrayElements) == 1504);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetFloatArrayElements) == 1512);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetDoubleArrayElements) == 1520);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ReleaseBooleanArrayElements) == 1528);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ReleaseByteArrayElements) == 1536);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ReleaseCharArrayElements) == 1544);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ReleaseShortArrayElements) == 1552);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ReleaseIntArrayElements) == 1560);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ReleaseLongArrayElements) == 1568);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ReleaseFloatArrayElements) == 1576);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ReleaseDoubleArrayElements) == 1584);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetBooleanArrayRegion) == 1592);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetByteArrayRegion) == 1600);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetCharArrayRegion) == 1608);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetShortArrayRegion) == 1616);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetIntArrayRegion) == 1624);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetLongArrayRegion) == 1632);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetFloatArrayRegion) == 1640);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetDoubleArrayRegion) == 1648);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetBooleanArrayRegion) == 1656);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetByteArrayRegion) == 1664);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetCharArrayRegion) == 1672);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetShortArrayRegion) == 1680);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetIntArrayRegion) == 1688);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetLongArrayRegion) == 1696);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetFloatArrayRegion) == 1704);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, SetDoubleArrayRegion) == 1712);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, RegisterNatives) == 1720);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, UnregisterNatives) == 1728);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, MonitorEnter) == 1736);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, MonitorExit) == 1744);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetJavaVM) == 1752);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStringRegion) == 1760);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStringUTFRegion) == 1768);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetPrimitiveArrayCritical) == 1776);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ReleasePrimitiveArrayCritical) == 1784);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetStringCritical) == 1792);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ReleaseStringCritical) == 1800);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewWeakGlobalRef) == 1808);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, DeleteWeakGlobalRef) == 1816);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, ExceptionCheck) == 1824);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, NewDirectByteBuffer) == 1832);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetDirectBufferAddress) == 1840);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetDirectBufferCapacity) == 1848);
const _: () = core::assert! (std::mem::offset_of! (JNINativeInterface, GetObjectRefType) == 1856);
