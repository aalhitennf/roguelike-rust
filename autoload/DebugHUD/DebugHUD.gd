extends CanvasLayer

var show_fps = false

onready var FPS_name = $CContainer/HContainer/Name/FPS
onready var FPS_value = $CContainer/HContainer/Value/FPS

#func _ready() -> void:
#	Console.register_command("show_fps", "Show frames per second", funcref(self, "toggle_fps"), [])

func toggle_fps() -> void:
	show_fps = !show_fps
	FPS_value.toggle_timer()
	FPS_value.visible = show_fps
	FPS_name.visible = show_fps
