use rand::random;

use crate::scene::material::material::Material;
use crate::scene::material::reflective::Reflective;
use crate::tracer::hit_point::HitPoint;
use crate::tracer::ray::{Ray, ScatteredRay};
use crate::vec3::{Color, Vec3};

// Dielectric material can reflect light and at the same time let light pass through.
pub struct Dielectric {
    refractive_index: f64, // Describes how fast light travels through the material. This determines how much the path of light is bent when entering the material ("refracted").
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Dielectric {
        Dielectric { refractive_index }
    }

    // Snell's law is a formula used to describe the relationship between angles of incidence and
    // refraction. We use this to determine the direction of the refracted ray.
    pub fn refract(ray_direction: Vec3, surface_normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -ray_direction.dot(surface_normal);
        let perpendicular_ray = etai_over_etat * (ray_direction + (cos_theta * surface_normal));
        let parallel_ray =
            surface_normal * -((1.0 - perpendicular_ray.length_squared()).abs().sqrt());
        return perpendicular_ray + parallel_ray;
    }

    // Real glass has reflectivity that varies with angle; look at a window at a steep angle and it
    // becomes a mirror. Schlick's approximation is a cheap algorithm to achieve this, when perfect
    // precision is not necessary (in computer graphics, for example).
    pub fn schlick(cosine: f64, refractive_index: f64) -> f64 {
        let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        let r0 = r0 * r0;
        return r0 + ((1.0 - r0) * (1.0 - cosine).powi(5));
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_point: &HitPoint) -> ScatteredRay {
        let light_attenuation = Color::new(1.0, 1.0, 1.0); // Glass doesn't absorb light.
        let normalized_ray_direction = ray.direction().unit_vector();
        let point = hit_point.point();
        let surface_normal = hit_point.surface_normal();

        let cos_theta = -normalized_ray_direction.dot(surface_normal).min(1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();
        let etai_over_etat = if hit_point.front_facing() {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        // When a ray is inside an object with a higher refractive index it could end up being
        // reflected by the inner boundary. An example of this is when the boundary between water and
        // air acts as a perfect mirror while submerged. This is called "total internal reflection". So
        // if the glass material can't refract the ray then it must reflect it.
        if (etai_over_etat * sin_theta) > 1.0 {
            let scatter_direction = Reflective::reflect(normalized_ray_direction, surface_normal);
            return ScatteredRay::new(point, scatter_direction, light_attenuation);
        }

        // Both reflection and refraction occurs for dielectric materials. But in this implementation
        // we can only return one ray for the next iteration of tracing! However, since we are
        // multisampling pixels and using that to average out traced colors, we can take advantage of
        // that here by randomly reflecting some rays and refracting others. Depending on the
        // reflective coefficient.
        let reflect_probability = Dielectric::schlick(cos_theta, etai_over_etat);
        if random::<f64>() < reflect_probability {
            let scatter_direction = Reflective::reflect(normalized_ray_direction, surface_normal);
            return ScatteredRay::new(point, scatter_direction, light_attenuation);
        } else {
            let scatter_direction =
                Dielectric::refract(normalized_ray_direction, surface_normal, etai_over_etat);
            return ScatteredRay::new(point, scatter_direction, light_attenuation);
        }
    }
}
