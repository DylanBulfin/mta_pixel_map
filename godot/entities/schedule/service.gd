extends Object
class_name SubwayService

@export var service_id: String
@export var monday: bool
@export var tuesday: bool
@export var wednesday: bool
@export var thursday: bool
@export var friday: bool
@export var saturday: bool
@export var sunday: bool
@export var start_date: String
@export var end_date: String

var exceptions: Array[SubwayServiceException]

static func from_dict(dict: Dictionary) -> SubwayService:
	var res = SubwayService.new()
	
	res.service_id = dict["service_id"]
	res.monday = dict["monday"]
	res.tuesday = dict["tuesday"]
	res.wednesday = dict["wednesday"]
	res.thursday = dict["thursday"]
	res.friday = dict["friday"]
	res.saturday = dict["saturday"]
	res.sunday = dict["sunday"]
	res.start_date = dict["start_date"]
	res.end_date = dict["end_date"]
	
	return res

func init_refs(schedule: SubwaySchedule) -> void:
	exceptions.assign(schedule.service_exceptions.filter(
		func(se): return se.service_id == service_id
	))
