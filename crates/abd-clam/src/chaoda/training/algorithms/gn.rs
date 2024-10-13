//! Graph Neighborhood Algorithm

use distances::Number;
use serde::{Deserialize, Serialize};

use crate::{chaoda::Graph, Cluster, Dataset};

use super::GraphEvaluator;

/// `Cluster`s in an isolated neighborhood are more likely to be anomalous.
#[derive(Clone, Serialize, Deserialize)]
pub struct GraphNeighborhood {
    /// The fraction of graph diameter to use as the neighborhood radius.
    diameter_fraction: f32,
}

impl GraphNeighborhood {
    /// Create a new `GraphNeighborhood` algorithm.
    ///
    /// # Parameters
    ///
    /// * `diameter_fraction`: The fraction of graph diameter to use as the neighborhood radius.
    pub fn new(diameter_fraction: f32) -> Result<Self, String> {
        if diameter_fraction <= 0.0 || diameter_fraction >= 1.0 {
            Err("Diameter fraction must be in the range [0, 1]".to_string())
        } else {
            Ok(Self { diameter_fraction })
        }
    }
}

impl<I, U: Number, D: Dataset<I, U>, S: Cluster<I, U, D>> GraphEvaluator<I, U, D, S> for GraphNeighborhood {
    fn name(&self) -> &str {
        "gn"
    }

    fn evaluate_clusters(&self, g: &Graph<I, U, D, S>) -> Vec<f32> {
        let diameter = g.diameter();
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let k = (self.diameter_fraction * diameter.as_f32()).round() as usize;
        g.iter_neighborhood_sizes()
            .map(|n| if n.len() <= k { n.last().unwrap_or(&0) } else { &n[k] })
            .map(|&n| -n.as_f32())
            .collect()
    }

    fn normalize_by_cluster(&self) -> bool {
        true
    }
}

impl Default for GraphNeighborhood {
    fn default() -> Self {
        Self { diameter_fraction: 0.1 }
    }
}