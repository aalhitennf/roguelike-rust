extends CanvasLayer

onready var width_slider = $Container/Items/WidthSlider
onready var width_text = $Container/Items/WidthValue
onready var height_slider = $Container/Items/HeightSlider
onready var height_text = $Container/Items/HeightValue
onready var type_option = $Container/Items/MapType

var type = 0;
var width: int = 50;
var height: int = 50;

func _ready():
	type_option.add_item("SquareRooms", 0);
	type_option.add_item("CirceRooms", 1);
	type_option.add_item("MixedRooms", 2);


func _on_Create_pressed():
	SignalManager.emit_signal("initialize_static", type, width, height)


func _on_WidthSlider_value_changed(value):
	width = int(value)
	width_text.text = str(width)


func _on_HeightSlider_value_changed(value):
	height = int(value)
	height_text.text = str(height)


func _on_MapType_item_selected(index):
	type = index
