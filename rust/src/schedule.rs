use crate::make_objs;
use godot::prelude::*;
use gtfs_parsing::schedule::{
    Schedule,
    calendar::{Activity, ExceptionType, Service, ServiceException},
    routes::Route,
    shapes::Shape,
    stop_times::StopTime,
    stops::Stop,
    transfers::{Transfer, TransferType},
    trips::{DirectionType, Trip},
};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

make_objs!(ShapeGodot, ShapeRust,
    shape_id: GString: String,
    lats: Array<f32>: Vec<f32>,
    lons: Array<f32>: Vec<f32>,
);

make_objs!(StopGodot, StopRust,
    stop_id: GString: String,
    name: GString: String,
    lat: f32: f32,
    lon: f32: f32,

    has_parent: bool: bool,
    parent_station_id: GString: String,
);

make_objs!(StopTimeGodot, StopTimeRust,
    trip_id: GString: String,
    stop_id: GString: String,
    arrival_time: GString: String,
    departure_time: GString: String,
    stop_sequence: u32: u32
);

make_objs!(TripGodot, TripRust,
    trip_id: GString: String,
    service_id: GString: String,
    route_id: GString: String,
    trip_headsign: GString: String,
    is_uptown: bool: bool,

    has_shape: bool: bool,
    shape_id: GString: String,
);

make_objs!(RouteGodot, RouteRust,
    route_id: GString: String,
    short_name: GString: String,
    long_name: GString: String,
    desc: GString: String,
    url: GString: String,

    has_color: bool: bool,
    color: GString: String,

    has_text_color: bool: bool,
    text_color: GString: String,
);

make_objs!(ServiceGodot, ServiceRust,
    service_id: GString: String,
    monday: bool: bool,
    tuesday: bool: bool,
    wednesday: bool: bool,
    thursday: bool: bool,
    friday: bool: bool,
    saturday: bool: bool,
    sunday: bool: bool,
    start_date: GString: String,
    end_date: GString: String,
);

make_objs!(ServiceExceptionGodot, ServiceExceptionRust,
    service_id: GString: String,
    date: GString: String,
    added: bool: bool,
);

make_objs!(TransferGodot, TransferRust,
    from_stop_id: GString: String,
    to_stop_id: GString: String,
    min_transfer_time: u32: u32,
);

