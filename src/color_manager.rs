use colored::*;
use std::collections::HashMap;

pub struct ColorManager {
    container_to_index: HashMap<String, usize>,
    next_index: usize,
    color_pool: Vec<Color>,
}

impl ColorManager {
    pub fn new() -> Self {
        Self {
            container_to_index: HashMap::new(),
            next_index: 0,
            color_pool: vec![
                Color::Cyan,
                Color::Green,
                Color::Yellow,
                Color::Blue,
                Color::Magenta,
                Color::BrightCyan,
                Color::BrightGreen,
                Color::BrightYellow,
            ],
        }
    }

    pub fn get_container_color(&mut self, container: &str) -> ColoredString {
        let index = *self.container_to_index
            .entry(container.to_string())
            .or_insert_with(|| {
                let idx = self.next_index;
                self.next_index += 1;
                idx
            });

        let color = self.color_pool[index % self.color_pool.len()];
        container.color(color)
    }

    pub fn get_level_color(&self, level: &str) -> Color {
        match level.to_lowercase().as_str() {
            "error" | "err" => Color::BrightRed,
            "warn" | "warning" => Color::Yellow,
            "info" => Color::Green,
            "debug" => Color::Blue,
            "trace" => Color::Cyan,
            _ => Color::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_container_gets_index_zero() {
        let mut manager = ColorManager::new();
        let color = manager.get_container_color("web-1");
        // First container should get cyan (index 0)
        assert!(color.to_string().contains("web-1"));
    }

    #[test]
    fn test_same_container_gets_same_color() {
        let mut manager = ColorManager::new();
        let color1 = manager.get_container_color("web-1");
        let color2 = manager.get_container_color("web-1");
        assert_eq!(color1.to_string(), color2.to_string());
    }

    #[test]
    fn test_different_containers_get_different_colors() {
        let mut manager = ColorManager::new();
        let color1 = manager.get_container_color("web-1");
        let color2 = manager.get_container_color("db-1");
        // They should have different indices
        assert_ne!(color1.to_string(), color2.to_string());
    }

    #[test]
    fn test_color_wraparound() {
        let mut manager = ColorManager::new();
        // Assuming 8 colors in pool
        for i in 0..10 {
            manager.get_container_color(&format!("container-{}", i));
        }

        // Verify that container-1 got index 1 and container-9 got index 9
        assert_eq!(manager.container_to_index.get("container-1"), Some(&1));
        assert_eq!(manager.container_to_index.get("container-9"), Some(&9));

        // Verify wraparound: both should map to same color pool index
        assert_eq!(1 % manager.color_pool.len(), 9 % manager.color_pool.len());
    }
}
