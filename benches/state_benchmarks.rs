// Benchmarks for game state operations
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use hack_simulator::state::GameState;
use std::fs;

fn bench_state_creation(c: &mut Criterion) {
    c.bench_function("create_new_state", |b| {
        b.iter(|| GameState::new(black_box("TestPlayer".to_string())));
    });

    c.bench_function("create_and_complete_challenge", |b| {
        b.iter(|| {
            let mut state = GameState::new("TestPlayer".to_string());
            state.complete_challenge(black_box("welcome"), black_box(50));
            state
        });
    });
}

fn bench_state_modifications(c: &mut Criterion) {
    let mut group = c.benchmark_group("state_modifications");

    group.bench_function("complete_single_challenge", |b| {
        let mut state = GameState::new("TestPlayer".to_string());
        b.iter(|| {
            state.complete_challenge(black_box("challenge_1"), black_box(50));
        });
    });

    group.bench_function("modify_sanity", |b| {
        let mut state = GameState::new("TestPlayer".to_string());
        b.iter(|| {
            state.modify_sanity(black_box(-10));
        });
    });

    group.bench_function("discover_secret", |b| {
        let mut state = GameState::new("TestPlayer".to_string());
        let mut counter = 0;
        b.iter(|| {
            counter += 1;
            state.discover_secret(black_box(format!("secret_{}", counter)));
        });
    });

    group.finish();
}

fn bench_state_queries(c: &mut Criterion) {
    let mut state = GameState::new("TestPlayer".to_string());
    for i in 0..10 {
        state.complete_challenge(&format!("challenge_{}", i), 50);
    }

    c.bench_function("has_completed_check", |b| {
        b.iter(|| state.has_completed(black_box("challenge_5")));
    });
}

fn bench_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");

    // Small state
    let small_state = GameState::new("TestPlayer".to_string());
    group.bench_function("serialize_small_state", |b| {
        b.iter(|| serde_json::to_string(black_box(&small_state)));
    });

    // Medium state
    let mut medium_state = GameState::new("TestPlayer".to_string());
    for i in 0..10 {
        medium_state.complete_challenge(&format!("challenge_{}", i), 50);
    }
    group.bench_function("serialize_medium_state", |b| {
        b.iter(|| serde_json::to_string(black_box(&medium_state)));
    });

    // Large state
    let mut large_state = GameState::new("TestPlayer".to_string());
    for i in 0..50 {
        large_state.complete_challenge(&format!("challenge_{}", i), 50);
        large_state.discover_secret(format!("secret_{}", i));
    }
    group.bench_function("serialize_large_state", |b| {
        b.iter(|| serde_json::to_string(black_box(&large_state)));
    });

    group.finish();
}

fn bench_deserialization(c: &mut Criterion) {
    let mut state = GameState::new("TestPlayer".to_string());
    for i in 0..10 {
        state.complete_challenge(&format!("challenge_{}", i), 50);
    }
    let json = serde_json::to_string(&state).unwrap();

    c.bench_function("deserialize_state", |b| {
        b.iter(|| serde_json::from_str::<GameState>(black_box(&json)));
    });
}

fn bench_save_load_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("save_load");

    // Setup
    let bench_file = "bench_save_load.json";
    let mut state = GameState::new("BenchPlayer".to_string());
    for i in 0..10 {
        state.complete_challenge(&format!("challenge_{}", i), 50);
    }

    group.bench_function("save_to_file", |b| {
        b.iter(|| state.save_to(black_box(bench_file)).unwrap());
    });

    // Create file for loading
    state.save_to(bench_file).unwrap();

    group.bench_function("load_from_file", |b| {
        b.iter(|| GameState::load_from(black_box(bench_file)).unwrap());
    });

    group.finish();

    // Cleanup
    let _ = fs::remove_file(bench_file);
}

fn bench_level_progression(c: &mut Criterion) {
    let mut group = c.benchmark_group("level_progression");

    for xp in [100, 500, 1000, 5000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(xp), xp, |b, &xp| {
            b.iter(|| {
                let mut state = GameState::new("TestPlayer".to_string());
                state.complete_challenge("mega_challenge", black_box(xp));
                state.current_level
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_state_creation,
    bench_state_modifications,
    bench_state_queries,
    bench_serialization,
    bench_deserialization,
    bench_save_load_operations,
    bench_level_progression
);
criterion_main!(benches);
