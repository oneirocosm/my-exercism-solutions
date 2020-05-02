use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
enum Score {
    OpenFrame(u16),
    Spare(u16),
    Strike(u16),
    FillBall(u16),
}

#[derive(Debug, Clone, Copy)]
enum Roll {
    Regular(u8),
    Fill(u8),
}

impl Score {
    fn unwrap(&self) -> u16 {
        match *self {
            Self::OpenFrame(val) => val,
            Self::Spare(val) => val,
            Self::Strike(val) => val,
            Self::FillBall(_) => 0,
        }
    }
}

pub struct BowlingGame {
    frame: u8,
    roll_this_frame: Roll,
    pins_standing: u16,

    scores: Vec<Score>,
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frame: 1,
            roll_this_frame: Roll::Regular(1),
            pins_standing: 10,

            scores: Vec::new(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frame > 10 {
            return Err(Error::GameComplete);
        }
        self.add_new_roll(pins)?;
        self.pins_standing -= pins;
        self.update_past_strike_spare(pins);
        self.update_frame_and_roll();

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        println!("{:?}", self.scores);
        if self.frame > 10 {
            Some(self.scores.iter().map(|x| x.unwrap()).sum())
        } else {
            None
        }
    }

    fn add_new_roll(&mut self, pins: u16) -> Result<(), Error> {
        match (pins.cmp(&self.pins_standing), self.roll_this_frame) {
            (Ordering::Greater, _) => return Err(Error::NotEnoughPinsLeft),
            (_, Roll::Fill(3)) => {
                self.scores.push(Score::FillBall(pins));
            }
            (_, Roll::Fill(2)) => {
                self.scores.push(Score::FillBall(pins));
            }
            (Ordering::Less, _) => {
                self.scores.push(Score::OpenFrame(pins));
            }
            (Ordering::Equal, Roll::Regular(1)) => {
                self.scores.push(Score::Strike(pins));
            }
            (Ordering::Equal, Roll::Regular(2)) => {
                self.scores.push(Score::Spare(pins));
            }
            (Ordering::Equal, roll) => unimplemented!("bad input: {:?}, {:?}", pins, roll),
        };

        Ok(())
    }

    fn update_past_strike_spare(&mut self, pins: u16) -> Option<()> {
        let len = self.scores.len();

        match self.scores.get_mut(len.wrapping_sub(2))? {
            Score::Strike(strike) => *strike += pins,
            Score::Spare(spare) => *spare += pins,
            _ => {}
        };

        if let Score::Strike(strike) = self.scores.get_mut(len.wrapping_sub(3))? {
            *strike += pins;
        }

        Some(())
    }

    fn update_frame_and_roll(&mut self) {
        match (
            self.frame,
            self.roll_this_frame,
            self.scores.iter().last().unwrap(),
        ) {
            (10, Roll::Fill(3), _) => {
                self.frame += 1;
                self.pins_standing = 10;
            }
            (10, Roll::Fill(2), Score::FillBall(10)) => {
                self.roll_this_frame = Roll::Fill(3);
                self.pins_standing = 10;
            }
            (10, Roll::Fill(2), Score::FillBall(_)) => {
                self.roll_this_frame = Roll::Fill(3);
            }
            (10, Roll::Regular(1), Score::Strike(_)) => {
                self.roll_this_frame = Roll::Fill(2);
                self.pins_standing = 10;
            }
            (10, Roll::Regular(2), Score::Spare(_)) => {
                self.roll_this_frame = Roll::Fill(3);
                self.pins_standing = 10;
            }
            (_, Roll::Regular(1), Score::Strike(_)) => {
                self.frame += 1;
                self.pins_standing = 10;
            }
            (_, Roll::Regular(1), _) => {
                self.roll_this_frame = Roll::Regular(2);
            }
            (_, Roll::Regular(2), _) => {
                self.roll_this_frame = Roll::Regular(1);
                self.frame += 1;
                self.pins_standing = 10;
            }
            (frame, roll, score) => unimplemented!(
                "should not reach here\ninput: {:?}, {:?}, {:?}",
                frame,
                roll,
                score
            ),
        };
    }
}
