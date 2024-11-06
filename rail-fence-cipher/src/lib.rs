pub struct RailFence {
    rails: u32
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        // Setup rails
        let mut rails = vec![Vec::new(); self.rails as usize];
        let mut result = String::new();
        let mut rail: i16 = 0;
        let mut direction: i16 = 1;

        // Fill the rails
        for c in text.chars() {
            rails[rail as usize].push(c);
            if rail == 0 {
                direction = 1;
            } else if rail == (rails.len() - 1) as i16 {
                direction = -1;
            }
            rail += direction;
        }

        // Read off the message
        for r in rails.iter() {
            result.push_str(&r.iter().collect::<String>());
        }

        result
    }

    pub fn decode(&self, cipher: &str) -> String {
        // Setup rails
        let rails = self.rails as i16;
        let len = cipher.chars().count();
        let mut fence = vec![vec!['\0'; len]; rails as usize];
        let mut result = vec!['\0'; len];
    
        // Mark the rail positions
        let mut rail: i16 = 0;
        let mut direction: i16 = 1;
        for i in 0..len {
            fence[rail as usize][i] = '*';
            rail += direction;
            if rail == 0 || rail == rails - 1 {
                direction = -direction;
            }
        }
    
        // Fill the fence with cipher characters
        let mut index = 0;
        for r in 0..rails {
            for c in 0..len {
                if fence[r as usize][c] == '*' && index < len {
                    fence[r as usize][c] = cipher.chars().nth(index).unwrap();
                    index += 1;
                }
            }
        }
    
        // Read off the message
        rail = 0;
        direction = 1;
        for i in 0..len {
            result[i] = fence[rail as usize][i];
            rail += direction;
            if rail == 0 || rail == rails - 1 {
                direction = -direction;
            }
        }
    
        result.into_iter().collect()
    }
}
