// Chapter 14: Groups

use crate::cones::Cone;
use crate::intersections::{intersect, Intersections};
use crate::rays::Ray;
use crate::tuples::{Point, Vector};
use crate::world::{ObjectIndex, World};

#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    pub members: Vec<ObjectIndex>,
}

impl Default for Group {
    fn default() -> Self {
        Group { members: vec![] }
    }
}

impl Group {
    pub fn new() -> Group {
        Group::default()
    }

    pub fn local_normal_at(&self, _local_point: &Point) -> Vector {
        panic!("local_normal_at() called on Group");
    }

    pub fn local_intersect<'a>(&'a self, local_ray: &Ray, world: &'a World) -> Intersections {
        let mut xs_all = vec![];
        for child in &self.members {
            let object = world.get_object_ref(child);
            xs_all.extend(intersect(&object, local_ray, Some(world)));
        }
        xs_all.sort_by(|a, b| a.t.total_cmp(&b.t));
        xs_all
    }

    fn members(&self) -> &Vec<ObjectIndex> {
        &self.members
    }
}

pub fn local_normal_at(c: &Cone, local_point: &Point) -> Vector {
    c.local_normal_at(local_point)
}

pub fn local_intersect<'a>(c: &'a Cone, local_ray: &Ray) -> Intersections<'a> {
    c.local_intersect(local_ray)
}

pub fn group() -> Group {
    Group::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Creating a new group
    #[test]
    fn creating_a_new_group() {
        let g = group();
        assert!(g.members().is_empty());
    }
}
