use std::{fmt::Display, hash::Hash};

use ordered_float::OrderedFloat;

macro_rules! wrap_type {
    (f32) => {
        OrderedFloat<f32>
    };
    ($type:ident) => {
        $type
    };
}

macro_rules! wrap_value {
    (f32, $name:ident) => {
        OrderedFloat::from($name)
    };
    ($type:ident, $name:ident) => {
        $type::from($name)
    };
}

macro_rules! make_struct {
    (
        $name:ident,
        $topdoc:expr,
        $(
            ($field:ident, $type:ident, $doc:expr),
        )+
    ) => {
        #[doc = $topdoc]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name {
            $(
                #[doc = $doc]
                pub $field: wrap_type!($type),
            )+
        }

        impl $name {
            #[doc = concat!("Creates a new `", stringify!($name), "` instance.")]
            pub fn new( $($field: $type,)+ ) -> Self {
                $(
                    let $field = wrap_value!($type, $field);
                )+
                $name {
                    $($field, )+
                }
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, concat!(
                    $(
                        stringify!($field), ": {} ",
                    )+
                ), $(self.$field, )+)
            }
        }
    };
}
make_struct!(
    Vector,
    "A struct that stores a vector.",
    (x, f32, "X coordinate."),
    (y, f32, "Y coordinate."),
    (z, f32, "Z coordinate."),
);

make_struct!(
    Rotator,
    "A struct that stores a rotator.",
    (pitch, f32, "Euclidean pitch."),
    (yaw, f32, "Euclidean yaw."),
    (roll, f32, "Euclidean roll."),
);

make_struct!(
    Quat,
    "A struct that stores a quaternion.",
    (x, f32, "X component."),
    (y, f32, "Y component."),
    (z, f32, "Z component."),
    (w, f32, "Real component."),
);

make_struct!(
    DateTime,
    "A struct that stores a date and time.",
    (ticks, u64, "Ticks."),
);

make_struct!(
    IntPoint,
    "A struct that stores a 2D integer point.",
    (x, i32, "X value."),
    (y, i32, "Y value."),
);