make_objs!(ScheduleGodot, ScheduleRust,
    shapes: Array<GString>: Vec<String>,
    stops: Array<GString>: Vec<String>,
    stop_times: Array<GString>: Vec<String>,
    transfers: Array<GString>: Vec<String>,
    services: Array<GString>: Vec<String>,
    service_exceptions: Array<GString>: Vec<String>,
    routes: Array<GString>: Vec<String>,
    trips: Array<GString>: Vec<String>,
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

impl From<Stop> for StopRust {
    fn from(value: Stop) -> Self {
        if let Stop {
            stop_lat: Some(lat),
            stop_lon: Some(lon),
            stop_name: Some(name),
            stop_id: id,
            parent_station,
            ..
        } = value
        {
            Self {
                lat: lat.parse().expect("Unable to parse latitude"),
                lon: lon.parse().expect("Unable to parse longitude"),
                name,
                stop_id: id,
                has_parent: parent_station.is_some(),
                parent_station_id: if let Some(id) = parent_station {
                    id
                } else {
                    String::new()
                },
            }
        } else {
            panic!("Wrong format for Stop: {:?}", value)
        }
    }
}

impl From<StopTime> for StopTimeRust {
    fn from(value: StopTime) -> Self {
        if let StopTime {
            stop_id: Some(stop_id),
            trip_id,
            arrival_time: Some(arrival_time),
            departure_time: Some(departure_time),
            stop_sequence,
            ..
        } = value
        {
            Self {
                stop_id,
                trip_id,
                arrival_time,
                departure_time,
                stop_sequence,
            }
        } else {
            panic!("Wrong format for StopTime: {:?}", value)
        }
    }
}

impl From<Trip> for TripRust {
    fn from(value: Trip) -> Self {
        if let Trip {
            trip_id,
            route_id,
            service_id,
            direction_id: Some(direction_id),
            trip_headsign: Some(trip_headsign),
            shape_id,
            ..
        } = value
        {
            Self {
                route_id,
                trip_id,
                service_id,
                trip_headsign,
                is_uptown: if direction_id == DirectionType::Uptown {
                    true
                } else {
                    false
                },

                has_shape: shape_id.is_some(),
                shape_id: if let Some(id) = shape_id {
                    id
                } else {
                    String::new()
                },
            }
        } else {
            panic!("Wrong format for Trip: {:?}", value)
        }
    }
}

impl From<Route> for RouteRust {
    fn from(value: Route) -> Self {
        if let Route {
            route_id,
            route_short_name: Some(short_name),
            route_long_name: Some(long_name),
            route_desc: Some(desc),
            route_url: Some(url),
            route_color,
            route_text_color,
            ..
        } = value
        {
            Self {
                route_id,
                short_name,
                long_name,
                desc,
                url,

                has_color: route_color.is_some(),
                color: if let Some(color) = route_color {
                    color
                } else {
                    String::new()
                },

                has_text_color: route_text_color.is_some(),
                text_color: if let Some(text_color) = route_text_color {
                    text_color
                } else {
                    String::new()
                },
            }
        } else {
            panic!("Wrong format for Route: {:?}", value)
        }
    }
}

impl From<Service> for ServiceRust {
    fn from(value: Service) -> Self {
        let Service {
            service_id,
            monday,
            tuesday,
            wednesday,
            thursday,
            friday,
            saturday,
            sunday,
            start_date,
            end_date,
        } = value;
        Self {
            service_id,
            monday: monday == Activity::Active,
            tuesday: tuesday == Activity::Active,
            wednesday: wednesday == Activity::Active,
            thursday: thursday == Activity::Active,
            friday: friday == Activity::Active,
            saturday: saturday == Activity::Active,
            sunday: sunday == Activity::Active,
            start_date,
            end_date,
        }
    }
}

impl From<ServiceException> for ServiceExceptionRust {
    fn from(value: ServiceException) -> Self {
        let ServiceException {
            service_id,
            date,
            exception_type,
        } = value;
        Self {
            service_id,
            date,
            added: exception_type == ExceptionType::Added,
        }
    }
}

impl From<Transfer> for TransferRust {
    fn from(value: Transfer) -> Self {
        if let Transfer {
            transfer_type: TransferType::MinimumTime,
            from_stop_id: Some(from_stop_id),
            to_stop_id: Some(to_stop_id),
            min_transfer_time: Some(min_transfer_time),
            ..
        } = value
        {
            Self {
                from_stop_id,
                to_stop_id,
                min_transfer_time,
            }
        } else {
            panic!("Invalid format for transfer: {:?}", value)
        }
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
            stops: value
                .stops
                .into_iter()
                .map(Stop::into)
                .map(|s: StopRust| serde_json::to_string(&s).expect("Unable to parse stop"))
                .collect(),
            stop_times: value
                .stop_times
                .into_iter()
                .map(StopTime::into)
                .map(|s: StopTimeRust| {
                    serde_json::to_string(&s).expect("Unable to parse stop time")
                })
                .collect(),
            transfers: value
                .transfers
                .into_iter()
                .map(Transfer::into)
                .map(|s: TransferRust| serde_json::to_string(&s).expect("Unable to parse transfer"))
                .collect(),
            services: value
                .services
                .into_iter()
                .map(Service::into)
                .map(|s: ServiceRust| serde_json::to_string(&s).expect("Unable to parse service"))
                .collect(),
            service_exceptions: value
                .service_exceptions
                .into_iter()
                .map(ServiceException::into)
                .map(|s: ServiceExceptionRust| {
                    serde_json::to_string(&s).expect("Unable to parse service exception")
                })
                .collect(),
            routes: value
                .routes
                .into_iter()
                .map(Route::into)
                .map(|s: RouteRust| serde_json::to_string(&s).expect("Unable to parse route"))
                .collect(),
            trips: value
                .trips
                .into_iter()
                .map(Trip::into)
                .map(|s: TripRust| serde_json::to_string(&s).expect("Unable to parse trip"))
                .collect(),
        }
    }
}

macro_rules! setup_godot {
    ($self:ident, $schedule:ident, [$($id:ident),+]) => {
        $(
            $self.$id = $schedule
                .$id
                .iter()
                .map(|st| GString::from(st))
                .collect::<Vec<_>>()
                .as_slice()
                .into();
        )+
    };
}

#[godot_api]
impl ScheduleGodot {
    #[func]
    fn setup(&mut self, dir_path: GString) {
        let schedule: ScheduleRust =
            gtfs_parsing::schedule::Schedule::from_dir(dir_path.to_string(), true).into();

        setup_godot!(
            self,
            schedule,
            [
                shapes,
                trips,
                routes,
                stops,
                stop_times,
                services,
                service_exceptions,
                transfers
            ]
        );
    }
}
