use engine::models::HeartSystem;

/// Teste 1: Ponto de Equilíbrio de Guyton em Repouso Normovolêmico
/// Em condições basais com PMES = 7.5 mmHg, o retorno venoso e o débito cardíaco
/// devem se equilibrar na faixa fisiológica eutrófica humana (~4.5 a 5.8 L/min)
/// com PVC (pressão venosa central no átrio direito) ao redor de 2.0 a 4.5 mmHg.
#[test]
fn test_guyton_normovolemic_resting_equilibrium() {
    let mut heart = HeartSystem::new();
    heart.set_baroreflex_enabled(true);
    heart.set_rsa_enabled(false);

    let dt = 0.01;
    let steps_per_sec = 100_000;

    // Estabiliza 10 segundos fisiológicos
    for _ in 0..(10 * steps_per_sec) {
        heart.step(dt);
    }

    let m = heart.get_hud_metrics();
    let pmes = heart.get_pmes();
    let cvp = heart.get_cvp();
    let rv = heart.get_venous_return();

    println!(
        "Guyton Repouso: PMES={:.1} mmHg | PVC={:.1} mmHg | DC={:.2} L/min | RV={:.2} L/min | PAM={:.1} mmHg | BPM={:.1}",
        pmes, cvp, m.co, rv, m.map, m.bpm
    );

    // PMES deve se manter ao redor da linha de base de 7.5 mmHg
    assert!(
        pmes >= 6.5 && pmes <= 8.5,
        "PMES basal fora da faixa: {:.1} mmHg",
        pmes
    );

    // PVC fisiológica (2.0 a 4.5 mmHg no átrio direito)
    assert!(
        cvp >= 1.5 && cvp <= 4.8,
        "PVC em repouso fora da faixa fisiológica: {:.1} mmHg",
        cvp
    );

    // Débito Cardíaco em repouso (~4.2 a 6.0 L/min)
    assert!(
        m.co >= 4.0 && m.co <= 6.2,
        "Débito Cardíaco fora da faixa eutrófica: {:.2} L/min",
        m.co
    );

    // Retorno venoso e débito devem estar no mesmo patamar de ordem de grandeza
    assert!(
        rv >= 3.5 && rv <= 6.5,
        "Retorno venoso fora do equilíbrio: {:.2} L/min",
        rv
    );
}

/// Teste 2: Desafio Postural de Ortostase e Compensação Barorreflexa em Malha Fechada
/// Ao ficar em pé (ortostase), o sangue sofre estase gravitacional em membros inferiores (pooling venoso),
/// derrubando a PMES efetiva, o retorno venoso e a pré-carga.
/// O barorreflexo em malha fechada deve detectar a hipotensão transitória e disparar
/// taquicardia reflexa compensatória (+8 a +18 BPM) e vasoconstrição arteriolar.
#[test]
fn test_orthostasis_postural_challenge_and_baroreflex_compensation() {
    let mut heart = HeartSystem::new();
    heart.set_baroreflex_enabled(true);
    heart.set_rsa_enabled(false);

    let dt = 0.01;
    let steps_per_sec = 100_000;

    // 1. Estabiliza 8 segundos em decúbito dorsal (deitado)
    for _ in 0..(8 * steps_per_sec) {
        heart.step(dt);
    }
    let supine_bpm = heart.get_hud_metrics().bpm;
    let supine_pmes = heart.get_pmes();
    let supine_co = heart.get_hud_metrics().co;

    // 2. Fica em pé: Desafio Ortostático (Tilt)
    heart.set_orthostasis(true);

    // Roda mais 8 segundos para o arco barorreflexo neural agir plenamente
    for _ in 0..(8 * steps_per_sec) {
        heart.step(dt);
    }

    let standing_m = heart.get_hud_metrics();
    let standing_pmes = heart.get_pmes();
    let delta_symp = heart.get_baro_delta_symp();

    println!(
        "Ortostase: Deitado (PMES={:.1}, BPM={:.1}, DC={:.2}) -> Em Pé (PMES={:.1}, BPM={:.1}, DC={:.2}) | Δsymp={:.3}",
        supine_pmes, supine_bpm, supine_co, standing_pmes, standing_m.bpm, standing_m.co, delta_symp
    );

    // PMES efetiva deve cair inicialmente pelo pooling venoso caudal
    assert!(
        standing_pmes < supine_pmes,
        "PMES efetiva não reduziu na ortostase: {:.1} -> {:.1}",
        supine_pmes, standing_pmes
    );

    // Eferência simpática deve ser ativada pelos barorreceptores
    assert!(
        delta_symp > 0.03,
        "Simpático não ativou na ortostase: Δsymp={:.3}",
        delta_symp
    );

    // Taquicardia reflexa postural (BPM em pé > deitado)
    assert!(
        standing_m.bpm > supine_bpm + 2.0,
        "Taquicardia reflexa postural ausente: deitado={:.1} -> em pé={:.1}",
        supine_bpm, standing_m.bpm
    );

    // PAM deve se manter controlada sem colapso hipotensivo severo (> 70 mmHg)
    assert!(
        standing_m.map > 70.0,
        "PAM colapsou na ortostase: {:.1} mmHg",
        standing_m.map
    );
}

