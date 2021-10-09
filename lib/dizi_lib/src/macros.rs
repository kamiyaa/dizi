use crate::traits::DiziJsonCommand;

macro_rules! dizi_json {
    ($struct_name:ident, $path:expr) => {
        impl DiziJsonCommand<'static> for $struct_name {
            fn path() -> &'static str {
                $path
            }
        }
    }
}

pub(crate) use dizi_json;
