use piston_window::*;
use rand::Rng;

use crate::colors;
use crate::draw::*;
use crate::physics::{Direction, Position};
use crate::snake::Snake;

use std::fs;
use std::vec::Vec;

const FPS: f64 = 30.0;
//const RESTART_TIME: f64 = 1.0;

fn fps_in_ms(fps: f64) -> f64 {
    1.0 / fps
}

fn calc_random_pos(width: u32, height: u32) -> Position {
    let mut rng = rand::thread_rng();

    Position {
        x: rng.gen_range(0, width as i32),
        y: rng.gen_range(0, height as i32),
    }
}



pub struct Game {
    snake: Snake,
    fruit: Position,
    size: (u32, u32),
    waiting_time: f64,
    score: u32,
    over: bool,
    paused: bool,
    wrote: bool,
    supfruit: Position,
}

impl Game {
    /// this is the starting values of everything present in the game
    pub fn new(width: u32, height: u32) -> Self {
        // use fn defined at eof to calc random fruit / snake pos here
        Self {
            snake: Snake::new(calc_random_pos(width, height)),
            fruit: calc_random_pos(width, height),
            size: (width, height),
            waiting_time: 0.0,
            score: 0,
            over: false,
            paused: true,
            wrote: false,
            supfruit: calc_random_pos(width, height),
        }
    }

    /// starts the game back up if paused, can do so by pressing any key.
    pub fn start(&mut self) {
        self.paused = false;
    }

    /// pauses the game, can do so with the key R
    pub fn pause(&mut self) {
        self.paused = true;
    }
    
    /// changes the paused state to either resume or pause the game
     pub fn toggle_game_state(&mut self) {
         if self.paused {
             self.start();
         } else {
             self.pause();
         }
     }
    /// added code that prints the high scores and if the user gets a new high score it's added to the top five and the smallest score is deleted
    pub fn draw(&mut self, ctx: Context, g: &mut G2d) {
        draw_block(&ctx, g, colors::FRUIT, &self.fruit);
        self.snake.draw(&ctx, g);
        draw_block(&ctx, g, colors::SUPFRUIT, &self.supfruit);
        // draw_text(&ctx, g, colors::SCORE, self.score.to_string());

        if self.over {
            draw_overlay(&ctx, g, colors::OVERLAY, self.size);
            // have this if statement so it doesn't spam the the code over and over
            if self.wrote == false
            {
                
                // this opens the highscores.txt file and puts the numbers into a string which is then made into a vector of numbers
                let  scores = fs::read_to_string("highscores.txt").unwrap(); // reads the file and makes it into a string
                let mut scores_final = String::new(); 
                
                // this converts the string of scores into a vector of i32 and puts the score of this session in the vector.
                let mut score_list: Vec<u32> = scores.split_whitespace().filter_map(|i| i.parse().ok()).collect();
                score_list.push(self.score);
                
                // this sorts and then removes the last value which in this case is the sixth value since its supposed to be a top 5. the sort_by also makes it highest to lowest
                score_list.sort_by(|a, b| b.cmp(a));
                score_list.pop();
                //prints the top 5 scores
                println!("Top 5 Scores");
                println!("1. {:?}", score_list[0]);
                println!("2. {:?}", score_list[1]);
                println!("3. {:?}", score_list[2]);
                println!("4. {:?}", score_list[3]);
                println!("5. {:?}", score_list[4]);
                println!("------------------------------------------------------------------------------------");
                // this converts the array back into a string so it can be written into the highscores.txt file.
                for i in &score_list
                {
                    scores_final.push_str(&i.to_string());
                    scores_final.push_str(&format!("{}", " "));
                }
                // this writes the scores to the highscores.txt file and sets wrote to true.
                fs::write("highscores.txt", scores_final).expect("could not write to file");
                self.wrote = true;
            }
        }
    }
    
    /// Updates game information such as the snake length and score when picking up a fruit
    /// Added a blue fruit that gives more points and segments
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

       // if self.over {
       // if self.waiting_time > RESTART_TIME {
        //     self.restart();
        // }
        // return;
        // }

        if self.waiting_time > fps_in_ms(FPS) && !self.over && !self.paused {
            // self.check_colision() use snake.get_head_pos;
            self.waiting_time = 0.0;

            if !self.snake.is_tail_overlapping() && !self.snake.will_tail_overlapp() {
                self.snake.update(self.size.0, self.size.1);

                if *self.snake.get_head_pos() == self.fruit {
                    self.snake.grow();
                    self.snake.update(self.size.0, self.size.1);
                    self.fruit = calc_random_pos(self.size.0, self.size.1);
                    self.calc_score();
                }
                // fruit that gives more point and makes you get 3 more segments on the snake.
                if *self.snake.get_head_pos() == self.supfruit {     
                    self.snake.grow();
                    self.snake.grow();
                    self.snake.grow();
                    self.snake.update(self.size.0, self.size.1);
                    self.supfruit = calc_random_pos(self.size.0, self.size.1);
                    self.calc_score();
                }
            } else {
                self.over = true;
            }
        }
    }

    /// takes key presses from the user uses them to move the snake or pause and restart the game.
    pub fn key_down(&mut self, key: keyboard::Key) {
        

         match key {
             Key::R => self.restart(), // this now restarts the game
             Key::Space => self.toggle_game_state(),
             _ => self.start(),
         }

        match key {
            Key::A | Key::Left => self.snake.set_dir(Direction::Left),
            Key::W | Key::Up => self.snake.set_dir(Direction::Up),
            Key::D | Key::Right => self.snake.set_dir(Direction::Right),
            Key::S | Key::Down => self.snake.set_dir(Direction::Down),
            _ => {}
        }
    }

    /// returns the game score
    pub fn get_score(&self) -> u32 {
        self.score
    }

    /// calculates the score based on the length of the snake times 10
    fn calc_score(&mut self) {
        self.score = (self.snake.get_len() * 10) as u32
    }

    /// This function restarts the game 
    fn restart(&mut self)
    {
       *self = Game::new(self.size.0, self.size.0)
    }

     

    

    // IMPORTANT!! -

    // fn update_snake(&mut self, dir: Option<Direction>) {
    //     if self.check_if_snake_alive(dir) {
    //         self.snake.move_forward(dir);
    //         self.check_eating();
    //     } else {
    //         self.game_over = true;
    //     }
    //     self.waiting_time = 0.0;
    // }
}

// fn calc_not_overlapping_pos(pos_vec: Vec<Position>, width: u32, height: u32) {
//     let mut fruit_pos: Position = calc_random_pos(width, height);

//     loop {
//         // if snake_pos.y != fruit_pos.y {
//         //     break;
//         // }

//         for pos in pos_vec {
//             if
//         }

//         snake_pos = calc_random_pos(width, height);
//         fruit_pos = calc_random_pos(width, height);
//     }
// }