use engine::models::HeartSystem;

// =============================================================================
// SUÍTE DE TESTES DOS 4 EXPERIMENTOS UNIVERSITÁRIOS (ROTEIRO DE AULAS PRÁTICAS)
// Validação biofísica dos cenários didáticos descritos em docs/roteiro_aulas_praticas.md
// =============================================================================

#[test]
fn test_experimento_1_disturbios_eletroliticos_hipercalemia() {
    let dt = 0.01;

    // --- Etapa A: Normocalemia Baixa (4.0 mM) ---
    let mut sys_hypo = HeartSystem::new();
    sys_hypo.update_params(
        4.0, 1.8, 140.0,
        1.0, 1.0, 1.0, 1.0,
        0.0, 0.0, 0.0, 0.0,
    );
    for _ in 0..250_000 { sys_hypo.step(dt); }
    let v_rest_hypo = sys_hypo.get_hud_metrics().v_rest;

    // --- Basal: 5.4 mM ---
    let mut sys_base = HeartSystem::new();
    for _ in 0..250_000 { sys_base.step(dt); }
    let v_rest_base = sys_base.get_hud_metrics().v_rest;

    // Pela Equação de Nernst para o Potássio (E_K = (RT/F)*ln([K+]o/[K+]i)),
    // menor [K+]o hiperpolariza o potencial de repouso (torna-o mais negativo).
    assert!(
        v_rest_hypo <= v_rest_base,
        "Normocalemia 4.0 mM ({:.1} mV) deveria ser <= basal 5.4 mM ({:.1} mV)",
        v_rest_hypo, v_rest_base
    );

    // --- Etapa B: Hipercalemia Moderada (6.8 mM) ---
    let mut sys_mod = HeartSystem::new();
    sys_mod.update_params(
        6.8, 1.8, 140.0,
        1.0, 1.0, 1.0, 1.0,
        0.0, 0.0, 0.0, 0.0,
    );
    for _ in 0..250_000 { sys_mod.step(dt); }
    let v_rest_mod = sys_mod.get_hud_metrics().v_rest;

    // Repouso deve despolarizar (menos negativo) em relação ao basal
    assert!(
        v_rest_mod > v_rest_base + 3.0,
        "Hipercalemia moderada 6.8 mM ({:.1} mV) não despolarizou adequadamente vs basal ({:.1} mV)",
        v_rest_mod, v_rest_base
    );

    // --- Etapa C: Hipercalemia Grave (8.5 mM) ---
    let mut sys_severe = HeartSystem::new();
    sys_severe.update_params(
        8.5, 1.8, 140.0,
        1.0, 1.0, 1.0, 1.0,
        0.0, 0.0, 0.0, 0.0,
    );
    for _ in 0..250_000 { sys_severe.step(dt); }
    let v_rest_severe = sys_severe.get_hud_metrics().v_rest;

    // Repouso atinge patamar crítico menos negativo que -76 mV (inativação de I_Na)
    assert!(
        v_rest_severe > -76.0,
        "Hipercalemia grave 8.5 mM ({:.1} mV) deveria despolarizar repouso para > -76 mV",
        v_rest_severe
    );
    assert!(
        v_rest_severe > v_rest_mod,
        "Progressão eletrolítica violada: V_rest grave ({:.1} mV) <= moderada ({:.1} mV)",
        v_rest_severe, v_rest_mod
    );
}

#[test]
fn test_experimento_2_bloqueio_av_e_escape_verapamil() {
    let dt = 0.01;

    // --- Condição Basal ---
    let mut sys_base = HeartSystem::new();
    for _ in 0..400_000 { sys_base.step(dt); }
    let pr_base = sys_base.get_hud_metrics().pr;

    // --- Etapa A: Verapamil Moderado (35% bloqueio de Ca2+ / block_ca = 0.65) ---
    let mut sys_bav1 = HeartSystem::new();
    sys_bav1.update_params(
        5.4, 1.8, 140.0,
        1.0, 1.0, 0.65, 1.0,
        0.0, 0.0, 0.0, 0.0,
    );
    for _ in 0..400_000 { sys_bav1.step(dt); }
    let pr_bav1 = sys_bav1.get_hud_metrics().pr;

    // Dromotropismo negativo no Nó AV deve alargar o intervalo PR (> 10 ms acima do basal)
    assert!(
        pr_bav1 > pr_base + 10.0,
        "Verapamil a 35% ({:.1} ms) não causou alargamento dromotrópico de PR vs basal ({:.1} ms)",
        pr_bav1, pr_base
    );

    // --- Etapa C: Bloqueio AV Total / BAVT (block_ca = 0.20, abaixo do limiar de 0.35) ---
    let mut sys_bavt = HeartSystem::new();
    sys_bavt.update_params(
        5.4, 1.8, 140.0,
        1.0, 1.0, 0.20, 1.0,
        0.0, 0.0, 0.0, 0.0,
    );
    
    let mut last_v_vent = sys_bavt.get_vent_endo_v();
    let mut vent_beats_in_bavt = 0;
    
    // Roda 10 segundos
    for _step in 0..1_000_000 {
        sys_bavt.step(dt);
        let v = sys_bavt.get_vent_endo_v();
        if last_v_vent <= -20.0 && v > -20.0 {
            vent_beats_in_bavt += 1;
        }
        last_v_vent = v;
    }

    // No BAVT, a frequência ventricular deve cair drasticamente para o ritmo de escape terciário
    // (em 10 segundos a ~30-50 BPM, devemos ter entre 4 e 9 batimentos de escape de Purkinje)
    assert!(
        vent_beats_in_bavt >= 4 && vent_beats_in_bavt <= 9,
        "Frequência de escape idioventricular em BAVT fora da faixa esperada: {} batimentos em 10s (~30-50 BPM)",
        vent_beats_in_bavt
    );
}

