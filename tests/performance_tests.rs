use crossterm::style::Color;
use hack_simulator::ui::{
    begin_frame, clear_screen_buffered, end_frame, print_colored_buffered, render_menu_buffered,
    render_screen, PerformanceTimer,
};
use std::time::Duration;

#[test]
fn test_performance_timer() {
    let timer = PerformanceTimer::new("test");
    std::thread::sleep(Duration::from_millis(1));
    let elapsed = timer.elapsed();
    assert!(elapsed.as_millis() >= 1);
}

#[test]
fn test_buffered_rendering() {
    // Test that buffered functions don't crash
    begin_frame().unwrap();
    print_colored_buffered("Test text", Color::White).unwrap();
    clear_screen_buffered().unwrap();
    end_frame().unwrap();
}

#[test]
fn test_render_screen() {
    // Test the render_screen wrapper
    render_screen(|| {
        print_colored_buffered("Screen content", Color::Green)?;
        Ok(())
    })
    .unwrap();
}

#[test]
fn test_render_menu_buffered() {
    let items = vec![
        ("1".to_string(), "First option".to_string()),
        ("2".to_string(), "Second option".to_string()),
    ];

    render_menu_buffered("Test Menu", &items, Some("Press a key to continue")).unwrap();
}

#[test]
fn test_performance_comparison() {
    // Measure performance of buffered vs non-buffered operations
    let timer1 = PerformanceTimer::new("unbuffered_operations");

    // Simulate some terminal operations
    for _ in 0..10 {
        // This would normally cause flicker in real usage
        std::thread::sleep(Duration::from_micros(100));
    }
    drop(timer1);

    let timer2 = PerformanceTimer::new("buffered_operations");
    render_screen(|| {
        for _ in 0..10 {
            print_colored_buffered(".", Color::White)?;
        }
        Ok(())
    })
    .unwrap();
    drop(timer2);
}
