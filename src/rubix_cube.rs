//Run with ' cargo run '

use rand::Rng;
use std::fmt::{self, Debug};
use std::time::Instant;
use rayon::prelude::*;

static MOVED_BY_3_UP : [u8; 9] =   [3, 4, 5, 9, 10, 11, 15, 16, 17];
static MOVED_BY_3_DOWN : [u8; 9] = [0, 1, 2, 6,  7,  8, 12, 13, 14];

#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub enum Colour{
    White = 0,
    Yellow = 1,
    Red = 2,
    Green = 3,
    Orange = 4,
    Blue = 5
}

impl Debug for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::White => write!(f, "W"),
            Self::Yellow => write!(f, "Y"),
            Self::Red => write!(f, "R"),
            Self::Green => write!(f, "G"),
            Self::Orange => write!(f, "O"),
            Self::Blue => write!(f, "B"),
        }
    }
}
impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::White => write!(f, "W"),
            Self::Yellow => write!(f, "Y"),
            Self::Red => write!(f, "R"),
            Self::Green => write!(f, "G"),
            Self::Orange => write!(f, "O"),
            Self::Blue => write!(f, "B"),
        }
    }
}

#[derive(Clone)]
pub struct RubixCube{
    faces : [[Colour; 9]; 6],
    past_moves : Vec<u8>,
    solving_route : Vec<u8>
}

impl RubixCube{
    pub fn get_faces(&self) ->  [[Colour; 9]; 6]{return self.faces;}
    pub fn get_past_moves(&self) ->  Vec<u8>{return self.past_moves.clone();}
    pub fn get_solving_route(&self) -> Vec<u8>{return self.solving_route.clone();}
    //    F1
    // F2 F3 F4
    //    F5
    //    F6

    // Vertical shifting -> affects 1, 3, 5, 6
    // Horizontal Shifting -> affects 2, 3, 4, 6 
    // Operates looking at face 3
    pub fn create_solved_rubix() -> RubixCube
    {
        let face3 = [Colour::White, Colour::White, Colour::White, Colour::White, Colour::White, Colour::White, Colour::White, Colour::White, Colour::White];
        let face6 = [Colour::Yellow, Colour::Yellow, Colour::Yellow, Colour::Yellow, Colour::Yellow, Colour::Yellow, Colour::Yellow, Colour::Yellow, Colour::Yellow];

        let face2 = [Colour::Red, Colour::Red, Colour::Red, Colour::Red, Colour::Red, Colour::Red, Colour::Red, Colour::Red, Colour::Red];
        let face4 = [Colour::Orange, Colour::Orange, Colour::Orange, Colour::Orange, Colour::Orange, Colour::Orange, Colour::Orange, Colour::Orange, Colour::Orange];

        let face1 = [Colour::Blue, Colour::Blue, Colour::Blue, Colour::Blue, Colour::Blue, Colour::Blue, Colour::Blue, Colour::Blue, Colour::Blue];
        let face5 = [Colour::Green, Colour::Green, Colour::Green, Colour::Green, Colour::Green, Colour::Green, Colour::Green, Colour::Green, Colour::Green];
        return RubixCube{faces : [face1, face2, face3, face4, face5, face6], past_moves : Vec::new(), solving_route : Vec::new()}
    }
    
    pub fn create_custom_rubix(f1 :  &[Colour; 9], f2 : &[Colour; 9], f3 : &[Colour; 9], f4 : &[Colour; 9], f5 : &[Colour; 9], f6 : &[Colour; 9]) -> RubixCube
    {
        return RubixCube{faces : [*f1, *f2, *f3, *f4, *f5, *f6], past_moves : Vec::new(), solving_route : Vec::new()}
    }
    
