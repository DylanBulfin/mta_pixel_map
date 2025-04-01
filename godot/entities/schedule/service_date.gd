extends Object
class_name ServiceDate

var yesterday: String
var today: String
var tomorrow: String

static func create_from_date(year: int, month: int, day: int) -> ServiceDate:
	var date := ServiceDate.new()
	var parts = [year, month, day]

	date.today = str("%04d" % parts[0], "%02d" % parts[1], "%02d" % parts[2])
	
	if day > 1:
		parts = [year, month, day - 1]
	elif month > 1:
		parts = [year, month - 1, Utils.days_in_month(month - 1, year)]
	else:
		parts = [year - 1, 12, 31]
	
	date.yesterday = str("%04d" % parts[0], "%02d" % parts[1], "%02d" % parts[2])
	
	if day < Utils.days_in_month(month, year):
		parts = [year, month, day + 1]
	elif month < 12:
		parts = [year, month + 1, 1]
	else:
		parts = [year + 1, 1, 1]
	
	date.tomorrow = str("%04d" % parts[0], "%02d" % parts[1], "%02d" % parts[2])
	
	return date

func check_relevant(date: String) -> bool:
	return date == yesterday or date == today or date == tomorrow
