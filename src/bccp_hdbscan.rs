    impl<T> Bccp<T>
    where
        T: Iterator<Item = (f64, f64)> + Copy,
    {
        pub fn calculate_distance(&self) -> BccpResult {
            let mut distance = BccpResult {
                closest_pairs: ((0., 0.), (0., 0.)),
                pair_distance: 0.,
            };
            let mut temp_var: bool = true;

            for left_point in self.left_node {
                for right_point in self.right_node {
                    let euclidean_dist = ((left_point.1 - left_point.0).powf(2.)
                        + (right_point.1 - right_point.0).powf(2.))
                    .sqrt();
                    if temp_var {
                        distance.pair_distance = euclidean_dist;
                        distance.closest_pairs = (left_point, right_point);
                        temp_var = false;
                    } else if euclidean_dist < distance.pair_distance {
                        distance.pair_distance = euclidean_dist;
                        distance.closest_pairs = (left_point, right_point);
                    }
                }
            }
            distance
        }
    }
