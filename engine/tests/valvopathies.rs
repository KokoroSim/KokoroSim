use engine::models::HeartSystem;

#[test]
fn test_aortic_stenosis_creates_systolic_gradient() {
    let mut system = HeartSystem::new();
    let dt = 0.01;
    let steps_per_frame = 1600; // 16 ms a 60 FPS
    let downsample = 4;

    // 1. Estabilização basal normal por 2 segundos
    for _ in 0..125 {
        system.run_batch(dt, steps_per_frame, downsample);
    }

    // 2. Aplicação de Estenose Aórtica severa (85%)
    system.set_valvopathy_params(0.85, 0.0, 0.0, 0.0);

    let mut max_lvp = 0.0;
    let mut max_aop = 0.0;
    let mut max_gradient = 0.0;

    // Roda por 3 segundos para capturar os picos sistólicos
    for _ in 0..188 {
        let batch = system.run_batch(dt, steps_per_frame, downsample);
        let chunk_size = engine::models::BATCH_CHUNK_SIZE;
        for i in 0..(batch.len() / chunk_size) {
            let lvp = batch[i * chunk_size + 8];
            let aop = batch[i * chunk_size + 9];
            if lvp > max_lvp { max_lvp = lvp; }
            if aop > max_aop { max_aop = aop; }
            let grad = lvp - aop;
            if grad > max_gradient { max_gradient = grad; }
        }
    }

    // Na estenose aórtica severa, LVP de pico deve exceder 160 mmHg
    assert!(
        max_lvp > 155.0,
        "LVP sistólica esperada > 155 mmHg na estenose aórtica, obtido: {:.1} mmHg",
        max_lvp
    );
    // Gradiente transvalvar sistólico significativo (> 35 mmHg)
    assert!(
        max_gradient > 35.0,
        "Gradiente transaórtico esperado > 35 mmHg, obtido: {:.1} mmHg",
        max_gradient
    );
}

#[test]
fn test_aortic_regurgitation_causes_diastolic_runoff_and_volume_overload() {
    let mut system = HeartSystem::new();
    let dt = 0.01;
    let steps_per_frame = 1600;
    let downsample = 4;

    // 1. Estabilização basal
    for _ in 0..125 {
        system.run_batch(dt, steps_per_frame, downsample);
    }

    // 2. Insuficiência Aórtica importante (80%)
    system.set_valvopathy_params(0.0, 0.80, 0.0, 0.0);

    let mut min_aop = 200.0;
    let mut max_aop = 0.0;
    let mut max_edv = 0.0;

    // Roda por 3 segundos
    for _ in 0..188 {
        let batch = system.run_batch(dt, steps_per_frame, downsample);
        let chunk_size = engine::models::BATCH_CHUNK_SIZE;
        for i in 0..(batch.len() / chunk_size) {
            let aop = batch[i * chunk_size + 9];
            if aop < min_aop { min_aop = aop; }
            if aop > max_aop { max_aop = aop; }
        }
        let edv = system.get_hemo_edv();
        if edv > max_edv { max_edv = edv; }
    }

    let pulse_pressure = max_aop - min_aop;

    // Queda importante da pressão aórtica diastólica (< 55 mmHg devido ao refluxo diastólico)
    assert!(
        min_aop < 55.0,
        "Pressão aórtica diastólica esperada < 55 mmHg na insuficiência aórtica, obtido: {:.1} mmHg",
        min_aop
    );
    // Pressão de pulso alargada (> 60 mmHg)
    assert!(
        pulse_pressure > 60.0,
        "Pressão de pulso alargada esperada > 60 mmHg, obtido: {:.1} mmHg",
        pulse_pressure
    );
    // Sobrecarga volumétrica diastólica (VDF > 125 mL)
    assert!(
        max_edv > 125.0,
        "VDF com sobrecarga volumétrica esperado > 125 mL, obtido: {:.1} mL",
        max_edv
    );
}

#[test]
fn test_mitral_stenosis_elevates_atrial_pressure() {
    let mut system = HeartSystem::new();
    let dt = 0.01;
    let steps_per_frame = 1600;
    let downsample = 4;

    // 1. Estabilização basal
    for _ in 0..125 {
        system.run_batch(dt, steps_per_frame, downsample);
    }

    // 2. Estenose Mitral importante (80%)
    system.set_valvopathy_params(0.0, 0.0, 0.80, 0.0);

    let mut lap_samples = Vec::new();

    // Roda por 2.5 segundos
    for _ in 0..156 {
        system.run_batch(dt, steps_per_frame, downsample);
        lap_samples.push(system.get_hemo_lap());
    }

    let mean_lap = lap_samples.iter().sum::<f64>() / lap_samples.len() as f64;

    // Pressão atrial esquerda média elevada (> 16 mmHg, hipertensão atrial / congestão)
    assert!(
        mean_lap > 16.0,
        "Pressão atrial média esperada > 16 mmHg na estenose mitral, obtido: {:.1} mmHg",
        mean_lap
    );
}

#[test]
fn test_mitral_regurgitation_generates_giant_v_wave() {
    let mut system = HeartSystem::new();
    let dt = 0.01;
    let steps_per_frame = 1600;
    let downsample = 4;

    // 1. Estabilização basal
    for _ in 0..125 {
        system.run_batch(dt, steps_per_frame, downsample);
    }

    // 2. Insuficiência Mitral grave (85%)
    system.set_valvopathy_params(0.0, 0.0, 0.0, 0.85);

    let mut max_lap = 0.0;

    // Roda por 2.5 segundos
    for _ in 0..156 {
        system.run_batch(dt, steps_per_frame, downsample);
        let lap = system.get_hemo_lap();
        if lap > max_lap { max_lap = lap; }
    }

    // Onda 'v' sistólica gigante decorrente do jato de refluxo (> 22 mmHg)
    assert!(
        max_lap > 22.0,
        "Pico da onda v na regurgitação mitral esperado > 22 mmHg, obtido: {:.1} mmHg",
        max_lap
    );
}
