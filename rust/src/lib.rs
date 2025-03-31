pub mod realtime;
pub mod schedule;

#[macro_export]
macro_rules! make_objs {
    ($obj1:ident, $obj2:ident, $($field:ident: $type1:ty: $type2:ty),+ $(,)?) => {
        #[derive(godot::prelude::GodotClass, Debug)]
        #[class(init, base=RefCounted)]
        pub struct $obj1 {
           $(
                #[var]
                $field: $type1,
            )+
        }

        #[derive(serde::Serialize, Debug)]
        pub struct $obj2 {
            $(
                $field: $type2,
            )+
        }
    };
}
