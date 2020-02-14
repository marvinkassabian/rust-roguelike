pub const LAYER_COUNT: usize = 1;
pub static CONSOLE_INDEX: ConsoleIndex = ConsoleIndex { base: 0, layers: [1], ui: 2 };

pub struct ConsoleIndex { pub base: usize, pub layers: [usize; LAYER_COUNT], pub ui: usize }

impl ConsoleIndex {
    pub fn get_all_indices(&self) -> Vec<usize> {
        let mut indices = Vec::new();

        indices.push(self.base);

        for layer in self.layers.iter() {
            indices.push(*layer);
        }

        indices.push(self.ui);

        indices
    }

    pub fn get_world_indices(&self, include_base: bool) -> Vec<usize> {
        let mut indices = Vec::new();

        if include_base {
            indices.push(self.base);
        }

        for layer in self.layers.iter() {
            indices.push(*layer);
        }

        indices
    }
}
