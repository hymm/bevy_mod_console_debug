use std::io::Stdout;

use bevy::prelude::{Plugin, Resource};
use ratatui::{prelude::CrosstermBackend, Terminal};

struct BevyConsoleDebugPlugin;

impl Plugin for BevyConsoleDebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        app.insert_resource(DebugTerminal {
            terminal
        });
    }
}

#[derive(Resource)]
struct DebugTerminal {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
