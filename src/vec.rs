use std::ops::{Add, Div, Mul, Sub, Neg};

#[derive(Debug, Clone, Copy)]
#[derive(Default)]
//#[derive(PartialEq, Eq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub fn zeros() -> Vec3 {
        Vec3 {
            x: 0.0f32,
            y: 0.0f32,
            z: 0.0f32,
        }
    }

    pub fn ones() -> Vec3 {
        Vec3 {
            x: 1.0f32,
            y: 1.0f32,
            z: 1.0f32,
        }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn vec_mult(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn vec_div(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }

    pub fn dist(&self, other: &Vec3) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    pub fn len(&self) -> f32 {
        ((self.x).powi(2) + (self.y).powi(2) + (self.z).powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let length = Vec3::len(self);
        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn scale(&self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn modulo(k: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            x: k.x % v.x,
            y: k.y % v.y,
            z: k.z % v.z,
        }
    }

    pub fn abs(v: &Vec3) -> Vec3 {
        Vec3 {
            x: v.x.abs(),
            y: v.y.abs(),
            z: v.z.abs(),
        }

    }

    pub fn rot_vector180(n: &Vec3, v: &Vec3) -> Vec3 {
        n.scale(2.0 * (n.dot(v))) - *v
    }

    pub fn rot_vector(k: &Vec3, v: &Vec3, angle: f32) -> Vec3 { //angle cos can eb calced before
        return *v*angle.cos() + k.cross(v)*angle.sin() + *k*(k.dot(v))*(1.0-angle.cos())
    }

    // pub fn hemisphere_bounce(normal: &Vec3, v: &Vec3) -> Vec3 {
    //     //let local_z=normal;
    //     let local_x=normal.cross(v);
    //     let local_y=normal.cross(local_x);
    //     let theta = rand::random::<f32>().acos();
    //     let phi = rand::random::<f32>()*2.0*PI;
    //     let output = Vec3 {
    //         x:theta.sin()*phi.cos(),
    //         y:theta.sin()*phi.sin(),
    //         z:theta.cos(),
    //     }
    //     return output
    // }

    pub fn hemisphere_bounce(normal: &Vec3) -> Vec3 {
        let mut ray_out = Vec3{x:rand::random::<f32>(), y:rand::random::<f32>(), z:rand::random::<f32>()}*2.0-Vec3{x:1.0, y:1.0, z:1.0};
        while ray_out.dot(&ray_out) > 1.0 || normal.dot(&ray_out) < 0.0 {
            ray_out = Vec3{x:rand::random::<f32>(), y:rand::random::<f32>(), z:rand::random::<f32>()}*2.0-Vec3{x:1.0, y:1.0, z:1.0};
        } 
        Vec3::normalize(&ray_out)
    }

    pub fn sphere_bounce() -> Vec3 {
        let mut ray_out = Vec3{x:rand::random::<f32>(), y:rand::random::<f32>(), z:rand::random::<f32>()}*2.0-Vec3{x:1.0, y:1.0, z:1.0};
        while ray_out.dot(&ray_out) > 1.0 {
            ray_out = Vec3{x:rand::random::<f32>(), y:rand::random::<f32>(), z:rand::random::<f32>()}*2.0-Vec3{x:1.0, y:1.0, z:1.0};
        } 
        Vec3::normalize(&ray_out)
    }

    // pub fn hemisphere_bounce(n: &Vec3, v: &Vec3) -> Vec3 {
    //     let v1 = Vec3::normalize(v);
    //     let cross = Vec3::normalize(&v1.cross(n));
    //     let angle1 = rand::random::<f32>().acos();
    //     let angle2 = rand::random::<f32>()*2.0*PI;
    //     let mut v_rot1 = Vec3::rot_vector(&cross, &n, angle1);
    //     v_rot1 = Vec3::normalize(&v_rot1);
    //     let mut ray_out = Vec3::rot_vector(&n, &v_rot1, angle2);
    //     ray_out = Vec3::normalize(&ray_out);
    //     //println!("{:?}", Vec3::len(&ray_out)); 
    //     return ray_out
    // }

    pub fn rotate_y(v: &Vec3, angle1: f32) -> Vec3 {
        Vec3 {
            x: v.x*angle1.cos() + v.z*angle1.sin(),
            y: v.y,
            z: -v.x*angle1.sin() + v.z*angle1.cos(),
        }
    }
    
    pub fn rotate_z(v: &Vec3, angle1: f32) -> Vec3 {
        Vec3 {
            x: v.x*angle1.cos() + v.y*angle1.sin(),
            y: -v.x*angle1.sin() + v.y*angle1.cos(),
            z: v.z,
        }
    }

    // def glossy(ray, normal, max_angle):
    // v_rot0 = multiply(rot_vector180(normal, ray), -1)
    // cross = normalize(cross_product(ray, normal))
    // v_rot1 = rot_vector(cross, v_rot0, random.uniform(0,max_angle))
    // ray_out = rot_vector(v_rot0, v_rot1, random.uniform(0, 2*np.pi))
    // return ray_out

}

// //TODO may be removed
// impl PartialEq for Vec3 {
//     fn eq(&self, other: &Self) -> bool {
//         return (self.x == other.x) && (self.y == other.y) && (self.z == other.z);
//     }
// }
// //TODO may be removed
// impl Eq for Vec3 {}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
    pub fn len(&self) -> f32 {
        ((self.x).powi(2) + (self.y).powi(2)).sqrt()
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, other: f32) -> Vec2 {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, other: f32) -> Vec2 {
        Vec2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}