use engine::models::HeartSystem;

#[test]
fn test_hyperkalemia_depolarizes_resting_potential() {
    let mut baseline = HeartSystem::new();
    let mut hyper = HeartSystem::new();
    let dt = 0.01;
    
    // Configura hipercalemia severa (K+ = 8.5 mM vs basal 5.4 mM)
    hyper.update_params(
        8.5, 1.8, 140.0,
        1.0, 1.0, 1.0, 1.0,
        0.0, 0.0, 0.0, 0.0
    );

    // Simula 3 segundos
    for _ in 0..300_000 {
        baseline.step(dt);
        hyper.step(dt);
    }
    
    let base_vrest = baseline.get_hud_metrics().v_rest;
    let hyper_vrest = hyper.get_hud_metrics().v_rest;
    
    // Potencial sob hipercalemia deve ser significativamente menos negativo (despolarizado pela equação de Nernst)
    assert!(
        hyper_vrest > base_vrest + 8.0,
        "Hipercalemia não despolarizou adequadamente o repouso: basal={:.1} mV, hyper={:.1} mV",
        base_vrest, hyper_vrest
    );
}

#[test]
fn test_verapamil_prolongs_pr_interval() {
    let mut baseline = HeartSystem::new();
    let mut treated = HeartSystem::new();
    let dt = 0.01;
    
    // Aplica Verapamil (bloqueio moderado de canais de cálcio do nó AV: block_ca = 0.6)
    treated.update_params(
        5.4, 1.8, 140.0,
        1.0, 1.0, 0.60, 1.0,
        0.0, 0.0, 0.0, 0.0
    );

    // Simula 4 segundos para estabilizar métricas do HUD
    for _ in 0..400_000 {
        baseline.step(dt);
        treated.step(dt);
    }
    
    let pr_base = baseline.get_hud_metrics().pr;
    let pr_treated = treated.get_hud_metrics().pr;
    
    assert!(
        pr_treated > pr_base + 8.0,
        "Verapamil não prolongou o intervalo PR: basal={:.1} ms, tratado={:.1} ms",
        pr_base, pr_treated
    );
}

#[test]
fn test_fibrosis_electrotonic_loading() {
    let mut baseline = HeartSystem::new();
    let mut fibrotic = HeartSystem::new();
    let dt = 0.01;
    
    // 80% de fibrose miocárdica (acoplamento eletrotônico maciço com fibroblastos)
    fibrotic.update_params(
        5.4, 1.8, 140.0,
        1.0, 1.0, 1.0, 1.0,
        0.0, 0.0, 0.0, 0.80
    );

    // Roda 3 segundos
    for _ in 0..300_000 {
        baseline.step(dt);
        fibrotic.step(dt);
    }
    
    // Fibroblastos despolarizam e drenam corrente capacitiva do miócito epicárdico
    let v_fibro_base = baseline.get_fibroblast_v();
    let v_fibro_treated = fibrotic.get_fibroblast_v();
    
    assert!(
        v_fibro_treated != v_fibro_base,
        "Fibrose não alterou a voltagem acoplada do fibroblasto"
    );
}
