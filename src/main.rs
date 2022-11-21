#![warn(clippy::all, clippy::pedantic)]

mod editor;
mod terminal;

use editor::Editor;
pub use editor::Position;
pub use terminal::Terminal;

// REPRENDRE À https://www.philippflenker.com/hecto-chapter-3/
// voir partie 'Clear the screen'
fn main() {
    Editor::default().run();
}
