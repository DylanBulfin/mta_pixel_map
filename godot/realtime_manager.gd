extends Node

var requests: Array[RealtimeRequest]
var since_last_update: float = 0.0

func _ready() -> void:
	for id in ["ace", "bdfm", "1234567s", "l", "jz", "nqrw", "g"]:
		var request = RealtimeRequest.new()
		request.line_id = id
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
