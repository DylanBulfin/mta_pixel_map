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
