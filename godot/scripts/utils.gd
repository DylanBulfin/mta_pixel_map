extends Node
func estimate_pixel_pos(latlong: Vector2) -> Vector2i:
	var total_distance := 0.0
	var total := Vector2.ZERO
	
	for point in State.reference_points:
		var diff = latlong - point.latlong
		var dist = latlong.distance_squared_to(point.latlong)
		
		total +=  (1 / dist) * (point.ppos + point.local_scale * (Consts.X_NORM * diff.x + Consts.Y_NORM * diff.y))
		total_distance += (1 / dist)
		
	var res := total / total_distance
	
	return Vector2i(round(res.x), round(res.y))

func image_from_pposs(pposs: Array[Vector2i], width: int, height: int, color: Color) -> Image:
	var row_width := width * 4 # 4 bytes per pixel
	var data: PackedByteArray
	data.resize(width * height * 4)
	
	for ppos in pposs:
		if ppos.x < 0 or ppos.y < 0: continue
		var arr_pos: int = (ppos.y * row_width) + (ppos.x * 4)
		if arr_pos >= len(data): continue
		data[arr_pos] = color.r8
		data[arr_pos + 1] = color.g8
		data[arr_pos + 2] = color.b8
		data[arr_pos + 3] = color.a8
	
	return Image.create_from_data(width, height, false, Image.FORMAT_RGBA8, data)
