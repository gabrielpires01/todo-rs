use std::{fs::OpenOptions, io::{self, stdout, Read, Write}};
use crossterm::{event::{self, Event, KeyCode, KeyEvent}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use ratatui::{backend::CrosstermBackend, layout::{Constraint, Layout, Rect}, style::{Color, Style}, text::Span, widgets::{Block, List, ListItem, Paragraph, Tabs}, Frame, Terminal};

struct Position {
    x: usize,
    y: usize,

    max_y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position{
       Position { x, y, max_y: 1 } 
    }

    fn change(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    fn change_max(&mut self, max_y: usize) {
        self.max_y = max_y;
    }

    fn up(&mut self){
        if self.y != 1 {
            self.y = self.y - 1;
        };
    }

    fn down(&mut self) {
        if self.max_y != self.y {
            self.y = self.y + 1;
        }
    }

    fn left(&mut self){
        if self.x != 0 {
            self.x = self.x - 1;
        };
    }

    fn right(&mut self) {
        self.x = self.x + 1;
    }
}

#[derive(Clone)]
struct Item {
    id: String,
    state: TabsState,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum TabsState {
    TODO,
    DONE
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen).expect("Alternate window");
    
    let path = "example_todo.txt";
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).expect("Could not create new terminal");
    let mut should_quit  = false;
    let mut enable_add = false;
    let mut todo = "".to_string();
    let mut position = Position::new(0,1);
    let mut current_tab =  TabsState::TODO;
    let mut todos = read_file(path);
    let tabs = ["TODO", "DONE"];

    while !should_quit {
        let mut filtered_todos = handle_todos(todos.clone(), &current_tab);
        position.change_max(filtered_todos.len() + 1);
        terminal.draw(|frame| ui(frame, &mut position, &filtered_todos, &enable_add, &todo, tabs, &current_tab)).expect("Drawing display frame");
        should_quit = handle_events(&mut position, &mut todos, &mut filtered_todos, &mut enable_add, &mut todo, &mut current_tab)?;
    };

    let mut s = "".to_string();
    for todo in todos.iter() {
        let state_str = match todo.state {
            TabsState::TODO => "TODO",
            TabsState::DONE => "DONE"
        };
        s.push_str(&format!("{}:{}\n", state_str, &todo.id));
    }

    let mut file = OpenOptions::new().truncate(true).write(true).open(path).expect("Truncate to write file");
    file.write(s.as_bytes()).expect("Write to file");
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen).expect("Leave alternate window");
    Ok(())
}

fn read_file(path: &str) -> Vec<Item> {
    let mut read_file = OpenOptions::new().create(true).read(true).write(true).open(path).expect("Read or create file");
    let mut contents = String::new();
    read_file.read_to_string(&mut contents).expect("Should have been able to read the file");
    
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut todos: Vec<Item> = vec![];
    for line in lines.iter() {
        let values: Vec<&str> = line.split(":").collect();
        if values.len() == 2 {
            let state_str = values[0];
            let id = values[1];

            let state;
            if state_str == "DONE" {
                state = TabsState::DONE;
            } else if state_str == "TODO" {
                state = TabsState::TODO;
            } else {
                panic!("Only accepts DONE or TODO state")
            };

            todos.push(Item { id: id.to_owned(), state });
        }
    };

    todos
}

fn handle_todos(todos: Vec<Item>, current_tab: &TabsState) -> Vec<Item> {
    let mut filter_todo: Vec<Item> = vec![];
    for todo in todos {
        if *current_tab == todo.state {
            filter_todo.push(todo)
        }
    }
    filter_todo
}

fn ui(frame: &mut Frame, position: &mut Position, todos: &Vec<Item>, enable_add: &bool, todo: &String, tabs: [&str;2], current_tab: &TabsState) {
    let main_layout = Layout::vertical([Constraint::Length(1), Constraint::Length(todos.len().try_into().unwrap())]).split(frame.size());
    let active_style = Style::new().bg(Color::White).fg(Color::Black);
    let active_tab_index: usize  = match current_tab {
        TabsState::TODO => 0,
        TabsState::DONE => 1
    };
    let tabs_component = Tabs::new(tabs).select(active_tab_index);

    let mut list_item: Vec<ListItem> = vec![];

    for (index, item) in todos.iter().enumerate() {
        if index + 1 == position.y && *enable_add == false {
            let text = Span::raw(&item.id).style(active_style);

            list_item.push(ListItem::new(text));
            continue;
        }
        list_item.push(ListItem::new(&*item.id))
    }
    let list = List::new(list_item);


    
    if *enable_add == true {
        frame.set_cursor(position.x.try_into().unwrap(), position.y.try_into().unwrap());
        frame.render_widget(Paragraph::new(todo.to_owned()).block(Block::bordered().title("Item :")), Rect::new(0, 0, frame.size().width, 3));
        return
    }

    frame.render_widget(tabs_component, main_layout[0]);
    frame.render_widget(list, main_layout[1]);
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
                todos.push(Item { id: todo.to_string(), state: TabsState::TODO});
                position.change(0, 1);
                todo.clear();
            }
            _ => {}
        }
    }
}

fn handle_nav(key: KeyEvent, position: &mut Position, enable_add: &mut bool, current_tab: &mut TabsState, filtered_todos: &mut Vec<Item>, todos: &mut Vec<Item>) -> bool {
    if key.kind == event::KeyEventKind::Press {
        match key.code {
            KeyCode::Char('q') => {
                if *enable_add == true {
                    return false;
                }
                return true
            }
            KeyCode::Tab => {
                match current_tab {
                    TabsState::TODO => *current_tab = TabsState::DONE,
                    TabsState::DONE => *current_tab = TabsState::TODO
                }
                position.change(0, 1);
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
            KeyCode::Enter => {
                if position.y > 0  && position.y < todos.len() + 1 {
                    for (index, item) in filtered_todos.iter().enumerate() {
                        if index + 1 == position.y {
                            let find = todos.iter_mut().find(|x| x.id == item.id);
                            match find {
                                Some(item) => {
                                    match current_tab {
                                        TabsState::TODO => item.state = TabsState::DONE,
                                        TabsState::DONE => item.state = TabsState::TODO
                                    }
                                }
                                None => {
                                    panic!("Could not find item")   
                                }
                            }
                            break
                        };
                    }
                }
            }
            _ => {}
        }
    }
    false
}

fn handle_events(position: &mut Position, todos: &mut Vec<Item>, filtered_todos: &mut Vec<Item>,enable_add: &mut bool, todo: &mut String, current_tab: &mut TabsState) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read().expect("Reading event") {
            if *enable_add == false {
                let result = handle_nav(key, position, enable_add, current_tab, filtered_todos, todos);
                return Ok(result);
            }
            handle_input(key, enable_add, position, todo, todos);
        }
    }
    Ok(false)
}
