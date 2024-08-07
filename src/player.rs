use crate::{
    frame::{Drawable, Frame},
    {NUM_COLS, NUM_ROWS},
    shot::Shot,
};
use std::time::Duration;

pub struct Player{
    x: usize,
    y: usize,
    //prev_x: usize,
    //prev_y: usize,
    shots: Vec<Shot>,
}

impl Player{
    pub fn new() -> Self{
        Self{
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            //prev_x: NUM_COLS / 2,
            //prev_y: NUM_ROWS - 1,
            shots: Vec::new(),
        }

    }

    pub fn move_left(&mut self){
        //self.prev_x = self.x;
        //self.prev_y = self.y;
        if self.x > 0 {self.x-=1;}
    }

    pub fn move_right(&mut self){
        //self.prev_x = self.x;
        //self.prev_y = self.y;
        if self.x < NUM_COLS - 1 {self.x+=1;}
    }

    pub fn shoot(&mut self) -> bool{
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y -1));
            true
        }
        else{false}
    }

    pub fn update(&mut self, delta: Duration){
        for shot in self.shots.iter_mut(){
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }
}

impl Drawable for Player{
    fn draw(&self, frame: &mut Frame){
        frame[self.x][self.y] = "A";
        // Clear the previous position
        /*if self.prev_x < NUM_COLS && self.prev_y < NUM_ROWS {
            frame[self.prev_y][self.prev_x] = " ";
        }
        // Draw the new position
        if self.x < NUM_COLS && self.y < NUM_ROWS {
            frame[self.y][self.x] = "A";
        }*/
        for shot in self.shots.iter(){
            shot.draw(frame);
        }
    }
}