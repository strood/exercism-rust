use core::option::Option;
use std::vec::Vec;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Clone)]
pub struct Frame {
    number: usize,
    rolls: Vec<u16>,
    next_frame: Option<Box<Frame>>,
}

struct Node<Frame> {
    data: Frame,
    next: Option<Box<Node<Frame>>>,
}

impl Frame {
    pub fn new(num: usize) -> Self {
        Frame {
            number: num,
            rolls: Vec::new(),
            next_frame: None,
        }
    }

    pub fn value(&self) -> u16 {
        // Assume all frames complete to call this.
        let frame_value = self.rolls.iter().sum();

        // Final frame just returns sum of rolls
        if self.number == 10 {
            frame_value
        }

        // Check rolls for values
        match self.rols.len() {
            // Strike, get next 2 roll values
            1 => {
                if let Some(next_frame) = &self.next_frame {
                    let next_rolls = next_frame.get_rolls();
                    // Strike strike x
                    if next_rolls.len() == 1 {
                        if let Some(next_next_frame) = &next_frame.next_frame {
                            let next_next_rolls = next_next_frame.get_rolls();
                            frame_value + next_rolls[0] + next_next_rolls[0]
                        }
                    } else {
                        // Strike x x
                        frame_value + next_rolls[0] + next_rolls[1]
                    }
                }
            }
            2 => {
                // Spare, get first roll of next frame value
                if let Some(next_frame) = &self.next_frame {
                    let next_rolls = next_frame.get_rolls();
                    frame_value + next_rolls[0]
                }
            }
            // Open frame, just return sum
            3 => frame_value,
        }
    }

    // Check if this frame is complete
    pub fn complete(&self) -> bool {
        let is_last_frame = self.number == 10;
        let is_strike = self.rolls.len() == 1 && self.rolls[0] == 10;
        let is_spare = self.rolls.len() == 2 && self.rolls.iter().sum::<u16>() == 10;
        if is_last_frame {
            (is_strike || is_spare && self.rolls.len() == 3)
                || (!is_strike && !is_spare && self.rolls.len() >= 2)
        } else {
            is_strike || is_spare || self.rolls.len() == 2
        }
    }
}

#[derive(Debug, Clone)]
pub struct BowlingGame {
    head_frame: Option<Box<Node<Frame>>>, // pointer to first frame
    current_frame: u16,                   // len
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            head_frame: None,
            current_frame: 0,
        }
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            None
        } else {
            let mut score = 0;
            if let Some(head_frame) = &self.head_frame {
                let mut current_frame = head_frame;
                while let Some(next_frame) = &current_frame.next {
                    score += current_frame.data.value();
                    current_frame = next_frame;
                }
                score += current_frame.data.value();
            }
            Some(score)
        }
    }

    pub fn roll(&self, pins: u16) -> Result<(), Error> {
        // Fail if pins too high
        if pins > 10 {
            Err(Error::NotEnoughPinsLeft)
        }

        // Fail if game complete
        if self.is_complete() {
            Err(Error::GameComplete)
        }

        // Should be a valid roll, lets proceed to add it
        // Check if we have a head frame
        if let Some(head_frame) = &self.head_frame {
            // Check if current frame is complete
            if head_frame.data.complete() {
                // Create new frame
                self.new_frame(pins, head_frame)
            } else {
                // Add roll to current frame
                head_frame.data.rolls.push(pins);
                Ok(())
            }
        } else {
            // Create new frame
            self.new_frame(pins, None)
        }
    }

    // Need to create this correctly w/ mutable ref ect
    // Create a new frame
    fn new_frame(&self, pins: u16, last_frame: &mut Option<Box<Node<Frame>>>) -> Result<(), Error> {
        // Create new frame
        let mut new_frame = Frame::new(self.current_frame);
        new_frame.rolls.push(pins);

        // Create new node
        let new_node = Box::new(Node {
            data: new_frame,
            next: None,
        });

        // Add new node to end of list, or start if first
        if let Some(last_frame) = last_frame {
            last_frame.next = Some(new_node);
        } else {
            self.head_frame = Some(new_node);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head_frame.is_none()
    }

    // Num of current frame
    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    // Check if all frames completed already
    pub fn is_complete(&self) -> bool {
        if self.current_frame != 10 {
            false
        }

        if let Some(frame) = &self.head_frame {
            let mut current_frame = frame;
            while let Some(next_frame) = &current_frame.next {
                current_frame = next_frame;
            }
            current_frame.data.complete()
        } else {
            false
        }
    }
}
