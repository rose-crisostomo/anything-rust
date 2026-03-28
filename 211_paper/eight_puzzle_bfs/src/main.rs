use std::collections::{HashSet, VecDeque};

mod tests;

#[derive(Clone, Copy)]
enum Actions {
    LEFT,
    RIGHT,
    DOWN,
    UP
}

type Board = [[u8; 3]; 3];
type Coordinates = (usize, usize);

#[derive(Clone, Copy)]
struct State {
    board: Board,
    cursor: Coordinates,
    action: Option<Actions>,
    parent: usize // index
}

fn main() {
    let board: Board = [[7, 2, 4], [5, 0, 6], [8, 3, 1]];
    let goal: Board = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];
    let cursor: Coordinates = (1, 1);
    let mut history: Vec<State> = vec![];
    // println!("Start position: {:?}", cursor);
    print_board(&board);

    let result = bfs_search(
        goal,
        State { board, cursor, action: None, parent: 0 },
        &mut history
    );
    match result {
        Ok(s) => {
            let mut states: Vec<State> = vec![];
            let mut curr = s;
            while curr.parent != 0 {
                // print_action(&curr.action);
                // print_board(&curr.board);
                states.push(curr);
                curr = history[curr.parent];
            }
            states.push(curr);
            // print_action(&curr.action);
            // print_board(&curr.board);

            states.reverse();
            for st in &states {
                print_action(&st.action);
                print_board(&st.board);
            }
            println!("Total moves: {}", states.len());
        }
        Err(e) => println!("{}", e),
    }


}

fn bfs_search(goal: Board, init_state: State, history: &mut Vec<State>) -> Result<State, Box<dyn std::error::Error>>{
    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(init_state);
    let mut visited: HashSet<Board> = HashSet::new();
    let mut iter = 0;
    while !queue.is_empty() {
        if let Some(s) = queue.pop_front() {
            // iter = (iter + 1) % 100; // for debugging, print every 100 steps
            // if iter == 0 {
            //     println!("Remaining in queue: {}",queue.len());
            // }

            let parent_index = history.len();
            history.push(s);

            // print_board(&s.board);
            // print_action(&s.action);
            let child_states = expand(s.board, s.cursor, parent_index);
            for cs in child_states {
                if is_goal_state(goal, cs.board) {
                    return Ok(cs);
                } else if !visited.insert(cs.board) {
                    continue;
                }
                queue.push_back(cs);
                // println!("Added child");
            }
        }

    }
    Err("Failed to find solution".into())
}

fn print_board(board: &Board) {
    for row in board {
        for col in row {
            print!("{} ", col)
        }
        println!();
    }
    println!("===");
}

fn print_action(action: &Option<Actions>) {
    if let Some(a) = action {
        match a {
            &Actions::LEFT => println!("left"),
            &Actions::RIGHT => println!("right"),
            &Actions::DOWN => println!("down"),
            &Actions::UP => println!("up"),
        }
    } else {
        println!("no action");
    }
}

fn expand(board: Board, cursor: Coordinates, parent_index: usize) -> Vec<State> {
    let mut child_states: Vec<State> = vec![];
    for action in [Actions::LEFT, Actions::RIGHT, Actions::DOWN, Actions::UP] {
        let child_state = move_cursor(board, cursor, action, parent_index);
        if let Some(child_state) = child_state  {
            child_states.push(child_state);
        }
    }
    child_states
}

fn move_cursor(board: Board, cursor: Coordinates, action: Actions, parent_index: usize) -> Option<State> {
    match action {
        Actions::LEFT => {
            if cursor.1 > 0 {
                let left: Coordinates = (cursor.0, cursor.1 - 1);
                let mut new_board = board;
                swap(&mut new_board, left, cursor);
                Option::from(State {
                    board: new_board,
                    cursor: left,
                    action: Option::from(action),
                    parent: parent_index
                })
            } else {
                None
            }
        }
        Actions::RIGHT => {
            if cursor.1 < board[0].len() - 1 {
                let right: Coordinates = (cursor.0, cursor.1 + 1);
                let mut new_board = board;
                swap(&mut new_board, right, cursor);
                Option::from(State {
                    board: new_board,
                    cursor: right,
                    action: Option::from(action),
                    parent: parent_index
                })
            } else {
                None
            }
        }
        Actions::DOWN => {
            if cursor.0 < board.len() - 1 {
                let down: Coordinates = (cursor.0 + 1, cursor.1);
                let mut new_board = board;
                swap(&mut new_board, down, cursor);
                Option::from(State {
                    board: new_board,
                    cursor: down,
                    action: Option::from(action),
                    parent: parent_index
                })
            } else {
                None
            }
        }
        Actions::UP => {
            if cursor.0 > 0 {
                let up: Coordinates = (cursor.0 - 1, cursor.1);
                let mut new_board = board;
                swap(&mut new_board, up, cursor);
                Option::from(State {
                    board: new_board,
                    cursor: up,
                    action: Option::from(action),
                    parent: parent_index
                })
            } else {
                None
            }
        }
    }
}

fn swap(board: &mut Board, a: Coordinates, b: Coordinates) {
    let a_value = board[a.0][a.1];
    board[a.0][a.1] = board[b.0][b.1];
    board[b.0][b.1] = a_value;
}

fn is_goal_state(goal: Board, board: Board) -> bool {
    goal == board
}