extends RefCounted
class_name SubwayStop

@export var stop_id: String
@export var stop_name: String
@export var lat: float
@export var lon: float
@export var maybe_parent_station_id: String

var maybe_parent_station: SubwayStop
var child_stations: Array[SubwayStop]

var latlong: Vector2:
	get: return Vector2(lon, lat)

static func from_dict(dict: Dictionary) -> SubwayStop:
	var res = SubwayStop.new()
	
	res.stop_id = dict["stop_id"]
	res.stop_name = dict["name"]
	res.lat = dict["lat"]
	res.lon = dict["lon"]
	if dict["has_parent"]: res.maybe_parent_station_id = dict["parent_station_id"]
	
	return res

func init_refs(schedule: SubwaySchedule) -> void:
	child_stations.assign(schedule.stops.values().filter(
		func(s): return s.maybe_parent_station_id and s.maybe_parent_station_id == stop_id
	))
	
	if maybe_parent_station_id:
		self.maybe_parent_station = schedule.stops[maybe_parent_station_id]
