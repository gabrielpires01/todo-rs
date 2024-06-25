use std::{io::{self, stdout}, u16};
use crossterm::{event::{self, Event, KeyCode, KeyEvent}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use ratatui::{backend::CrosstermBackend, layout::{Constraint, Layout}, style::{Color, Style}, text::Span, widgets::{Block, List, ListItem, Paragraph}, Frame, Terminal};

struct Position {
    x: u16,
    y: u16,

    max_y: u16,
}

impl Position {
    fn new(x: u16, y: u16, max_y: u16) -> Position{
       Position { x, y, max_y } 
    }

    fn change(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    fn change_max(&mut self, max_y: u16) {
        self.max_y = max_y;
    }

    fn up (&mut self){
        if self.y != 0 {
            self.y = self.y - 1;
        };
    }

    fn down (&mut self) {
        if self.max_y - 1 != self.y {
            self.y = self.y + 1;
        }
    }

    fn left (&mut self){
        if self.x != 0 {
            self.x = self.x - 1;
        };
    }

    fn right (&mut self) {
        self.x = self.x + 1;
    }
}

struct Item {
    id: String,
    done: bool,
}

enum Tabs {
    TODO,
    DONE
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen).expect("Alternate window");

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).expect("Could not create new terminal");
    let mut should_quit  = false;
    let mut todos: Vec<Item> = vec![];
    let mut enable_add = false;
    let mut todo = "".to_string();
    let mut position = Position::new(0,0, todos.len().try_into().unwrap());
    let _current_tab =  Tabs::TODO;

    while !should_quit {
        terminal.draw(|frame| ui(frame, &mut position, &todos, &enable_add, &todo)).expect("Drawing display frame");
        should_quit = handle_events(&mut position, &mut todos, &mut enable_add, &mut todo)?;
    };

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen).expect("Leave alterante window");
    Ok(())
}


fn ui(frame: &mut Frame, position: &mut Position, todos: &Vec<Item>, enable_add: &bool, todo: &String) {
    let active_style = Style::new().bg(Color::White).fg(Color::Black);
    let mut list_item: Vec<ListItem> = vec![];
    for (index, item) in todos.iter().enumerate() {
        if index == position.y.into() && *enable_add == false {
            let text = Span::raw(&item.id).style(active_style);
            list_item.push(ListItem::new(text));
            continue;
        }
        list_item.push(ListItem::new(&*item.id))
    }
    let list = List::new(list_item);

    let mut layout_len: u16 = 0;
    if *enable_add == true {
        frame.set_cursor(position.x, position.y);
        layout_len = 3
    }

    let main_layout = Layout::vertical([Constraint::Length(layout_len), Constraint::Length(todos.len().try_into().unwrap())]).split(frame.size());
    
    if *enable_add == true {
        frame.render_widget(Paragraph::new(todo.to_owned()).block(Block::bordered().title("Item :")), main_layout[0]);
    }

    frame.render_widget(list, main_layout[1])
}

        
fn handle_input(key: KeyEvent, enable_add: &mut bool, position: &mut Position, todo: &mut String, todos: &mut Vec<Item>) {
    if key.kind == event::KeyEventKind::Press {
        match key.code {
            KeyCode::Char(c) => {
                todo.push(c);
                position.right();
            }
            KeyCode::Backspace => {
                if position.x == 1 {
                    return 
                }
                todo.pop();
                position.left();
            }
            KeyCode::Esc => {
                *enable_add = false;
            }
            KeyCode::Enter => {
                *enable_add = false;
                todos.push(Item { id: todo.to_string(), done: false});
                position.change_max(todos.len().try_into().unwrap());
                position.change(0, 0);
                todo.clear();
            }
            _ => {}
        }
    }
}

fn handle_nav(key: KeyEvent, position: &mut Position, enable_add: &mut bool) -> bool {
    if key.kind == event::KeyEventKind::Press {
        match key.code {
            KeyCode::Char('q') => {
                if *enable_add == true {
                    return false;
                }
                return true
            }
            KeyCode::Up => {
                position.up();
            }
            KeyCode::Down => {
                position.down();
            }
            KeyCode::Char('i') => {
                position.change(1, 1);
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

fn handle_events(position: &mut Position, todos: &mut Vec<Item>, enable_add: &mut bool, todo: &mut String) -> io::Result<bool> {
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
