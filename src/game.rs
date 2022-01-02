use crate::prelude::*;
use std::collections::HashMap;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

enum GameState {
    Loading,
    Intro,
    Uncovering,
    Covering,
    Running,
    Dead,
    Completed,
}

enum MagicWall {
    Off,
    On,
    Expired,
}
impl Default for MagicWall {
    fn default() -> Self {
        MagicWall::Off
    }
}

#[derive(Default)]
struct CurrentPlayerData {
    cave: u8,
    score: u32,
    lives: u8,
    flash_screen: bool,
    round: u8,
    next_bonus_life_score: u32,
    current_diamond_value: u8,
    diamonds_collected: u8,
    got_enough_diamonds: bool,
}

#[derive(Default)]
struct GameData {
    num_rounds_since_rockford_seen_alive: usize,
    rockford_animation_facing_direction: Action,
    rockford_moving: bool,
    rockford_movement: Action,
    rockford_sprite: usize,
    rockford_frame: usize,
    tapping: bool,
    blinking: bool,
    point_to_transfer: usize,
    amoeba_sound: bool,
    number_of_amoeba_found_this_frame: usize,
    total_amoeba_found_last_frame: usize,
    amoeba_suffocated_last_frame: bool,
    at_least_one_amoeba_found_this_frame_which_can_grow: bool,
    an_amoeba_random_factor: usize,
    start_magic_wall: usize,
}

