#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct State {
    bucket1: u8,
    bucket2: u8,
}

enum Action {
    Fill1,
    Fill2,
    Empty1,
    Empty2,
    Pour1to2,
    Pour2to1,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    // Setup state,
    // Make initial fill on start bucket,
    // check all permutations at each state until we come across state we have visited, or we reach goal(state that icludes goal)

    let (start_state, invalid_state) = match start_bucket {
        Bucket::One => (
            State {
                bucket1: capacity_1,
                bucket2: 0,
            },
            State {
                bucket1: 0,
                bucket2: capacity_2,
            },
        ),
        Bucket::Two => (
            State {
                bucket1: 0,
                bucket2: capacity_2,
            },
            State {
                bucket1: capacity_1,
                bucket2: 0,
            },
        ),
    };

    let mut visited = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::new();

    queue.push_back((start_state.clone(), 1));

    while let Some((state, moves)) = queue.pop_front() {
        if visited.contains(&state) || state == invalid_state {
            continue;
        }

        visited.insert(state.clone());

        if state.bucket1 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: state.bucket2,
            });
        }

        if state.bucket2 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::Two,
                other_bucket: state.bucket1,
            });
        }

        for (next_state, _) in next_states(&state, capacity_1, capacity_2) {
            queue.push_back((next_state, moves + 1));
        }
    }

    None
}

fn next_states(state: &State, capacity_1: u8, capacity_2: u8) -> Vec<(State, Action)> {
    let mut next_states = Vec::new();
    let actions = vec![
        Action::Fill1,
        Action::Fill2,
        Action::Empty1,
        Action::Empty2,
        Action::Pour1to2,
        Action::Pour2to1,
    ];

    for action in actions {
        let next_state = handle_action(state, &action, capacity_1, capacity_2);
        next_states.push((next_state, action));
    }

    next_states
}

fn handle_action(state: &State, action: &Action, capacity_1: u8, capacity_2: u8) -> State {
    match action {
        Action::Fill1 => State {
            bucket1: capacity_1,
            bucket2: state.bucket2,
        },
        Action::Fill2 => State {
            bucket1: state.bucket1,
            bucket2: capacity_2,
        },
        Action::Empty1 => State {
            bucket1: 0,
            bucket2: state.bucket2,
        },
        Action::Empty2 => State {
            bucket1: state.bucket1,
            bucket2: 0,
        },
        Action::Pour1to2 => {
            let pour = state.bucket1.min(capacity_2 - state.bucket2);
            State {
                bucket1: state.bucket1 - pour,
                bucket2: state.bucket2 + pour,
            }
        }
        Action::Pour2to1 => {
            let pour = state.bucket2.min(capacity_1 - state.bucket1);
            State {
                bucket1: state.bucket1 + pour,
                bucket2: state.bucket2 - pour,
            }
        }
    }
}
