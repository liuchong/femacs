#![allow(non_snake_case, non_camel_case_types, unused_assignments, unused_mut)]
use std::ffi::{c_char,c_int,c_void,CString};
type size_t=usize;
type jobject=*mut c_void;
type jclass=jobject;
type jstring=jobject;
type jmethodID=*mut c_void;
type jint=c_int;
type JNIEnv=*const JniTable;
struct JniTable {
    CallNonvirtualObjectMethod:Option<unsafe extern "C" fn(*mut JNIEnv,jobject,jclass,jmethodID,...)->jobject>,
    GetStringUTFChars:Option<unsafe extern "C" fn(*mut JNIEnv,jstring,*mut u8)->*const c_char>,
    ReleaseStringUTFChars:Option<unsafe extern "C" fn(*mut JNIEnv,jstring,*const c_char)>,
    DeleteLocalRef:Option<unsafe extern "C" fn(*mut JNIEnv,jobject)>,
}
struct Service { name_keysym:jmethodID, class:jclass }
static TABLE:JniTable=JniTable {
    CallNonvirtualObjectMethod:Some(call_method),GetStringUTFChars:Some(get_chars),
    ReleaseStringUTFChars:Some(release_chars),DeleteLocalRef:Some(delete_ref),
};
static mut ENV:JNIEnv=&TABLE;
static mut android_java_env:*mut JNIEnv=&raw mut ENV;
static mut service_class:Service=Service { name_keysym:3usize as jmethodID,class:2usize as jclass };
static mut emacs_service:jobject=1usize as jobject;
static mut RETURN_NULL:bool=false;
static mut RETURN_TEXT:*const c_char=std::ptr::null();
static mut EXPECT_KEY:c_int=0;
static mut CALLS:[usize;4]=[0;4];
unsafe extern "C" fn call_method(env:*mut JNIEnv,object:jobject,class:jclass,method:jmethodID,mut args:...)->jobject {
    unsafe {
        let expected_env=android_java_env;let expected_object=emacs_service;
        let expected_class=service_class.class;let expected_method=service_class.name_keysym;
        let expected_key=EXPECT_KEY;
        assert_eq!(env,expected_env);assert_eq!(object,expected_object);
        assert_eq!(class,expected_class);assert_eq!(method,expected_method);
        assert_eq!(args.next_arg::<jint>(),expected_key);CALLS[0]+=1;
        if RETURN_NULL { std::ptr::null_mut() } else { 4usize as jobject }
    }
}
unsafe extern "C" fn get_chars(_: *mut JNIEnv,string:jstring,_:*mut u8)->*const c_char {
    unsafe { assert_eq!(string,4usize as jobject);CALLS[1]+=1;RETURN_TEXT }
}
unsafe extern "C" fn release_chars(_: *mut JNIEnv,string:jstring,bytes:*const c_char) {
    unsafe { let expected=RETURN_TEXT;assert_eq!(string,4usize as jobject);assert_eq!(bytes,expected);CALLS[2]+=1; }
}
unsafe extern "C" fn delete_ref(_: *mut JNIEnv,string:jobject) {
    unsafe { assert_eq!(string,4usize as jobject);CALLS[3]+=1; }
}
unsafe fn android_exception_check() {}
unsafe fn android_exception_check_nonnull(bytes:*mut c_void,string:jobject) {
    assert!(!bytes.is_null());assert!(!string.is_null());
}
unsafe extern "C" { fn strncpy(dest:*mut c_char,source:*const c_char,count:usize)->*mut c_char; }
macro_rules! ANDROID_DELETE_LOCAL_REF { ($object:expr) => {
    ((*(*android_java_env)).DeleteLocalRef.unwrap())(android_java_env,$object)
}; }

// ACTUAL_FUNCTION

#[repr(C)]
struct Guard { before:[u8;8],data:[u8;64],after:[u8;8] }
fn exercise(key:c_int,source:Option<&[u8]>,capacity:usize,expected:&[u8],jni:bool) {
    let text=CString::new(source.unwrap_or(b"unused")).unwrap();
    let mut guard=Guard { before:[0xa5;8],data:[0x55;64],after:[0xa6;8] };
    unsafe {
        CALLS=[0;4];EXPECT_KEY=key;RETURN_NULL=source.is_none();RETURN_TEXT=text.as_ptr();
        android_get_keysym_name(key,guard.data.as_mut_ptr().cast(),capacity);
        let calls=CALLS;
        assert_eq!(calls,if capacity==0 || !jni { [0;4] }
                   else if source.is_none() { [1,0,0,0] } else { [1,1,1,1] });
    }
    assert_eq!(guard.before,[0xa5;8]);assert_eq!(guard.after,[0xa6;8]);
    assert!(guard.data[capacity..].iter().all(|&b|b==0x55));
    if capacity>0 {
        let copied=expected.len().min(capacity-1);
        assert_eq!(&guard.data[..copied],&expected[..copied]);
        assert!(guard.data[copied..capacity].iter().all(|&b|b==0));
    }
}
macro_rules! case { ($name:ident,$body:block) => {
    #[test] fn $name() { let started=std::time::Instant::now();$body
        println!("CASE {} seconds={:.6}",stringify!($name),started.elapsed().as_secs_f64()); }
}; }
case!(select_all,{exercise(65537,Some(b"unused"),64,b"select-all",false);});
case!(start_selecting,{exercise(65538,Some(b"unused"),64,b"start-selecting-text",false);});
case!(stop_selecting,{exercise(65539,Some(b"unused"),64,b"stop-selecting-text",false);});
case!(null_java_fallback,{exercise(29,None,64,b"stop-selecting-text",true);});
case!(java_name,{exercise(29,Some(b"KEYCODE_A"),64,b"KEYCODE_A",true);});
case!(java_truncation,{let text=vec![b'K';200];exercise(29,Some(&text),64,&text,true);});
case!(exact_fit,{let text=vec![b'X';63];exercise(29,Some(&text),64,&text,true);});
case!(one_byte_special,{exercise(65537,Some(b"unused"),1,b"select-all",false);});
case!(one_byte_java,{exercise(29,Some(b"KEYCODE_A"),1,b"KEYCODE_A",true);});
case!(zero_capacity_preserves_storage,{exercise(65537,Some(b"unused"),0,b"",false);});
case!(zero_capacity_accepts_null_without_jni,{unsafe {
    CALLS=[0;4];android_get_keysym_name(29,std::ptr::null_mut(),0);
    let calls=CALLS;assert_eq!(calls,[0;4]);
}});