    pub fn print_cube(&self) 
    {
        print!("\n       {} {} {}\n       {} {} {}\n       {} {} {}\n\n", &self.faces[0][0], &self.faces[0][1], &self.faces[0][2], &self.faces[0][3], &self.faces[0][4], &self.faces[0][5], &self.faces[0][6], &self.faces[0][7], &self.faces[0][8]);

        print!("{} {} {}  {} {} {}  {} {} {}\n", &self.faces[1][0], &self.faces[1][1], &self.faces[1][2], &self.faces[2][0], &self.faces[2][1], &self.faces[2][2], &self.faces[3][0], &self.faces[3][1], &self.faces[3][2]);
        print!("{} {} {}  {} {} {}  {} {} {}\n", &self.faces[1][3], &self.faces[1][4], &self.faces[1][5], &self.faces[2][3], &self.faces[2][4], &self.faces[2][5], &self.faces[3][3], &self.faces[3][4], &self.faces[3][5]);
        print!("{} {} {}  {} {} {}  {} {} {}\n\n", &self.faces[1][6], &self.faces[1][7], &self.faces[1][8], &self.faces[2][6], &self.faces[2][7], &self.faces[2][8], &self.faces[3][6], &self.faces[3][7], &self.faces[3][8]);

        print!("       {} {} {}\n       {} {} {}\n       {} {} {}\n\n", &self.faces[4][0], &self.faces[4][1], &self.faces[4][2], &self.faces[4][3], &self.faces[4][4], &self.faces[4][5], &self.faces[4][6], &self.faces[4][7], &self.faces[4][8]);
        print!("       {} {} {}\n       {} {} {}\n       {} {} {}\n\n", &self.faces[5][0], &self.faces[5][1], &self.faces[5][2], &self.faces[5][3], &self.faces[5][4], &self.faces[5][5], &self.faces[5][6], &self.faces[5][7], &self.faces[5][8]);
        println!("");
        for val in &self.past_moves{
            print!("{} ", val);
        }
        print!("\n");
    }

