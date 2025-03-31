extends Object
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

static func from_dict(dict: Dictionary) -> SubwayRoute:
	var res := SubwayRoute.new()
	res.route_id = dict["route_id"]
	res.short_name = dict["short_name"]
	res.long_name = dict["long_name"]
	res.desc = dict["desc"]
	res.url = dict["url"]
	
	if dict["has_color"]:
		res.maybe_color = Color(dict["color"])
	if dict["has_text_color"]:
		res.maybe_text_color = Color(dict["text_color"])
	
	return res

# Initialize references for this object
func init_refs(schedule: SubwaySchedule) -> void:
	trips.assign(schedule.trips.values().filter(
		func(t): return t.route_id == route_id
	))
