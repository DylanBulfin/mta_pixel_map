use godot::{classes::json, meta::ArrayElement, prelude::*};
use gtfs_parsing::schedule::{
    Schedule,
    shapes::{Shape, ShapePointData},
};
use serde::{Serialize, ser::SerializeStruct};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

macro_rules! make_objs {
    ($obj1:ident, $obj2:ident, $($field:ident: $type1:ty: $type2:ty),+ $(,)?) => {
        #[derive(GodotClass, Debug)]
        #[class(init, base=RefCounted)]
        pub struct $obj1 {
           $(
                #[var]
                $field: $type1,
            )+
        }

        #[derive(Serialize, Debug)]
        pub struct $obj2 {
            $(
                $field: $type2,
            )+
        }
    };
}

make_objs!(ShapeGodot, ShapeRust,
    shape_id: GString: String,
    lats: Array<f32>: Vec<f32>,
    lons: Array<f32>: Vec<f32>,
);

make_objs!(ScheduleGodot, ScheduleRust,
    shapes: Array<GString>: Vec<String>
);

impl From<Shape> for ShapeRust {
    fn from(value: Shape) -> Self {
        let Shape { shape_id, points } = value;
        let lats = points.iter().map(|sp| sp.shape_pt_lat as f32).collect();
        let lons = points.iter().map(|sp| sp.shape_pt_lon as f32).collect();
        Self {
            shape_id: shape_id.into(),
            lats,
            lons,
        }
    }
}

#[godot_api]
impl ScheduleGodot {
    #[func]
    fn setup(&mut self) {
        let schedule: ScheduleRust =
            gtfs_parsing::schedule::Schedule::from_dir("./test_data", true).into();

        self.shapes = schedule
            .shapes
            .iter()
            .map(|st| GString::from(st))
            .collect::<Vec<_>>()
            .as_slice()
            .into()
    }
}

impl From<Schedule> for ScheduleRust {
    fn from(value: Schedule) -> Self {
        Self {
            shapes: value
                .shapes
                .into_iter()
                .map(Shape::into)
                .map(|s: ShapeRust| serde_json::to_string(&s).expect("Unable to parse shapes"))
                .collect(),
        }
    }
}

#[godot_api]
impl ShapeGodot {
    #[func]
    fn get_one_shape() -> GString {
        Self::get_shape_by_id("1..N03R")
    }

    #[func]
    fn get_six_shape() -> GString {
        Self::get_shape_by_id("6..N01R")
    }

    #[func]
    fn get_sev_shape() -> GString {
        Self::get_shape_by_id("7..N95R")
    }

    #[func]
    fn get_q_shape() -> GString {
        Self::get_shape_by_id("Q..N16R")
    }

    #[func]
    fn get_n_shape() -> GString {
        Self::get_shape_by_id("N..N20R")
    }

    fn get_shape_by_id(id: &str) -> GString {
        let schedule = gtfs_parsing::schedule::Schedule::from_dir("./test_data", true);

        let mut shapes: Vec<ShapeRust> = schedule.shapes.into_iter().map(Shape::into).collect();

        assert_eq!(shapes.len(), 311);

        shapes = shapes.into_iter().filter(|s| s.shape_id == id).collect();
        serde_json::to_string(&shapes.pop().expect("try"))
            .expect("exp")
            .into()
    }
}
