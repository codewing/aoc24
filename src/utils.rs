use crate::vec::Vec2i;

pub fn index(pos: &Vec2i, width: usize) -> usize {
    debug_assert!(pos.x >= 0 && pos.x < width as i32);
    debug_assert!(pos.y >= 0);

    pos.x as usize + pos.y as usize * width
}
