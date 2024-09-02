use std::{
    cmp::Ordering,
    collections::{btree_map::Entry, BTreeMap},
    str::FromStr,
};

use bevy::prelude::Resource;

use crate::game_object::{Direction, ObjectType, Position};

pub const LEVELS: &[&str] = &[
    include_str!("../assets/levels/level000"),
    include_str!("../assets/levels/level001"),
    include_str!("../assets/levels/level002"),
    include_str!("../assets/levels/level003"),
    include_str!("../assets/levels/level004"),
    include_str!("../assets/levels/level005"),
    include_str!("../assets/levels/level006"),
    include_str!("../assets/levels/level007"),
    include_str!("../assets/levels/level008"),
    include_str!("../assets/levels/level009"),
    include_str!("../assets/levels/level010"),
    include_str!("../assets/levels/level011"),
    include_str!("../assets/levels/level012"),
    include_str!("../assets/levels/level013"),
    include_str!("../assets/levels/level014"),
    include_str!("../assets/levels/level015"),
    include_str!("../assets/levels/level016"),
    include_str!("../assets/levels/level017"),
    include_str!("../assets/levels/level018"),
];

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

pub struct InitialPositionAndDirection {
    pub position: Position,
    pub direction: Option<Direction>,
}

pub struct Level {
    pub dimensions: Dimensions,
    pub objects: BTreeMap<ObjectType, Vec<InitialPositionAndDirection>>,
}

impl Level {
    pub fn load(content: &str) -> Self {
        let mut dimensions = Dimensions::default();
        let mut direction = None;
        let mut objects: BTreeMap<ObjectType, Vec<InitialPositionAndDirection>> = BTreeMap::new();

        let mut section_name = None;
        for line in content.lines() {
            let line = line.trim();

            if line.starts_with('[') && line.ends_with(']') {
                direction = None;
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
                let positions: Vec<InitialPositionAndDirection> = value
                    .split(';')
                    .filter_map(|location| match location.split_once(',') {
                        Some((x, y)) => match (x.parse(), y.parse()) {
                            (Ok(x), Ok(y)) => Some(InitialPositionAndDirection {
                                position: Position { x, y },
                                direction,
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

        let mut content = format!("[General]\nWidth={width}\nHeight={height}\n");

        for (object_type, mut positions) in self.objects {
            content.push_str(&format!("\n[{object_type}]\n"));

            positions.sort_unstable_by(|a, b| match a.direction.cmp(&b.direction) {
                Ordering::Equal => a.position.cmp(&b.position),
                ordering => ordering,
            });

            let mut current_direction = Direction::default();
            for InitialPositionAndDirection {
                position,
                direction,
            } in positions
            {
                if let Some(direction) = direction {
                    if direction != current_direction {
                        content.push_str(&format!("Direction={direction}\n"));
                        current_direction = direction;
                    }
                }

                content.push_str(&format!("Position={position}\n"));
            }
        }

        content
    }
}
