use rand::{random, thread_rng, Rng};
use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy)] // RUST: Enable "copy semantics". Copy, instead of move, when reassigning a variable.
pub struct Vec3 {
  components: [f64; 3], // RUST: 64-bit floating-point is roughly the same speed as 32-bit but is capable of more precision.
}

// Type aliases.
pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
  // Constructors.
  pub fn new(c1: f64, c2: f64, c3: f64) -> Vec3 {
    Vec3 {
      components: [c1, c2, c3],
    }
  }

  pub fn random() -> Vec3 {
    let c1 = random::<f64>();
    let c2 = random::<f64>();
    let c3 = random::<f64>();

    Vec3 {
      components: [c1, c2, c3],
    }
  }

  // pub fn random_in_hemisphere(surface_normal: Vec3) -> Vec3 {
  //   let random_in_unit_sphere = Vec3::random_in_unit_sphere();
  //   // In the same hemisphere as the surface normal.
  //   if random_in_unit_sphere.dot(surface_normal) > 0.0 {
  //     return random_in_unit_sphere;
  //   }
  //   return -random_in_unit_sphere;
  // }

  pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = thread_rng();

    loop {
      let c1 = rng.gen_range(-1.0, 1.0);
      let c2 = rng.gen_range(-1.0, 1.0);
      let c3 = 0.0;

      let random_point = Vec3::new(c1, c2, c3);
      if random_point.length_squared() < 1.0 {
        break random_point;
      }
    }
  }

  pub fn random_in_unit_sphere() -> Vec3 {
    loop {
      let random_point = Vec3::random_within_range(-1.0, 1.0);
      // 1.0 is the radius of the unit sphere.
      if random_point.length_squared() < 1.0 {
        break random_point;
      }
    }
  }

  pub fn random_unit_vector() -> Vec3 {
    let mut rng = thread_rng();

    let a = rng.gen_range(0.0, PI * 2.0);
    let z = rng.gen_range(-1.0, 1.0);
    let r = ((1.0 - (z * z)) as f64).sqrt();

    let c1 = a.cos() * r;
    let c2 = a.sin() * r;
    let c3 = z;

    Vec3 {
      components: [c1, c2, c3],
    }
  }

  pub fn random_within_range(min: f64, max: f64) -> Vec3 {
    let mut rng = thread_rng();

    let c1 = rng.gen_range(min, max);
    let c2 = rng.gen_range(min, max);
    let c3 = rng.gen_range(min, max);

    Vec3 {
      components: [c1, c2, c3],
    }
  }

  pub fn zero() -> Vec3 {
    Vec3 {
      components: [0.0, 0.0, 0.0],
    }
  }

  // Getters.
  pub fn x(&self) -> f64 {
    self.components[0]
  }

  pub fn y(&self) -> f64 {
    self.components[1]
  }

  pub fn z(&self) -> f64 {
    self.components[2]
  }

  // Utility functions.
  pub fn cross(&self, vector: Vec3) -> Vec3 {
    Vec3 {
      components: [
        (self.components[1] * vector.components[2]) - (self.components[2] * vector.components[1]),
        (self.components[2] * vector.components[0]) - (self.components[0] * vector.components[2]),
        (self.components[0] * vector.components[1]) - (self.components[1] * vector.components[0]),
      ],
    }
  }

  pub fn dot(&self, vector: Vec3) -> f64 {
    (self.components[0] * vector.components[0])
      + (self.components[1] * vector.components[1])
      + (self.components[2] * vector.components[2])
  }

  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn length_squared(&self) -> f64 {
    (self.components[0] * self.components[0])
      + (self.components[1] * self.components[1])
      + (self.components[2] * self.components[2])
  }

  // Return vector normalized to magnitude (length) of 1.
  pub fn unit_vector(&self) -> Vec3 {
    Vec3 {
      components: [
        self.components[0] / self.length(),
        self.components[1] / self.length(),
        self.components[2] / self.length(),
      ],
    }
  }
}

// Operator overloads ("traits").
// Addition operator: +
impl Add for Vec3 {
  type Output = Vec3;
  fn add(self, rhs: Vec3) -> Vec3 {
    Vec3 {
      components: [
        self.components[0] + rhs.components[0],
        self.components[1] + rhs.components[1],
        self.components[2] + rhs.components[2],
      ],
    }
  }
}

// Division operator: /
impl Div<f64> for Vec3 {
  type Output = Vec3;
  fn div(self, rhs: f64) -> Vec3 {
    self * (1.0 / rhs)
  }
}

// Multiplication operator: *
impl Mul for Vec3 {
  type Output = Vec3;
  fn mul(self, rhs: Vec3) -> Vec3 {
    Vec3 {
      components: [
        self.components[0] * rhs.components[0],
        self.components[1] * rhs.components[1],
        self.components[2] * rhs.components[2],
      ],
    }
  }
}

// Multiplication operator: *
impl Mul<Vec3> for f64 {
  type Output = Vec3;
  fn mul(self, rhs: Vec3) -> Vec3 {
    Vec3 {
      components: [
        self * rhs.components[0],
        self * rhs.components[1],
        self * rhs.components[2],
      ],
    }
  }
}

// Multiplication operator: *
impl Mul<f64> for Vec3 {
  type Output = Vec3;
  fn mul(self, rhs: f64) -> Vec3 {
    rhs * self
  }
}

// Unary negation operator: -
impl Neg for Vec3 {
  type Output = Vec3;
  fn neg(self) -> Vec3 {
    Vec3 {
      components: [
        -self.components[0],
        -self.components[1],
        -self.components[2],
      ],
    }
  }
}

// Subtraction operator: -
impl Sub for Vec3 {
  type Output = Vec3;
  fn sub(self, rhs: Vec3) -> Vec3 {
    Vec3 {
      components: [
        self.components[0] - rhs.components[0],
        self.components[1] - rhs.components[1],
        self.components[2] - rhs.components[2],
      ],
    }
  }
}
