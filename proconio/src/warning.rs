#![allow(path_statements, clippy::no_effect)]

#[cfg(not(feature = "disable_compat_warning"))]
const _: () = {
    #[rustfmt::skip]
    (WARN, "please use proconio v0.3.x in AtCoder 2020");

    #[deprecated(note = "this version of proconio is not compatible\n\
        please use the v0.3.x version instead \
        if you use this in the AtCoder 2020 update.\n\
        note: for uses outside of AtCoder, \
        you can disable this warning \
        via the `disable_compat_warning` feature.")]
    const WARN: () = ();
};
