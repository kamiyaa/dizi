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

macro_rules! dizi_json_stub {
    ($struct_name:ident, $path:expr) => {
        #[derive(Clone, Debug, Deserialize, Serialize)]
        pub struct $struct_name {
            command: String,
        }
        impl DiziJsonCommand<'static> for $struct_name {
            fn path() -> &'static str {
                $path
            }
        }
        impl $struct_name {
            pub fn new() -> Self {
                Self {
                    command: Self::path().to_string(),
                }
            }
        }
    }
}

pub(crate) use dizi_json;
pub(crate) use dizi_json_stub;
