/// A macro to define extension traits for a given type.
///
/// This macro allows defining extension traits for a type with various combinations of `self` and generics.
/// It generates the trait definition and the corresponding implementation for the given type.
///
/// # Examples
///
/// ```
/// define_extension_trait! {
///     pub trait ScriptBufExt impl for String {
///         fn new_p2pk(pubkey: u64) -> Self {
///             format!("OP_PUSHBYTES_{}", pubkey)
///         }
///
///         fn new_p2pkh(pubkey_hash: String) -> Self {
///             format!("Hash_{}", pubkey_hash)
///         }
///     }
/// }
/// ```
macro_rules! define_extension_trait {
    // With self, no generics.
    ($(#[$($trait_attrs:tt)*])* $trait_vis:vis trait $trait_name:ident impl for $ty:ident {
        $(
            $(#[$($fn_attrs:tt)*])*
            fn $fn:ident($slf:ident $(, $param_name:ident: $param_type:ty)*) $( -> $ret:ty )? $body:block
        )*
    }) => {
        $(#[$($trait_attrs)*])* $trait_vis trait $trait_name {
            $(
                $(#[$($fn_attrs)*])*
                fn $fn($slf $(, $param_name: $param_type)*) $( -> $ret )?;
            )*
        }

        impl $trait_name for $ty {
            $(
                fn $fn($slf $(, $param_name: $param_type)*) $( -> $ret )? $body
            )*
        }
    };
    // With &self, no generics.
    ($(#[$($trait_attrs:tt)*])* $trait_vis:vis trait $trait_name:ident impl for $ty:ident {
        $(
            $(#[$($fn_attrs:tt)*])*
            fn $fn:ident(&$slf:ident $(, $param_name:ident: $param_type:ty)*) $( -> $ret:ty )? $body:block
        )*
    }) => {
        $(#[$($trait_attrs)*])* $trait_vis trait $trait_name {
            $(
                $(#[$($fn_attrs)*])*
                fn $fn(&$slf $(, $param_name: $param_type)*) $( -> $ret )?;
            )*
        }

        impl $trait_name for $ty {
            $(
                fn $fn(&$slf $(, $param_name: $param_type)*) $( -> $ret )? $body
            )*
        }
    };
    // With &self, with generics.
    ($(#[$($trait_attrs:tt)*])* $trait_vis:vis trait $trait_name:ident impl for $ty:ident {
        $(
            $(#[$($fn_attrs:tt)*])*
            fn $fn:ident$(<$($gen:ident: $gent:ident),*>)?(&$slf:ident $(, $param_name:ident: $param_type:ty)*) $( -> $ret:ty )? $body:block
        )*
    }) => {
        $(#[$($trait_attrs)*])* $trait_vis trait $trait_name {
            $(
                $(#[$($fn_attrs)*])*
                fn $fn$(<$($gen: $gent),*>)?(&$slf $(, $param_name: $param_type)*) $( -> $ret )?;
            )*
        }

        impl $trait_name for $ty {
            $(
                fn $fn$(<$($gen: $gent),*>)?(&$slf $(, $param_name: $param_type)*) $( -> $ret )? $body
            )*
        }
    };
    // No self, with generics.
    ($(#[$($trait_attrs:tt)*])* $trait_vis:vis trait $trait_name:ident impl for $ty:ident {
        $(
            $(#[$($fn_attrs:tt)*])*
            fn $fn:ident$(<$($gen:ident: $gent:ident),*>)?($($param_name:ident: $param_type:ty),*) $( -> $ret:ty )? $body:block
        )*
    }) => {
        $(#[$($trait_attrs)*])* $trait_vis trait $trait_name {
            $(
                $(#[$($fn_attrs)*])*
                fn $fn$(<$($gen: $gent),*>)?($($param_name: $param_type),*) $( -> $ret )?;
            )*
        }

        impl $trait_name for $ty {
            $(
                fn $fn$(<$($gen: $gent),*>)?($($param_name: $param_type),*) $( -> $ret )? $body
            )*
        }
    };
    // No self, with generic `<T: AsRef<PushBytes>>`
    ($(#[$($trait_attrs:tt)*])* $trait_vis:vis trait $trait_name:ident impl for $ty:ident {
        $(
            $(#[$($fn_attrs:tt)*])*
            fn $fn:ident<T: AsRef<PushBytes>>($($param_name:ident: $param_type:ty),*) $( -> $ret:ty )? $body:block
        )*
    }) => {
        $(#[$($trait_attrs)*])* $trait_vis trait $trait_name {
            $(
                $(#[$($fn_attrs)*])*
                fn $fn<T: AsRef<PushBytes>>($($param_name: $param_type),*) $( -> $ret )?;
            )*
        }

        impl $trait_name for $ty {
            $(
                fn $fn<T: AsRef<PushBytes>>($($param_name: $param_type),*) $( -> $ret )? $body
            )*
        }
    };
}

pub(crate) use define_extension_trait;