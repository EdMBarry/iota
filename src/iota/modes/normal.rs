use super::Mode;
use super::KeyMap;
use super::Key;
use super::Command;
use super::KeyMapState;
use super::Direction;
use super::WordEdgeMatch;
use super::OverlayType;

use command;


/// NormalMode mimics Vi's Normal mode.
pub struct NormalMode {
    keymap: KeyMap<Command>,
    builder: command::Builder,
}

impl NormalMode {

    /// Create a new instance of NormalMode
    pub fn new() -> NormalMode {
        NormalMode {
            keymap: NormalMode::key_defaults(),
            builder: command::Builder::new(),
        }
    }

    /// Creates a KeyMap with default NormalMode key bindings
    fn key_defaults() -> KeyMap<Command> {
        let mut keymap = KeyMap::new();

        // movement
        keymap.bind_key(Key::Char('W'), Command::MoveCursor(Direction::RightWord(WordEdgeMatch::Whitespace), 1));
        keymap.bind_key(Key::Char('B'), Command::MoveCursor(Direction::LeftWord(WordEdgeMatch::Whitespace), 1));
        keymap.bind_key(Key::Char('w'), Command::MoveCursor(Direction::RightWord(WordEdgeMatch::Alphabet), 1));
        keymap.bind_key(Key::Char('b'), Command::MoveCursor(Direction::LeftWord(WordEdgeMatch::Alphabet), 1));
        keymap.bind_key(Key::Char('G'), Command::MoveCursor(Direction::LastLine, 0));
        keymap.bind_keys(&[Key::Char('g'), Key::Char('g')], Command::MoveCursor(Direction::FirstLine, 0));

        // editing
        keymap.bind_keys(&[Key::Char('d'), Key::Char('W')], Command::Delete(Direction::RightWord(WordEdgeMatch::Whitespace), 1));
        keymap.bind_keys(&[Key::Char('d'), Key::Char('B')], Command::Delete(Direction::LeftWord(WordEdgeMatch::Whitespace), 1));
        keymap.bind_keys(&[Key::Char('d'), Key::Char('w')], Command::Delete(Direction::RightWord(WordEdgeMatch::Alphabet), 1));
        keymap.bind_keys(&[Key::Char('d'), Key::Char('b')], Command::Delete(Direction::LeftWord(WordEdgeMatch::Alphabet), 1));
        keymap.bind_key(Key::Char('x'), Command::Delete(Direction::Right, 1));
        keymap.bind_key(Key::Char('X'), Command::Delete(Direction::Left, 1));

        keymap.bind_key(Key::Char('u'), Command {
            number: 1,
            action: Action::Operation(Operation::Undo),
            object: None
        });
        keymap.bind_key(Key::Ctrl('r'), Command {
            number: 1,
            action: Action::Operation(Operation::Redo),
            object: None
        });

        keymap
    }

}

impl Mode for NormalMode {
    /// Given a key, pass it through the NormalMode KeyMap and return the associated Command, if any.
    fn handle_key_event(&mut self, key: Key) -> command::BuilderEvent {
        self.builder.check_key(key)
    }
}
