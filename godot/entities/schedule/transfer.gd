extends RefCounted
class_name SubwayTransfer

@export var from_stop_id: String
@export var to_stop_id: String
@export var minimum_transfer_time: int

var from_stop: SubwayStop
var to_stop: SubwayStop
