extends Node

var num_closest: int = 10
var pixel_factor: int = 5

var background_image: Image = preload("res://NYC.png")

var schedule: ScheduleGodot

var reference_points: Array[ReferencePoint]
var shapes: Dictionary#[String, ShapeGodot]
var routes: Array[Route]
var stations: Dictionary#[String, StationGodot]

func _ready() -> void:
	var data = preload("res://resources/data.tres")
	routes.assign(data.routes)
	reference_points.assign(data.reference_points)
	
	# Calculate local scale for each point
	for point in reference_points:
		var closest = Utils.find_closest_points(point.latlong, 5)
		var local_scale_total := 0.0
		var divisor_total := 0.0
		
		for other in closest:
			if other == point: continue
			var dist:float = point.latlong.distance_to(other.latlong)
			divisor_total += (1 / dist)
		
		for other in closest:
			if other == point: continue
			var ll_distance = point.latlong.distance_to(other.latlong)
			var pix_distance = point.ppos.distance_to(other.ppos)
			local_scale_total += (1 / ll_distance) * (pix_distance / ll_distance)
		
		point.local_scale = local_scale_total / divisor_total

	schedule = ScheduleGodot.new()
	schedule.setup("./test_data")
	
	for shape_json in schedule.shapes:
		var shape = ShapeGodot.new()
		var json: Dictionary = JSON.parse_string(shape_json)
		for field: Dictionary in shape.get_property_list():
			var n = field.get("name")
			var u = field.get("usage")
			if n is String and u == PROPERTY_USAGE_NONE:
				if shape[n] is Array:
					# Force type conversions
					shape[n].assign(json[n])
				else:
					shape[n] = json[n]
		shapes[shape.shape_id] = shape

	for station_json in schedule.stops:
		var station = StopGodot.new()
		var json: Dictionary = JSON.parse_string(station_json)
		for field: Dictionary in station.get_property_list():
			var n = field.get("name")
			var u = field.get("usage")
			if n is String and u == PROPERTY_USAGE_NONE:
				if station[n] is Array:
					# Force type conversions
					station[n].assign(json[n])
				else:
					station[n] = json[n]
		stations[station.stop_id] = station
	
