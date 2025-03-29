use godot::{classes::json, meta::ArrayElement, prelude::*};
use gtfs_parsing::schedule::{
    Schedule,
    mta::SubwayStation,
    shapes::{Shape, ShapePointData},
    stops::Stop,
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
    shapes: Array<GString>: Vec<String>,
    stations: Array<GString>: Vec<String>,
);

make_objs!(StationGodot, StationRust,
    id: GString: String,
    name: GString: String,
    lat: f32: f32,
    lon: f32: f32,
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

impl From<Stop> for StationRust {
    fn from(value: Stop) -> Self {
        if let Stop {
            stop_lat: Some(lat),
            stop_lon: Some(lon),
            stop_name: Some(name),
            stop_id: id,
            ..
        } = value
        {
            Self {
                lat: lat.parse().expect("Unable to parse latitude"),
                lon: lon.parse().expect("Unable to parse longitude"),
                name,
                id,
            }
        } else {
            panic!("Wrong format")
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
            .into();

        self.stations = schedule
            .stations
            .iter()
            .map(|st| GString::from(st))
            .collect::<Vec<_>>()
            .as_slice()
            .into();
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
            stations: value
                .stops
                .into_iter()
                .map(Stop::into)
                .map(|s: StationRust| serde_json::to_string(&s).expect("Unable to parse shapes"))
                .collect(),
        }
    }
}
