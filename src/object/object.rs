use crate::{spaces, Color, Intersections, Mat, Material, ObjectIndex, Point, Ray, Vector, World};

/// ObjectInnner defines methods to handle the particularities of an object, in object space.
pub trait ObjectInner: std::fmt::Debug + Sync + Send {
    /// Intersect calculates the intersections of the given ray with this object.
    fn intersect(
        &self,
        object_index: ObjectIndex,
        ray: Ray<spaces::Object>,
        inters: &mut Intersections,
    );

    /// Normal calculates the normal of the given point on the surface of this object.
    fn normal(&self, point: Point<spaces::Object>) -> Vector<spaces::Object>;
}

#[derive(Debug)]
pub struct Object {
    /// The implementation of this object, in object space
    inner: Box<dyn ObjectInner>,

    /// The transformation from world to object space.
    transform: Mat<4, spaces::World, spaces::Object>,

    /// The transformation of object-space normals to world-space normals.
    transp_transform: Mat<4, spaces::Object, spaces::World>,

    /// The material comprising this object.
    pub(crate) material: Material,
}

impl Object {
    /// Create a new object.
    ///
    /// Use the `with_...` methods to adjust the transform and material, in
    /// a builder pattern.
    pub fn new(inner: impl ObjectInner + 'static) -> Self {
        Self {
            inner: Box::new(inner),
            transform: Mat::identity(),
            transp_transform: Mat::identity(),
            material: Material::default(),
        }
    }

    /// Return an updated object with the given transform.
    pub fn with_transform(mut self, obj_to_world: Mat<4, spaces::Object, spaces::World>) -> Self {
        self.transform = obj_to_world.inverse();
        self.transp_transform = self.transform.transpose();
        self
    }

    /// Return an updated object with the given material.
    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    /// Calculate the intersections of the given ray with this object.
    pub(crate) fn intersect(
        &self,
        object_index: ObjectIndex,
        ray: &Ray<spaces::World>,
        inters: &mut Intersections,
    ) {
        let obj_ray = self.transform * *ray;
        self.inner.intersect(object_index, obj_ray, inters);
    }

    pub(crate) fn color_at(
        &self,
        world: &World,
        from_obj: Option<&Object>,
        t: f64,
        ray: &Ray<spaces::World>,
        total_contribution: f64,
        debug: bool,
    ) -> Color {
        // the point at which the intersection occurred
        let point = ray.position(t);

        // vector from point to the eye
        let eyev = -ray.direction;

        // point in object space
        let obj_point = self.transform * point;

        // normal in object space
        let obj_normal = self.inner.normal(obj_point);

        // normal in world space
        let mut normalv = (self.transp_transform * obj_normal).normalize();
        if normalv.dot(eyev) < 0.0 {
            // use the inside surface, with the opposite normal
            normalv = -normalv;
        }

        self.material.color_at(
            world,
            from_obj.map(|o| &o.material),
            ray,
            point,
            obj_point,
            eyev,
            normalv,
            total_contribution,
            debug,
        )
    }

    /// Get only the normal (used for testing objects)
    #[cfg(test)]
    pub fn normal(&self, point: Point<spaces::World>) -> Vector<spaces::World> {
        let obj_point = self.transform * point;
        let obj_normal = self.inner.normal(obj_point);
        let world_normal = self.transp_transform * obj_normal;
        world_normal.normalize()
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use approx::*;

    #[test]
    fn defaults() {
        let o = Object::new(Sphere);
        assert_relative_eq!(o.transform, Mat::identity());
        assert_relative_eq!(o.transp_transform, Mat::identity());
    }

    #[test]
    fn with_transform() {
        let t = Mat::identity().rotate_x(1.0).rotate_y(2.0);
        let o = Object::new(Sphere).with_transform(t);
        assert_relative_eq!(o.transform, t.inverse());
        assert_relative_eq!(o.transp_transform, t.inverse().transpose());
    }

    #[test]
    fn with_material() {
        let o = Object::new(Sphere).with_material(Material::default());
        assert_relative_eq!(o.transform, Mat::identity());
        assert_relative_eq!(o.transp_transform, Mat::identity());
    }

    #[test]
    fn with_both() {
        let t = Mat::identity().rotate_x(1.0).rotate_y(2.0);
        let o = Object::new(Sphere).with_transform(t);
        assert_relative_eq!(o.transform, t.inverse());
        assert_relative_eq!(o.transp_transform, t.inverse().transpose());
    }

    #[test]
    fn intersect() {
        let o = Object::new(Sphere)
            .with_transform(Mat::identity().translate(0, 0, 10))
            .with_material(Material::default().with_ambient(13.0));
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 2));
        let mut inters = Intersections::default();
        o.intersect(ObjectIndex::test_value(0), &r, &mut inters);
        // ray of length 2 hits sphere at 0, 0, 9, so t=4.5
        let (from_obj, t, to_obj) = inters.hit();
        assert!(from_obj.is_none());
        assert_relative_eq!(t.unwrap(), 4.5);
        assert_eq!(to_obj, Some(ObjectIndex::test_value(0)));
    }

    #[test]
    fn normal() {
        // a sphere stretched 2x vertically
        let o = Object::new(Sphere).with_transform(Mat::identity().scale(1, 2, 1));
        let r: Ray<spaces::World> = Ray::new(Point::new(0, 1, -10), Vector::new(0, 0, 1));

        // ray hits halfway up the sphere.
        let mut inters = Intersections::default();
        o.intersect(ObjectIndex::test_value(0), &r, &mut inters);
        let (_, t, to_obj) = inters.hit();
        let p = r.position(t.unwrap());
        assert_eq!(to_obj, Some(ObjectIndex::test_value(0)));

        let n = o.normal(p);

        assert_relative_eq!(n.magnitude(), 1.0);
    }
}
