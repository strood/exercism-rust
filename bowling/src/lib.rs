#[derive(Debug, Clone)]
pub struct BowlingGame {
    frames: Vec<Frame>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: Vec::with_capacity(10),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // Fail if pins too high
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        // Fail if game complete
        if self.is_complete() {
            return Err(Error::GameComplete);
        }

        // Should be a valid roll, lets proceed to add it
        // Check if we have a head frame
        if self.frames.is_empty() || self.frames.last().unwrap().is_complete() {
            self.frames.push(Frame::new(self.frames.len() as u16 + 1));
        }

        let current_frame = self.frames.last_mut().unwrap();
        if !current_frame.rolls().is_empty() {
            // Check pin combo is valid for non final frame
            if current_frame.number() != 10 && current_frame.rolls()[0] + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            } else {
                // Handle final frame special cases
                if current_frame.rolls().len() == 1
                    && !current_frame.is_strike()
                    && current_frame.rolls()[0] + pins > 10
                {
                    return Err(Error::NotEnoughPinsLeft);
                }
                if current_frame.is_strike()
                    && current_frame.rolls().len() == 2
                    && current_frame.rolls[1] != 10
                    && current_frame.rolls()[1] + pins > 10
                {
                    return Err(Error::NotEnoughPinsLeft);
                }
            }
        }

        current_frame.roll(pins);

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        // Return sum of all frames if complete, none if not
        if self.is_complete() {
            let mut score = 0;
            for frame in &self.frames {
                if frame.is_strike() {
                    if let Some(next_frame) = self.frames.get(frame.number()) {
                        // Check for strike points, either both next or one next
                        if next_frame.is_strike() {
                            if let Some(next_next_frame) = self.frames.get(frame.number() + 1) {
                                score +=
                                    frame.score() + next_frame.score() + next_next_frame.rolls()[0];
                            } else {
                                // Final frame just count its score
                                score +=
                                    frame.score() + next_frame.rolls()[0] + next_frame.rolls()[1];
                            }
                        } else {
                            score += frame.score() + next_frame.rolls()[0] + next_frame.rolls()[1];
                        }
                    } else {
                        // Final frame just count its score
                        score += frame.score();
                    }
                } else if frame.is_spare() {
                    score += frame.score();
                    if let Some(next_frame) = self.frames.get(frame.number()) {
                        score += next_frame.rolls()[0];
                    }
                } else {
                    score += frame.score();
                }
            }
            Some(score)
        } else {
            None
        }
    }

    fn is_complete(&self) -> bool {
        self.frames.len() == 10 && self.frames[9].is_complete()
    }
}

#[derive(Debug, Clone)]
struct Frame {
    number: u16,
    rolls: Vec<u16>,
}

impl Frame {
    fn new(number: u16) -> Self {
        Frame {
            number,
            rolls: Vec::with_capacity(if number == 10 { 3 } else { 2 }),
        }
    }

    pub fn number(&self) -> usize {
        self.number as usize
    }

    pub fn rolls(&self) -> &Vec<u16> {
        &self.rolls
    }

    fn roll(&mut self, pins: u16) {
        self.rolls.push(pins);
    }

    fn is_complete(&self) -> bool {
        if self.number == 10 {
            if self.rolls.len() == 3 {
                return true;
            }
            if self.rolls.len() > 1 {
                return self.rolls[0] + self.rolls[1] < 10;
            }
            false
        } else {
            if self.rolls.len() > 1 {
                return true;
            }
            if self.rolls.len() == 1 {
                return self.rolls[0] == 10;
            }
            false
        }
    }

    fn is_strike(&self) -> bool {
        self.rolls[0] == 10
    }

    fn is_spare(&self) -> bool {
        self.rolls.len() == 2 && self.rolls[0] + self.rolls[1] == 10
    }

    fn score(&self) -> u16 {
        self.rolls.iter().sum()
    }
}
