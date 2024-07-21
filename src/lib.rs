pub mod config;
pub mod request;
pub mod response;


mod lib {
    macro_rules! append_field_names {
        (pub struct $name:ident { $(pub $fname:ident : $ftype:ty),* }) => {
            pub struct $name {
                pub $($fname : $ftype),*
            }
    
            impl $name {
                pub fn field_names() -> &'static [&'static str] {
                    static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                    NAMES
                }
            }
        }
    }

    pub(crate) use append_field_names;    // <-- the trick
}