#[test]
fn test_experimento_3_modulacao_autonomica_e_wiggers() {
    let dt = 0.01;

    // --- Estimulação Simpática (70%) ---
    let mut sys_symp = HeartSystem::new();
    sys_symp.update_params(
        5.4, 1.8, 140.0,
        1.0, 1.0, 1.0, 1.0,
        0.70, 0.0, 0.0, 0.0,
    );
    
    let mut max_lvp_symp = 0.0;
    for step in 0..400_000 {
        sys_symp.step(dt);
        if step > 200_000 {
            let lvp = sys_symp.get_lvp();
            if lvp > max_lvp_symp { max_lvp_symp = lvp; }
        }
    }
    let bpm_symp = sys_symp.get_hud_metrics().bpm;

    // Taquicardia sinusal por estimulação simpática beta-1 (> 90 BPM)
    assert!(
        bpm_symp > 90.0,
        "Estimulação Simpática 70% não gerou taquicardia sinusal esperada: {:.1} BPM",
        bpm_symp
    );
    // Efeito inotrópico positivo eleva pressão sistólica ventricular
    assert!(
        max_lvp_symp > 105.0,
        "Inotropismo simpático não elevou adequadamente LVP máx: {:.1} mmHg",
        max_lvp_symp
    );

    // --- Estimulação Parassimpática / Vagal (60%) ---
    let mut sys_vagal = HeartSystem::new();
    sys_vagal.update_params(
        5.4, 1.8, 140.0,
        1.0, 1.0, 1.0, 1.0,
        0.0, 0.60, 0.0, 0.0,
    );
    for _ in 0..400_000 { sys_vagal.step(dt); }
    let bpm_vagal = sys_vagal.get_hud_metrics().bpm;

    // Bradicardia sinusal por estimulação vagal M2 (< 65 BPM)
    assert!(
        bpm_vagal < 65.0,
        "Estimulação Parassimpática 60% não gerou bradicardia sinusal esperada: {:.1} BPM",
        bpm_vagal
    );
    assert!(
        bpm_symp > bpm_vagal + 25.0,
        "Diferencial autonômico insuficiente: simp={:.1} BPM, vagal={:.1} BPM",
        bpm_symp, bpm_vagal
    );
}

#[test]
fn test_experimento_4_fibrose_miocardica_acoplamento_eletrotonico() {
    let dt = 0.01;

    // --- Basal: Fibrose 0% (Fibroblasto eletroticamente isolado) ---
    let mut sys_clean = HeartSystem::new();
    
    // Deixar estabilizar 1.5 segundo para passar o transiente inicial assintótico
    for _ in 0..150_000 {
        sys_clean.step(dt);
    }
    
    let mut v_fib_clean_min = 999.0;
    let mut v_fib_clean_max = -999.0;
    
    for _ in 0..250_000 {
        sys_clean.step(dt);
        let vf = sys_clean.get_fibroblast_v();
        if vf < v_fib_clean_min { v_fib_clean_min = vf; }
        if vf > v_fib_clean_max { v_fib_clean_max = vf; }
    }

    // Sem fibrose, fibroblasto estável sem deflexões eletrotônicas (delta infinitesimal < 0.15 mV)
    let oscillation_clean = v_fib_clean_max - v_fib_clean_min;
    assert!(
        oscillation_clean < 0.15,
        "Fibroblasto sem acoplamento não deveria oscilar: delta = {:.4} mV",
        oscillation_clean
    );

    // --- Fibrose Moderada a Alta: 60% ---
    let mut sys_fib = HeartSystem::new();
    sys_fib.update_params(
        5.4, 1.8, 140.0,
        1.0, 1.0, 1.0, 1.0,
        0.0, 0.0, 0.0, 0.60,
    );
    
    let mut v_fib_active_max = -999.0;
    for _ in 0..250_000 {
        sys_fib.step(dt);
        let vf = sys_fib.get_fibroblast_v();
        if vf > v_fib_active_max { v_fib_active_max = vf; }
    }

    // Com acoplamento eletrotônico via gap junctions, o potencial do fibroblasto
    // é puxado para cima sincronicamente durante a despolarização do miócito (pico > -32 mV)
    assert!(
        v_fib_active_max > -32.0,
        "Acoplamento miócito-fibroblasto falhou: pico máximo do fibroblasto = {:.1} mV (esperado > -32 mV)",
        v_fib_active_max
    );
}