    pub fn turn_cube(&mut self, &col_row: &u8)
    {
        // Col_row : which row/column is moving
        // turn vertically: if true, trun vertical direction else horizontal direction
        // right_down : if true, turn right or down (dependant on turn_vertical)

        // 0. > 9 10 11 < 3    12  13  14
        // 1. > v v  v  < 4    v^  v^  v^
        // 2. >         < 5    15  16  17
        //      ^  ^  ^
        //      6  7  8

        //    F1    - top
        // F2 F3 F4
        //    F5    - bottom
        //    F6    - rear

        // Vertical shifting   -> affects 1, 3, 5, 6  - side faces: 2, 4
        // Horizontal Shifting -> affects 2, 3, 4, 6  - side faces: 1, 5
        // backside shifting   -> affects 1, 2, 5, 4  - side faces: 3, 6


        //Convert it to fully written code? -> should be faster as all reallocations take time?
        assert!(self.faces.len() >= 5);
        assert!(self.faces[0].len() >= 9);
        assert!(self.faces[1].len() >= 9);
        assert!(self.faces[2].len() >= 9);
        assert!(self.faces[3].len() >= 9);
        assert!(self.faces[4].len() >= 9);
        assert!(self.faces[5].len() >= 9); 

        self.past_moves.push(col_row);

        match col_row {
            0  => {
                let tmp = [self.faces[1][0], self.faces[1][1], self.faces[1][2]];

                self.faces[1][0] = self.faces[5][6];
                self.faces[1][1] = self.faces[5][7];
                self.faces[1][2] = self.faces[5][8];

                self.faces[5][6] = self.faces[3][0];
                self.faces[5][7] = self.faces[3][1];
                self.faces[5][8] = self.faces[3][2];

                self.faces[3][0] = self.faces[2][0];
                self.faces[3][1] = self.faces[2][1];
                self.faces[3][2] = self.faces[2][2];

                self.faces[2][0] = tmp[0];
                self.faces[2][1] = tmp[1];
                self.faces[2][2] = tmp[2];

                //Clockwise
                //Corners
                let mut tmp = self.faces[0][0];

                self.faces[0][0] = self.faces[0][2];//takes from 2 & puts in 0
                self.faces[0][2] = self.faces[0][8];
                self.faces[0][8] = self.faces[0][6];
                self.faces[0][6] = tmp;

                //Side Pieces
                tmp = self.faces[0][1];
            
                self.faces[0][1] = self.faces[0][5];
                self.faces[0][5] = self.faces[0][7];
                self.faces[0][7] = self.faces[0][3];

                self.faces[0][3] = tmp;
                return;
            }
            1  => {
                let tmp = [self.faces[1][3], self.faces[1][4], self.faces[1][5]];

                self.faces[1][3] = self.faces[5][5];
                self.faces[1][4] = self.faces[5][4];
                self.faces[1][5] = self.faces[5][3];

                self.faces[5][5] = self.faces[3][3];
                self.faces[5][4] = self.faces[3][4];
                self.faces[5][3] = self.faces[3][5];

                self.faces[3][3] = self.faces[2][3];
                self.faces[3][4] = self.faces[2][4];
                self.faces[3][5] = self.faces[2][5];

                self.faces[2][3] = tmp[0];
                self.faces[2][4] = tmp[1];
                self.faces[2][5] = tmp[2];
                return;
            }
            2  => {
                let tmp = [self.faces[1][6], self.faces[1][7], self.faces[1][8]];

                self.faces[1][6] = self.faces[5][0];
                self.faces[1][7] = self.faces[5][1];
                self.faces[1][8] = self.faces[5][2];

                self.faces[5][0] = self.faces[3][6];
                self.faces[5][1] = self.faces[3][7];
                self.faces[5][2] = self.faces[3][8];

                self.faces[3][6] = self.faces[2][6];
                self.faces[3][7] = self.faces[2][7];
                self.faces[3][8] = self.faces[2][8];

                self.faces[2][6] = tmp[0];
                self.faces[2][7] = tmp[1];
                self.faces[2][8] = tmp[2];

                //Clockwise
                let mut tmp = self.faces[4][0];

                self.faces[4][0] = self.faces[4][6];
                self.faces[4][6] = self.faces[4][8];
                self.faces[4][8] = self.faces[4][2];
                self.faces[4][2] = tmp;

                //Side Pieces
                tmp = self.faces[4][1];
            
                self.faces[4][1] = self.faces[4][3];
                self.faces[4][3] = self.faces[4][7];
                self.faces[4][7] = self.faces[4][5];

                self.faces[4][5] = tmp;
                return;
            }
            3  => {
                let tmp = [self.faces[5][6], self.faces[5][7], self.faces[5][8]];

                self.faces[5][6] = self.faces[1][0];
                self.faces[5][7] = self.faces[1][1];
                self.faces[5][8] = self.faces[1][2];

                self.faces[1][0] = self.faces[2][0];
                self.faces[1][1] = self.faces[2][1];
                self.faces[1][2] = self.faces[2][2];

                self.faces[2][0] = self.faces[3][0];
                self.faces[2][1] = self.faces[3][1];
                self.faces[2][2] = self.faces[3][2];

                self.faces[3][0] = tmp[0];
                self.faces[3][1] = tmp[1];
                self.faces[3][2] = tmp[2];

                //Clockwise
                //Corners
                let mut tmp = self.faces[0][0];

                self.faces[0][0] = self.faces[0][6];//takes from 2 & puts in 0
                self.faces[0][6] = self.faces[0][8];
                self.faces[0][8] = self.faces[0][2];
                self.faces[0][2] = tmp;

                //Side Pieces
                tmp = self.faces[0][1];
            
                self.faces[0][1] = self.faces[0][3];
                self.faces[0][3] = self.faces[0][7];
                self.faces[0][7] = self.faces[0][5];

                self.faces[0][5] = tmp;
                return;
            }
            4  => {
                let tmp = [self.faces[1][3], self.faces[1][4], self.faces[1][5]];
                // 1, 5, 3, 2
                self.faces[1][3] = self.faces[2][3];
                self.faces[1][4] = self.faces[2][4];
                self.faces[1][5] = self.faces[2][5];

                self.faces[2][3] = self.faces[3][3];
                self.faces[2][4] = self.faces[3][4];
                self.faces[2][5] = self.faces[3][5];

                self.faces[3][3] = self.faces[5][5];
                self.faces[3][4] = self.faces[5][4];
                self.faces[3][5] = self.faces[5][3];

                self.faces[5][5] = tmp[0];
                self.faces[5][4] = tmp[1];
                self.faces[5][3] = tmp[2];
                return;
            }
            5  => {
                let tmp = [self.faces[1][6], self.faces[1][7], self.faces[1][8]];

                self.faces[1][6] = self.faces[2][6];
                self.faces[1][7] = self.faces[2][7];
                self.faces[1][8] = self.faces[2][8];

                self.faces[2][6] = self.faces[3][6];
                self.faces[2][7] = self.faces[3][7];
                self.faces[2][8] = self.faces[3][8];

                self.faces[3][6] = self.faces[5][0];
                self.faces[3][7] = self.faces[5][1];
                self.faces[3][8] = self.faces[5][2];

                self.faces[5][0] = tmp[0];
                self.faces[5][1] = tmp[1];
                self.faces[5][2] = tmp[2];

                //Clockwise
                let mut tmp = self.faces[4][0];

                self.faces[4][0] = self.faces[4][2];
                self.faces[4][2] = self.faces[4][8];
                self.faces[4][8] = self.faces[4][6];
                self.faces[4][6] = tmp;

                //Side Pieces
                tmp = self.faces[4][1];
            
                self.faces[4][1] = self.faces[4][5];
                self.faces[4][5] = self.faces[4][7];
                self.faces[4][7] = self.faces[4][3];

                self.faces[4][3] = tmp;
                return;
            }
            6  => {
                let tmp = [self.faces[0][0], self.faces[0][3], self.faces[0][6]];

                self.faces[0][0] = self.faces[2][0];
                self.faces[0][3] = self.faces[2][3];
                self.faces[0][6] = self.faces[2][6];

                self.faces[2][0] = self.faces[4][0];
                self.faces[2][3] = self.faces[4][3];
                self.faces[2][6] = self.faces[4][6];

                self.faces[4][0] = self.faces[5][0];
                self.faces[4][3] = self.faces[5][3];
                self.faces[4][6] = self.faces[5][6];

                self.faces[5][0] = tmp[0];
                self.faces[5][3] = tmp[1];
                self.faces[5][6] = tmp[2];

                //Anti-Clockwise

                let mut tmp = self.faces[1][0];

                self.faces[1][0] = self.faces[1][2];
                self.faces[1][2] = self.faces[1][8];
                self.faces[1][8] = self.faces[1][6];
                self.faces[1][6] = tmp;

                tmp = self.faces[1][1];

                self.faces[1][1] = self.faces[1][5];
                self.faces[1][5] = self.faces[1][7];
                self.faces[1][7] = self.faces[1][3];
                self.faces[1][3] = tmp;

                return;
            }
            7  => {
                let tmp = [self.faces[0][1], self.faces[0][4], self.faces[0][7]];

                self.faces[0][1] = self.faces[2][1];
                self.faces[0][4] = self.faces[2][4];
                self.faces[0][7] = self.faces[2][7];

                self.faces[2][1] = self.faces[4][1];
                self.faces[2][4] = self.faces[4][4];
                self.faces[2][7] = self.faces[4][7];

                self.faces[4][1] = self.faces[5][1];
                self.faces[4][4] = self.faces[5][4];
                self.faces[4][7] = self.faces[5][7];

                self.faces[5][1] = tmp[0];
                self.faces[5][4] = tmp[1];
                self.faces[5][7] = tmp[2];
                return;
            }
            8  => {
                let tmp = [self.faces[0][2], self.faces[0][5], self.faces[0][8]];

                self.faces[0][2] = self.faces[2][2];
                self.faces[0][5] = self.faces[2][5];
                self.faces[0][8] = self.faces[2][8];

                self.faces[2][2] = self.faces[4][2];
                self.faces[2][5] = self.faces[4][5];
                self.faces[2][8] = self.faces[4][8];

                self.faces[4][2] = self.faces[5][2];
                self.faces[4][5] = self.faces[5][5];
                self.faces[4][8] = self.faces[5][8];

                self.faces[5][2] = tmp[0];
                self.faces[5][5] = tmp[1];
                self.faces[5][8] = tmp[2];

                //Clockwise
                let mut tmp = self.faces[3][0];

                self.faces[3][0] = self.faces[3][6];
                self.faces[3][6] = self.faces[3][8];
                self.faces[3][8] = self.faces[3][2];
                self.faces[3][2] = tmp;

                tmp = self.faces[3][1];

                self.faces[3][1] = self.faces[3][3];
                self.faces[3][3] = self.faces[3][7];
                self.faces[3][7] = self.faces[3][5];
                self.faces[3][5] = tmp;
                return;
            }
            9  => {
                let tmp = [self.faces[0][0], self.faces[0][3], self.faces[0][6]];

                self.faces[0][0] = self.faces[5][0];
                self.faces[0][3] = self.faces[5][3];
                self.faces[0][6] = self.faces[5][6];

                self.faces[5][0] = self.faces[4][0];
                self.faces[5][3] = self.faces[4][3];
                self.faces[5][6] = self.faces[4][6];

                self.faces[4][0] = self.faces[2][0];
                self.faces[4][3] = self.faces[2][3];
                self.faces[4][6] = self.faces[2][6];

                self.faces[2][0] = tmp[0];
                self.faces[2][3] = tmp[1];
                self.faces[2][6] = tmp[2];

                //Anti-Clockwise

                let mut tmp = self.faces[1][0];

                self.faces[1][0] = self.faces[1][6];
                self.faces[1][6] = self.faces[1][8];
                self.faces[1][8] = self.faces[1][2];
                self.faces[1][2] = tmp;

                tmp = self.faces[1][1];

                self.faces[1][1] = self.faces[1][3];
                self.faces[1][3] = self.faces[1][7];
                self.faces[1][7] = self.faces[1][5];
                self.faces[1][5] = tmp;

                return;
            }
            10 => {
                let tmp = [self.faces[0][1], self.faces[0][4], self.faces[0][7]];

                self.faces[0][1] = self.faces[5][1];
                self.faces[0][4] = self.faces[5][4];
                self.faces[0][7] = self.faces[5][7];

                self.faces[5][1] = self.faces[4][1];
                self.faces[5][4] = self.faces[4][4];
                self.faces[5][7] = self.faces[4][7];

                self.faces[4][1] = self.faces[2][1];
                self.faces[4][4] = self.faces[2][4];
                self.faces[4][7] = self.faces[2][7];

                self.faces[2][1] = tmp[0];
                self.faces[2][4] = tmp[1];
                self.faces[2][7] = tmp[2];
                return;
            }
            11 => {
                let tmp = [self.faces[0][2], self.faces[0][5], self.faces[0][8]];

                self.faces[0][2] = self.faces[5][2];
                self.faces[0][5] = self.faces[5][5];
                self.faces[0][8] = self.faces[5][8];

                self.faces[5][2] = self.faces[4][2];
                self.faces[5][5] = self.faces[4][5];
                self.faces[5][8] = self.faces[4][8];

                self.faces[4][2] = self.faces[2][2];
                self.faces[4][5] = self.faces[2][5];
                self.faces[4][8] = self.faces[2][8];

                self.faces[2][2] = tmp[0];
                self.faces[2][5] = tmp[1];
                self.faces[2][8] = tmp[2];

                //Clockwise
                let mut tmp = self.faces[3][0];

                self.faces[3][0] = self.faces[3][2];
                self.faces[3][2] = self.faces[3][8];
                self.faces[3][8] = self.faces[3][6];
                self.faces[3][6] = tmp;

                tmp = self.faces[3][1];

                self.faces[3][1] = self.faces[3][5];
                self.faces[3][5] = self.faces[3][7];
                self.faces[3][7] = self.faces[3][3];
                self.faces[3][3] = tmp;
                return;
            }
            12 => {
                let tmp = [self.faces[0][6], self.faces[0][7], self.faces[0][8]];

                self.faces[0][6] = self.faces[3][0];
                self.faces[0][7] = self.faces[3][3];
                self.faces[0][8] = self.faces[3][6];

                self.faces[3][0] = self.faces[4][2];
                self.faces[3][3] = self.faces[4][1];
                self.faces[3][6] = self.faces[4][0];

                self.faces[4][2] = self.faces[1][8];
                self.faces[4][1] = self.faces[1][5];
                self.faces[4][0] = self.faces[1][2];

                self.faces[1][8] = tmp[0];
                self.faces[1][5] = tmp[1];
                self.faces[1][2] = tmp[2];

                //Clockwise
                let mut tmp = self.faces[2][0];

                self.faces[2][0] = self.faces[2][2];
                self.faces[2][2] = self.faces[2][8];
                self.faces[2][8] = self.faces[2][6];
                self.faces[2][6] = tmp;

                tmp = self.faces[2][1];

                self.faces[2][1] = self.faces[2][5];
                self.faces[2][5] = self.faces[2][7];
                self.faces[2][7] = self.faces[2][3];
                self.faces[2][3] = tmp;
                return;
            }
            13 => {
                let tmp = [self.faces[0][3], self.faces[0][4], self.faces[0][5]];

                self.faces[0][3] = self.faces[3][1];
                self.faces[0][4] = self.faces[3][4];
                self.faces[0][5] = self.faces[3][7];

                self.faces[3][1] = self.faces[4][5];
                self.faces[3][4] = self.faces[4][4];
                self.faces[3][7] = self.faces[4][3];

                self.faces[4][5] = self.faces[1][7];
                self.faces[4][4] = self.faces[1][4];
                self.faces[4][3] = self.faces[1][1];

                self.faces[1][7] = tmp[0];
                self.faces[1][4] = tmp[1];
                self.faces[1][1] = tmp[2];
                return;
            } 
            14 => {
                let tmp = [self.faces[0][0], self.faces[0][1], self.faces[0][2]];

                self.faces[0][0] = self.faces[3][2];
                self.faces[0][1] = self.faces[3][5];
                self.faces[0][2] = self.faces[3][8];

                self.faces[3][2] = self.faces[4][8];
                self.faces[3][5] = self.faces[4][7];
                self.faces[3][8] = self.faces[4][6];

                self.faces[4][8] = self.faces[1][6];
                self.faces[4][7] = self.faces[1][3];
                self.faces[4][6] = self.faces[1][0];

                self.faces[1][6] = tmp[0];
                self.faces[1][3] = tmp[1];
                self.faces[1][0] = tmp[2];

                //Clockwise
                let mut tmp = self.faces[5][0];

                self.faces[5][0] = self.faces[5][2];
                self.faces[5][2] = self.faces[5][8];
                self.faces[5][8] = self.faces[5][6];
                self.faces[5][6] = tmp;

                tmp = self.faces[5][1];

                self.faces[5][1] = self.faces[5][5];
                self.faces[5][5] = self.faces[5][7];
                self.faces[5][7] = self.faces[5][3];
                self.faces[5][3] = tmp;
                return;
            }
            15 => {
                let tmp = [self.faces[0][6], self.faces[0][7], self.faces[0][8]];

                self.faces[0][6] = self.faces[1][8];
                self.faces[0][7] = self.faces[1][5];
                self.faces[0][8] = self.faces[1][2];

                self.faces[1][8] = self.faces[4][2];
                self.faces[1][5] = self.faces[4][1];
                self.faces[1][2] = self.faces[4][0];

                self.faces[4][2] = self.faces[3][0];
                self.faces[4][1] = self.faces[3][3];
                self.faces[4][0] = self.faces[3][6];

                self.faces[3][0] = tmp[0];
                self.faces[3][3] = tmp[1];
                self.faces[3][6] = tmp[2];

                //Clockwise
                let mut tmp = self.faces[2][0];

                self.faces[2][0] = self.faces[2][6];
                self.faces[2][6] = self.faces[2][8];
                self.faces[2][8] = self.faces[2][2];
                self.faces[2][2] = tmp;

                tmp = self.faces[2][1];

                self.faces[2][1] = self.faces[2][3];
                self.faces[2][3] = self.faces[2][7];
                self.faces[2][7] = self.faces[2][5];
                self.faces[2][5] = tmp;
                return;
            }
            16 => {
                let tmp = [self.faces[0][3], self.faces[0][4], self.faces[0][5]];

                self.faces[0][3] = self.faces[1][7];
                self.faces[0][4] = self.faces[1][4];
                self.faces[0][5] = self.faces[1][1];

                self.faces[1][7] = self.faces[4][5];
                self.faces[1][4] = self.faces[4][4];
                self.faces[1][1] = self.faces[4][3];

                self.faces[4][5] = self.faces[3][1];
                self.faces[4][4] = self.faces[3][4];
                self.faces[4][3] = self.faces[3][7];

                self.faces[3][1] = tmp[0];
                self.faces[3][4] = tmp[1];
                self.faces[3][7] = tmp[2];
                return;
            }
            17 => {
                let tmp = [self.faces[0][0], self.faces[0][1], self.faces[0][2]];

                self.faces[0][0] = self.faces[1][6];
                self.faces[0][1] = self.faces[1][3];
                self.faces[0][2] = self.faces[1][0];

                self.faces[1][6] = self.faces[4][8];
                self.faces[1][3] = self.faces[4][7];
                self.faces[1][0] = self.faces[4][6];

                self.faces[4][8] = self.faces[3][2];
                self.faces[4][7] = self.faces[3][5];
                self.faces[4][6] = self.faces[3][8];

                self.faces[3][2] = tmp[0];
                self.faces[3][5] = tmp[1];
                self.faces[3][8] = tmp[2];

                //Clockwise
                let mut tmp = self.faces[5][0];

                self.faces[5][0] = self.faces[5][6];
                self.faces[5][6] = self.faces[5][8];
                self.faces[5][8] = self.faces[5][2];
                self.faces[5][2] = tmp;

                tmp = self.faces[5][1];

                self.faces[5][1] = self.faces[5][3];
                self.faces[5][3] = self.faces[5][7];
                self.faces[5][7] = self.faces[5][5];
                self.faces[5][5] = tmp;
                return;
            }
            _default => {
                panic!("Cannot turn the rubix cube this way");
            }
        }
    }

