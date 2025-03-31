extends Object
class_name SubwayTrip

@export var trip_id: String
@export var service_id: String
@export var route_id: String
@export var trip_headsign: String
@export var is_uptown: bool
@export var maybe_shape_id: String

var route: SubwayRoute
var maybe_service: SubwayService
var service_exceptions: Array[SubwayServiceException]
var maybe_shape: SubwayShape

static func from_dict(dict: Dictionary) -> SubwayTrip:
	var res = SubwayTrip.new()
	
	res.route_id = dict["route_id"]
	res.service_id = dict["service_id"]
	res.trip_id = dict["trip_id"]
	res.trip_headsign = dict["trip_headsign"]
	res.is_uptown = dict["is_uptown"]
	
	if dict["has_shape"]: res.maybe_shape_id = dict["shape_id"]
	
	return res

func init_refs(schedule: SubwaySchedule) -> void:
	route = schedule.routes[route_id]
	
	if schedule.services.has(service_id):
		# Represented by a real service
		maybe_service = schedule.services[service_id]
	
	service_exceptions.assign(schedule.service_exceptions.filter(
		func(e): return e.service_id == service_id
	))
	
	if maybe_shape_id:
		maybe_shape = schedule.shapes[maybe_shape_id]
