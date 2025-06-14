use crate::*;

fn run_thread(p: Pool<String>, i: u64) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(i));
        let s: String = p
            .get(&"test".to_string(), std::time::Duration::from_secs(1))
            .unwrap()
            .unwrap()
            .call("count_vowels", "abc")
            .unwrap();
        println!("{}", s);
    })
}

#[test]
fn test_threads() {
    for i in 1..=3 {
        let data = include_bytes!("../../../wasm/code.wasm");
        let pool: Pool<String> = PoolBuilder::new().with_max_instances(i).build();

        let test = "test".to_string();
        pool.add_builder(
            test.clone(),
            extism::PluginBuilder::new(extism::Manifest::new([extism::Wasm::data(data)]))
                .with_wasi(true),
        );

        let mut threads = vec![];
        threads.push(run_thread(pool.clone(), 1000));
        threads.push(run_thread(pool.clone(), 1000));
        threads.push(run_thread(pool.clone(), 1000));
        threads.push(run_thread(pool.clone(), 1000));
        threads.push(run_thread(pool.clone(), 1000));
        threads.push(run_thread(pool.clone(), 1000));
        threads.push(run_thread(pool.clone(), 500));
        threads.push(run_thread(pool.clone(), 500));
        threads.push(run_thread(pool.clone(), 500));
        threads.push(run_thread(pool.clone(), 500));
        threads.push(run_thread(pool.clone(), 500));
        threads.push(run_thread(pool.clone(), 0));

        for t in threads {
            t.join().unwrap();
        }
        assert!(pool.count(&test) <= i);
    }
}