    pub fn is_solved(&mut self) -> bool
    {
        for face in 0..5{
            for piece in 1..8{
                if self.faces[face][0] != self.faces[face][piece]{
                    return false;
                };
            }
        }
        return true;
    }

    pub fn undo_turn(&mut self)
    {
        let move_to_undo = self.past_moves.pop().expect("should be u8");

        self.turn_cube(&(if MOVED_BY_3_UP.contains(&move_to_undo){move_to_undo - 3} else {move_to_undo + 3} ));

        self.past_moves.pop();
    }

    pub fn make_random_moves(&mut self, number_of_moves: u8)
    {
        let mut rng = rand::thread_rng();
        for _ in 0..number_of_moves{
            let n1 = rng.gen_range(0..18);
            self.turn_cube(&n1);
        }
    }
    
    pub fn search_astar(&mut self){
        let mut threshold = self.heuristic();
        let now = Instant::now();
        loop{
            let (result, new_threshold) = self.astar(0, threshold);
            let elapsed_time = now.elapsed();
            if result{
                println!("found solution at threshold: {}; Elapsed time: {:?}.{:?}s", new_threshold+1, elapsed_time.as_secs(), elapsed_time.as_millis());
                for v in self.get_solving_route(){
                    print!("{} ", v);
                }
                break;
            }
            else{
                
                threshold = new_threshold as u8;
    
                println!("found no solution at threshold: {}; Elapsed time: {:?}.{:?}s", threshold, elapsed_time.as_secs(), elapsed_time.as_millis());
            }
        }
    }
    
