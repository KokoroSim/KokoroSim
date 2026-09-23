use engine::models::HeartSystem;

/// Teste 1: Estabilidade Eutrófica da Linha de Base em Repouso
/// Com o barorreflexo em malha fechada ativo, o sistema em repouso deve manter
/// a PAM ao redor do setpoint (~90-100 mmHg), sem drift, sem taquicardia ou bradicardia espúrias.
#[test]
fn test_baroreflex_eutrophic_resting_baseline_stability() {
    let mut heart = HeartSystem::new();
    heart.set_baroreflex_enabled(true);
    heart.set_rsa_enabled(false);

    let dt = 0.01; // 0.01 ms
    let steps_per_sec = 100_000;
    let sim_seconds = 15; // 15 segundos biológicos

    for _sec in 1..=sim_seconds {
        for _ in 0..steps_per_sec {
            heart.step(dt);
        }
    }

    let map = heart.get_hemo_map();
    let delta_symp = heart.get_baro_delta_symp();
    let delta_parasymp = heart.get_baro_delta_parasymp();
    let metrics = heart.get_hud_metrics();

    println!(
        "Barorreflexo Repouso: PAM={:.1} mmHg | BPM={:.1} | Δsymp={:.3} | Δparasymp={:.3}",
        map, metrics.bpm, delta_symp, delta_parasymp
    );

    // Em repouso normotenso: PAM deve ficar na faixa fisiológica (85 a 105 mmHg)
    assert!(
        map >= 85.0 && map <= 105.0,
        "PAM em repouso fora da faixa fisiológica: {:.1} mmHg",
        map
    );

    // Frequência Cardíaca estável (~70 a 85 BPM)
    assert!(
        metrics.bpm >= 70.0 && metrics.bpm <= 85.0,
        "FC em repouso instável sob barorreflexo: {:.1} BPM",
        metrics.bpm
    );

    // O desvio autonômico deve ser praticamente nulo em repouso (setpoint calibrado)
    assert!(
        delta_symp.abs() < 0.15,
        "Δsymp excessivo em repouso: {:.3}",
        delta_symp
    );
    assert!(
        delta_parasymp.abs() < 0.05,
        "Δparasymp excessivo em repouso: {:.3}",
        delta_parasymp
    );
}

/// Teste 2: Resposta Compensatória à Hipotensão Aguda (Taquicardia e Inotropismo Reflexos)
/// Quando a pressão cai significativamente abaixo do setpoint (ex: vasodilatação periférica aguda / nitroprussiato),
/// os barorreceptores reduzem seu disparo, gerando desinibição simpática (Δsymp > 0)
/// e aceleração compensatória do nó SA (taquicardia reflexa).
#[test]
fn test_baroreflex_compensatory_tachycardia_under_acute_hypotension() {
    let mut heart = HeartSystem::new();
    heart.set_baroreflex_enabled(true);
    heart.set_rsa_enabled(false);

    let dt = 0.01;
    let steps_per_sec = 100_000;

    // 1. Estabiliza 8 segundos em repouso
    for _ in 0..(8 * steps_per_sec) {
        heart.step(dt);
    }
    let baseline_bpm = heart.get_hud_metrics().bpm;
    let baseline_map = heart.get_hemo_map();

    // 2. Provoca queda abrupta de resistência vascular sistêmica (vasodilatação aguda / choque distributivo)
    heart.set_peripheral_resistance_ratio(0.55);

    // Roda mais 8 segundos para o arco barorreflexo simpático (~3.5s) agir e acelerar o nó SA
    for _ in 0..(8 * steps_per_sec) {
        heart.step(dt);
    }

    let map = heart.get_hemo_map();
    let delta_symp = heart.get_baro_delta_symp();
    let s_baro = heart.get_baro_s_baro();
    let post_bpm = heart.get_hud_metrics().bpm;

    println!(
        "Barorreflexo Hipotensão: PAM: {:.1} -> {:.1} mmHg | s_baro={:.3} | Δsymp={:.3} | BPM: {:.1} -> {:.1}",
        baseline_map, map, s_baro, delta_symp, baseline_bpm, post_bpm
    );

    // PAM sofre queda em relação à linha de base
    assert!(map < baseline_map, "PAM não reduziu sob vasodilatação (base: {:.1}, atual: {:.1})", baseline_map, map);

    // Barorreceptor detecta hipotensão (s_baro negativo)
    assert!(
        s_baro < 0.0,
        "Disparo barorreceptor deveria ser negativo sob hipotensão: {:.3}",
        s_baro
    );

    // Eferência simpática deve estar ativada (Δsymp > 0)
    assert!(
        delta_symp > 0.05,
        "Eferência simpática não ativou sob hipotensão: Δsymp={:.3}",
        delta_symp
    );

    // Taquicardia reflexa compensatória
    assert!(
        post_bpm > baseline_bpm,
        "Frequência cardíaca não aumentou sob hipotensão reflexa: {:.1} -> {:.1}",
        baseline_bpm, post_bpm
    );
}

