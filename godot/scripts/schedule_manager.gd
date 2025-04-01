extends Node

var curr_schedule: SubwaySchedule

func _ready() -> void:
	var schedule_godot = ScheduleGodot.new()
	
	schedule_godot.setup("./test_data")
	update_curr_schedule(schedule_godot)

func update_curr_schedule(schedule: ScheduleGodot) -> void:
	# This assumes that schedule has been setup already
	var new_schedule = SubwaySchedule.new()
	
	var dd := Time.get_datetime_dict_from_system()
	var curr_date := ServiceDate.create_from_date(dd.year, dd.month, dd.day)
	
	for service_json: String in schedule.services:
		var service: SubwayService = SubwayService.from_dict(JSON.parse_string(service_json))
		if service.start_date <= curr_date.tomorrow and service.end_date >= curr_date.today:
			new_schedule.services[service.service_id] = service
	for service_exception_json: String in schedule.service_exceptions:
		var service_exception: SubwayServiceException = SubwayServiceException.from_dict(JSON.parse_string(service_exception_json))
		if service_exception.date >= curr_date.today and service_exception.date <= curr_date.tomorrow:
			if not new_schedule.service_exceptions.has(service_exception.service_id):
				new_schedule.service_exceptions[service_exception.service_id] = []
			new_schedule.service_exceptions[service_exception.service_id].append(service_exception)
			
			# Initialize refs for both service and service_exception here
			if new_schedule.services.has(service_exception.service_id):
				service_exception.maybe_service = new_schedule.services[service_exception.service_id]
				new_schedule.services[service_exception.service_id].exceptions.append(service_exception)
	# We initialize route refs in trips step
	for route_json: String in schedule.routes:
		var route: SubwayRoute = SubwayRoute.from_dict(JSON.parse_string(route_json))
		new_schedule.routes[route.route_id] = route
	# Shapes don't require refs
	for shape_json: String in schedule.shapes:
		var shape: SubwayShape = SubwayShape.from_dict(JSON.parse_string(shape_json))
		new_schedule.shapes[shape.shape_id] = shape
	# Stops can't be initialized until stops itself is populated
	for stop_json: String in schedule.stops:
		var stop: SubwayStop = SubwayStop.from_dict(JSON.parse_string(stop_json))
		new_schedule.stops[stop.stop_id] = stop
	for transfer_json: String in schedule.transfers:
		var transfer: SubwayTransfer = SubwayTransfer.from_dict(JSON.parse_string(transfer_json))
		if not new_schedule.transfers.has(transfer.from_stop_id):
			new_schedule.transfers[transfer.from_stop_id] = []
		new_schedule.transfers.get(transfer.from_stop_id).append(transfer)
		transfer.init_refs(new_schedule)
	for trip_json: String in schedule.trips:
		var trip: SubwayTrip = SubwayTrip.from_dict(JSON.parse_string(trip_json))
		if new_schedule.services.has(trip.service_id) or new_schedule.service_exceptions.has(trip.service_id):
			new_schedule.trips[trip.trip_id] = trip
			trip.init_refs(new_schedule)
	
			trip.route.trips.append(trip)
	for stop_time_json: String in schedule.stop_times:
		var stop_time: SubwayStopTime = SubwayStopTime.from_dict(JSON.parse_string(stop_time_json))
		if new_schedule.trips.has(stop_time.trip_id):
			if not new_schedule.stop_times.has(stop_time.trip_id):
				new_schedule.stop_times[stop_time.trip_id] = []
			new_schedule.stop_times.get(stop_time.trip_id).append(stop_time)
			stop_time.init_refs(new_schedule)
	
	for stop: SubwayStop in new_schedule.stops.values():
		if stop.maybe_parent_station_id:
			stop.maybe_parent_station = new_schedule.stops[stop.maybe_parent_station_id]
			stop.maybe_parent_station.child_stations.append(stop)
	
	print("Update complete")
	curr_schedule = new_schedule

# For safety
# TODO probably have to also free objects nested-ly
func _exit_tree() -> void:
	if curr_schedule: curr_schedule.free()
