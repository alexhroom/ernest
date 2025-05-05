use std::io::Error;

use crate::utils::lrps::LRPS;
use nalgebra::DMatrix;

struct MultiNest;

impl LRPS for MultiNest{
    fn sample(&mut self, min_likelihood: f64) -> (Vec<f64>, f64) {
        todo!()
    }
}

/// A struct to represent an n-dimensional ellipsoid.
struct Ellipsoid {
    bounding_matrix: DMatrix<f64>,
    centroid: Vec<f64>,
    volume: f64
}

/// Calculate a bounding ellipsoid around a set of points with a minimum volume.
///
/// Parameters
/// ----------
/// points : Vec<f64>
///     The set of points to be bounded.
/// min_volume : f64
///     The minimum volume of the ellipsoid.
///
/// Returns
/// -------
/// Result:
///     bounding_matrix : Matrix<f64>
///         The bounding matrix for the ellipsoid.
///     mu : Vec<f64>
///         The centroid of the ellipsoid.
///     volume : f64
///         The volume of the ellipsoid.
/// Error:
///     If the ellipsoid cannot be calculated.
///
fn calc_bounding_ellipsoid(points: Vec<f64>, min_volume: f64) -> Result<Ellipsoid, Error> {
    todo!()
} 

/// Partition a set of points into two optimally bounded ellipsoids.
///
/// Initially does a k-means cluster, and then uses expectation-maximisation 
/// to reassign points for the optimal result.
///
/// Parameters
/// ----------
/// points : Vec<f64>
///     The set of points to be bounded.
/// min_volume : f64
///     The minimum combined volume of the two ellipsoids.
///
/// Returns
/// -------
/// Result:
///     S1 : Vec<f64>
///         The points in the first ellipsoid.
///     S2 : Vec<f64>
///         The points in the second ellipsoid.
///     volume_1 : f64
///         The volume of the first ellipsoid.
///     volume_2 : f64
///         The volume of the second ellipsoid.
/// Error:
///     If the ellipsoid could not be split in two.
fn create_partition(points: Vec<f64>, min_volume: f64) -> Result<(Vec<f64>, Vec<f64>, f64, f64), Error> {
    todo!()
}

/// Partition a set of points into a set of optimal subclusters bounded by ellipsoids.
///
/// Parameters
/// ----------
/// points : Vec<f64>
///     The set of points to be bounded.
/// min_volume : f64
///     The minimum combined volume of the set of ellipsoids.
///
/// Returns
/// -------
/// bounding_matrices : Vec<DMatrix<f64>>
///     The bounding matrices for each ellipsoid.
/// centroids : Vec<Vec<f64>>
///     The centroid for each bounding matrix.
/// 
fn optimal_ellipsoids(points: Vec<f64>, min_volume: f64) -> Vec<Ellipsoid> {
    todo!()
}