/// Teste 3: Resposta Compensatória à Hipertensão Aguda (Bradicardia Reflexa Vagal)
/// Quando a pressão arterial média sobe consideravelmente acima de 92 mmHg,
/// os barorreceptores disparam em alta frequência, recrutando o núcleo motor do vago
/// e desacelerando o coração através de acetilcolina no nó SA.
#[test]
fn test_baroreflex_compensatory_bradycardia_under_acute_hypertension() {
    let mut heart = HeartSystem::new();
    heart.set_baroreflex_enabled(true);
    heart.set_rsa_enabled(false);

    let dt = 0.01;
    let steps_per_sec = 100_000;

    // 1. Estabiliza 5 segundos em repouso
    for _ in 0..(5 * steps_per_sec) {
        heart.step(dt);
    }
    let baseline_map = heart.get_hemo_map();

    // 2. Simula hipertensão / vasoconstrição maciça exógena através de infusão adrenérgica controlada
    heart.update_params(
        5.4, 2.0, 140.0,
        1.0, 1.0, 1.0, 1.0,
        0.50, 0.0, 0.0, 0.0 // symp exógeno = 0.50
    );

    // Roda mais 8 segundos
    for _ in 0..(8 * steps_per_sec) {
        heart.step(dt);
    }

    let map = heart.get_hemo_map();
    let delta_parasymp = heart.get_baro_delta_parasymp();
    let s_baro = heart.get_baro_s_baro();

    println!(
        "Barorreflexo Hipertensão: PAM={:.1} mmHg | s_baro={:.3} | Δparasymp={:.3}",
        map, s_baro, delta_parasymp
    );

    // PAM subiu acima da linha de base
    assert!(map > baseline_map, "PAM não subiu sob estímulo hipertensivo");

    // Disparo aferente positivo
    assert!(
        s_baro > 0.0,
        "s_baro deveria ser positivo sob hipertensão: {:.3}",
        s_baro
    );

    // Eferência parassimpática (vagal) ativada para frear o coração
    assert!(
        delta_parasymp > 0.01,
        "Eferência vagal não ativou sob hipertensão: Δparasymp={:.3}",
        delta_parasymp
    );
}

/// Teste 4: Desativação (Denervação Barorreceptora / Toggle OFF)
/// Ao desligar o barorreflexo, os offsets de modulação devem relaxar a zero suavemente.
#[test]
fn test_baroreflex_toggle_on_off_behavior() {
    let mut heart = HeartSystem::new();
    assert!(!heart.get_baroreflex_enabled());

    heart.set_baroreflex_enabled(true);
    assert!(heart.get_baroreflex_enabled());

    heart.set_baroreflex_enabled(false);
    assert!(!heart.get_baroreflex_enabled());

    let dt = 0.01;
    let steps_per_sec = 100_000;

    // Roda 4 segundos desligado
    for _ in 0..(4 * steps_per_sec) {
        heart.step(dt);
    }

    let delta_symp = heart.get_baro_delta_symp();
    let delta_parasymp = heart.get_baro_delta_parasymp();

    assert!(
        delta_symp.abs() < 0.01,
        "Δsymp não decaiu para zero com barorreflexo desativado: {:.3}",
        delta_symp
    );
    assert!(
        delta_parasymp.abs() < 0.01,
        "Δparasymp não decaiu para zero com barorreflexo desativado: {:.3}",
        delta_parasymp
    );
}
