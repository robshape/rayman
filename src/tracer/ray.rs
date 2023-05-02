use crate::vec3::{Color, Point3, Vec3};

// A light ray. Photon.
pub struct Ray {
    direction: Vec3,
    origin: Point3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { direction, origin }
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    // Returns a position along the ray (position at t).
    pub fn point_at(&self, t: f64) -> Point3 {
        self.origin + (t * self.direction)
    }
}

// A light ray that has bounced of an object. Some light might have been absorbed (light
// attenuation).
pub struct ScatteredRay {
    light_attenuation: Color,
    ray: Ray,
}

impl ScatteredRay {
    pub fn new(origin: Point3, direction: Vec3, light_attenuation: Color) -> ScatteredRay {
        ScatteredRay {
            light_attenuation,
            ray: Ray::new(origin, direction),
        }
    }

    pub fn light_attenuation(&self) -> Color {
        self.light_attenuation
    }

    pub fn ray(&self) -> &Ray {
        &self.ray
    }
}
