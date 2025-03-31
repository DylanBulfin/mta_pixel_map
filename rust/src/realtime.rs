use godot::{
    builtin::{Array, GString, PackedByteArray},
    prelude::godot_api,
};
use gtfs_parsing::realtime::{
    protos::gtfs_realtime::{FeedMessage, VehiclePosition},
    try_parse_bytes,
};

use crate::make_objs;

make_objs!(VehicleGodot, VehicleRust,
    stop_id: GString: String,
    status: i32: i32,
);

impl From<VehiclePosition> for VehicleRust {
    fn from(value: VehiclePosition) -> Self {
        Self {
            stop_id: value.stop_id.expect("Unable to find stop_id"),
            status: match value.current_status {
                Some(val) => val.value(),
                None => -1,
            },
        }
    }
}

make_objs!(RealtimeUpdateGodot, RealtimeUpdateRust,
    vehicles: Array<GString>: Vec<String>,
);

impl From<FeedMessage> for RealtimeUpdateRust {
    fn from(value: FeedMessage) -> Self {
        let mut vehicles = Vec::new();

        for entity in value.entity {
            if entity.vehicle.is_some() {
                let vr = VehicleRust::from(entity.vehicle.unwrap());
                vehicles.push(serde_json::to_string(&vr).expect("Unable to parse vehicle"));
            }
        }

        Self { vehicles }
    }
}

#[godot_api]
impl RealtimeUpdateGodot {
    #[func]
    fn setup(&mut self, bytes: PackedByteArray) {
        let update_rs: RealtimeUpdateRust = try_parse_bytes(bytes.as_slice())
            .expect("Unable to parse update")
            .into();

        self.vehicles = update_rs
            .vehicles
            .iter()
            .map(GString::from)
            .collect::<Vec<_>>()
            .as_slice()
            .into();
    }
}
