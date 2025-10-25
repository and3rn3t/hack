use crossterm::style::Color;
use hack_simulator::ui::{
    get_theme, set_theme, theme_accent, theme_border, theme_error, theme_info, theme_muted,
    theme_primary, theme_success, theme_warning, ColorTheme,
};

#[test]
fn test_default_theme_is_horror() {
    let theme = get_theme();
    assert_eq!(theme.name, "Horror");
    assert_eq!(theme.primary, Color::White);
}

#[test]
fn test_all_themes_exist() {
    let themes = ColorTheme::all_themes();
    assert_eq!(themes.len(), 5);

    let theme_names: Vec<&str> = themes.iter().map(|t| t.name.as_str()).collect();
    assert!(theme_names.contains(&"Horror"));
    assert!(theme_names.contains(&"High Contrast"));
    assert!(theme_names.contains(&"Dark"));
    assert!(theme_names.contains(&"Colorblind Friendly"));
    assert!(theme_names.contains(&"Retro Green"));
}

#[test]
fn test_theme_switching() {
    let original_theme = get_theme();

    // Switch to high contrast theme
    let high_contrast = ColorTheme::high_contrast();
    set_theme(high_contrast.clone()).unwrap();

    let current_theme = get_theme();
    assert_eq!(current_theme.name, "High Contrast");
    assert_eq!(current_theme.primary, Color::White);
    assert_eq!(current_theme.background, Color::Black);

    // Switch back to original
    set_theme(original_theme.clone()).unwrap();
    let restored_theme = get_theme();
    assert_eq!(restored_theme.name, original_theme.name);
}

#[test]
fn test_theme_accessor_functions() {
    // Test with horror theme
    let horror_theme = ColorTheme::horror();
    set_theme(horror_theme.clone()).unwrap();

    assert_eq!(theme_primary(), horror_theme.primary);
    assert_eq!(theme_accent(), horror_theme.accent);
    assert_eq!(theme_success(), horror_theme.success);
    assert_eq!(theme_warning(), horror_theme.warning);
    assert_eq!(theme_error(), horror_theme.error);
    assert_eq!(theme_info(), horror_theme.info);
    assert_eq!(theme_muted(), horror_theme.muted);
    assert_eq!(theme_border(), horror_theme.border);
}

#[test]
fn test_colorblind_friendly_theme() {
    let theme = ColorTheme::colorblind_friendly();

    // Should use high contrast, colorblind-safe colors
    assert_eq!(theme.name, "Colorblind Friendly");
    assert_eq!(theme.primary, Color::White);
    assert_eq!(theme.success, Color::Blue); // Blue instead of green
    assert_eq!(theme.error, Color::Magenta); // Magenta instead of red
}

#[test]
fn test_dark_theme() {
    let theme = ColorTheme::dark();

    assert_eq!(theme.name, "Dark");
    assert_eq!(theme.background, Color::Black);
    assert_eq!(theme.primary, Color::Grey);
    assert_eq!(theme.accent, Color::Blue);
}

#[test]
fn test_retro_theme() {
    let theme = ColorTheme::retro();

    assert_eq!(theme.name, "Retro Green");
    assert_eq!(theme.primary, Color::Green);
    assert_eq!(theme.accent, Color::Green);
    assert_eq!(theme.background, Color::Black);
}

#[test]
fn test_theme_names_are_unique() {
    let themes = ColorTheme::all_themes();
    let mut names = std::collections::HashSet::new();

    for theme in themes {
        assert!(
            names.insert(theme.name.clone()),
            "Duplicate theme name found: {}",
            theme.name
        );
    }
}

#[test]
fn test_theme_persistence() {
    let original_theme = get_theme();

    // Switch to a different theme
    let retro_theme = ColorTheme::retro();
    set_theme(retro_theme.clone()).unwrap();

    // Verify it persists
    let current_theme = get_theme();
    assert_eq!(current_theme.name, "Retro Green");

    // Restore original
    set_theme(original_theme).unwrap();
}
