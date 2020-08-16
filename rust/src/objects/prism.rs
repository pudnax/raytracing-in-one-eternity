use crate::{
    objects::{And, FlipNormals, Object, Rect, StaticX, StaticY, StaticZ},
    vec3::{Axis::*, Vec3},
    Material,
};
/// Generates a rectangular prism having min and max corners `p0` and `p1`.
pub fn rect_prism(p0: Vec3, p1: Vec3, material: Material) -> impl Object {
    And(
        And(
            Rect {
                orthogonal_to: StaticZ,
                range0: p0[X]..p1[X],
                range1: p0[Y]..p1[Y],
                k: p1[Z],
                material: material.clone(),
            },
            And(
                Rect {
                    orthogonal_to: StaticY,
                    range0: p0[X]..p1[X],
                    range1: p0[Z]..p1[Z],
                    k: p1[Y],
                    material: material.clone(),
                },
                Rect {
                    orthogonal_to: StaticX,
                    range0: p0[Y]..p1[Y],
                    range1: p0[Z]..p1[Z],
                    k: p1[X],
                    material: material.clone(),
                },
            ),
        ),
        And(
            FlipNormals(Rect {
                orthogonal_to: StaticZ,
                range0: p0[X]..p1[X],
                range1: p0[Y]..p1[Y],
                k: p0[Z],
                material: material.clone(),
            }),
            And(
                FlipNormals(Rect {
                    orthogonal_to: StaticY,
                    range0: p0[X]..p1[X],
                    range1: p0[Z]..p1[Z],
                    k: p0[Y],
                    material: material.clone(),
                }),
                FlipNormals(Rect {
                    orthogonal_to: StaticX,
                    range0: p0[Y]..p1[Y],
                    range1: p0[Z]..p1[Z],
                    k: p0[X],
                    material: material.clone(),
                }),
            ),
        ),
    )
}
