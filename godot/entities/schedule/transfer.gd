extends Object
class_name SubwayTransfer

@export var from_stop_id: String
@export var to_stop_id: String
@export var min_transfer_time: int

var from_stop: SubwayStop
var to_stop: SubwayStop

static func from_dict(dict: Dictionary) -> SubwayTransfer:
	var res = SubwayTransfer.new()
	
	res.from_stop_id = dict["from_stop_id"]
	res.to_stop_id = dict["to_stop_id"]
	res.min_transfer_time = dict["min_transfer_time"]
	
	return res

func init_refs(schedule: SubwaySchedule) -> void:
	from_stop = schedule.stops[from_stop_id]
	to_stop = schedule.stops[to_stop_id]
