use engine::models::HeartSystem;

#[test]
fn test_ventricular_volumes_and_ejection_fraction() {
    let mut system = HeartSystem::new();
    let dt = 0.01;
    let total_steps = 300_000; // 3.000 ms = 3 segundos (comporta múltiplos ciclos)

    for _ in 0..total_steps {
        system.step(dt);
    }

    let edv = system.get_hemo_edv();
    let esv = system.get_hemo_esv();
    let sv = system.get_hemo_sv();
    let ef = system.get_hemo_ef();
    let co = system.get_hemo_co();

    // Validações fisiológicas em repouso
    assert!(edv >= 105.0 && edv <= 140.0, "VDF fora da faixa esperada: {:.2} mL", edv);
    assert!(esv >= 35.0 && esv <= 65.0, "VSF fora da faixa esperada: {:.2} mL", esv);
    assert!(sv >= 55.0 && sv <= 85.0, "Volume sistólico fora da faixa: {:.2} mL", sv);
    assert!(ef >= 50.0 && ef <= 70.0, "Fração de ejeção fora da faixa: {:.2}%", ef);
    assert!(co >= 3.5 && co <= 7.0, "Débito cardíaco fora da faixa: {:.2} L/min", co);
}

#[test]
fn test_left_atrial_pressure_dynamics_and_waves() {
    let mut system = HeartSystem::new();
    let dt = 0.01;

    let mut lap_min = 100.0;
    let mut lap_max = -100.0;

    // Roda por 2.000 ms monitorando a pressão atrial
    for _ in 0..200_000 {
        system.step(dt);
        let lap = system.get_hemo_lap();
        if lap < lap_min { lap_min = lap; }
        if lap > lap_max { lap_max = lap; }
    }

    assert!(lap_min >= 4.0 && lap_min <= 9.0, "Pressão atrial diastólica basal fora da faixa: {:.2} mmHg", lap_min);
    assert!(lap_max >= 11.0 && lap_max <= 20.0, "Pico de pressão atrial (ondas a/v) fora da faixa: {:.2} mmHg", lap_max);
    assert!(lap_max > lap_min + 3.0, "Falta amplitude dinâmica nas ondas atriais");
}

#[test]
fn test_sympathetic_increases_inotropy_and_ef() {
    let mut basal = HeartSystem::new();
    let mut symp = HeartSystem::new();
    symp.update_params(5.4, 2.0, 140.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0); // 100% simpático

    let dt = 0.01;
    for _ in 0..250_000 {
        basal.step(dt);
        symp.step(dt);
    }

    let ef_basal = basal.get_hemo_ef();
    let ef_symp = symp.get_hemo_ef();

    assert!(ef_symp > ef_basal, "Estimulação simpática deve elevar a fração de ejeção: basal={:.1}%, symp={:.1}%", ef_basal, ef_symp);
}
