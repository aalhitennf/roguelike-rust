use gdnative::{api::{CanvasLayer, MarginContainer, LineEdit, TextEdit}, prelude::*};

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
pub struct Console {
    main_node: Option<TRef<'static, MarginContainer>>,
    input_node: Option<TRef<'static, LineEdit>>,
    output_node: Option<TRef<'static, TextEdit>>,
    history: Vec<GodotString>,
    current_line: usize,
}

#[methods]
impl Console {
    fn new(_owner: &CanvasLayer) -> Self {
        Console {
            main_node: None,
            input_node: None,
            output_node: None,
            history: vec![GodotString::from("")],
            current_line: 0,
        }
    }
    #[export]
    fn _ready(&mut self, owner: &CanvasLayer) {
        self.main_node = unsafe { owner.get_node_as::<MarginContainer>("Container") };

        self.input_node =
            unsafe { owner.get_node_as::<LineEdit>("Container/Background/Items/Input") };

        self.output_node =
            unsafe { owner.get_node_as::<TextEdit>("Container/Background/Items/Output") };


        self.current_line = self.history.len();

        if self.main_node.is_none() {
            godot_print!("[libconsole::Console] _ready failed: no main node");
        }
        if self.input_node.is_none() {
            godot_print!("[libconsole::Console] _ready failed: no input node");
        }
        if self.output_node.is_none() {
            godot_print!("[libconsole::Console] _ready failed: no output node");
        }
    }
    #[export]
    fn _input(&mut self, _owner: &CanvasLayer, _event: Ref<InputEvent>) {
        let input = Input::godot_singleton();

        if Input::is_action_just_pressed(input, "toggle_console") {
            self.toggle();
        }

        if Input::is_action_just_pressed(input, "move_up") {
            self.goto_history(-1);
        }

        if Input::is_action_just_pressed(input, "move_down") {
            self.goto_history(1);
        }
    }
    #[export]
    fn _on_input_text_entered(&mut self, _owner: &CanvasLayer, text: GodotString) {
        self.history.push(text.clone());
        self.current_line = self.history.len();
        self.write_line(_owner, text);
    }
    #[export]
    fn _on_input_focus_exited(&mut self, _owner: &CanvasLayer) {
        if let Some(line_edit) = self.input_node {
            line_edit.grab_focus();
        };
    }
    #[export]
    fn write_line(&self, _owner: &CanvasLayer, text: GodotString) {
        if let Some(text_edit) = self.output_node {
            let new_text = text_edit.text() + text + GodotString::from("\n");
            text_edit.set_text(new_text);
            text_edit.set_v_scroll(999999999f64);
        }
        self.clear_input();
    }
}

impl Console {
    fn toggle(&self) {
        if let Some(splitter) = self.main_node {
            let visible = !splitter.is_visible();
            splitter.set_visible(visible);

            if visible {
                if let Some(line_edit) = self.input_node {
                    line_edit.grab_focus();
                    line_edit.clear();
                }
            }
        }
    }
    fn clear_input(&self) {
        if let Some(line_edit) = self.input_node {
            line_edit.clear();
        };
    }
    fn set_input_text(&self, text: GodotString) {
        if let Some(line_edit) = self.input_node {
            line_edit.set_text(text);
        };
    }
    fn goto_history(&mut self, offset: i8) {
        // New line index
        let new_line = self.current_line as i8 + offset;

        // Dont loop history going down
        if new_line == -1 {
            return;
        }

        // Dont loop history going up
        if new_line > self.history.len() as i8 - 1 {
            self.set_input_text(GodotString::from(""));
            return;
        }

        self.current_line = new_line as usize;

        if let Some(line) = self.history.get(self.current_line) {
            self.set_input_text(line.clone());
        } else {
            godot_print!(
                "[libconsole::Console] goto_history: no history line {}",
                self.current_line
            );
        }
    }
}
