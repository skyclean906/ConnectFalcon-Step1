/* automatically generated by rust-bindgen */

pub const CRYPTO_SECRETKEYBYTES: u32 = 4097;
pub const CRYPTO_PUBLICKEYBYTES: u32 = 897;
pub const CRYPTO_BYTES: u32 = 690;
pub const CRYPTO_ALGNAME: &'static [u8; 11usize] = b"Falcon-512\0";
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const WCHAR_MIN: u32 = 0;
pub const WCHAR_MAX: u32 = 65535;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 65535;
pub const FALCON_COMP_NONE: u32 = 0;
pub const FALCON_COMP_STATIC: u32 = 1;
pub const _ARGMAX: u32 = 100;
pub const _CRT_INT_MAX: u32 = 2147483647;
pub const _CRT_FUNCTIONS_REQUIRED: u32 = 1;
pub const _CRT_HAS_CXX17: u32 = 0;
pub const _ARM_WINAPI_PARTITION_DESKTOP_SDK_AVAILABLE: u32 = 1;
pub const _CRT_BUILD_DESKTOP_APP: u32 = 1;
pub const __STDC_SECURE_LIB__: u32 = 200411;
pub const __GOT_SECURE_LIB__: u32 = 200411;
pub const __STDC_WANT_SECURE_LIB__: u32 = 1;
pub const _SECURECRT_FILL_BUFFER_PATTERN: u32 = 254;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_COUNT: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES: u32 = 1;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_MEMORY: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES_MEMORY: u32 = 0;
pub const _CRT_INTERNAL_STDIO_SYMBOL_PREFIX: &'static [u8; 1usize] = b"\0";
pub const _CRT_INTERNAL_PRINTF_LEGACY_VSPRINTF_NULL_TERMINATION: u32 = 1;
pub const _CRT_INTERNAL_PRINTF_STANDARD_SNPRINTF_BEHAVIOR: u32 = 2;
pub const _CRT_INTERNAL_PRINTF_LEGACY_WIDE_SPECIFIERS: u32 = 4;
pub const _CRT_INTERNAL_PRINTF_LEGACY_MSVCRT_COMPATIBILITY: u32 = 8;
pub const _CRT_INTERNAL_PRINTF_LEGACY_THREE_DIGIT_EXPONENTS: u32 = 16;
pub const _CRT_INTERNAL_SCANF_SECURECRT: u32 = 1;
pub const _CRT_INTERNAL_SCANF_LEGACY_WIDE_SPECIFIERS: u32 = 2;
pub const _CRT_INTERNAL_SCANF_LEGACY_MSVCRT_COMPATIBILITY: u32 = 4;
pub const BUFSIZ: u32 = 512;
pub const _NSTREAM_: u32 = 512;
pub const _IOB_ENTRIES: u32 = 3;
pub const EOF: i32 = -1;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 64;
pub const _IONBF: u32 = 4;
pub const L_tmpnam: u32 = 260;
pub const L_tmpnam_s: u32 = 260;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const SEEK_SET: u32 = 0;
pub const FILENAME_MAX: u32 = 260;
pub const FOPEN_MAX: u32 = 20;
pub const _SYS_OPEN: u32 = 20;
pub const TMP_MAX: u32 = 2147483647;
pub const TMP_MAX_S: u32 = 2147483647;
pub const _TMP_MAX_S: u32 = 2147483647;
pub const SYS_OPEN: u32 = 20;
pub const RNG_SUCCESS: u32 = 0;
pub const RNG_BAD_MAXLEN: i32 = -1;
pub const RNG_BAD_OUTBUF: i32 = -2;
pub const RNG_BAD_REQ_LEN: i32 = -3;
extern "C" {
    pub fn crypto_sign_keypair(
        pk: *mut ::std::os::raw::c_uchar,
        sk: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign(
        sm: *mut ::std::os::raw::c_uchar,
        smlen: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_open(
        m: *mut ::std::os::raw::c_uchar,
        mlen: *mut ::std::os::raw::c_ulonglong,
        sm: *const ::std::os::raw::c_uchar,
        smlen: ::std::os::raw::c_ulonglong,
        pk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
pub type size_t = ::std::os::raw::c_ulonglong;
pub type wchar_t = ::std::os::raw::c_ushort;
pub type max_align_t = f64;
pub type va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type __vcrt_bool = bool;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize);
}
extern "C" {
    pub static mut __security_cookie: usize;
}
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct falcon_vrfy_ {
    _unused: [u8; 0],
}
pub type falcon_vrfy = falcon_vrfy_;
extern "C" {
    pub fn falcon_vrfy_new() -> *mut falcon_vrfy;
}
extern "C" {
    pub fn falcon_vrfy_free(fv: *mut falcon_vrfy);
}
extern "C" {
    pub fn falcon_vrfy_set_public_key(
        fv: *mut falcon_vrfy,
        pkey: *const ::std::os::raw::c_void,
        len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn falcon_vrfy_start(fv: *mut falcon_vrfy, r: *const ::std::os::raw::c_void, rlen: size_t);
}
extern "C" {
    pub fn falcon_vrfy_update(
        fv: *mut falcon_vrfy,
        data: *const ::std::os::raw::c_void,
        len: size_t,
    );
}
extern "C" {
    pub fn falcon_vrfy_verify(
        fv: *mut falcon_vrfy,
        sig: *const ::std::os::raw::c_void,
        len: size_t,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct falcon_sign_ {
    _unused: [u8; 0],
}
pub type falcon_sign = falcon_sign_;
extern "C" {
    pub fn falcon_sign_new() -> *mut falcon_sign;
}
extern "C" {
    pub fn falcon_sign_free(fs: *mut falcon_sign);
}
extern "C" {
    pub fn falcon_sign_set_seed(
        fs: *mut falcon_sign,
        seed: *const ::std::os::raw::c_void,
        len: size_t,
        replace: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn falcon_sign_set_private_key(
        fs: *mut falcon_sign,
        skey: *const ::std::os::raw::c_void,
        len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn falcon_sign_start(
        fs: *mut falcon_sign,
        r: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn falcon_sign_start_external_nonce(
        fs: *mut falcon_sign,
        r: *const ::std::os::raw::c_void,
        rlen: size_t,
    );
}
extern "C" {
    pub fn falcon_sign_update(
        fs: *mut falcon_sign,
        data: *const ::std::os::raw::c_void,
        len: size_t,
    );
}
extern "C" {
    pub fn falcon_sign_generate(
        fs: *mut falcon_sign,
        sig: *mut ::std::os::raw::c_void,
        sig_max_len: size_t,
        comp: ::std::os::raw::c_int,
    ) -> size_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct falcon_keygen_ {
    _unused: [u8; 0],
}
pub type falcon_keygen = falcon_keygen_;
extern "C" {
    pub fn falcon_keygen_new(
        logn: ::std::os::raw::c_uint,
        ternary: ::std::os::raw::c_int,
    ) -> *mut falcon_keygen;
}
extern "C" {
    pub fn falcon_keygen_free(fk: *mut falcon_keygen);
}
extern "C" {
    pub fn falcon_keygen_max_privkey_size(fk: *mut falcon_keygen) -> size_t;
}
extern "C" {
    pub fn falcon_keygen_max_pubkey_size(fk: *mut falcon_keygen) -> size_t;
}
extern "C" {
    pub fn falcon_keygen_set_seed(
        fk: *mut falcon_keygen,
        seed: *const ::std::os::raw::c_void,
        len: size_t,
        replace: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn falcon_keygen_make(
        fk: *mut falcon_keygen,
        comp: ::std::os::raw::c_int,
        privkey: *mut ::std::os::raw::c_void,
        privkey_len: *mut size_t,
        pubkey: *mut ::std::os::raw::c_void,
        pubkey_len: *mut size_t,
    ) -> ::std::os::raw::c_int;
}
pub type __crt_bool = bool;
extern "C" {
    pub fn _invalid_parameter_noinfo();
}
extern "C" {
    pub fn _invalid_parameter_noinfo_noreturn();
}
extern "C" {
    pub fn _invoke_watson(
        _Expression: *const wchar_t,
        _FunctionName: *const wchar_t,
        _FileName: *const wchar_t,
        _LineNo: ::std::os::raw::c_uint,
        _Reserved: usize,
    );
}
pub type errno_t = ::std::os::raw::c_int;
pub type wint_t = ::std::os::raw::c_ushort;
pub type wctype_t = ::std::os::raw::c_ushort;
pub type __time32_t = ::std::os::raw::c_long;
pub type __time64_t = ::std::os::raw::c_longlong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data_public {
    pub _locale_pctype: *const ::std::os::raw::c_ushort,
    pub _locale_mb_cur_max: ::std::os::raw::c_int,
    pub _locale_lc_codepage: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___crt_locale_data_public() {
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_data_public>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_data_public))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_data_public>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_data_public))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__crt_locale_data_public>()))._locale_pctype as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_pctype)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__crt_locale_data_public>()))._locale_mb_cur_max as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_mb_cur_max)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__crt_locale_data_public>()))._locale_lc_codepage as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_lc_codepage)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_pointers {
    pub locinfo: *mut __crt_locale_data,
    pub mbcinfo: *mut __crt_multibyte_data,
}
#[test]
fn bindgen_test_layout___crt_locale_pointers() {
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_pointers>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_pointers>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__crt_locale_pointers>())).locinfo as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(locinfo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__crt_locale_pointers>())).mbcinfo as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(mbcinfo)
        )
    );
}
pub type _locale_t = *mut __crt_locale_pointers;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Mbstatet {
    pub _Wchar: ::std::os::raw::c_ulong,
    pub _Byte: ::std::os::raw::c_ushort,
    pub _State: ::std::os::raw::c_ushort,
}
#[test]
fn bindgen_test_layout__Mbstatet() {
    assert_eq!(
        ::std::mem::size_of::<_Mbstatet>(),
        8usize,
        concat!("Size of: ", stringify!(_Mbstatet))
    );
    assert_eq!(
        ::std::mem::align_of::<_Mbstatet>(),
        4usize,
        concat!("Alignment of ", stringify!(_Mbstatet))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Mbstatet>()))._Wchar as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Wchar)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Mbstatet>()))._Byte as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Byte)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Mbstatet>()))._State as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_State)
        )
    );
}
pub type mbstate_t = _Mbstatet;
pub type time_t = __time64_t;
pub type rsize_t = size_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _iobuf {
    pub _Placeholder: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__iobuf() {
    assert_eq!(
        ::std::mem::size_of::<_iobuf>(),
        8usize,
        concat!("Size of: ", stringify!(_iobuf))
    );
    assert_eq!(
        ::std::mem::align_of::<_iobuf>(),
        8usize,
        concat!("Alignment of ", stringify!(_iobuf))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_iobuf>()))._Placeholder as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_iobuf),
            "::",
            stringify!(_Placeholder)
        )
    );
}
pub type FILE = _iobuf;
extern "C" {
    pub fn __acrt_iob_func(_Ix: ::std::os::raw::c_uint) -> *mut FILE;
}
extern "C" {
    pub fn fgetwc(_Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _fgetwchar() -> wint_t;
}
extern "C" {
    pub fn fputwc(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _fputwchar(_Character: wchar_t) -> wint_t;
}
extern "C" {
    pub fn getwc(_Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn getwchar() -> wint_t;
}
extern "C" {
    pub fn fgetws(
        _Buffer: *mut wchar_t,
        _BufferCount: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn fputws(_Buffer: *const wchar_t, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _getws_s(_Buffer: *mut wchar_t, _BufferCount: size_t) -> *mut wchar_t;
}
extern "C" {
    pub fn putwc(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn putwchar(_Character: wchar_t) -> wint_t;
}
extern "C" {
    pub fn _putws(_Buffer: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ungetwc(_Character: wint_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _wfdopen(_FileHandle: ::std::os::raw::c_int, _Mode: *const wchar_t) -> *mut FILE;
}
extern "C" {
    pub fn _wfopen(_FileName: *const wchar_t, _Mode: *const wchar_t) -> *mut FILE;
}
extern "C" {
    pub fn _wfopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _wfreopen(
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
        _OldStream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn _wfreopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
        _OldStream: *mut FILE,
    ) -> errno_t;
}
extern "C" {
    pub fn _wfsopen(
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
        _ShFlag: ::std::os::raw::c_int,
    ) -> *mut FILE;
}
extern "C" {
    pub fn _wperror(_ErrorMessage: *const wchar_t);
}
extern "C" {
    pub fn _wpopen(_Command: *const wchar_t, _Mode: *const wchar_t) -> *mut FILE;
}
extern "C" {
    pub fn _wremove(_FileName: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wtempnam(_Directory: *const wchar_t, _FilePrefix: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _wtmpnam_s(_Buffer: *mut wchar_t, _BufferCount: size_t) -> errno_t;
}
extern "C" {
    pub fn _wtmpnam(_Buffer: *mut wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _fgetwc_nolock(_Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _fputwc_nolock(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _getwc_nolock(_Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _putwc_nolock(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _ungetwc_nolock(_Character: wint_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn __stdio_common_vfwprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfwprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfwprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfwscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vswprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: size_t,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vswprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: size_t,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsnwprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: size_t,
        _MaxCount: size_t,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vswprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: size_t,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vswscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *const wchar_t,
        _BufferCount: size_t,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
pub type fpos_t = ::std::os::raw::c_longlong;
extern "C" {
    pub fn _get_stream_buffer_pointers(
        _Stream: *mut FILE,
        _Base: *mut *mut *mut ::std::os::raw::c_char,
        _Pointer: *mut *mut *mut ::std::os::raw::c_char,
        _Count: *mut *mut ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn clearerr_s(_Stream: *mut FILE) -> errno_t;
}
extern "C" {
    pub fn fopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
    ) -> errno_t;
}
extern "C" {
    pub fn fread_s(
        _Buffer: *mut ::std::os::raw::c_void,
        _BufferSize: size_t,
        _ElementSize: size_t,
        _ElementCount: size_t,
        _Stream: *mut FILE,
    ) -> size_t;
}
extern "C" {
    pub fn freopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
        _OldStream: *mut FILE,
    ) -> errno_t;
}
extern "C" {
    pub fn gets_s(
        _Buffer: *mut ::std::os::raw::c_char,
        _Size: rsize_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpfile_s(_Stream: *mut *mut FILE) -> errno_t;
}
extern "C" {
    pub fn tmpnam_s(_Buffer: *mut ::std::os::raw::c_char, _Size: rsize_t) -> errno_t;
}
extern "C" {
    pub fn clearerr(_Stream: *mut FILE);
}
extern "C" {
    pub fn fclose(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fcloseall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fdopen(
        _FileHandle: ::std::os::raw::c_int,
        _Mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn feof(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fgetchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetpos(_Stream: *mut FILE, _Position: *mut fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgets(
        _Buffer: *mut ::std::os::raw::c_char,
        _MaxCount: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _fileno(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _flushall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fopen(
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fputc(_Character: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fputchar(_Character: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputs(
        _Buffer: *const ::std::os::raw::c_char,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread(
        _Buffer: *mut ::std::os::raw::c_void,
        _ElementSize: ::std::os::raw::c_ulonglong,
        _ElementCount: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn freopen(
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
        _Stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn _fsopen(
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
        _ShFlag: ::std::os::raw::c_int,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fsetpos(_Stream: *mut FILE, _Position: *const fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fseek(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_long,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fseeki64(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_longlong,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftell(_Stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn _ftelli64(_Stream: *mut FILE) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fwrite(
        _Buffer: *const ::std::os::raw::c_void,
        _ElementSize: ::std::os::raw::c_ulonglong,
        _ElementCount: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn getc(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _getmaxstdio() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _getw(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn perror(_ErrorMessage: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn _pclose(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _popen(
        _Command: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn putc(_Character: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar(_Character: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn puts(_Buffer: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _putw(_Word: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn remove(_FileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rename(
        _OldFileName: *const ::std::os::raw::c_char,
        _NewFileName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _unlink(_FileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unlink(_FileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rewind(_Stream: *mut FILE);
}
extern "C" {
    pub fn _rmtmp() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuf(_Stream: *mut FILE, _Buffer: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn _setmaxstdio(_Maximum: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setvbuf(
        _Stream: *mut FILE,
        _Buffer: *mut ::std::os::raw::c_char,
        _Mode: ::std::os::raw::c_int,
        _Size: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _tempnam(
        _DirectoryName: *const ::std::os::raw::c_char,
        _FilePrefix: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(_Buffer: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ungetc(_Character: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _lock_file(_Stream: *mut FILE);
}
extern "C" {
    pub fn _unlock_file(_Stream: *mut FILE);
}
extern "C" {
    pub fn _fclose_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fflush_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fgetc_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fputc_nolock(
        _Character: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fread_nolock(
        _Buffer: *mut ::std::os::raw::c_void,
        _ElementSize: size_t,
        _ElementCount: size_t,
        _Stream: *mut FILE,
    ) -> size_t;
}
extern "C" {
    pub fn _fread_nolock_s(
        _Buffer: *mut ::std::os::raw::c_void,
        _BufferSize: size_t,
        _ElementSize: size_t,
        _ElementCount: size_t,
        _Stream: *mut FILE,
    ) -> size_t;
}
extern "C" {
    pub fn _fseek_nolock(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_long,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fseeki64_nolock(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_longlong,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _ftell_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn _ftelli64_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _fwrite_nolock(
        _Buffer: *const ::std::os::raw::c_void,
        _ElementSize: size_t,
        _ElementCount: size_t,
        _Stream: *mut FILE,
    ) -> size_t;
}
extern "C" {
    pub fn _getc_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _putc_nolock(
        _Character: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _ungetc_nolock(
        _Character: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __p__commode() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _set_printf_count_output(_Value: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _get_printf_count_output() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _Arglist: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsnprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _MaxCount: size_t,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *const ::std::os::raw::c_char,
        _BufferCount: size_t,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tempnam(
        _Directory: *const ::std::os::raw::c_char,
        _FilePrefix: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fcloseall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fdopen(
        _FileHandle: ::std::os::raw::c_int,
        _Format: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fgetchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fileno(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn flushall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputchar(_Ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getw(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putw(_Ch: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rmtmp() -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AES_XOF_struct {
    pub buffer: [::std::os::raw::c_uchar; 16usize],
    pub buffer_pos: ::std::os::raw::c_int,
    pub length_remaining: ::std::os::raw::c_ulong,
    pub key: [::std::os::raw::c_uchar; 32usize],
    pub ctr: [::std::os::raw::c_uchar; 16usize],
}
#[test]
fn bindgen_test_layout_AES_XOF_struct() {
    assert_eq!(
        ::std::mem::size_of::<AES_XOF_struct>(),
        72usize,
        concat!("Size of: ", stringify!(AES_XOF_struct))
    );
    assert_eq!(
        ::std::mem::align_of::<AES_XOF_struct>(),
        4usize,
        concat!("Alignment of ", stringify!(AES_XOF_struct))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AES_XOF_struct>())).buffer as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AES_XOF_struct),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AES_XOF_struct>())).buffer_pos as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AES_XOF_struct),
            "::",
            stringify!(buffer_pos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AES_XOF_struct>())).length_remaining as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(AES_XOF_struct),
            "::",
            stringify!(length_remaining)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AES_XOF_struct>())).key as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AES_XOF_struct),
            "::",
            stringify!(key)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AES_XOF_struct>())).ctr as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(AES_XOF_struct),
            "::",
            stringify!(ctr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AES256_CTR_DRBG_struct {
    pub Key: [::std::os::raw::c_uchar; 32usize],
    pub V: [::std::os::raw::c_uchar; 16usize],
    pub reseed_counter: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_AES256_CTR_DRBG_struct() {
    assert_eq!(
        ::std::mem::size_of::<AES256_CTR_DRBG_struct>(),
        52usize,
        concat!("Size of: ", stringify!(AES256_CTR_DRBG_struct))
    );
    assert_eq!(
        ::std::mem::align_of::<AES256_CTR_DRBG_struct>(),
        4usize,
        concat!("Alignment of ", stringify!(AES256_CTR_DRBG_struct))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AES256_CTR_DRBG_struct>())).Key as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AES256_CTR_DRBG_struct),
            "::",
            stringify!(Key)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AES256_CTR_DRBG_struct>())).V as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(AES256_CTR_DRBG_struct),
            "::",
            stringify!(V)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AES256_CTR_DRBG_struct>())).reseed_counter as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(AES256_CTR_DRBG_struct),
            "::",
            stringify!(reseed_counter)
        )
    );
}
extern "C" {
    pub fn AES256_CTR_DRBG_Update(
        provided_data: *mut ::std::os::raw::c_uchar,
        Key: *mut ::std::os::raw::c_uchar,
        V: *mut ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn seedexpander_init(
        ctx: *mut AES_XOF_struct,
        seed: *mut ::std::os::raw::c_uchar,
        diversifier: *mut ::std::os::raw::c_uchar,
        maxlen: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn seedexpander(
        ctx: *mut AES_XOF_struct,
        x: *mut ::std::os::raw::c_uchar,
        xlen: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn randombytes_init(
        entropy_input: *mut ::std::os::raw::c_uchar,
        personalization_string: *mut ::std::os::raw::c_uchar,
        security_strength: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn randombytes(
        x: *mut ::std::os::raw::c_uchar,
        xlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_multibyte_data {
    pub _address: u8,
}
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
