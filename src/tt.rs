
    let mut supreme_birb: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::new()));

    let step_size:f64 = 0.33;

    let i  = 7;
    let pl = 3;
    let (tx, rx) = channel();

    // Get variables
    let supreme = Arc::clone(&supreme_birb);
    let index     = i;
    let l_channel = tx.clone();
    let phase_len = pl;
    let timestamp = 0;
    let timeout   = 0;
    let sample_count = 10;
    let warmup   = 10;
    let iterations = 30;
    let corner_1 = math::Complex::new(33.0, 44.0);
    let corner_2 = math::Complex::new(33.0, 44.0);
    let c = tx.clone();

    let t = thread::spawn(move || {

        // Create necessary data structures
        let mut orbits: Vec<Vec<math::Complex>> = Vec::with_capacity(phase_len);
        let mut mh_orbits = math::MHOrbits::new(sample_count, warmup, iterations, corner_1, corner_2);

        // Compute!
        while !mh_orbits.finished() {

            /* Produce new orbits */

            for _ in 0..phase_len {

                if let Some(o) = mh_orbits.next() {
                    orbits.push(o);
                    // not breaking on None here, cause we need logging info send to the logger fn
                }
            }


            /* Write back to supreme birb */

            let mut birb = supreme.lock().unwrap();

            orbits.iter().for_each(|o| write_back(o, &mut *birb, step_size));

            /* Send logging info */
            l_channel.send((index, mh_orbits.remaining()));

        }

    });

    t.join();

