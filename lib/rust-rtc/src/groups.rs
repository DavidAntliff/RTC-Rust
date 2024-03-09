// Chapter 14: Groups

use crate::cones::Cone;
use crate::intersections::{Intersections};
use crate::rays::Ray;
use crate::tuples::{Point, Vector};

#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    pub members: Vec<usize>,
}

impl Default for Group {
    fn default() -> Self {
        Group {
            members: vec![],
        }
    }
}

impl Group {
    pub fn new() -> Group { Group::default() }

    pub fn local_normal_at(&self, _local_point: &Point) -> Vector {
        panic!("local_normal_at() called on Group");
    }

    pub fn local_intersect(&self, _local_ray: &Ray) -> Intersections {
        todo!()
    }

    fn members(&self) -> Vec<usize> {
        vec![]
    }
}

pub fn local_normal_at(c: &Cone, local_point: &Point) -> Vector {
    c.local_normal_at(local_point)
}

pub fn local_intersect<'a>(c: &'a Cone, local_ray: &Ray) -> Intersections<'a> {
    c.local_intersect(local_ray)
}

pub fn group() -> Group { Group::default() }

#[cfg(test)]
mod tests {
    use super::*;

    // Creating a new group
    #[test]
    fn creating_a_new_group() {
        let g = group();
        assert_eq!(g.members(), vec![]);
    }


}