#[derive(Default)]
struct Level {
    tiles: Vec<Tile>,
    magic_wall_milling_time: u8,
    diamond_point: u8,
    extra_diamond_value: u8,
    diamonds_needed: u8,
    cave_time: u16,
    number_of_amoeba_found_this_frame: u16,
    total_amoeba_found_last_frame: u16,
    amoeba_suffocated_last_frame: bool,
    at_least_one_amoeba_found_this_frame_can_grow: bool,
    an_amoeba_random_factor: u8,
    magic_wall_status_magic_wall: MagicWall,
}
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Object {
    Space,
    Dirt,
    Boulder,
    Diamond,
    ExpandingWall,
    Rockford,
    Firefly,
    Butterfly,
    Amoeba,
    BrickWall,
    SteelWall,
    MagicWall,
    Inbox,
    Outbox,
    Slime,
    Explosion,
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Action {
    None,
    Stationary,
    Falling,
    RollingLeft,
    RollingRight,
    MovingUp,
    MovingDown,
    MovingLeft,
    MovingRight,
}

impl Default for Action {
    fn default() -> Self {
        Action::None
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Facing {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(PartialEq, Copy, Clone)]
enum Turning {
    Left,
    Right,
    Ahead,
}

struct Properties {
    texture: usize,
    rounded: bool,
    animated: bool,
    impact_explosive: bool,
}

struct Status {
    action: Action,
    facing: Facing,
}

struct Tile {
    object: Object,
    status: Status,
    scanned: bool,
}

pub struct Game<'a> {
    pub running: bool,
    state: GameState,
    pub textures: Vec<sdl2::render::Texture<'a>>,
    tick: i32,
    frame_counter: i32,
    keys: Keys,
    demo_mode: bool,
    current_player_data: CurrentPlayerData,
    data: GameData,
    level: Level,
    tiles_properties: HashMap<Object, Properties>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let mut hm = HashMap::new();
        hm.insert(
            Object::Space,
            Properties {
                texture: 16,
                rounded: false,
                animated: false,
                impact_explosive: false,
            },
        );
        hm.insert(
            Object::Dirt,
            Properties {
                texture: 11,
                rounded: false,
                animated: false,
                impact_explosive: false,
            },
        );
        hm.insert(
            Object::Boulder,
            Properties {
                texture: 10,
                rounded: true,
                animated: false,
                impact_explosive: false,
            },
        );
        hm.insert(
            Object::Diamond,
            Properties {
                texture: 40,
                rounded: true,
                animated: true,
                impact_explosive: false,
            },
        );
        hm.insert(
            Object::BrickWall,
            Properties {
                texture: 1,
                rounded: true,
                animated: false,
                impact_explosive: false,
            },
        );
        hm.insert(
            Object::SteelWall,
            Properties {
                texture: 7,
                rounded: false,
                animated: false,
                impact_explosive: false,
            },
        );
        hm.insert(
            Object::Rockford,
            Properties {
                texture: 9,
                rounded: false,
                animated: false,
                impact_explosive: true,
            },
        );
        hm.insert(
            Object::Firefly,
            Properties {
                texture: 32,
                rounded: false,
                animated: false,
                impact_explosive: true,
            },
        );
        hm.insert(
            Object::Butterfly,
            Properties {
                texture: 48,
                rounded: false,
                animated: false,
                impact_explosive: true,
            },
        );
        hm.insert(
            Object::Explosion,
            Properties {
                texture: 2,
                rounded: false,
                animated: false,
                impact_explosive: false,
            },
        );
        hm.insert(
            Object::Amoeba,
            Properties {
                texture: 24,
                rounded: false,
                animated: true,
                impact_explosive: false,
            },
        );
        hm.insert(
            Object::Inbox,
            Properties {
                texture: 2,
                rounded: false,
                animated: false,
                impact_explosive: false,
            },
        );
        hm.insert(
            Object::Outbox,
            Properties {
                texture: 7,
                rounded: false,
                animated: false,
                impact_explosive: false,
            },
        );
        hm.insert(
            Object::MagicWall,
            Properties {
                texture: 1,
                rounded: false,
                animated: false,
                impact_explosive: false,
            },
        );
        Game {
            running: true,
            state: GameState::Loading,
            textures: Vec::new(),
            tick: 0,
            keys: Keys::new(),
            demo_mode: false,
            current_player_data: CurrentPlayerData::default(),
            level: Level::default(),
            data: GameData::default(),
            tiles_properties: hm,
            frame_counter: 0,
        }
    }

    pub fn set_intro_state(&mut self) {
        self.state = GameState::Intro;
    }

    fn set_new_game(&mut self) {
        self.current_player_data.cave = 0;
        self.current_player_data.flash_screen = false;
        self.current_player_data.lives = 3;
        self.current_player_data.next_bonus_life_score = 500;
        self.current_player_data.score = 0;
        self.current_player_data.round = 1;
    }
    fn set_uncovering_state(&mut self) {
        let cave = crate::caves::get_cave(self.current_player_data.cave as usize);
        self.state = GameState::Running;
        self.level.tiles = self.parse_cave(cave.layout);
        //console.log(level.tiles);
        self.level.magic_wall_milling_time = cave.amoeba_wall_time;
        self.level.diamond_point = cave.diamond_point;
        self.level.extra_diamond_value = cave.extra_point;
        self.level.diamonds_needed =
            cave.diamond_needed[(self.current_player_data.round - 1) as usize];
        self.level.cave_time = cave.cave_time[(self.current_player_data.round - 1) as usize];
        self.current_player_data.current_diamond_value = self.level.diamond_point;
        self.current_player_data.diamonds_collected = 0;
        self.current_player_data.got_enough_diamonds = false;
        // inizializzo variabili di gioco che sposto in level
        self.level.number_of_amoeba_found_this_frame = 0;
        self.level.total_amoeba_found_last_frame = 0;
        self.level.amoeba_suffocated_last_frame = false;
        self.level.at_least_one_amoeba_found_this_frame_can_grow = false;
        self.level.an_amoeba_random_factor = 64;
        self.level.magic_wall_status_magic_wall = MagicWall::Off;

        // TOGLIERE DOPO
        self.tick = 0;
    }

    fn scan(&mut self) {
        for t in self.level.tiles.iter_mut() {
            t.scanned = false;
        }
        for i in 0..NUM_TILES {
            if !self.level.tiles[i].scanned {
                match self.level.tiles[i].object {
                    Object::Butterfly => self.scan_butterfly(i),
                    Object::Firefly => self.scan_firefly(i),
                    Object::Rockford => self.scan_rockford(i),
                    _ => {}
                }
            }
            self.level.tiles[i].scanned = true;
        }
    }

    fn fly_will_explode(&self, i: usize) -> bool {
        false
    }
    fn explode(&mut self, i: usize, preferred: Turning) {}

    fn get_object_at_position(&self, p: usize) -> Object {
        self.level.tiles[p].object
    }

    fn place_object(&mut self, object: Object, action: Action, facing: Facing, i: usize) {
        self.level.tiles[i].object = object;
        self.level.tiles[i].status = Status { action, facing };
        // forse anceh scanned?
        self.level.tiles[i].scanned = true;
    }

    fn scan_rockford(&mut self, i: usize) {
        self.move_rockford_stage1(i);
    }

    fn move_rockford_stage1(&mut self, i: usize) {
        let mut joy = Action::None;
        if self.keys.up_pressed {
            joy = Action::MovingUp;
        }
        if self.keys.down_pressed {
            joy = Action::MovingDown;
        }
        if self.keys.left_pressed {
            joy = Action::MovingLeft;
        }
        if self.keys.right_pressed {
            joy = Action::MovingRight;
        }

        let mut new_position = i;

        match joy {
            Action::MovingUp => {
                self.data.rockford_moving = true;
                self.data.rockford_movement = Action::MovingUp;
                new_position = i - MAP_WIDTH;
            }
            Action::MovingDown => {
                self.data.rockford_moving = true;
                self.data.rockford_movement = Action::MovingDown;
                new_position = i + MAP_WIDTH;
            }
            Action::MovingLeft => {
                self.data.rockford_moving = true;
                self.data.rockford_movement = Action::MovingLeft;
                self.data.rockford_animation_facing_direction = Action::MovingLeft;
                new_position = i - 1;
            }
            Action::MovingRight => {
                self.data.rockford_moving = true;
                self.data.rockford_movement = Action::MovingRight;
                self.data.rockford_animation_facing_direction = Action::MovingRight;
                new_position = i + 1;
            }
            _ => {
                self.data.rockford_moving = false;
                self.level.tiles[i].status.action = Action::None;
            }
        }

        if self.data.rockford_moving {
            let actually_moved = self.move_rockford_stage2(i, new_position, joy);
            if actually_moved {
            } else {
                self.level.tiles[i].status.action = Action::None;
            }
        }
    }

    fn move_rockford_stage2(&mut self, i: usize, new_position: usize, joy: Action) -> bool {
        let mut actually_moved = self.move_rockford_stage3(new_position, joy);

        // If the movement was successful, we check the fire button to determine
        // whether Rockford actually physically moves to the new positon or not.
        if actually_moved {
            if (self.keys.shift_pressed) {
                self.place_object(Object::Space, Action::None, Facing::None, new_position);
                actually_moved = false;
            } else {
                self.place_object(
                    Object::Rockford,
                    self.data.rockford_movement,
                    Facing::None,
                    new_position,
                );
                self.place_object(Object::Space, Action::None, Facing::None, i);
            }
        }

        actually_moved
    }

    fn move_rockford_stage3(&mut self, new_position:usize, joy:Action)-> bool {
        let mut movement_successful = false;
        let obj = self.get_object_at_position(new_position);
        match obj {
            Object::Space => {
                movement_successful = true;
            },
            Object::Dirt => {
                movement_successful = true;
            }
            Object::Diamond => {
                movement_successful = true;
            }
            Object::Outbox => {
                movement_successful = true;
            }
            Object::Boulder => {
                movement_successful = true;
            }
            _=>{}
        }
        movement_successful
    }

    fn scan_butterfly(&mut self, i: usize) {
        //let NewPosition;
        //let NewDirection;
        let direction_of_fly = self.level.tiles[i].status.facing;

        // First check whether the firefly will explode by being next to Rockford,
        // Rockford-scanned-this-frame or amoeba but not amoeba-scanned-this-frame.
        if self.fly_will_explode(i) {
            self.explode(i, Turning::Left);
        } else {
            // Failing that, attempt to move turn left and move there if possible
            let new_position = get_next_fly_position(i, direction_of_fly, Turning::Right);
            if self.get_object_at_position(new_position) == Object::Space {
                let new_direction = get_new_direction(direction_of_fly, Turning::Right);
                self.place_object(
                    self.level.tiles[i].object,
                    set_move_from_dir(new_direction),
                    new_direction,
                    new_position,
                );
                self.place_object(Object::Space, Action::None, Facing::None, i);
            // ie old position
            } else {
                // Failing that, attempt to move straight ahead
                let new_position = get_next_fly_position(i, direction_of_fly, Turning::Ahead);
                if self.get_object_at_position(new_position) == Object::Space {
                    self.place_object(
                        self.level.tiles[i].object,
                        set_move_from_dir(direction_of_fly),
                        direction_of_fly,
                        new_position,
                    ); // ie keep same direction
                    self.place_object(Object::Space, Action::None, Facing::None, i);
                // ie old position
                } else {
                    // Failing that, turn to the right but do not move
                    let new_direction = get_new_direction(direction_of_fly, Turning::Left);
                    //PlaceObject(type, NewDirection, positionOfFirefly); // old position, new direction
                    self.level.tiles[i].status.facing = new_direction;
                    self.level.tiles[i].status.action = Action::None;
                }
            }
        }
    }

    fn scan_firefly(&mut self, i: usize) {
        //let NewPosition;
        //let NewDirection;
        let direction_of_fly = self.level.tiles[i].status.facing;

        // First check whether the firefly will explode by being next to Rockford,
        // Rockford-scanned-this-frame or amoeba but not amoeba-scanned-this-frame.
        if self.fly_will_explode(i) {
            self.explode(i, Turning::Left);
        } else {
            // Failing that, attempt to move turn left and move there if possible
            let new_position = get_next_fly_position(i, direction_of_fly, Turning::Left);
            if self.get_object_at_position(new_position) == Object::Space {
                let new_direction = get_new_direction(direction_of_fly, Turning::Left);
                self.place_object(
                    self.level.tiles[i].object,
                    set_move_from_dir(new_direction),
                    new_direction,
                    new_position,
                );
                self.place_object(Object::Space, Action::None, Facing::None, i);
            // ie old position
            } else {
                // Failing that, attempt to move straight ahead
                let new_position = get_next_fly_position(i, direction_of_fly, Turning::Ahead);
                if self.get_object_at_position(new_position) == Object::Space {
                    self.place_object(
                        self.level.tiles[i].object,
                        set_move_from_dir(direction_of_fly),
                        direction_of_fly,
                        new_position,
                    ); // ie keep same direction
                    self.place_object(Object::Space, Action::None, Facing::None, i);
                // ie old position
                } else {
                    // Failing that, turn to the right but do not move
                    let new_direction = get_new_direction(direction_of_fly, Turning::Right);
                    //PlaceObject(type, NewDirection, positionOfFirefly); // old position, new direction
                    self.level.tiles[i].status.facing = new_direction;
                    self.level.tiles[i].status.action = Action::None;
                }
            }
        }
    }

    fn animate_rockford(&mut self) {}

    fn inc_ticks(&mut self) {
        self.tick += 1;
        // CurrentPlayerData.levelTimeElapsed += TIME_STEP;
        if self.tick == 8 {
            self.tick = 0;
            self.frame_counter += 1;
            if self.frame_counter == 8 {
                self.frame_counter = 0;
            }
        }
    }

    pub fn update(&mut self) {
        match self.state {
            GameState::Intro => {
                self.tick += 1;
                if self.keys.enter_pressed {
                    self.demo_mode = false;
                    self.set_new_game();
                    self.set_uncovering_state();
                }
            }
            GameState::Uncovering => {}
            GameState::Running => {
                if self.tick == 7 {
                    //numRoundsSinceRockfordSeenAlive++;
                    //UpdateTime();
                    self.scan();
                }
                /*
                if (numRoundsSinceRockfordSeenAlive > 10) {
                   //console.log("set dead not seen!", numRoundsSinceRockfordSeenAlive)
                    SetDeadState();
                }
                if (Input.Keys.enterFull && demo_mode) {
                    demo_mode = false;
                    SetIntroState();
                }
                */
                self.animate_rockford();
                //console.log(Input.Keys.escapePressed && !demo_mode);
                /*
                if (Input.Keys.escapePressed && !demo_mode) {
                   //console.log("set dead key escape")
                    SetDeadState();
                    SetCoveringState();
                }
                */

                self.inc_ticks();
            }
            _ => {}
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas) {
        canvas.clear();
        match self.state {
            GameState::Intro => {
                let d1 = 320 * crate::ZOOM;
                let d2 = 8 * crate::ZOOM as i32;
                let pos = 3 * d2 - self.tick % d2;

                let r1 = Rect::new(
                    0 + crate::BORDER as i32,
                    pos + crate::BORDER as i32,
                    d1,
                    16 * d2 as u32,
                );
                let r2 = Rect::new(
                    0 + crate::BORDER as i32,
                    2 * d2 as i32 + crate::BORDER as i32,
                    d1,
                    17 * d2 as u32,
                );
                canvas.copy(&self.textures[1], None, r1).unwrap();
                canvas.copy(&self.textures[0], None, r2).unwrap();
                // "CICCIOCAPPA PRESENTS"
                self.write_line(
                    canvas,
                    &[
                        241, 19, 25, 19, 19, 25, 31, 19, 17, 32, 32, 17, 16, 32, 34, 21, 35, 21,
                        30, 36, 35,
                    ],
                    0,
                    0,
                );
                // "   BY PETER LIEPA   "
                self.write_line(
                    canvas,
                    &[
                        240, 16, 16, 16, 18, 41, 16, 32, 21, 36, 21, 34, 16, 28, 25, 21, 32, 17,
                        16, 16, 16,
                    ],
                    0,
                    19 * d2,
                );
                //  "   WITH CHRIS GRAY  "
                self.write_line(
                    canvas,
                    &[
                        16, 16, 16, 39, 25, 36, 24, 16, 19, 24, 34, 25, 35, 16, 23, 34, 17, 41, 16,
                        16,
                    ],
                    0,
                    20 * d2,
                );
                // "PRESS BUTTON TO PLAY",
                self.write_line(
                    canvas,
                    &[
                        32, 34, 21, 35, 35, 16, 18, 37, 36, 36, 31, 30, 16, 36, 31, 16, 32, 28, 17,
                        41,
                    ],
                    0,
                    21 * d2,
                );
                //  "1 PLAYER  1 JOYSTICK",
                self.write_line(
                    canvas,
                    &[
                        242, 1, 240, 16, 32, 28, 17, 41, 21, 34, 16, 16, 242, 1, 240, 16, 26, 31,
                        41, 35, 36, 25, 19, 27,
                    ],
                    0,
                    22 * d2,
                );
                // " CAVE: A  LEVEL: 1  "
                self.write_line(
                    canvas,
                    &[
                        16, 19, 17, 38, 21, 10, 16, 242, 17, 240, 16, 16, 28, 21, 38, 21, 28, 10,
                        16, 242, 1, 240, 16, 16,
                    ],
                    0,
                    23 * d2,
                );
            }
            GameState::Uncovering | GameState::Running => {
                //println!("{}",self.tick);
                for y in 0..13 {
                    for x in 0..21 {
                        let i = y * MAP_WIDTH + x;
                        let t = &self.level.tiles[i].object;
                        let texture = if self.tiles_properties[t].animated {
                            0
                        } else {
                            self.tiles_properties[t].texture as i32
                        };
                        let ottavo = 2 * ZOOM as i32;

                        let size = 16 * ZOOM;

                        let dx = match self.level.tiles[i].status.action {
                            Action::MovingLeft => (7 - self.tick) * ottavo,
                            Action::MovingRight => (-7 + self.tick) * ottavo,
                            _ => 0,
                        };
                        let dy = match self.level.tiles[i].status.action {
                            Action::MovingDown => (-7 + self.tick) * ottavo,
                            Action::MovingUp => (7 - self.tick) * ottavo,
                            _ => 0,
                        };

                        let py = (texture / 8) * (size as i32);
                        let px = (texture % 8) * (size as i32);
                        let src = Rect::new(px, py, size, size);
                        let dst = Rect::new(
                            (x as u32 * size + BORDER) as i32 + dx,
                            ((y + 1) as u32 * size + BORDER) as i32 + dy,
                            size,
                            size,
                        );
                        if *t != Object::Space {
                            canvas.copy(&self.textures[2], src, dst).unwrap();
                        }
                        //if *t==Object::Firefly {
                        //    println!("action:{:?}, direction:{:?}, tick:{}, pos:{},{}, tile: {},{}", self.level.tiles[i].status.action, self.level.tiles[i].status.facing, self.tick, dst.left(), dst.top(),x,y);
                        //}
                    }
                }
            }
            _ => {}
        }
        canvas.present();
    }

    fn write_line(&mut self, canvas: &mut Canvas, text: &[i32], px: i32, py: i32) {
        let size = 16 * ZOOM as i32;
        let scaley: i32 = size / 2;
        let base: i32 = 13 * size as i32;
        let mut colcodes: i32 = 0;
        for (i, byte) in text.iter().enumerate() {
            if *byte >= 240 {
                match *byte {
                    // 0xffffff white 0Xbfce72 yellow  0x8b3f96 purple
                    240 => self.textures[2].set_color_mod(0xFF, 0xFF, 0xFF),
                    241 => self.textures[2].set_color_mod(0xBF, 0xCE, 0x72),
                    242 => self.textures[2].set_color_mod(0x8B, 0x3F, 0x96),
                    _ => {}
                }
                colcodes += 1;
            } else {
                let sx = byte & 7;
                let sy = byte >> 3;
                let src = Rect::new(sx * size, base + sy * scaley, size as u32, scaley as u32);
                let dst = Rect::new(
                    px + (i as i32 - colcodes) * size + crate::BORDER as i32,
                    py + crate::BORDER as i32,
                    size as u32,
                    scaley as u32,
                );
                canvas
                    .copy_ex(&self.textures[2], src, dst, 0.0, None, false, false)
                    .unwrap();
            }
        }
    }

    pub fn get_input(&mut self, event_pump: &mut sdl2::EventPump) {
        self.keys.update(event_pump);
        if self.keys.request_quit {
            self.running = false;
        }
    }

    fn parse_cave(&mut self, map: &str) -> Vec<Tile> {
        let mut tileset = Vec::new();
        for b in map.bytes() {
            match b {
                b' ' => tileset.push(Tile {
                    object: Object::Space,
                    status: Status {
                        action: Action::None,
                        facing: Facing::None,
                    },
                    scanned: false,
                }),
                b'W' => tileset.push(Tile {
                    object: Object::SteelWall,
                    status: Status {
                        action: Action::None,
                        facing: Facing::None,
                    },
                    scanned: false,
                }),
                b'.' => tileset.push(Tile {
                    object: Object::Dirt,
                    status: Status {
                        action: Action::None,
                        facing: Facing::None,
                    },
                    scanned: false,
                }),
                b'w' => tileset.push(Tile {
                    object: Object::BrickWall,
                    status: Status {
                        action: Action::None,
                        facing: Facing::None,
                    },
                    scanned: false,
                }),
                b'r' => tileset.push(Tile {
                    object: Object::Boulder,
                    status: Status {
                        action: Action::Stationary,
                        facing: Facing::None,
                    },
                    scanned: false,
                }),
                b'd' => tileset.push(Tile {
                    object: Object::Diamond,
                    status: Status {
                        action: Action::Stationary,
                        facing: Facing::None,
                    },
                    scanned: false,
                }),
                b'X' => tileset.push(Tile {
                    object: Object::Inbox,
                    status: Status {
                        action: Action::None,
                        facing: Facing::None,
                    },
                    scanned: false,
                }),
                b'P' => tileset.push(Tile {
                    object: Object::Outbox,
                    status: Status {
                        action: Action::None,
                        facing: Facing::None,
                    },
                    scanned: false,
                }),
                b'b' => tileset.push(Tile {
                    object: Object::Butterfly,
                    status: Status {
                        action: Action::None,
                        facing: Facing::Down,
                    },
                    scanned: false,
                }),
                b'q' => tileset.push(Tile {
                    object: Object::Firefly,
                    status: Status {
                        action: Action::None,
                        facing: Facing::Left,
                    },
                    scanned: false,
                }),
                b'R' => tileset.push(Tile {
                    object: Object::Rockford,
                    status: Status {
                        action: Action::None,
                        facing: Facing::None,
                    },
                    scanned: false,
                }),

                _ => {
                    println!("unknow tile: {}", b)
                }
            }
        }
        tileset
    }
}

fn get_new_direction(direction_of_fly: Facing, turning: Turning) -> Facing {
    if turning == Turning::Left {
        match direction_of_fly {
            Facing::Up => Facing::Left,
            Facing::Left => Facing::Down,
            Facing::Down => Facing::Right,
            Facing::Right => Facing::Up,
            Facing::None => Facing::None,
        }
    } else if turning == Turning::Right {
        match direction_of_fly {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::None => Facing::None,
        }
    } else {
        direction_of_fly
    }
}

fn get_next_fly_position(i: usize, direction: Facing, preferred: Turning) -> usize {
    let new_direction = get_new_direction(direction, preferred);
    let new_pos = match new_direction {
        Facing::Up => i - MAP_WIDTH,
        Facing::Down => i + MAP_WIDTH,
        Facing::Left => i - 1,
        Facing::Right => i + 1,
        Facing::None => i,
    };
    new_pos
}

pub fn set_move_from_dir(facing: Facing) -> Action {
    match facing {
        Facing::Up => Action::MovingUp,
        Facing::Down => Action::MovingDown,
        Facing::Left => Action::MovingLeft,
        Facing::Right => Action::MovingRight,
        Facing::None => Action::None,
    }
}
