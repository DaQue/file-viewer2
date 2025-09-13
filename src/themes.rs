use eframe::egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CodeTheme {
    OneDarkPro,
    Dracula,
    MaterialTheme,
    MonokaiPro,
    NightOwl,
    Ayu,
    Cobalt2,
    Palenight,
    ShadesOfPurple,
    Noctis,
}

impl Default for CodeTheme {
    fn default() -> Self {
        Self::OneDarkPro
    }
}

impl CodeTheme {
    pub fn name(&self) -> &'static str {
        match self {
            Self::OneDarkPro => "One Dark Pro",
            Self::Dracula => "Dracula",
            Self::MaterialTheme => "Material Theme",
            Self::MonokaiPro => "Monokai Pro",
            Self::NightOwl => "Night Owl",
            Self::Ayu => "Ayu",
            Self::Cobalt2 => "Cobalt2",
            Self::Palenight => "Palenight",
            Self::ShadesOfPurple => "Shades of Purple",
            Self::Noctis => "Noctis",
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::OneDarkPro,
            Self::Dracula,
            Self::MaterialTheme,
            Self::MonokaiPro,
            Self::NightOwl,
            Self::Ayu,
            Self::Cobalt2,
            Self::Palenight,
            Self::ShadesOfPurple,
            Self::Noctis,
        ]
    }

    pub fn background(&self) -> Color32 {
        match self {
            Self::OneDarkPro => Color32::from_rgb(40, 44, 52),      // Dark gray-blue
            Self::Dracula => Color32::from_rgb(40, 42, 54),         // Dark purple-gray
            Self::MaterialTheme => Color32::from_rgb(33, 33, 33),   // Dark gray
            Self::MonokaiPro => Color32::from_rgb(39, 40, 34),      // Dark olive
            Self::NightOwl => Color32::from_rgb(1, 22, 39),         // Very dark blue
            Self::Ayu => Color32::from_rgb(15, 20, 25),             // Very dark blue-gray
            Self::Cobalt2 => Color32::from_rgb(13, 13, 13),         // Almost black
            Self::Palenight => Color32::from_rgb(41, 45, 62),       // Dark blue-gray
            Self::ShadesOfPurple => Color32::from_rgb(45, 42, 85),  // Dark purple
            Self::Noctis => Color32::from_rgb(25, 25, 25),          // Dark gray
        }
    }

    pub fn foreground(&self) -> Color32 {
        match self {
            Self::OneDarkPro => Color32::from_rgb(171, 178, 191),   // Light gray
            Self::Dracula => Color32::from_rgb(248, 248, 242),      // Off-white
            Self::MaterialTheme => Color32::from_rgb(238, 255, 65), // Bright yellow-green
            Self::MonokaiPro => Color32::from_rgb(248, 248, 242),   // Off-white
            Self::NightOwl => Color32::from_rgb(131, 148, 150),     // Light blue-gray
            Self::Ayu => Color32::from_rgb(203, 204, 198),          // Light gray
            Self::Cobalt2 => Color32::from_rgb(255, 255, 255),      // White
            Self::Palenight => Color32::from_rgb(169, 183, 198),    // Light blue-gray
            Self::ShadesOfPurple => Color32::from_rgb(255, 255, 255), // White
            Self::Noctis => Color32::from_rgb(255, 255, 255),       // White
        }
    }

    pub fn keyword(&self) -> Color32 {
        match self {
            Self::OneDarkPro => Color32::from_rgb(198, 120, 221),   // Purple
            Self::Dracula => Color32::from_rgb(255, 121, 198),      // Pink
            Self::MaterialTheme => Color32::from_rgb(199, 146, 234), // Light purple
            Self::MonokaiPro => Color32::from_rgb(249, 38, 114),    // Pink-red
            Self::NightOwl => Color32::from_rgb(195, 232, 141),     // Light green
            Self::Ayu => Color32::from_rgb(255, 204, 102),          // Orange
            Self::Cobalt2 => Color32::from_rgb(255, 168, 33),       // Orange
            Self::Palenight => Color32::from_rgb(195, 232, 141),    // Light green
            Self::ShadesOfPurple => Color32::from_rgb(255, 121, 198), // Pink
            Self::Noctis => Color32::from_rgb(255, 204, 102),       // Orange
        }
    }

    pub fn string(&self) -> Color32 {
        match self {
            Self::OneDarkPro => Color32::from_rgb(152, 195, 121),   // Green
            Self::Dracula => Color32::from_rgb(241, 250, 140),      // Yellow
            Self::MaterialTheme => Color32::from_rgb(195, 232, 141), // Light green
            Self::MonokaiPro => Color32::from_rgb(230, 219, 116),   // Yellow
            Self::NightOwl => Color32::from_rgb(173, 219, 103),     // Green
            Self::Ayu => Color32::from_rgb(201, 208, 255),          // Light blue
            Self::Cobalt2 => Color32::from_rgb(255, 255, 255),      // White
            Self::Palenight => Color32::from_rgb(195, 232, 141),    // Light green
            Self::ShadesOfPurple => Color32::from_rgb(255, 255, 255), // White
            Self::Noctis => Color32::from_rgb(255, 255, 255),       // White
        }
    }

    pub fn number(&self) -> Color32 {
        match self {
            Self::OneDarkPro => Color32::from_rgb(209, 154, 102),   // Orange
            Self::Dracula => Color32::from_rgb(189, 147, 249),      // Light purple
            Self::MaterialTheme => Color32::from_rgb(255, 213, 79), // Yellow
            Self::MonokaiPro => Color32::from_rgb(174, 129, 255),   // Purple
            Self::NightOwl => Color32::from_rgb(255, 203, 107),     // Orange
            Self::Ayu => Color32::from_rgb(255, 204, 102),          // Orange
            Self::Cobalt2 => Color32::from_rgb(255, 168, 33),       // Orange
            Self::Palenight => Color32::from_rgb(255, 203, 107),    // Orange
            Self::ShadesOfPurple => Color32::from_rgb(255, 121, 198), // Pink
            Self::Noctis => Color32::from_rgb(255, 204, 102),       // Orange
        }
    }

    pub fn comment(&self) -> Color32 {
        match self {
            Self::OneDarkPro => Color32::from_rgb(92, 99, 112),     // Gray
            Self::Dracula => Color32::from_rgb(98, 114, 164),       // Blue-gray
            Self::MaterialTheme => Color32::from_rgb(117, 113, 94), // Brown-gray
            Self::MonokaiPro => Color32::from_rgb(117, 113, 94),    // Brown-gray
            Self::NightOwl => Color32::from_rgb(99, 119, 119),      // Blue-gray
            Self::Ayu => Color32::from_rgb(92, 99, 112),            // Gray
            Self::Cobalt2 => Color32::from_rgb(0, 255, 255),        // Cyan
            Self::Palenight => Color32::from_rgb(99, 119, 119),     // Blue-gray
            Self::ShadesOfPurple => Color32::from_rgb(255, 121, 198), // Pink
            Self::Noctis => Color32::from_rgb(92, 99, 112),         // Gray
        }
    }

    pub fn bracket_colors(&self) -> [Color32; 5] {
        match self {
            Self::OneDarkPro => [
                Color32::from_rgb(152, 195, 121), // Green
                Color32::from_rgb(224, 108, 117), // Red
                Color32::from_rgb(97, 175, 239),  // Blue
                Color32::from_rgb(229, 192, 123), // Yellow
                Color32::from_rgb(86, 182, 194),  // Cyan
            ],
            Self::Dracula => [
                Color32::from_rgb(80, 250, 123),  // Green
                Color32::from_rgb(255, 85, 85),   // Red
                Color32::from_rgb(139, 233, 253), // Cyan
                Color32::from_rgb(255, 184, 108), // Orange
                Color32::from_rgb(189, 147, 249), // Purple
            ],
            Self::MaterialTheme => [
                Color32::from_rgb(195, 232, 141), // Light green
                Color32::from_rgb(255, 83, 112),  // Red
                Color32::from_rgb(130, 170, 255), // Blue
                Color32::from_rgb(255, 213, 79),  // Yellow
                Color32::from_rgb(199, 146, 234), // Purple
            ],
            Self::MonokaiPro => [
                Color32::from_rgb(166, 226, 46),  // Green
                Color32::from_rgb(249, 38, 114),  // Pink-red
                Color32::from_rgb(102, 217, 239), // Cyan
                Color32::from_rgb(230, 219, 116), // Yellow
                Color32::from_rgb(174, 129, 255), // Purple
            ],
            Self::NightOwl => [
                Color32::from_rgb(173, 219, 103), // Green
                Color32::from_rgb(255, 99, 99),   // Red
                Color32::from_rgb(130, 170, 255), // Blue
                Color32::from_rgb(255, 203, 107), // Orange
                Color32::from_rgb(199, 146, 234), // Purple
            ],
            Self::Ayu => [
                Color32::from_rgb(201, 208, 255), // Light blue
                Color32::from_rgb(255, 204, 102), // Orange
                Color32::from_rgb(255, 255, 255), // White
                Color32::from_rgb(255, 204, 102), // Orange
                Color32::from_rgb(201, 208, 255), // Light blue
            ],
            Self::Cobalt2 => [
                Color32::from_rgb(0, 255, 0),     // Green
                Color32::from_rgb(255, 0, 0),     // Red
                Color32::from_rgb(0, 0, 255),     // Blue
                Color32::from_rgb(255, 255, 0),   // Yellow
                Color32::from_rgb(0, 255, 255),   // Cyan
            ],
            Self::Palenight => [
                Color32::from_rgb(195, 232, 141), // Light green
                Color32::from_rgb(255, 99, 99),   // Red
                Color32::from_rgb(130, 170, 255), // Blue
                Color32::from_rgb(255, 203, 107), // Orange
                Color32::from_rgb(199, 146, 234), // Purple
            ],
            Self::ShadesOfPurple => [
                Color32::from_rgb(255, 255, 255), // White
                Color32::from_rgb(255, 121, 198), // Pink
                Color32::from_rgb(255, 255, 255), // White
                Color32::from_rgb(255, 121, 198), // Pink
                Color32::from_rgb(255, 255, 255), // White
            ],
            Self::Noctis => [
                Color32::from_rgb(255, 255, 255), // White
                Color32::from_rgb(255, 204, 102), // Orange
                Color32::from_rgb(255, 255, 255), // White
                Color32::from_rgb(255, 204, 102), // Orange
                Color32::from_rgb(255, 255, 255), // White
            ],
        }
    }

    pub fn search_highlight(&self) -> Color32 {
        match self {
            Self::OneDarkPro => Color32::from_rgba_premultiplied(255, 255, 0, 64),     // Yellow
            Self::Dracula => Color32::from_rgba_premultiplied(255, 184, 108, 64),      // Orange
            Self::MaterialTheme => Color32::from_rgba_premultiplied(255, 213, 79, 64), // Yellow
            Self::MonokaiPro => Color32::from_rgba_premultiplied(230, 219, 116, 64),   // Yellow
            Self::NightOwl => Color32::from_rgba_premultiplied(255, 203, 107, 64),     // Orange
            Self::Ayu => Color32::from_rgba_premultiplied(255, 204, 102, 64),          // Orange
            Self::Cobalt2 => Color32::from_rgba_premultiplied(255, 255, 0, 64),        // Yellow
            Self::Palenight => Color32::from_rgba_premultiplied(255, 203, 107, 64),    // Orange
            Self::ShadesOfPurple => Color32::from_rgba_premultiplied(255, 121, 198, 64), // Pink
            Self::Noctis => Color32::from_rgba_premultiplied(255, 204, 102, 64),       // Orange
        }
    }

    pub fn search_current(&self) -> Color32 {
        match self {
            Self::OneDarkPro => Color32::from_rgba_premultiplied(224, 108, 117, 96),   // Red
            Self::Dracula => Color32::from_rgba_premultiplied(255, 85, 85, 96),        // Red
            Self::MaterialTheme => Color32::from_rgba_premultiplied(255, 83, 112, 96), // Red
            Self::MonokaiPro => Color32::from_rgba_premultiplied(249, 38, 114, 96),    // Pink-red
            Self::NightOwl => Color32::from_rgba_premultiplied(255, 99, 99, 96),       // Red
            Self::Ayu => Color32::from_rgba_premultiplied(255, 204, 102, 96),          // Orange
            Self::Cobalt2 => Color32::from_rgba_premultiplied(255, 0, 0, 96),          // Red
            Self::Palenight => Color32::from_rgba_premultiplied(255, 99, 99, 96),      // Red
            Self::ShadesOfPurple => Color32::from_rgba_premultiplied(255, 121, 198, 96), // Pink
            Self::Noctis => Color32::from_rgba_premultiplied(255, 204, 102, 96),       // Orange
        }
    }
}
