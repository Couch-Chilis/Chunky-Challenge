use std::{
    borrow::Cow,
    cmp::Ordering,
    collections::{btree_map::Entry, BTreeMap},
    fmt::Write,
    str::FromStr,
};

use bevy::prelude::Resource;

use crate::game_object::{Direction, ObjectType, Position};

pub const LEVELS: &[(u16, &str)] = &[
    (0, include_str!("../assets/levels/level000")),
    (1, include_str!("../assets/levels/level001")),
    (2, include_str!("../assets/levels/level002")),
    (3, include_str!("../assets/levels/level003")),
    (4, include_str!("../assets/levels/level004")),
    (5, include_str!("../assets/levels/level005")),
    (6, include_str!("../assets/levels/level006")),
    (7, include_str!("../assets/levels/level007")),
    (8, include_str!("../assets/levels/level008")),
    (9, include_str!("../assets/levels/level009")),
    (10, include_str!("../assets/levels/level010")),
    (11, include_str!("../assets/levels/level011")),
    (12, include_str!("../assets/levels/level012")),
    (13, include_str!("../assets/levels/level013")),
    (14, include_str!("../assets/levels/level014")),
    (15, include_str!("../assets/levels/level015")),
    (16, include_str!("../assets/levels/level016")),
    (17, include_str!("../assets/levels/level017")),
    (18, include_str!("../assets/levels/level018")),
    (19, include_str!("../assets/levels/level019")),
    (20, include_str!("../assets/levels/level020")),
    (21, include_str!("../assets/levels/level021")),
    (22, include_str!("../assets/levels/level022")),
    (23, include_str!("../assets/levels/level023")),
    (24, include_str!("../assets/levels/level024")),
    (25, include_str!("../assets/levels/level025")),
    (26, include_str!("../assets/levels/level026")),
    (27, include_str!("../assets/levels/level027")),
    (28, include_str!("../assets/levels/level028")),
    (29, include_str!("../assets/levels/level029")),
    (30, include_str!("../assets/levels/level030")),
    (31, include_str!("../assets/levels/level031")),
    (32, include_str!("../assets/levels/level032")),
    (33, include_str!("../assets/levels/level033")),
    (56, include_str!("../assets/levels/level056")),
    (57, include_str!("../assets/levels/level057")),
    (66, include_str!("../assets/levels/level066")),
    (67, include_str!("../assets/levels/level067")),
    (71, include_str!("../assets/levels/level071")),
    (72, include_str!("../assets/levels/level072")),
    (73, include_str!("../assets/levels/level073")),
    (74, include_str!("../assets/levels/level074")),
    (76, include_str!("../assets/levels/level076")),
    (77, include_str!("../assets/levels/level077")),
    (78, include_str!("../assets/levels/level078")),
    (82, include_str!("../assets/levels/level082")),
    (84, include_str!("../assets/levels/level084")),
    (85, include_str!("../assets/levels/level085")),
    (86, include_str!("../assets/levels/level086")),
    (87, include_str!("../assets/levels/level087")),
    (88, include_str!("../assets/levels/level088")),
    (95, include_str!("../assets/levels/level095")),
    (96, include_str!("../assets/levels/level096")),
    (100, include_str!("../assets/levels/level100")),
];

#[derive(Resource)]
pub struct Levels(BTreeMap<u16, LevelData>);

impl Levels {
    /// Returns the contents of a level.
    pub fn get(&self, level: u16) -> Option<&str> {
        self.0.get(&level).map(|data| match &data.current {
            Some(current) => current.as_str(),
            None => data.stored.as_ref(),
        })
    }

    /// Inserts the contents as the new current state of the level.
    pub fn insert_current(&mut self, level: u16, contents: String) {
        if let Some(data) = self.0.get_mut(&level) {
            data.current = Some(contents);
        }
    }

    /// Inserts the contents as the new stored contents of the level.
    ///
    /// This should only be used by the level editor.
    pub fn insert_stored(&mut self, level: u16, contents: String) {
        let level_data = LevelData {
            stored: Cow::Owned(contents),
            current: None,
        };

        self.0.insert(level, level_data);
    }

    /// Resets the given level to its original state.
    pub fn reset_level(&mut self, level: u16) {
        if let Some(data) = self.0.get_mut(&level) {
            data.current = None;
        }
    }
}

impl Default for Levels {
    fn default() -> Self {
        Self(
            LEVELS
                .iter()
                .map(|(level_num, contents)| {
                    let data = LevelData {
                        current: None,
                        stored: (*contents).into(),
                    };

                    (*level_num, data)
                })
                .collect(),
        )
    }
}

struct LevelData {
    current: Option<String>,
    stored: Cow<'static, str>,
}

pub struct Level {
    pub dimensions: Dimensions,
    pub objects: BTreeMap<ObjectType, Vec<InitialPositionAndMetadata>>,
}

