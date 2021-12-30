#[macro_export]
macro_rules! return_iferror {
    ( $code:expr ) => {{
        let res = unsafe { $code };
        if res != gdiplus_sys2::Status_Ok {
            return Err(crate::Error::from(res));
        }
    }};
}

#[macro_export]
macro_rules! panic_iferror {
    ( $code:expr ) => {{
        let res = unsafe { $code };
        if res != gdiplus_sys2::Status_Ok {
            panic!("{:?}", crate::Error::from(res));
        }
    }};
}
