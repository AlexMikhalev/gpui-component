# Phase 2: Font Integration Implementation

## FontAwesome Pro Integration Plan

### 1. Font File Organization
```
assets/fonts/fontawesome/
├── webfonts/
│   ├── fa-thin-100.woff2
│   ├── fa-light-300.woff2
│   ├── fa-regular-400.woff2
│   ├── fa-solid-700.woff2
│   ├── fa-duotone-900.woff2
│   └── fa-brands-400.woff2
└── css/
    └── fontawesome.css (custom generated)
```

### 2. Font Loading Strategy
```rust
// crates/assets/src/fontawesome.rs
use gpui::{Font, FontWeight, FontStyle, SharedString};

pub struct FontAwesomeFont {
    pub weight: FontWeight,
    pub style: FontStyle,
    pub family: SharedString,
}

impl FontAwesomeFont {
    pub fn thin() -> Self {
        Self {
            weight: FontWeight::Thin,
            style: FontStyle::Normal,
            family: "Font Awesome 6 Pro Thin".into(),
        }
    }
    
    pub fn light() -> Self {
        Self {
            weight: FontWeight::Light,
            style: FontStyle::Normal,
            family: "Font Awesome 6 Pro Light".into(),
        }
    }
    
    pub fn regular() -> Self {
        Self {
            weight: FontWeight::Normal,
            style: FontStyle::Normal,
            family: "Font Awesome 6 Pro Regular".into(),
        }
    }
    
    pub fn solid() -> Self {
        Self {
            weight: FontWeight::Bold,
            style: FontStyle::Normal,
            family: "Font Awesome 6 Pro Solid".into(),
        }
    }
}
```

### 3. Icon Mapping System
```rust
// crates/assets/src/icon_mapping.rs
use crate::IconName;

#[derive(Debug, Clone)]
pub struct FontAwesomeMapping {
    pub fontawesome_class: &'static str,
    pub unicode: &'static str,
    pub weight: FontWeight,
}

pub const LUCIDE_TO_FONTAWESOME: &[(&IconName, FontAwesomeMapping)] = &[
    (IconName::Check, FontAwesomeMapping {
        fontawesome_class: "fa-solid fa-check",
        unicode: "\u{f00c}",
        weight: FontWeight::Bold,
    }),
    (IconName::Close, FontAwesomeMapping {
        fontawesome_class: "fa-solid fa-xmark",
        unicode: "\u{f00d}",
        weight: FontWeight::Bold,
    }),
    (IconName::Plus, FontAwesomeMapping {
        fontawesome_class: "fa-solid fa-plus",
        unicode: "\u{f067}",
        weight: FontWeight::Bold,
    }),
    (IconName::Minus, FontAwesomeMapping {
        fontawesome_class: "fa-solid fa-minus",
        unicode: "\u{f068}",
        weight: FontWeight::Bold,
    }),
    // ... more mappings
];
```

### 4. Enhanced Theme Schema Extension
```json
{
  "fontawesome": {
    "enabled": true,
    "family": "Font Awesome 6 Pro",
    "weights": {
      "thin": 100,
      "light": 300,
      "regular": 400,
      "solid": 700,
      "duotone": 900
    },
    "default_weight": "regular"
  },
  "icons": {
    "library": "dual", // "lucide", "fontawesome", or "dual"
    "fallback": "lucide"
  }
}
```

## Implementation Steps

### Step 1: Create Font Loading System
1. Define font family constants
2. Create font weight enums
3. Implement font loading logic
4. Add font caching system

### Step 2: Create Icon Mapping
1. Map Lucide icons to FontAwesome equivalents
2. Handle missing mappings gracefully
3. Implement icon selection logic
4. Add icon preview functionality

### Step 3: Update Theme System
1. Extend existing theme schema
2. Add FontAwesome configuration
3. Implement dual icon support
4. Create theme switching logic

### Step 4: Component Integration
1. Update existing components
2. Add icon source selection
3. Implement font weight options
4. Test integration across components

## Next Actions
1. Create FontAwesome integration files
2. Implement font loading system
3. Create icon mapping logic
4. Update theme schema
5. Test integration with sample components

**Implementation started on 2025-11-27**