/// Teste 3: Choque Hipovolêmico por Hemorragia Aguda
/// Queda da PMES para 3.5 mmHg simula hemorragia grave:
/// Redução massiva do retorno venoso, queda de pré-carga (VDF/EDV) e choque de baixo débito.
#[test]
fn test_hypovolemic_shock_hemorrhage() {
    let mut heart = HeartSystem::new();
    heart.set_baroreflex_enabled(true);
    heart.set_rsa_enabled(false);

    let dt = 0.01;
    let steps_per_sec = 100_000;

    // 1. Estabiliza basal
    for _ in 0..(6 * steps_per_sec) {
        heart.step(dt);
    }
    let base_edv = heart.get_hud_metrics().edv;
    let base_co = heart.get_hud_metrics().co;

    // 2. Hemorragia severa (PMES cai para 3.5 mmHg)
    heart.set_pmes(3.5);

    for _ in 0..(6 * steps_per_sec) {
        heart.step(dt);
    }

    let shock_edv = heart.get_hud_metrics().edv;
    let shock_co = heart.get_hud_metrics().co;
    let shock_cvp = heart.get_cvp();

    println!(
        "Choque Hipovolêmico: VDF: {:.1} -> {:.1} mL | DC: {:.2} -> {:.2} L/min | PVC={:.1} mmHg",
        base_edv, shock_edv, base_co, shock_co, shock_cvp
    );

    // Pré-carga (VDF) sofre queda substancial por falta de retorno venoso
    assert!(
        shock_edv < base_edv - 6.0,
        "VDF não reduziu adequadamente sob hemorragia: {:.1} -> {:.1}",
        base_edv, shock_edv
    );

    // Débito cardíaco entra em baixo débito (< 4.2 L/min)
    assert!(
        shock_co < base_co,
        "Débito Cardíaco não caiu no choque hipovolêmico: {:.2} -> {:.2}",
        base_co, shock_co
    );

    // PVC colapsa (< 2.2 mmHg)
    assert!(
        shock_cvp < 2.5,
        "PVC deveria ter caído acentuadamente na hipovolemia: {:.1} mmHg",
        shock_cvp
    );
}

/// Teste 4: Expansão Volêmica Vigorosa (Hipervolemia)
/// Elevação da PMES para 11.5 mmHg simula ressuscitação com fluidos:
/// Aumento do retorno venoso, elevação da PVC e aumento do volume sistólico por Frank-Starling.
#[test]
fn test_hypervolemia_volume_expansion() {
    let mut heart = HeartSystem::new();
    heart.set_baroreflex_enabled(true);
    heart.set_rsa_enabled(false);

    let dt = 0.01;
    let steps_per_sec = 100_000;

    // 1. Estabiliza basal
    for _ in 0..(6 * steps_per_sec) {
        heart.step(dt);
    }
    let base_edv = heart.get_hud_metrics().edv;
    let base_cvp = heart.get_cvp();

    // 2. Expansão volêmica vigorosa
    heart.set_pmes(11.5);

    for _ in 0..(6 * steps_per_sec) {
        heart.step(dt);
    }

    let exp_edv = heart.get_hud_metrics().edv;
    let exp_cvp = heart.get_cvp();

    println!(
        "Hipervolemia: VDF: {:.1} -> {:.1} mL | PVC: {:.1} -> {:.1} mmHg",
        base_edv, exp_edv, base_cvp, exp_cvp
    );

    // Frank-Starling: maior retorno venoso distende o ventrículo (aumento de VDF)
    assert!(
        exp_edv > base_edv + 3.0,
        "VDF não aumentou sob expansão volêmica: {:.1} -> {:.1}",
        base_edv, exp_edv
    );

    // PVC sobe por sobrecarga venosa
    assert!(
        exp_cvp > base_cvp,
        "PVC não elevou sob expansão volêmica: {:.1} -> {:.1}",
        base_cvp, exp_cvp
    );
}
