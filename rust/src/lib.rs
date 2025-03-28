use godot::{meta::ArrayElement, prelude::*};
use gtfs_parsing::schedule::shapes::{Shape, ShapePointData};
use serde::Serialize;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass, Debug)]
#[class(no_init, base = RefCounted)]
pub struct GShape {
    pub shape_id: GString,
    pub lats: Array<f64>,
    pub lons: Array<f64>,
}

impl From<Shape> for GShape {
    fn from(value: Shape) -> Self {
        let Shape { shape_id, points } = value;
        Self {
            shape_id: shape_id.into(),
            lats: (points
                .iter()
                .map(|sp| sp.shape_pt_lat)
                .collect::<Vec<_>>()
                .as_slice())
            .into(),
            lons: (points
                .iter()
                .map(|sp| sp.shape_pt_lon)
                .collect::<Vec<_>>()
                .as_slice())
            .into(),
        }
    }
}

#[godot_api]
impl GShape {
    #[func]
    fn get_lats() -> GString {
        let schedule = gtfs_parsing::schedule::Schedule::from_dir("./test_data", true);

        let mut shapes: Vec<Shape> = schedule.shapes;

        shapes = shapes
            .into_iter()
            .filter(|s| s.shape_id == "1..N03R")
            .collect();
        serde_json::to_string(&shapes.pop().unwrap())
            .unwrap()
            .into()
    }
}
