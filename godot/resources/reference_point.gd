extends Resource
class_name ReferencePoint

@export var lat: float
@export var long: float

var latlong: Vector2:
	get: return Vector2(long, lat)
	set(v):
		lat = v.y
		long = v.x

@export var px: float
@export var py: float

var ppos: Vector2:
	get: return Vector2(px, py)
	set(v):
		px = v.x
		py = v.y

var local_scale: float
