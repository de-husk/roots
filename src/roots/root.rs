use crate::roots::direction::{Direction, Position, TurnDirection};
use colored::*;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::time::SystemTime;

// when grow() reaches max_steps it will stop computing new values (this is capped so there is a maximum tree age)
const MAX_STEPS: u64 = 60;

#[derive(Serialize, Deserialize)]
pub struct Root {
    pub name: String,
    pub seed: u64,
    planted_time: SystemTime,
    last_watered_time: SystemTime,

    #[serde(skip)]
    pub tree: Tree,
}

impl Default for Root {
    fn default() -> Self {
        let now = SystemTime::now();
        Self {
            name: "Max".to_string(),
            seed: 1337,
            planted_time: now,
            last_watered_time: now,
            tree: Default::default(),
        }
    }
}

impl Root {
    pub fn new(name: String, seed: u64) -> Self {
        Self {
            name,
            seed,
            ..Default::default()
        }
    }

    // Deterministically generate a tree based on the starting seed and time elapsed
    pub fn generate(&mut self) -> &Vec<Vec<TreeCell>> {
        //growth_rate_secs is how long we have to wait for another step to be computed by grow()
        let growth_rate_secs = Duration::new(60 * 60 * 3, 0).as_secs();

        let elapsed = SystemTime::now().duration_since(self.planted_time).unwrap();
        let steps = elapsed.as_secs() / growth_rate_secs;

        self.grow(steps);

        self.tree.t.as_ref()
    }

    fn grow(&mut self, steps: u64) {
        let mut r = StdRng::seed_from_u64(self.seed);
        let step_limit = if steps > MAX_STEPS { MAX_STEPS } else { steps };

        let mut trunk = Branch {
            pos: Position {
                x: self.tree.width / 2,
                y: 0,
            },
            direction: Direction::North,
            branch_type: BranchType::GrowingNorth,
        };

        let mut branch_count = 0;
        self.grow_rec(step_limit, &mut r, &mut trunk, &mut branch_count);
    }

    fn grow_rec(
        &mut self,
        max_step: u64,
        rng: &mut StdRng,
        branch: &mut Branch,
        branch_count: &mut u8,
    ) {
        const MAX_BRANCHES: u8 = 10;

        let mut step = 0;

        while step <= max_step {
            self.tree.t[branch.pos.y][branch.pos.x] = branch.to_tree_cell();

            let pct_done: f32 = step as f32 / max_step as f32;
            let leaf_count = if branch.pos.y < 10 {
                rng.gen_range(1..=15)
            } else {
                rng.gen_range(branch.pos.y / 2..=branch.pos.y)
            };

            if matches!(branch.branch_type, BranchType::GrowingNorth)
                && branch.pos.y > self.tree.height - 5
            {
                branch.branch_type = BranchType::Stem;
                let mut b = branch.clone();
                b.branch_type = BranchType::Stem;
                self.grow_rec(leaf_count as u64, rng, &mut b, branch_count);
            }

            if (matches!(branch.branch_type, BranchType::GrowingNorth)
                || matches!(branch.branch_type, BranchType::GrowingWest)
                || matches!(branch.branch_type, BranchType::GrowingEast))
                && pct_done > 0.90
            {
                let mut b = branch.clone();
                b.branch_type = BranchType::Leaf;
                self.grow_rec(leaf_count as u64, rng, &mut b, branch_count);
            } else if (matches!(branch.branch_type, BranchType::GrowingNorth)
                || matches!(branch.branch_type, BranchType::GrowingWest)
                || matches!(branch.branch_type, BranchType::GrowingEast))
                && pct_done > 0.85
            {
                let mut b = branch.clone();
                b.branch_type = BranchType::Stem;
                self.grow_rec(leaf_count as u64, rng, &mut b, branch_count);
            } else if matches!(branch.branch_type, BranchType::GrowingNorth)
                && *branch_count < MAX_BRANCHES
            {
                let r = rng.gen_range(0..=10);
                let rr = rng.gen_range(0..=100);

                if r <= 3 && rr % 4 == 0 {
                    *branch_count += 1;
                    let mut b = branch.clone();
                    let ts = [BranchType::GrowingWest, BranchType::GrowingEast];
                    b.branch_type = ts[r % 2];

                    let mut len = if branch.pos.y < self.tree.height / 2 {
                        rng.gen_range(50..100)
                    } else {
                        rng.gen_range(10..25)
                    };

                    if len > max_step {
                        len = max_step;
                    }

                    self.grow_rec(len, rng, &mut b, branch_count);
                }
            }

            step += 1;

            // Grow into new cell:
            calc_direction(branch, rng);

            let new_pos = calc_position(branch, self.tree.width, self.tree.height);
            match new_pos {
                Some(pos) => {
                    branch.pos.x = pos.x;
                    branch.pos.y = pos.y;
                }
                None => {
                    continue;
                }
            }
        }
    }
}

