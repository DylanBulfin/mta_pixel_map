extends Object
class_name SubwayStopTime

@export var stop_id: String
@export var trip_id: String
@export var arrival_time: String
@export var departure_time: String
@export var stop_sequence: int

var stop: SubwayStop

static func from_dict(dict: Dictionary) -> SubwayStopTime:
	var res = SubwayStopTime.new()
	
	res.trip_id = dict["trip_id"]
	res.stop_id = dict["stop_id"]
	res.arrival_time = dict["arrival_time"]
	res.departure_time = dict["departure_time"]
	res.stop_sequence = dict["stop_sequence"]
	
	return res

func init_refs(schedule: SubwaySchedule) -> void:
	stop = schedule.stops[stop_id]
