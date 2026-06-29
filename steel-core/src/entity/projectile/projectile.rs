use glam::Vec3;
use steel_utils::random::Random;

pub trait Projectile {
    type R: Random;

    fn random(&mut self) -> &mut Self::R;

    fn movement_to_shoot(
        &mut self,
        xd: f32,
        yd: f32,
        zd: f32,
        power: f32, // is a f64 in vanilla but idk
        uncertainty: f32, // is a f64 in vanilla but idk
    ) -> Vec3 {
        let spread = 0.017_227_5 * uncertainty;
        let rng = self.random();

        (Vec3::new(xd, yd, zd).normalize()
            + Vec3::new(
                rng.triangle_f32(0.0, spread),
                rng.triangle_f32(0.0, spread),
                rng.triangle_f32(0.0, spread),
            ))
            * power
    }
}
