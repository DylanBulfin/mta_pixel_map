extends Node

var requests: Array[RealtimeRequest]
var since_last_update: float = 30.0

var rects: Dictionary#[String, TextureRect]

func _ready() -> void:
	for id in ["ace", "bdfm", "1234567s", "l", "jz", "nqrw", "g"]:
		var request = RealtimeRequest.new()
		request.line_id = id
		
		var rect = TextureRect.new()
		%RealtimeRect.add_child(rect)
		rects[id] = rect
		
		requests.append(request)
		self.add_child(request)
		
		request.request_completed.connect(
			func(a, b, c, d): _on_line_request_completed(id, a, b, c, d))

func _process(delta: float) -> void:
	since_last_update += delta
	
	if since_last_update >= 30:
		print("Updating")
		since_last_update = 0.0
		for request: RealtimeRequest in requests:
			print(request.url)
			var error = request.request(request.url)

func _on_line_request_completed(id: String, result, response_code, headers, body) -> void:
	print("Request complete: ", len(body))
	var update := RealtimeUpdateGodot.new()
	update.setup(body)
	
	var image = Image.create_empty(State.background_image.get_width(), State.background_image.get_height(), false, Image.FORMAT_RGBA8)
	
	for vehicle_json in update.vehicles:
		var json: Dictionary = JSON.parse_string(vehicle_json)
		var vehicle: VehicleGodot = VehicleGodot.new()
		
		vehicle.stop_id = json["stop_id"]
		vehicle.status = json["status"] as int
		
		if vehicle.status == 1: # Stopped at
			var station: StationGodot = State.stations.get(vehicle.stop_id)
			if not station: continue
			var pos := Vector2(station.lon, station.lat)
			
			var ppos := Utils.estimate_pixel_pos(pos)
			print("Station", ppos)
			
			Utils.draw_megapixel(ppos, image, Color.HOT_PINK)
		
		var rect = rects.get(id)
		var texture = ImageTexture.create_from_image(image)
		rect.texture = texture
