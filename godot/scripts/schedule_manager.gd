extends Node

var curr_schedule: SubwaySchedule

func _ready() -> void:
	var schedule_godot = ScheduleGodot.new()
	
	schedule_godot.setup("./test_data")
	set_current_schedule(schedule_godot)

func set_current_schedule(schedule: ScheduleGodot) -> void:
	# This assumes that schedule has been setup already
	curr_schedule = SubwaySchedule.new()
	
	for route_json: String in schedule.routes:
		var route: SubwayRoute = SubwayRoute.from_dict(JSON.parse_string(route_json))
		curr_schedule.routes[route.route_id] = route
	for trip_json: String in schedule.trips:
		var trip: SubwayTrip = SubwayTrip.from_dict(JSON.parse_string(trip_json))
		curr_schedule.trips[trip.trip_id] = trip
	for shape_json: String in schedule.shapes:
		var shape: SubwayShape = SubwayShape.from_dict(JSON.parse_string(shape_json))
		curr_schedule.shapes[shape.shape_id] = shape
	for stop_json: String in schedule.stops:
		var stop: SubwayStop = SubwayStop.from_dict(JSON.parse_string(stop_json))
		curr_schedule.stops[stop.stop_id] = stop
	for stop_time_json: String in schedule.stop_times:
		var stop_time: SubwayStopTime = SubwayStopTime.from_dict(JSON.parse_string(stop_time_json))
		if not curr_schedule.stop_times.has(stop_time.trip_id):
			curr_schedule.stop_times[stop_time.trip_id] = []
		curr_schedule.stop_times.get(stop_time.trip_id).append(stop_time)
	for service_json: String in schedule.services:
		var service: SubwayService = SubwayService.from_dict(JSON.parse_string(service_json))
		curr_schedule.services[service.service_id] = service
	for service_exception_json: String in schedule.service_exceptions:
		var service_exception: SubwayServiceException = SubwayServiceException.from_dict(JSON.parse_string(service_exception_json))
		curr_schedule.service_exceptions.append(service_exception)
	for transfer_json: String in schedule.transfers:
		var transfer: SubwayTransfer = SubwayTransfer.from_dict(JSON.parse_string(transfer_json))
		if not curr_schedule.transfers.has(transfer.from_stop_id):
			curr_schedule.transfers[transfer.from_stop_id] = []
		curr_schedule.transfers.get(transfer.from_stop_id).append(transfer)
		
		
	# TODO make this more efficient. Currently we end up iterating through every collection too many times
	for route: SubwayRoute in curr_schedule.routes.values():
		route.init_refs(curr_schedule)
	for service: SubwayService in curr_schedule.services.values():
		service.init_refs(curr_schedule)
	for service_exception: SubwayServiceException in curr_schedule.service_exceptions:
		service_exception.init_refs(curr_schedule)
	for stop: SubwayStop in curr_schedule.stops.values():
		stop.init_refs(curr_schedule)
	for stop_time_array: Array in curr_schedule.stop_times.values():
		for stop_time: SubwayStopTime in stop_time_array:
			stop_time.init_refs(curr_schedule)
	for transfer_array: Array in curr_schedule.transfers.values():
		for transfer: SubwayTransfer in transfer_array:
			transfer.init_refs(curr_schedule)
	for trip: SubwayTrip in curr_schedule.trips.values():
		trip.init_refs(curr_schedule)
		
	print(len(curr_schedule.trips.values()))
