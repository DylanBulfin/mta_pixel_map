extends HTTPRequest
class_name RealtimeRequest

@export var line_id: String
var url: String:
	get: return "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs"\
			+ ("" if line_id == "1234567s" else str("-", line_id))
