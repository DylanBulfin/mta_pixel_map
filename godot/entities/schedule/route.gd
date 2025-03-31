extends RefCounted
class_name SubwayRoute

@export var route_id: String
@export var short_name: String
@export var long_name: String
@export var desc: String
@export var url: String
@export var maybe_color: Color
@export var maybe_text_color: Color

var trips: Array[SubwayTrip]

func has_color() -> bool:
	return maybe_color != null

func has_text_color() -> bool:
	return maybe_text_color != null
