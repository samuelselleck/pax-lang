pub extern crate pax_macro;
pub use pax_macro::*;

pub use pax_runtime_api as api;

pub use declarative_macros::*;
pub use pax_runtime_api::log;
pub use pax_runtime_api::Property;

mod declarative_macros {
    #[macro_export]
    macro_rules! pax_struct {
        ($name:ident { $($field:ident : $value:expr),* $(,)? }) => {
            $name {
                $(
                    $field: Box::new(PropertyLiteral::new($value)),
                )*
            }
        };
    }
}
