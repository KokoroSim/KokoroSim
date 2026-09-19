use engine::models::HeartSystem;

#[test]
fn test_wiggers_cycle_pressures() {
    let mut system = HeartSystem::new();
    let dt = 0.01;
    
    let mut max_lvp: f64 = 0.0;
    let mut min_lvp: f64 = 999.0;
    let mut max_aop: f64 = 0.0;
    let mut min_aop: f64 = 999.0;
    
    // Deixa passar o primeiro ciclo de acomodação e monitora dos 2s aos 6s
    for step in 0..600_000 {
        let t = step as f64 * dt;
        system.step(dt);
        
        if t >= 2000.0 {
            let lvp = system.get_lvp();
            let aop = system.get_aop();
            
            if lvp > max_lvp { max_lvp = lvp; }
            if lvp < min_lvp { min_lvp = lvp; }
            if aop > max_aop { max_aop = aop; }
            if aop < min_aop { min_aop = aop; }
        }
    }
    
    // Pressão Sistólica Ventricular Esquerda deve atingir ~95 a 125 mmHg em repouso basal
    assert!(
        max_lvp >= 90.0 && max_lvp <= 130.0,
        "Pico de LVP sistólico fora da faixa normal: {:.1} mmHg",
        max_lvp
    );
    
    // Pressão Diastólica Ventricular Esquerda deve cair para < 15 mmHg
    assert!(
        min_lvp <= 15.0,
        "Mínimo de LVP diastólico muito elevado: {:.1} mmHg",
        min_lvp
    );
    
    // Pressão Aórtica Sistólica em repouso basal (~95-125 mmHg)
    assert!(
        max_aop >= 90.0 && max_aop <= 130.0,
        "Pico de AoP sistólico fora da faixa: {:.1} mmHg",
        max_aop
    );
    
    // Pressão Aórtica Diastólica deve permanecer entre ~50-85 mmHg (amortecimento Windkessel)
    assert!(
        min_aop >= 50.0 && min_aop <= 88.0,
        "AoP diastólica fora da faixa Windkessel: {:.1} mmHg",
        min_aop
    );
}

#[test]
fn test_heart_sound_events_in_batch() {
    let mut system = HeartSystem::new();
    let dt = 0.01;
    let steps_per_frame = 1600; // 16 ms a 60 FPS
    let downsample = 4;
    
    let mut b1_count = 0;
    let mut b2_count = 0;
    let mut r_peak_count = 0;
    
    // Roda 4 segundos em lotes (como o Dioxus faz)
    for _ in 0..250 {
        let batch = system.run_batch(dt, steps_per_frame, downsample);
        let chunk_size = 12;
        for i in 0..(batch.len() / chunk_size) {
            let sound_code = batch[i * chunk_size + 11] as usize;
            if (sound_code & 1) != 0 { r_peak_count += 1; }
            if (sound_code & 2) != 0 { b1_count += 1; }
            if (sound_code & 4) != 0 { b2_count += 1; }
        }
    }
    
    // Em 4 segundos a ~78 BPM, devemos registrar aproximadamente 4 a 6 ciclos completos
    assert!(
        r_peak_count >= 4 && r_peak_count <= 6,
        "Eventos de Onda R fora da faixa esperada: {}",
        r_peak_count
    );
    assert!(
        b1_count >= 4 && b1_count <= 6,
        "Eventos da bulha B1 fora da faixa esperada: {}",
        b1_count
    );
    assert!(
        b2_count >= 4 && b2_count <= 6,
        "Eventos da bulha B2 fora da faixa esperada: {}",
        b2_count
    );
}