impl Level {
    pub fn load(content: &str) -> Self {
        let mut dimensions = Dimensions::default();
        let mut direction = None;
        let mut identifier = None;
        let mut level = None;
        let mut open = false;
        let mut objects: BTreeMap<ObjectType, Vec<InitialPositionAndMetadata>> = BTreeMap::new();

        let mut section_name = None;
        for line in content.lines() {
            let line = line.trim();

            if line.starts_with('[') && line.ends_with(']') {
                direction = None;
                identifier = None;
                level = None;
                open = false;
                section_name = Some(&line[1..line.len() - 1]);
                continue;
            }

            let Some((key, value)) = line.split_once('=') else {
                continue;
            };

            let Some(section_name) = section_name else {
                continue;
            };

            if section_name == "General" {
                match (key, value.parse()) {
                    ("Width", Ok(value)) => dimensions.width = value,
                    ("Height", Ok(value)) => dimensions.height = value,
                    (_, Ok(_)) => println!("Unknown key: {key}"),
                    (_, Err(error)) => println!("Invalid dimension in key {key}: {error}"),
                }
                continue;
            }

            let object_type = match ObjectType::from_str(section_name) {
                Ok(object_type) => object_type,
                Err(_) => {
                    println!("Unknown object type: {section_name}");
                    continue;
                }
            };

            if key == "Position" {
                let positions: Vec<InitialPositionAndMetadata> = value
                    .split(';')
                    .filter_map(|location| match location.split_once(',') {
                        Some((x, y)) => match (x.parse(), y.parse()) {
                            (Ok(x), Ok(y)) => Some(InitialPositionAndMetadata {
                                position: Position { x, y },
                                direction,
                                identifier,
                                level,
                                open,
                            }),
                            _ => {
                                println!("Invalid location ({x},{y})");
                                None
                            }
                        },
                        _ => None,
                    })
                    .collect();

                if !positions.is_empty() {
                    let entry = objects.entry(object_type);
                    match entry {
                        Entry::Occupied(mut entry) => entry.get_mut().extend(positions),
                        Entry::Vacant(entry) => {
                            entry.insert(positions);
                        }
                    }
                }
            } else if key == "Direction" {
                match Direction::from_str(value) {
                    Ok(value) => direction = Some(value),
                    Err(_) => {
                        println!("Unknown direction: {value}");
                    }
                }
            } else if key == "Identifier" {
                match value.parse() {
                    Ok(value) => identifier = Some(value),
                    Err(_) => {
                        println!("Cannot parse identifier: {value}");
                    }
                }
            } else if key == "Level" {
                match value.parse() {
                    Ok(value) => level = Some(value),
                    Err(_) => {
                        println!("Cannot parse level number: {value}");
                    }
                }
            } else if key == "Open" {
                match value {
                    "true" => open = true,
                    "false" => open = false,
                    _ => {
                        println!("Cannot parse open value: {value}");
                    }
                }
            } else {
                println!("Unknown key: {key}");
            }
        }

        Self {
            dimensions,
            objects,
        }
    }

    pub fn save(self) -> String {
        let Dimensions { width, height } = self.dimensions;

        let mut content = format!("[General]\nWidth={width}\nHeight={height}");

        for (object_type, mut positions) in self.objects {
            writeln!(content, "\n\n[{object_type}]").expect("writing failed");

            positions.sort_unstable_by(|a, b| match a.level.cmp(&b.level) {
                Ordering::Equal => match a.direction.cmp(&b.direction) {
                    Ordering::Equal => a.position.cmp(&b.position),
                    ordering => ordering,
                },
                ordering => ordering,
            });

            let mut current_direction = Direction::default();
            let mut current_identifier = 0;
            let mut current_level = 0;
            let mut current_open = false;
            let mut last_x = None;
            for InitialPositionAndMetadata {
                position,
                direction,
                identifier,
                level,
                open,
            } in positions
            {
                if let Some(direction) = direction {
                    if direction != current_direction {
                        if !content.ends_with('\n') {
                            content.push('\n');
                        }

                        writeln!(content, "Direction={direction}").expect("writing failed");
                        current_direction = direction;
                    }
                }

                if let Some(identifier) = identifier {
                    if identifier != current_identifier {
                        if !content.ends_with('\n') {
                            content.push('\n');
                        }

                        writeln!(content, "Identifier={identifier}").expect("writing failed");
                        current_identifier = identifier;
                    }
                }

                if let Some(level) = level {
                    if level != current_level {
                        if !content.ends_with('\n') {
                            content.push('\n');
                        }

                        writeln!(content, "Level={level}").expect("writing failed");
                        current_level = level;
                    }
                }

                if open != current_open {
                    if !content.ends_with('\n') {
                        content.push('\n');
                    }

                    writeln!(content, "Open={open}").expect("writing failed");
                    current_open = open;
                }

                if content.ends_with('\n') {
                    write!(content, "Position={position}").expect("writing failed");
                } else if last_x != Some(position.x) {
                    write!(content, "\nPosition={position}").expect("writing failed");
                } else {
                    write!(content, ";{position}").expect("writing failed");
                }

                last_x = Some(position.x);
            }
        }

        content.push('\n');
        content
    }
}

#[derive(Clone, Copy, Resource)]
pub struct Dimensions {
    pub width: i16,
    pub height: i16,
}

impl Default for Dimensions {
    fn default() -> Self {
        Self {
            width: 16,
            height: 16,
        }
    }
}

impl Dimensions {
    pub fn contains(&self, position: Position) -> bool {
        let Position { x, y } = position;
        x >= 1 && x <= self.width && y >= 1 && y <= self.height
    }
}

#[derive(Clone)]
pub struct InitialPositionAndMetadata {
    pub position: Position,
    pub direction: Option<Direction>,
    pub identifier: Option<u16>,
    pub level: Option<u16>,
    pub open: bool,
}

impl From<&Position> for InitialPositionAndMetadata {
    fn from(position: &Position) -> Self {
        Self {
            position: *position,
            direction: None,
            identifier: None,
            level: None,
            open: false,
        }
    }
}
