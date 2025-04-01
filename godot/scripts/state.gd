extends Node

var reference_points: Array[ReferencePoint]

func _ready() -> void:
	var data = preload("res://resources/data.tres")
	reference_points.assign(data.reference_points)
	
	# Calculate local scale for each point
	for point in reference_points:
		var local_scale_total := 0.0
		var divisor_total := 0.0
		
		for other in reference_points:
			if other == point: continue
			var ll_distance = point.latlong.distance_to(other.latlong)
			var pix_distance = point.ppos.distance_to(other.ppos)
			divisor_total += (1 / ll_distance)
			local_scale_total += (1 / ll_distance) * (pix_distance / ll_distance)
		
		point.local_scale = local_scale_total / divisor_total
