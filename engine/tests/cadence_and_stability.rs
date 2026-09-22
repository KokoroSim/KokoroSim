use engine::models::HeartSystem;

#[test]
fn test_sinus_cadence_and_bpm_stability() {
    let mut system = HeartSystem::new();
    let dt = 0.01;
    let mut last_v_vent = system.get_vent_endo_v();
    let mut last_beat_time = -1.0;
    let mut beat_num = 0;

    // Simula 10 segundos fisiológicos (1.000.000 de passos)
    for step in 0..1_000_000 {
        let t = step as f64 * dt;
        system.step(dt);
        let v_vent = system.get_vent_endo_v();
        
        // Detecção de ativação ventricular (cruzamento ascendente de -20 mV)
        if last_v_vent <= -20.0 && v_vent > -20.0 {
            beat_num += 1;
            
            // O primeiro batimento DEVE vir da propagação sinusal normal (> 450 ms),
            // comprovando a ausência de extrassístoles precoces de Purkinje (artefato de 111 ms).
            if beat_num == 1 {
                assert!(
                    t > 450.0,
                    "Primeiro batimento ocorreu prematuramente em t = {:.1} ms (esperado > 450 ms pós-SA)",
                    t
                );
            }
            
            if last_beat_time > 0.0 {
                let rr = t - last_beat_time;
                let inst_bpm = 60000.0 / rr;
                
                // Frequência instantânea deve estar estritamente na faixa fisiológica (70 a 85 BPM)
                assert!(
                    inst_bpm >= 70.0 && inst_bpm <= 85.0,
                    "Batimento #{} com BPM instantâneo anômalo: {:.1} (RR = {:.1} ms)",
                    beat_num, inst_bpm, rr
                );
                
                // Métrica suavizada do HUD não pode saltar para ~98 BPM
                let hud = system.get_hud_metrics();
                assert!(
                    hud.bpm >= 72.0 && hud.bpm <= 80.0,
                    "Batimento #{} com HUD BPM fora da faixa: {:.1}",
                    beat_num, hud.bpm
                );
            }
            last_beat_time = t;
        }
        last_v_vent = v_vent;
    }

    // Deve ter produzido entre 11 e 15 batimentos em 10 segundos (~76-78 BPM)
    assert!(
        beat_num >= 11 && beat_num <= 15,
        "Total de batimentos em 10s fora do esperado: {}",
        beat_num
    );
}

#[test]
fn test_numerical_stability_no_nan_or_inf() {
    let mut system = HeartSystem::new();
    let dt = 0.01;

    for step in 0..500_000 { // 5 segundos
        system.step(dt);
        
        // Verifica que nenhuma variável vital é NaN ou Infinita
        let sa = system.get_sa_v();
        let av = system.get_av_v();
        let atr = system.get_atrium_v();
        let purk = system.get_purkinje_v();
        let endo = system.get_vent_endo_v();
        let epi = system.get_vent_epi_v();
        let lvp = system.get_lvp();
        let aop = system.get_aop();
        let ecg = system.compute_ecg();

        assert!(!sa.is_nan() && !sa.is_infinite(), "SA V é NaN/Inf no passo {}", step);
        assert!(!av.is_nan() && !av.is_infinite(), "AV V é NaN/Inf no passo {}", step);
        assert!(!atr.is_nan() && !atr.is_infinite(), "Atrium V é NaN/Inf no passo {}", step);
        assert!(!purk.is_nan() && !purk.is_infinite(), "Purkinje V é NaN/Inf no passo {}", step);
        assert!(!endo.is_nan() && !endo.is_infinite(), "Endo V é NaN/Inf no passo {}", step);
        assert!(!epi.is_nan() && !epi.is_infinite(), "Epi V é NaN/Inf no passo {}", step);
        assert!(!lvp.is_nan() && !lvp.is_infinite(), "LVP é NaN/Inf no passo {}", step);
        assert!(!aop.is_nan() && !aop.is_infinite(), "AoP é NaN/Inf no passo {}", step);
        assert!(!ecg.is_nan() && !ecg.is_infinite(), "ECG é NaN/Inf no passo {}", step);
    }
}

#[test]
fn test_rsa_long_term_stability_and_normotension() {
    let mut system = HeartSystem::new();
    system.set_rsa_enabled(true);
    let dt = 0.01;
    let mut last_v_vent = system.get_vent_endo_v();
    let mut beat_count = 0;
    let mut last_beat_time = -1.0;
    let mut min_bpm = 200.0;
    let mut max_bpm = 0.0;

    // Simula 30 segundos fisiológicos (3.000.000 de passos) sob respiração contínua com RSA
    for step in 0..3_000_000 {
        let t = step as f64 * dt;
        system.step(dt);
        let v_vent = system.get_vent_endo_v();

        if last_v_vent <= -20.0 && v_vent > -20.0 {
            beat_count += 1;
            if last_beat_time > 0.0 {
                let rr = t - last_beat_time;
                let inst_bpm = 60000.0 / rr;
                if inst_bpm < min_bpm { min_bpm = inst_bpm; }
                if inst_bpm > max_bpm { max_bpm = inst_bpm; }

                // Mesmo com flutuações respiratórias, a FC instantânea nunca deve colapsar (< 65 BPM) nem disparar (> 92 BPM)
                assert!(
                    inst_bpm >= 65.0 && inst_bpm <= 92.0,
                    "Batimento #{} com BPM instantâneo anômalo sob RSA: {:.1} (RR = {:.1} ms em t={:.1}s)",
                    beat_count, inst_bpm, rr, t / 1000.0
                );
            }
            last_beat_time = t;
        }
        last_v_vent = v_vent;
    }

    // Deve ter produzido entre 35 e 45 batimentos em 30 segundos (~75-80 BPM médio)
    assert!(
        beat_count >= 35 && beat_count <= 45,
        "Total de batimentos em 30s sob RSA fora da faixa fisiológica: {}",
        beat_count
    );

    // Métrica do HUD deve estar plenamente na faixa eutrófica
    let hud = system.get_hud_metrics();
    assert!(
        hud.bpm >= 72.0 && hud.bpm <= 84.0,
        "HUD BPM final em 30s sob RSA fora da faixa esperada: {:.1} BPM",
        hud.bpm
    );

    // Estabilidade hemodinâmica preservada (sem hipotensão por bradicardia)
    let ef = system.get_hemo_ef();
    let sv = system.get_hemo_sv();
    let edv = system.get_hemo_edv();
    assert!(ef >= 50.0 && ef <= 65.0, "Fração de ejeção fora da faixa sob RSA: {:.1}%", ef);
    assert!(sv >= 60.0 && sv <= 85.0, "Volume sistólico fora da faixa sob RSA: {:.1} mL", sv);
    assert!(edv >= 115.0 && edv <= 140.0, "VDF fora da faixa sob RSA: {:.1} mL", edv);
}

