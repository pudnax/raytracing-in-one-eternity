use crate::vec3::Vec3;
pub struct Color(pub Vec3<f64>);

impl std::ops::Deref for Color {
    type Target = nalgebra::Vector3<f64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Color {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Color(nalgebra::Vector3::new(x, y, z))
    }

    pub fn write_to<W: std::io::Write>(&self, out: &mut W) -> Result<(), std::io::Error> {
        let color = self.map(|x| (255.999f64 * <f64>::from(x).powf(1. / 1.2)) as u8);
        out.write(format!("{} {} {}\n", color.x, color.y, color.z).as_bytes())?;
        Ok(())
    }
}
