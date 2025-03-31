extends Object
class_name SubwaySchedule

var routes: Dictionary[String, SubwayRoute]
var trips: Dictionary[String, SubwayTrip]
var stops: Dictionary[String, SubwayStop]
var services: Dictionary[String, SubwayService]
var shapes: Dictionary[String, SubwayShape]

# Unique id is from_stop_id, Array[SubwayTransfer]
var transfers: Dictionary[String, Array]

# Unique id is trip_id, Array[SubwayStopTime]
var stop_times: Dictionary[String, Array]

# Doesn't have unique IDs, so no use for Dict
var service_exceptions: Array[SubwayServiceException]
