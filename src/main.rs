use rand::Rng;
use plotters::{prelude::*, style::full_palette::ORANGE};
use std::collections::HashSet;


#[derive(Debug)]
struct Board {
    car: usize,
    doors: Vec<i32>,
}

impl Board {
    fn new(car: usize, doors: Vec<i32>) -> Self {
        Board { car, doors }
    }
}
const MAX_VALUE: i32 = 10000;

fn main() {
   let total_win = play_no_change(); 
   let total_win_change = play_change(); 

   println!("total win when no change: {} ", total_win);
   println!("total win when change: {} ", total_win_change);
   draw(MAX_VALUE, total_win, total_win_change);
}
/**
 * 
 * Play MAX_VALUE times when participant will chose a random position out of 3 door and stick with the decision.
*/
fn play_no_change()-> i32{
    let mut count=0;
    for _number in 0..MAX_VALUE {
        let board = setup();
        let res = get_result_choose_but_no_change(board);
        if res==1{
            count+=1;
        }
    }
    return count;
}

/**
 * 
 * Play MAX_VALUE times when participant will chose a random position out of 3 door and change later with the call.
*/
fn play_change()-> i32{
    let mut count=0;
    for _number in 0..MAX_VALUE {
        let board = setup();
        let res = get_result_choose_but_change(board);
        if res==1{
            count+=1;
        }
    }
    return count;
}

/**
 * Participant will choose one random door and stick with it. 
 * If the selected door match with door where the car behind, participant wins or lose
*/
fn get_result_choose_but_no_change(board:Board)-> u8 {
    let mut rng = rand::thread_rng();

    // Generate a random index (position) between 0 and 2
    let random_position = rng.gen_range(0..3);
    // println!("Participant has selected: {}", random_position);
    if random_position==board.car {
        return 1;
    }
    return 0;
}

/**
 * Participant will choose one random door and chage his mind after monty shows the goat. 
 * If the selected door match with door where the car behind, participant wins or lose
*/
fn get_result_choose_but_change(board:Board)-> u8 {
    let mut rng = rand::thread_rng();
    // this is actually a invalid track, what participant choosen first time and 
    // what monty showed as goat
    let mut invalid = HashSet::new();

    // Participant selected a door randomly at the first time, it will be invalid since participant will change
    // his/her mind and monty can not select to show the goat
    let participant_initial_selection = rng.gen_range(0..3);
    invalid.insert(participant_initial_selection);
    

    // Monty will show door with a goat, monty can only choose if 
    // 1. A door was not choosen by participant
    // 2. And the door does not have car
    let mut door_for_monty = Vec::new();
    for i in 0..3{
        // monty can not choose what participant has already choosen
        // Or that door has a car behind
        if i!=participant_initial_selection && board.doors[i]==0 {
            door_for_monty.push(i);
        }
    }
    // If monty has two door left two choose because participant selected car, then monty choose random and marked to change
    // other wise return whatever one door left with goat

    if door_for_monty.len()==1 {
        invalid.insert(door_for_monty[0]);
    } else{
        // select a random with goat from the two door left for monty
        let monty = rng.gen_range(0..2);
        door_for_monty.remove(monty);
        invalid.insert(door_for_monty[0]);
    }

    let mut final_selection:usize=10;// take 10 as a protection to make sure code works fine
    for i in 0..3{
        if !invalid.contains(&i){
            final_selection= i;
        }
    }

    if final_selection==10{
        panic!()
    }
    if final_selection == board.car{
        return 1;
    }
    return 0;
}

/**
 * Set up the board
 * Three window respresented as a vector
 * Car is represented by '1' and goat is represented by '0'
 * Car has been kept in a random door
*/
fn setup() -> Board {
    let mut result = vec![0,0,0];
    let mut rng = rand::thread_rng();

    // Generate a random index (position) between 0 and 2
    let random_position = rng.gen_range(0..3);
    result[random_position] = 1;

    let board = Board::new(random_position, result);
    board
}

fn draw(yheight: i32, no_change: i32, change:i32){
    let root_area = BitMapBackend::new("output.png", (600, 400))
    .into_drawing_area();
    root_area.fill(&ORANGE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Result Comparison", ("sans-serif", 40))
        .build_cartesian_2d((0..2).into_segmented(), 0..yheight)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [no_change, change];

    ctx.draw_series((0..).zip(data.iter()).map(|(x, y)| {
        let x0 = SegmentValue::Exact(x);
        let x1 = SegmentValue::Exact(x + 1);
        let mut bar = Rectangle::new([(x0, 0), (x1, *y)], GREEN.filled());
        bar.set_margin(0, 0, 5, 5);
        bar
    }))
    .unwrap();

  
}