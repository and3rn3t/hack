// Benchmarks for challenge validation performance
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hack_simulator::challenges::get_all_challenges;

fn bench_challenge_validation(c: &mut Criterion) {
    let challenges = get_all_challenges();

    c.bench_function("validate_correct_answer", |b| {
        let challenge = &challenges[0]; // welcome challenge
        b.iter(|| (challenge.check_answer)(black_box("welcome")));
    });

    c.bench_function("validate_incorrect_answer", |b| {
        let challenge = &challenges[0];
        b.iter(|| (challenge.check_answer)(black_box("wrong_answer")));
    });

    c.bench_function("validate_long_input", |b| {
        let challenge = &challenges[0];
        let long_input = "a".repeat(10000);
        b.iter(|| (challenge.check_answer)(black_box(&long_input)));
    });
}

fn bench_challenge_retrieval(c: &mut Criterion) {
    c.bench_function("get_all_challenges", |b| {
        b.iter(|| black_box(get_all_challenges()));
    });

    c.bench_function("filter_challenges_by_level", |b| {
        let challenges = get_all_challenges();
        b.iter(|| {
            challenges
                .iter()
                .filter(|c| c.level == black_box(1))
                .count()
        });
    });
}

fn bench_cryptography_challenges(c: &mut Criterion) {
    let challenges = get_all_challenges();

    // Base64 decode
    if let Some(base64_challenge) = challenges.iter().find(|c| c.id == "base64_decode") {
        c.bench_function("base64_decode_validation", |b| {
            b.iter(|| (base64_challenge.check_answer)(black_box("SGFja2VyIEdob3N0")));
        });
    }

    // Caesar cipher
    if let Some(caesar_challenge) = challenges.iter().find(|c| c.id == "caesar_cipher") {
        c.bench_function("caesar_cipher_validation", |b| {
            b.iter(|| (caesar_challenge.check_answer)(black_box("URYYB")));
        });
    }

    // ROT13
    if let Some(rot13_challenge) = challenges.iter().find(|c| c.id == "rot13") {
        c.bench_function("rot13_validation", |b| {
            b.iter(|| (rot13_challenge.check_answer)(black_box("ERIREFR")));
        });
    }
}

fn bench_validation_complexity(c: &mut Criterion) {
    let challenges = get_all_challenges();

    // Simple string comparison (welcome)
    if let Some(simple) = challenges.iter().find(|c| c.id == "welcome") {
        c.bench_function("simple_validation", |b| {
            b.iter(|| (simple.check_answer)(black_box("welcome")));
        });
    }

    // Medium complexity (hex decode)
    if let Some(medium) = challenges.iter().find(|c| c.id == "hex_decode") {
        c.bench_function("medium_validation", |b| {
            b.iter(|| (medium.check_answer)(black_box("4465636f646564")));
        });
    }

    // Complex (JWT token)
    if let Some(complex) = challenges.iter().find(|c| c.id == "jwt_token") {
        c.bench_function("complex_validation", |b| {
            b.iter(|| (complex.check_answer)(black_box("eyJhbGciOiJub25lIn0")));
        });
    }
}

criterion_group!(
    benches,
    bench_challenge_validation,
    bench_challenge_retrieval,
    bench_cryptography_challenges,
    bench_validation_complexity
);
criterion_main!(benches);
