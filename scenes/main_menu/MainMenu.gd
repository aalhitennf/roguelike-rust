extends Control

func _on_NewGameButton_pressed() -> void:
	SceneSwitcher.load_scene("res://scenes/levels/rooms/LevelRooms.tscn")

func _on_LoadGameButton2_pressed() -> void:
	print_debug("Load game")

func _on_SettingsButton_pressed() -> void:
	print_debug("Settings")

func _on_ExitButton_pressed() -> void:
	get_tree().quit()
