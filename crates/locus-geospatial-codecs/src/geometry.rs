//! Core geometric primitive types for geospatial codecs.

use serde::{Deserialize, Serialize};

/// 2D Coordinate.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// 3D Coordinate.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn to_2d(self) -> Point2D {
        Point2D::new(self.x, self.y)
    }
}

/// A polygon with exterior ring and optional interior hole rings.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Polygon2D {
    pub exterior: Vec<Point2D>,
    pub interiors: Vec<Vec<Point2D>>,
}

impl Polygon2D {
    pub fn new(exterior: Vec<Point2D>, interiors: Vec<Vec<Point2D>>) -> Self {
        Self { exterior, interiors }
    }
}

/// Universal categorical geometry enum.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Geometry {
    Point(Point2D),
    PointZ(Point3D),
    MultiPoint(Vec<Point2D>),
    LineString(Vec<Point2D>),
    LineStringZ(Vec<Point3D>),
    MultiLineString(Vec<Vec<Point2D>>),
    Polygon(Polygon2D),
    MultiPolygon(Vec<Polygon2D>),
    GeometryCollection(Vec<Geometry>),
}
