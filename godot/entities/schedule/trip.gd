extends RefCounted
class_name SubwayTrip

@export var trip_id: String
@export var service_id: String
@export var route_id: String
@export var trip_headsign: String
@export var direction_id: int
@export var shape_id: String

var route: SubwayRoute
var maybe_service: SubwayService
var service_exceptions: SubwayServiceException
var maybe_shape: SubwayShape
