extends Object
class_name SubwayShape

@export var shape_id: String
@export var lats: Array[float]
@export var lons: Array[float]

static func from_dict(dict: Dictionary) -> SubwayShape:
	var res = SubwayShape.new()
	
	res.shape_id = dict["shape_id"]
	res.lats.assign(dict["lats"])
	res.lons.assign(dict["lons"])
	
	return res
