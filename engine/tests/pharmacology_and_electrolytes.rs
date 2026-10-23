use engine::models::HeartSystem;

#[test]
fn test_hyperkalemia_depolarizes_resting_potential() {
    let mut baseline = HeartSystem::new();
    let mut hyper = HeartSystem::new();
    let dt = 0.01;

    // Configura hipercalemia severa (K+ = 8.5 mM vs basal 5.4 mM)
    hyper.update_params(8.5, 1.8, 140.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0);

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
        base_vrest,
        hyper_vrest
    );
}

#[test]
fn test_verapamil_prolongs_pr_interval() {
    let mut baseline = HeartSystem::new();
    let mut treated = HeartSystem::new();
    let dt = 0.01;

    // Aplica Verapamil (bloqueio moderado de canais de cálcio do nó AV: block_ca = 0.6)
    treated.update_params(5.4, 1.8, 140.0, 1.0, 1.0, 0.60, 1.0, 0.0, 0.0, 0.0, 0.0);

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
        pr_base,
        pr_treated
    );
}

#[test]
fn test_fibrosis_electrotonic_loading() {
    let mut baseline = HeartSystem::new();
    let mut fibrotic = HeartSystem::new();
    let dt = 0.01;

    // 80% de fibrose miocárdica (acoplamento eletrotônico maciço com fibroblastos)
    fibrotic.update_params(5.4, 1.8, 140.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.80);

    let mut v_fib_max = -999.0;
    let mut v_fib_min = 999.0;

    // Roda 3 segundos
    for _ in 0..300_000 {
        baseline.step(dt);
        fibrotic.step(dt);
        let vf = fibrotic.get_fibroblast_v();
        if vf > v_fib_max {
            v_fib_max = vf;
        }
        if vf < v_fib_min {
            v_fib_min = vf;
        }
    }

    // 1. Fibroblasto basal desacoplado repousa no valor canônico de MacCannell (-49.6 mV nominal)
    // muito acima do rodapé antigo do gráfico (-71 mV)
    let v_fibro_base = baseline.get_fibroblast_v();
    assert!(
        v_fibro_base > -58.0 && v_fibro_base < -45.0,
        "Potencial de repouso basal do fibroblasto ({:.1} mV) fora da faixa canônica de MacCannell (-45 a -58 mV)",
        v_fibro_base
    );

    // 2. Com fibrose, o fibroblasto apresenta ampla excursão oscilatória acoplada (> 50 mV)
    let excursion = v_fib_max - v_fib_min;
    assert!(
        excursion > 50.0,
        "Excursão oscilatória do fibroblasto acoplado insuficiente: {:.1} mV",
        excursion
    );
}

#[test]
fn test_sotalol_ikr_block_prolongs_action_potential_duration() {
    let mut baseline = HeartSystem::new();
    let mut sotalol = HeartSystem::new();
    let dt = 0.01;

    // Sotalol: 75% de bloqueio de I_Kr (block_ikr = 0.25)
    sotalol.set_advanced_pharm_params(0.25, 0.0, 0.0);

    // Simula 3.5 segundos para computar métricas eletrocardiográficas
    for _ in 0..350_000 {
        baseline.step(dt);
        sotalol.step(dt);
    }

    let qt_base = baseline.get_hud_metrics().qt;
    let qt_sotalol = sotalol.get_hud_metrics().qt;

    // Sotalol deve prolongar o intervalo QT de repolarização
    assert!(
        qt_sotalol > qt_base + 15.0,
        "Sotalol (bloqueio I_Kr) não prolongou o intervalo QT: basal={:.1} ms, sotalol={:.1} ms",
        qt_base,
        qt_sotalol
    );
}

#[test]
fn test_beta_blocker_attenuates_sympathetic_tachycardia() {
    let mut symp_only = HeartSystem::new();
    let mut symp_beta = HeartSystem::new();
    let dt = 0.01;

    // Estímulo simpático alto em ambos (symp = 0.8)
    symp_only.update_params(5.4, 1.8, 140.0, 1.0, 1.0, 1.0, 1.0, 0.8, 0.0, 0.0, 0.0);
    symp_beta.update_params(5.4, 1.8, 140.0, 1.0, 1.0, 1.0, 1.0, 0.8, 0.0, 0.0, 0.0);

    // Beta-bloqueador competitivo no segundo (Propranolol 85% de antagonismo)
    symp_beta.set_advanced_pharm_params(1.0, 0.85, 0.0);

    // Simula 4 segundos
    for _ in 0..400_000 {
        symp_only.step(dt);
        symp_beta.step(dt);
    }

    let bpm_symp = symp_only.get_hud_metrics().bpm;
    let bpm_beta = symp_beta.get_hud_metrics().bpm;

    assert!(
        bpm_symp > bpm_beta + 5.0,
        "Beta-bloqueador não atenuou a taquicardia simpática: symp_livre={:.1} BPM, com_beta_blocker={:.1} BPM",
        bpm_symp,
        bpm_beta
    );
}

#[test]
fn test_atropine_reverses_vagal_bradycardia() {
    let mut vagal_only = HeartSystem::new();
    let mut vagal_atropine = HeartSystem::new();
    let dt = 0.01;

    // Estímulo vagal/parassimpático moderado em ambos (parasymp = 0.6)
    vagal_only.update_params(5.4, 1.8, 140.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.6, 0.0, 0.0);
    vagal_atropine.update_params(5.4, 1.8, 140.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.6, 0.0, 0.0);

    // Atropina (antagonismo muscarínico total 1.0)
    vagal_atropine.set_advanced_pharm_params(1.0, 0.0, 1.0);

    // Simula 4 segundos
    for _ in 0..400_000 {
        vagal_only.step(dt);
        vagal_atropine.step(dt);
    }

    let bpm_vagal = vagal_only.get_hud_metrics().bpm;
    let bpm_atropine = vagal_atropine.get_hud_metrics().bpm;

    assert!(
        bpm_atropine > bpm_vagal + 5.0,
        "Atropina não reverteu a bradicardia vagal: vagal_puro={:.1} BPM, com_atropina={:.1} BPM",
        bpm_vagal,
        bpm_atropine
    );
}