    pub fn astar(&mut self, depth : u8, threshold : u8) -> (bool, u8){
        if self.is_solved(){
            return (true, threshold);
        }

        let estimated_total_path_length = depth + self.heuristic();

        if estimated_total_path_length > threshold{
            return (false, estimated_total_path_length);
        }

        let mut min_threshold_exceeded: u8 = 255;
        for next_move in 0..18{
            if (self.past_moves[self.past_moves.len()-1] == 3 + next_move && MOVED_BY_3_UP.contains(&next_move)) || (self.past_moves[self.past_moves.len()-1] + 3 == next_move && MOVED_BY_3_DOWN.contains(&next_move)){
                continue;
            }

            self.turn_cube(&next_move);

            let (result, new_threshold) = self.astar(depth+1, threshold);
            self.undo_turn();
            if result{
                self.solving_route.push(next_move);
                return (result, new_threshold);
            }
            if new_threshold < min_threshold_exceeded{
                min_threshold_exceeded = new_threshold;
            }
        };
        return (false, min_threshold_exceeded);
    }
    

    fn heuristic(&self) -> u8 {
        // Literally isnt even correct
        // What am i doing with my life
        let mut total = [0; 6]; // Array to store total counts for each value
        for face in self.faces.iter() {
            for &val in face {
                total[val as usize] += 1;
            }
        }
        let max_counts = total.iter().max().expect("failed");
        return 9 - max_counts;
    }

}


