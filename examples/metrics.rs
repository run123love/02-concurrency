use std::{thread, time::Duration};

use concurrency::Metrics;
use rand::Rng;

fn main() -> anyhow::Result<()> {
    let metrics = Metrics::new();
    println!("{:?}", metrics.snapshot());

    for idx in 0..5 {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..5 {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{:?}", metrics.snapshot());
    }
    #[allow(unreachable_code)]
    Ok(())
}

fn task_worker(idx: usize, metrics: Metrics) -> anyhow::Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(100..500)));
            metrics.inc(format!("call.thread.worker.{}", idx).as_str())?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(metrics: Metrics) -> anyhow::Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(100..500)));
            let page = rng.gen_range(1..5);
            metrics
                .inc(format!("request.page.{}", page).as_str())
                .unwrap();
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}
