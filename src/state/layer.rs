use crate::model::TileLayer;

/// State for managing tile layers
#[derive(Default, PartialEq, Clone)]
pub struct State {
    layers: Vec<TileLayer>,
}

impl State {
    /// Creates new state with given layers
    pub fn new(layers: Vec<TileLayer>) -> Self {
        Self { layers }
    }

    /// Sets layer visibility status by index
    pub fn set_visible(&mut self, idx: usize, visible: bool) {
        if let Some(ref mut layer) = self.layers.get_mut(idx) {
            layer.visible = visible;
        }
    }

    /// Returns all held layers
    pub fn layers(&self) -> Vec<TileLayer> {
        self.layers.clone()
    }

    /// Returns layer list filtered by visibility status
    pub fn layers_by_visibility(&self, visible: bool) -> Vec<TileLayer> {
        self.layers
            .iter()
            .filter(|l| l.visible == visible)
            .map(|l| l.clone())
            .collect()
    }
}
