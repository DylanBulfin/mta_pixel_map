extends Object
class_name SubwayServiceException

@export var service_id: String
@export var date: String
@export var is_added: bool

var maybe_service: SubwayService

func has_service() -> bool:
	return maybe_service != null

static func from_dict(dict: Dictionary) -> SubwayServiceException:
	var res = SubwayServiceException.new()
	
	res.service_id = dict["service_id"]
	res.date = dict["date"]
	res.is_added = dict["added"]
	
	return res

func init_refs(schedule: SubwaySchedule) -> void:
	if schedule.services.has(service_id):
		maybe_service = schedule.services.get(service_id)
