use engine::models::HeartSystem;

/// Teste super longo de 300 segundos fisiológicos (5 minutos contínuos = 30.000.000 passos com dt=0.01ms).
/// Marcado como `#[ignore]` para ser executado estritamente sob demanda sem atrasar a suíte rápida de CI.
///
/// Como executar sob demanda:
/// ```bash
/// cargo test --test long_baseline_300s -- --ignored --nocapture
/// ```
#[test]
#[ignore = "Teste de calibração de longa duração (300 segundos / 5 minutos). Executar sob demanda via cargo test --test long_baseline_300s -- --ignored --nocapture"]
fn test_long_baseline_300s_stress_and_bounds() {
    let mut system = HeartSystem::new();
    system.set_rsa_enabled(true);
    let dt = 0.01;
    let total_steps: usize = 30_000_000; // 300 segundos = 5 minutos biológicos

    let mut last_v_vent = system.get_vent_endo_v();
    let mut beat_count: usize = 0;
    let mut last_beat_time: f64 = -1.0;

    // Acumuladores de Mínimo e Máximo para definição da Baseline
    let mut min_inst_bpm = f64::MAX;
    let mut max_inst_bpm = f64::MIN;
    let mut min_hud_bpm = f64::MAX;
    let mut max_hud_bpm = f64::MIN;

    let mut min_rr = f64::MAX;
    let mut max_rr = f64::MIN;
    let mut min_pr = f64::MAX;
    let mut max_pr = f64::MIN;
    let mut min_qrs = f64::MAX;
    let mut max_qrs = f64::MIN;
    let mut min_qt = f64::MAX;
    let mut max_qt = f64::MIN;
    let mut min_vrest = f64::MAX;
    let mut max_vrest = f64::MIN;

    let mut min_lvp = f64::MAX;
    let mut max_lvp = f64::MIN;
    let mut min_aop = f64::MAX;
    let mut max_aop = f64::MIN;
    let mut min_lap = f64::MAX;
    let mut max_lap = f64::MIN;

    let mut min_edv = f64::MAX;
    let mut max_edv = f64::MIN;
    let mut min_esv = f64::MAX;
    let mut max_esv = f64::MIN;
    let mut min_sv = f64::MAX;
    let mut max_sv = f64::MIN;
    let mut min_ef = f64::MAX;
    let mut max_ef = f64::MIN;
    let mut min_co = f64::MAX;
    let mut max_co = f64::MIN;

    let mut beats_last_interval = 0;
    let report_interval_steps = 3_000_000; // A cada 30 segundos

    println!("\n=========================================================================================");
    println!("🧪 INICIANDO SIMULAÇÃO BASELINE DE LONGA DURAÇÃO: 300 SEGUNDOS (5 MINUTOS CONTÍNUOS)");
    println!("   Passo de integração: dt = 0.01 ms | Passos totais: 30.000.000 | RSA: Ativada");
    println!("=========================================================================================\n");

    for step in 0..total_steps {
        let t = step as f64 * dt;
        system.step(dt);

        let v_vent = system.get_vent_endo_v();
        let lvp = system.get_lvp();
        let aop = system.get_aop();
        let lap = system.get_hemo_lap();

        // Rastreamento instantâneo contínuo de pressões (desconsiderando os primeiros 2s de inicialização)
        if lvp < min_lvp { min_lvp = lvp; }
        if lvp > max_lvp { max_lvp = lvp; }
        if t > 2000.0 {
            if aop < min_aop { min_aop = aop; }
        }
        if aop > max_aop { max_aop = aop; }
        if lap < min_lap { min_lap = lap; }
        if lap > max_lap { max_lap = lap; }

        // Validação contínua contra NaN e Infinitos
        assert!(!lvp.is_nan() && !lvp.is_infinite(), "LVP é NaN/Inf no passo {}", step);
        assert!(!aop.is_nan() && !aop.is_infinite(), "AoP é NaN/Inf no passo {}", step);
        assert!(!lap.is_nan() && !lap.is_infinite(), "LAP é NaN/Inf no passo {}", step);

        // Detecção de ativação ventricular (cruzamento ascendente de -20 mV)
        if last_v_vent <= -20.0 && v_vent > -20.0 {
            beat_count += 1;
            beats_last_interval += 1;

            if last_beat_time > 0.0 {
                let rr = t - last_beat_time;
                let inst_bpm = 60000.0 / rr;

                if inst_bpm < min_inst_bpm { min_inst_bpm = inst_bpm; }
                if inst_bpm > max_inst_bpm { max_inst_bpm = inst_bpm; }
                if rr < min_rr { min_rr = rr; }
                if rr > max_rr { max_rr = rr; }

                let hud = system.get_hud_metrics();
                if hud.bpm < min_hud_bpm { min_hud_bpm = hud.bpm; }
                if hud.bpm > max_hud_bpm { max_hud_bpm = hud.bpm; }
                if hud.pr < min_pr { min_pr = hud.pr; }
                if hud.pr > max_pr { max_pr = hud.pr; }
                if hud.qrs < min_qrs { min_qrs = hud.qrs; }
                if hud.qrs > max_qrs { max_qrs = hud.qrs; }
                if hud.qt < min_qt { min_qt = hud.qt; }
                if hud.qt > max_qt { max_qt = hud.qt; }
                if hud.v_rest < min_vrest { min_vrest = hud.v_rest; }
                if hud.v_rest > max_vrest { max_vrest = hud.v_rest; }

                if hud.edv < min_edv { min_edv = hud.edv; }
                if hud.edv > max_edv { max_edv = hud.edv; }
                if hud.esv < min_esv { min_esv = hud.esv; }
                if hud.esv > max_esv { max_esv = hud.esv; }
                if hud.sv < min_sv { min_sv = hud.sv; }
                if hud.sv > max_sv { max_sv = hud.sv; }
                if hud.ef < min_ef { min_ef = hud.ef; }
                if hud.ef > max_ef { max_ef = hud.ef; }
                if hud.co < min_co { min_co = hud.co; }
                if hud.co > max_co { max_co = hud.co; }
            }
            last_beat_time = t;
        }
        last_v_vent = v_vent;

        // Telemetria intermediária a cada 30 segundos
        if step > 0 && step % report_interval_steps == 0 {
            let elapsed_sec = (step as f64 * dt) / 1000.0;
            let hud = system.get_hud_metrics();
            println!(
                "⏱️ t = {:5.0}s | Batimentos: {:4} (+{:2} nos últimos 30s) | FC: {:4.1} bpm | AoP: {:5.1}/{:4.1} mmHg | VDF: {:5.1} mL | FE: {:4.1}% | DC: {:4.2} L/min",
                elapsed_sec, beat_count, beats_last_interval, hud.bpm,
                system.get_aop(), min_aop, hud.edv, hud.ef, hud.co
            );
            beats_last_interval = 0;
        }
    }

    let hud_final = system.get_hud_metrics();

    println!("\n=========================================================================================");
    println!("📊 RELATÓRIO DE BASELINE FISIOLÓGICA CONSOLIDADA (300 SEGUNDOS / 5 MINUTOS)");
    println!("=========================================================================================");
    println!("Parâmetro                       | Mínimo Observado | Máximo Observado | Valor Final    | Faixa Padrão");
    println!("--------------------------------+------------------+------------------+----------------+------------------");
    println!("Frequência Cardíaca Instantânea | {:6.1} bpm       | {:6.1} bpm       | ---            | 65.0 - 90.0 bpm", min_inst_bpm, max_inst_bpm);
    println!("Frequência Cardíaca Suavizada   | {:6.1} bpm       | {:6.1} bpm       | {:6.1} bpm     | 70.0 - 85.0 bpm", min_hud_bpm, max_hud_bpm, hud_final.bpm);
    println!("Intervalo RR                    | {:6.1} ms        | {:6.1} ms        | {:6.1} ms      | 660 - 920 ms", min_rr, max_rr, 60000.0 / hud_final.bpm);
    println!("Intervalo PR                    | {:6.1} ms        | {:6.1} ms        | {:6.1} ms      | 120 - 200 ms", min_pr, max_pr, hud_final.pr);
    println!("Duração do Complexo QRS         | {:6.1} ms        | {:6.1} ms        | {:6.1} ms      | 80 - 100 ms", min_qrs, max_qrs, hud_final.qrs);
    println!("Intervalo QT                    | {:6.1} ms        | {:6.1} ms        | {:6.1} ms      | 360 - 450 ms", min_qt, max_qt, hud_final.qt);
    println!("Potencial de Repouso Diastólico | {:6.1} mV        | {:6.1} mV        | {:6.1} mV      | -88.0 a -82.0 mV", min_vrest, max_vrest, hud_final.v_rest);
    println!("Pressão Ventricular Sistólica   | ---              | {:6.1} mmHg      | ---            | 110 - 135 mmHg", max_lvp);
    println!("Pressão Aórtica Sistólica (Pico)| ---              | {:6.1} mmHg      | ---            | 110 - 135 mmHg", max_aop);
    println!("Pressão Aórtica Diastólica (Mín)| {:6.1} mmHg      | ---              | ---            | >= 65.0 mmHg", min_aop);
    println!("Pressão Atrial Esquerda (Mín/Mx)| {:6.1} mmHg      | {:6.1} mmHg      | {:6.1} mmHg    | 4.0 a 16.0 mmHg", min_lap, max_lap, system.get_hemo_lap());
    println!("Volume Diastólico Final (VDF)   | {:6.1} mL        | {:6.1} mL        | {:6.1} mL      | 110 - 135 mL", min_edv, max_edv, hud_final.edv);
    println!("Volume Sistólico Final (VSF)    | {:6.1} mL        | {:6.1} mL        | {:6.1} mL      | 45 - 65 mL", min_esv, max_esv, hud_final.esv);
    println!("Volume Sistólico Efetivo (VS)   | {:6.1} mL        | {:6.1} mL        | {:6.1} mL      | 55 - 80 mL", min_sv, max_sv, hud_final.sv);
    println!("Fração de Ejeção (FE)           | {:6.1} %         | {:6.1} %         | {:6.1} %       | 50.0 - 65.0 %", min_ef, max_ef, hud_final.ef);
    println!("Débito Cardíaco (DC)            | {:6.2} L/min     | {:6.2} L/min     | {:6.2} L/min   | 4.0 - 6.5 L/min", min_co, max_co, hud_final.co);
    println!("Total de Batimentos em 5 min    | {:6} batimentos|                  |                | 360 - 420 batimentos", beat_count);
    println!("=========================================================================================\n");

    // ASSERÇÕES ESTRITAS DE BASELINE (Garantia de que a simulação nunca definha e permanece eutrófica)
    // 1. Cadência do Marcapasso
    assert!(
        beat_count >= 360 && beat_count <= 430,
        "Total de batimentos em 300s fora da faixa eutrófica esperada: {} (esperado entre 360 e 430 batimentos)",
        beat_count
    );
    assert!(
        min_inst_bpm >= 65.0,
        "FC instantânea mínima sofreu colapso por bradicardia excessiva: {:.1} bpm (esperado >= 65.0 bpm)",
        min_inst_bpm
    );
    assert!(
        max_inst_bpm <= 90.0,
        "FC instantânea máxima sofreu taquicardia anômala: {:.1} bpm (esperado <= 90.0 bpm)",
        max_inst_bpm
    );
    assert!(
        hud_final.bpm >= 72.0 && hud_final.bpm <= 84.0,
        "HUD BPM final fora da faixa basal: {:.1} bpm (esperado 72 a 84 bpm)",
        hud_final.bpm
    );

    // 2. Hemodinâmica e Pressões Cardiovasculares
    assert!(
        max_aop >= 110.0 && max_aop <= 140.0,
        "Pressão aórtica sistólica máxima fora da faixa normotensa: {:.1} mmHg",
        max_aop
    );
    assert!(
        min_aop >= 55.0,
        "Pressão aórtica diastólica mínima sofreu colapso/choque: {:.1} mmHg (esperado >= 55.0 mmHg)",
        min_aop
    );
    assert!(
        max_lvp >= 110.0 && max_lvp <= 140.0,
        "Pressão sistólica máxima do VE fora da faixa: {:.1} mmHg",
        max_lvp
    );
    assert!(
        min_lap >= 4.0 && max_lap <= 20.0,
        "Pressão atrial esquerda fora da faixa fisiológica: min={:.1}, max={:.1} mmHg",
        min_lap, max_lap
    );

    // 3. Volumes e Função de Bomba
    assert!(
        hud_final.ef >= 50.0 && hud_final.ef <= 65.0,
        "Fração de ejeção final do VE fora da faixa: {:.1}%",
        hud_final.ef
    );
    assert!(
        hud_final.co >= 4.0 && hud_final.co <= 6.5,
        "Débito cardíaco final fora da faixa: {:.2} L/min",
        hud_final.co
    );
    assert!(
        hud_final.edv >= 110.0 && hud_final.edv <= 135.0,
        "VDF final fora da faixa: {:.1} mL",
        hud_final.edv
    );
}
