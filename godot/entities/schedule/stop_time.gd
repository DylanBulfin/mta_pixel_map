extends RefCounted
class_name SubwayStopTime

@export var stop_id: String
@export var trip_id: String
@export var arrival_time: String
@export var departure_time: String
@export var stop_sequence: int

var stop: SubwayStop
