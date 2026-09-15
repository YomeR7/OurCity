/// Building position on the grid.
#[derive(bevy::prelude::Component)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    /// Center of the tile, on the ground plane.
    pub fn world(&self) -> bevy::prelude::Vec3 {
        bevy::prelude::Vec3::new(self.x as f32, 0.0, self.y as f32)
    }
}