pub(crate) trait Iddfs {
    fn thread_search_iddfs(&mut self, depth_to_search: u8);
 
    fn thread_iddfs(&mut self, depth: u8) -> (bool, Vec<u8>);
 }

impl Iddfs for  RubixCube{
    fn thread_search_iddfs(&mut self, depth_to_search : u8) {
        //fine up to depth 6 but beyond that begins to take too long

        //depth 7 -> 100 seconds
        //depth 8 -> ~40000 seconds (expected)

        for depth in 0..depth_to_search{
            let now = Instant::now();
        
            // Split the work among threads
            let solutions: Vec<(bool, Vec<u8>)> = (0..18)
                .into_par_iter()
                .map(|init_move| {
                    let mut v = self.clone();
                    v.turn_cube(&init_move);
                    v.thread_iddfs(depth)
                })
                .collect();
            
            for (solution, path) in solutions {
                if solution{
                    println!("found solution at depth: {}; Elapsed time: {:?}", depth, now.elapsed());
                    for v in path {
                       print!("{} ", v);
                    }
                    return;
                }
            };
            println!("found no solution at depth: {}; Elapsed time: {:?}", depth, now.elapsed());
        }
    }

    fn thread_iddfs(&mut self, depth: u8) -> (bool, Vec<u8>)
    {

        if depth <= 0{
            return (self.is_solved(), Vec::new());
        }

        for next_move in MOVED_BY_3_UP{
            if self.past_moves[self.past_moves.len()-1] + 3 == next_move{
                continue;
            };
            

            self.turn_cube(&next_move);
            let (solved, mut path) = self.thread_iddfs(depth-1);
            if solved{
                path.push(next_move);
                return (true, path);
            }
            self.undo_turn();
        };
        for next_move in MOVED_BY_3_DOWN{
            if self.past_moves[self.past_moves.len()-1] == 3 + next_move{
                continue;
            };
            
            self.turn_cube(&next_move);
            let (solved, mut path) = self.thread_iddfs(depth-1);
            if solved{
                path.push(next_move);
                return (true, path);
            }
            self.undo_turn();
        };
        return (false, Vec::new());
    }  
}