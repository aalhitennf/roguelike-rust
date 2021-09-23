extends Node2D

func _ready():
	#Console.register_command("new", "create new level", funcref(self, "initialize_static"), [])
	#Console.register_command("mask", "reapply bittmask", funcref(self, "mask"), [])
	#Console.register_command("clear", "clear tilemap", funcref(self, "clear"), [])
	SignalManager.connect("initialize_static", self, "initialize_static")

func initialize_static(type: int = 0, width: int = 50, height: int = 50) -> void:
	var time_before = OS.get_ticks_msec()
	var success = $Layers/Static.initialize(type, width, height)
	var total_time = OS.get_ticks_msec() - time_before
	print("Map generation status: ", success, " in " + str(total_time) + " ms")
	var player_spawn_point = $Layers/Static.get_player_spawnpoint()
	$Player.position = player_spawn_point * 16


func mask() -> void:
	$Layers/Static.mask();

func clear() -> void:
	$Layers/Static.clear();
