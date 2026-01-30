use colored::{Color, ColoredString};

use crate::color_manager::ColorManager;

pub struct FormatContext {
    color_manager: ColorManager,
    min_indent: usize,
}

impl FormatContext {
    pub fn new() -> Self {
        FormatContext {
            color_manager: ColorManager::new(),
            min_indent: 0,
        }
    }

    pub fn get_container_color(&mut self, container: &str) -> ColoredString {
        self.min_indent = std::cmp::max(self.min_indent, container.len());
        self.color_manager.get_container_color(container)
    }

    pub fn get_level_color(&self, level: &str) -> Color {
        self.color_manager.get_level_color(level)
    }

    pub fn indent(&self) -> usize {
        self.min_indent
    }
}
