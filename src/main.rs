use std::io::{self, stdout};
use crossterm::{event::{self, Event, KeyCode, KeyEvent}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use ratatui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, text::Line, widgets::List, Frame, Terminal};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen).expect("Alternate window");

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).expect("Could not create new terminal");
    let mut should_quit  = false;
    let mut position = (0, 0);
    let mut todos: Vec<String> = vec!["TODO".to_string(), "Teste".to_string()];
    let mut enable_add = false;
    let mut todo = "".to_string();

    while !should_quit {
        terminal.draw(|frame| ui(frame, &mut position, &todos, &enable_add, &todo)).expect("Drawing display frame");
        should_quit = handle_events(&mut position, &mut todos, &mut enable_add, &mut todo)?;
    };

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen).expect("Leave alterante window");
    Ok(())
}

fn ui(frame: &mut Frame, position: &mut (u16, u16), todos: &Vec<String>, enbale_add: &bool, todo: &String) {
    let list = List::new(todos.to_owned());
    
    let mut layout_len: u16 = 0;
    if *enbale_add == true {
        layout_len = 1
    }

    let main_layout = Layout::vertical([Constraint::Length(layout_len), Constraint::Length(todos.len().try_into().unwrap())]).split(frame.size());
    
    if *enbale_add == true {
        frame.render_widget(Line::raw(todo), main_layout[0]);
    }

    frame.set_cursor(position.0, position.1);
    frame.render_widget(list, main_layout[1])
}

        
fn handle_input(key: KeyEvent, enable_add: &mut bool, position: &mut (u16, u16), todo: &mut String, todos: &mut Vec<String>) {
    if key.kind == event::KeyEventKind::Press {
        match key.code {
            KeyCode::Char(c) => {
                todo.push(c);
                position.0 = position.0 + 1;
            }
            KeyCode::Backspace => {
                if position.0 == 0 {
                    return 
                }
                todo.pop();
                position.0 = position.0 - 1;
            }
            KeyCode::Esc => {
                *enable_add = false;
            }
            KeyCode::Enter => {
                *enable_add = false;
                todos.push(todo.to_string());
                *position = (0,0);
            }
            _ => {}
        }
    }
}

fn handle_nav(key: KeyEvent, position: &mut (u16, u16), enable_add: &mut bool) -> bool {
    if key.kind == event::KeyEventKind::Press {
        match key.code {
            KeyCode::Char('q') => {
                if *enable_add == true {
                    return false;
                }
                return true
            }
            KeyCode::Right => {
                position.0 = position.0 + 1;
            }
            KeyCode::Left => {
                if position.0 == 0 {
                    return false
                }
                position.0 = position.0 - 1;
            }
            KeyCode::Up => {
                if position.1 == 0 {
                    return false
                }
                position.1 = position.1 - 1;
            }
            KeyCode::Down => {
                position.1 = position.1 + 1;
            }
            KeyCode::Char('i') => {
                *position = (0,0);
                *enable_add = true;
            }
            KeyCode::Esc => {
                *enable_add = false;
            }
            _ => {}
        }
    }
    false
}

fn handle_events(position: &mut (u16, u16), todos: &mut Vec<String>, enable_add: &mut bool, todo: &mut String) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read().expect("Reading event") {
            if *enable_add == false {
                let result = handle_nav(key, position, enable_add);
                return Ok(result);
            }
            handle_input(key, enable_add, position, todo, todos);
        }
    }
    Ok(false)
}