fn calc_position(branch: &Branch, tree_width: usize, tree_height: usize) -> Option<Position> {
    // Returns None if Position would be outside of tree grid bounds

    match branch.direction {
        Direction::North => {
            if branch.pos.y == tree_height - 1 {
                None
            } else {
                Some(Position {
                    x: branch.pos.x,
                    y: branch.pos.y + 1,
                })
            }
        }
        Direction::NorthEast => {
            if branch.pos.y == tree_height - 1 || branch.pos.x == tree_width - 1 {
                None
            } else {
                Some(Position {
                    x: branch.pos.x + 1,
                    y: branch.pos.y + 1,
                })
            }
        }
        Direction::East => {
            if branch.pos.x == tree_width - 1 {
                None
            } else {
                Some(Position {
                    x: branch.pos.x + 1,
                    y: branch.pos.y,
                })
            }
        }

        Direction::SouthEast => {
            if branch.pos.x == tree_width - 1 || branch.pos.y == 0 {
                None
            } else {
                Some(Position {
                    x: branch.pos.x + 1,
                    y: branch.pos.y - 1,
                })
            }
        }
        Direction::South => {
            if branch.pos.y == 0 {
                None
            } else {
                Some(Position {
                    x: branch.pos.x,
                    y: branch.pos.y,
                })
            }
        }
        Direction::SouthWest => {
            if branch.pos.x == 0 || branch.pos.y == 0 {
                None
            } else {
                Some(Position {
                    x: branch.pos.x - 1,
                    y: branch.pos.y - 1,
                })
            }
        }
        Direction::West => {
            if branch.pos.x == 0 {
                None
            } else {
                Some(Position {
                    x: branch.pos.x - 1,
                    y: branch.pos.y,
                })
            }
        }
        Direction::NorthWest => {
            if branch.pos.x == 0 || branch.pos.y == tree_height - 1 {
                None
            } else {
                Some(Position {
                    x: branch.pos.x - 1,
                    y: branch.pos.y + 1,
                })
            }
        }
    }
}

fn calc_direction(branch: &mut Branch, rng: &mut StdRng) {
    let angle = [TurnDirection::Left, TurnDirection::Right];

    let r: u8 = rng.gen_range(0..=9);
    let t: usize = rng.gen_range(0..=1);

    match branch.branch_type {
        BranchType::GrowingNorth => {
            // GrowingNorth branches are favored to move vertically
            if branch.direction.is_moving_north() {
                if r <= 2 {
                    branch.direction = branch.direction.turn(angle[t]);
                }
            } else if r <= 7 {
                branch.direction = branch.direction.turn(angle[t]);
            }
        }
        BranchType::GrowingWest => {
            // Heavily favor leftwards movement
            if branch.direction.is_moving_west() {
                if r <= 2 {
                    branch.direction = branch.direction.turn(angle[t]);
                }
            } else if r <= 8 {
                branch.direction = branch.direction.turn(angle[t]);
            }
        }
        BranchType::GrowingEast => {
            // Heavily favor rightwards movement
            if branch.direction.is_moving_east() {
                if r <= 2 {
                    branch.direction = branch.direction.turn(angle[t]);
                }
            } else if r <= 8 {
                branch.direction = branch.direction.turn(angle[t]);
            }
        }
        BranchType::Stem => {
            // Leafs are slightly favored to move horizontally but will still move vertically
            if branch.direction.is_moving_horizontally() {
                if r <= 4 {
                    branch.direction = branch.direction.turn(angle[t]);
                }
            } else if r <= 6 {
                branch.direction = branch.direction.turn(angle[t]);
            }
        }
        BranchType::Leaf => {
            // Leafs are favored to move horizontally but will still move vertically
            if branch.direction.is_moving_horizontally() {
                if r <= 2 {
                    branch.direction = branch.direction.turn(angle[t]);
                }
            } else if r <= 8 {
                branch.direction = branch.direction.turn(angle[t]);
            }
        }
    }
}

