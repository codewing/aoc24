use std::{
    fmt::{Display, Write},
    ops::Index,
};

use crate::{field::Field, movement::Movement, utils::index, vec::Vec2i};

pub struct Storage {
    content: Vec<Field>,
    width: usize,
    height: usize,
    robot_pos: Vec2i,
}

impl Storage {
    pub fn new(map_lines: Vec<&str>) -> Self {
        let width = map_lines.first().unwrap().len();
        let height = map_lines.len();

        let mut content = Vec::with_capacity(width * height);

        let mut robot_pos = Vec2i { x: 0, y: 0 };

        for (y, line) in map_lines.iter().enumerate() {
            for (x, elem) in line.chars().into_iter().enumerate() {
                let field = Field::try_from(elem).unwrap();

                if field == Field::Robot {
                    robot_pos = Vec2i {
                        x: x as i32,
                        y: y as i32,
                    }
                }

                content.push(field);
            }
        }

        Self {
            content,
            width,
            height,
            robot_pos,
        }
    }

    fn at(&self, pos: &Vec2i) -> &Field {
        let index = index(pos, self.width);
        self.content.index(index)
    }

    pub fn move_robot(&mut self, movement: &Movement) {
        let dir: Vec2i = movement.into();

        let next_pos = self.robot_pos + dir;
        let next_element = self.at(&next_pos);

        let has_moved = match next_element {
            Field::Empty => {
                self.content.swap(
                    index(&self.robot_pos, self.width),
                    index(&next_pos, self.width),
                );
                true
            }
            Field::Box => match self.can_push_until(dir) {
                Some(target_pos) => {
                    let mut current_pos = target_pos;
                    let robot_pos = self.robot_pos;

                    while current_pos != robot_pos {
                        let next_pos = current_pos - dir;
                        self.content.swap(
                            index(&current_pos, self.width),
                            index(&next_pos, self.width),
                        );

                        current_pos = next_pos;
                    }

                    true
                }
                None => false,
            },
            Field::Robot => false,
            Field::Wall => false,
        };

        if has_moved {
            self.robot_pos = next_pos;
        }
    }

    fn can_push_until(&mut self, dir: Vec2i) -> Option<Vec2i> {
        let mut current_pos = self.robot_pos + dir;
        let mut current_element = self.at(&current_pos);

        while current_element == &Field::Box {
            current_pos = current_pos + dir;
            current_element = self.at(&current_pos)
        }

        if current_element == &Field::Empty {
            Some(current_pos)
        } else {
            None
        }
    }

    pub fn calculate_gps_sum(&self) -> usize {
        self.content
            .iter()
            .enumerate()
            .filter_map(|(index, field)| match field {
                Field::Box => {
                    let x = index % self.width;
                    let y = index / self.width;

                    Some(x + y * 100)
                }
                _ => None,
            })
            .sum()
    }
}

impl Display for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, elem) in self.content.iter().enumerate() {
            if i % self.width == 0 && i != 0 {
                f.write_char('\n')?;
            }
            elem.fmt(f)?;
        }
        std::fmt::Result::Ok(())
    }
}
