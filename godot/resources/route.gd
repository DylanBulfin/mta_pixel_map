extends Resource
class_name Route

@export var name: String
@export var default_shape: String
@export var alternate_shapes: Array[String]
@export var color: Color

var image: Image

var default_shape_full_name: String:
	get: return str(name, "..", default_shape)