pub struct Tree {
    t: Vec<Vec<TreeCell>>,
    pub width: usize,
    pub height: usize,
}

impl Tree {
    pub fn new() -> Self {
        let (width, height) = Self::calc_dimensions();

        Self {
            width,
            height,
            t: vec![vec![TreeCell { ch: " ".black() }; width]; height],
        }
    }

    fn calc_dimensions() -> (usize, usize) {
        // TODO: put lower and upper bounds on the acceptable terminal window sizes
        if let Some((w, h)) = term_size::dimensions() {
            (w - 20, h)
        } else {
            // Cant determine terminal dimensions, use defaults
            (100, 40)
        }
    }
}

impl Default for Tree {
    fn default() -> Self {
        Tree::new()
    }
}

#[derive(Clone)]
pub struct TreeCell {
    pub ch: colored::ColoredString,
}

#[derive(Clone, Debug)]
struct Branch {
    // pos is the current position in the 2d tree grid
    pos: Position,
    // direction is the direction of growth (its like velocity but with a set value of 1 unit per step)
    direction: Direction,
    branch_type: BranchType,
}

impl Branch {
    fn to_tree_cell(&self) -> TreeCell {
        let str = match self.branch_type {
            BranchType::GrowingNorth => match self.direction {
                Direction::North => "/|\\",
                Direction::NorthEast => "|/",
                Direction::East => "/~",
                Direction::SouthEast => "|\\",
                Direction::South => "\\|/",
                Direction::SouthWest => "//|",
                Direction::West => "~/",
                Direction::NorthWest => "\\|",
            },
            BranchType::GrowingWest => match self.direction {
                Direction::North => "/|",
                Direction::NorthEast => "|/",
                Direction::East => "~",
                Direction::SouthEast => "\\\\",
                Direction::South => "|\\",
                Direction::SouthWest => "//",
                Direction::West => "=", // NOTE: rn GrowingWest and GrowingEast char mappings are the same except for West / East.
                Direction::NorthWest => "\\\\",
            },
            BranchType::GrowingEast => match self.direction {
                Direction::North => "|\\",
                Direction::NorthEast => "|/",
                Direction::East => "=",
                Direction::SouthEast => "\\\\",
                Direction::South => "|\\",
                Direction::SouthWest => "//",
                Direction::West => "~",
                Direction::NorthWest => "\\\\",
            },
            BranchType::Stem => "&",
            BranchType::Leaf => "*",
        };

        match self.branch_type {
            BranchType::GrowingNorth => TreeCell {
                ch: str.truecolor(150, 75, 0),
            },
            BranchType::GrowingWest => TreeCell {
                ch: str.truecolor(150, 96, 77),
            },
            BranchType::GrowingEast => TreeCell {
                ch: str.truecolor(150, 96, 77),
            },
            BranchType::Stem => TreeCell { ch: str.green() },
            BranchType::Leaf => TreeCell {
                ch: str.bright_green(),
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum BranchType {
    GrowingNorth,
    GrowingWest,
    GrowingEast,
    Stem,
    Leaf,
}
