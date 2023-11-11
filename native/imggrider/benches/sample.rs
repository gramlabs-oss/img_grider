use criterion::{criterion_group, criterion_main, Criterion};
use imggrider::{r_generate, Scheme};

fn generate_benchmark(c: &mut Criterion) {
    c.bench_function("generate", |b| {
        b.iter(|| {
            let assets_path = std::path::PathBuf::from("..")
                .join("..")
                .join("test")
                .join("assets");
            let mut photos = vec![];

            for i in 1..10 {
                let fpath = assets_path.clone().join(format!("photo-{}.jpg", i));

                photos.push(fpath.to_str().unwrap().to_string());
            }

            let scheme = Scheme {
                target_dir: assets_path.join("output").to_str().unwrap().to_string(),
                format: String::from("jpg"),
                indi_width: 180,
                indi_height: 120,
                watermark_font_family: String::from("FreeMono"),
                watermark_font_size: 54.0,
                watermark_font_weight: 600,
            };

            r_generate(photos, scheme)
        })
    });
}

criterion_group!(benches, generate_benchmark);
criterion_main!(benches);
