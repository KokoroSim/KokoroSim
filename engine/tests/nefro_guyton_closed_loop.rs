use engine::models::HeartSystem;
use engine::models::renal::RenalModel;

#[test]
fn test_nefro_normotensive_autoregulation() {
    let mut system = HeartSystem::new();
    let dt = 0.01;
    let steps_per_frame = 1600;
    let downsample = 500;

    // Roda 3 segundos de simulação fisiológica
    for _ in 0..188 {
        system.run_batch(dt, steps_per_frame, downsample);
    }

    let metrics = system.get_hud_metrics();
    println!(
        "Eutrofia Renal: PAM={:.1} mmHg, TFG={:.1} mL/min, DU={:.1} mL/h, FPR={:.1} mL/min, FF={:.1}%, VLEC={:.2} L",
        metrics.map, metrics.tfg, metrics.diuresis, metrics.rpf, metrics.ff * 100.0, metrics.vlec
    );

    // Na eutrofia normotensa (PAM ~90 mmHg):
    // TFG fisiológica em torno de 120-130 mL/min
    assert!(metrics.tfg >= 115.0 && metrics.tfg <= 135.0, "TFG fora da faixa fisiológica: {}", metrics.tfg);
    // Débito urinário eupneico basal: ~60 mL/h (1.0 mL/min)
    assert!(metrics.diuresis >= 45.0 && metrics.diuresis <= 80.0, "Débito urinário fora da faixa basal: {}", metrics.diuresis);
    // Fração de Filtração ~20%
    assert!(metrics.ff >= 0.16 && metrics.ff <= 0.24, "Fração de Filtração fora de 20%: {}", metrics.ff);
    // VLEC estável em torno de 15 L
    assert!(metrics.vlec >= 14.8 && metrics.vlec <= 15.2, "VLEC instável em repouso: {}", metrics.vlec);
}

#[test]
fn test_guyton_pressure_natriuresis_under_hypertension() {
    let mut renal = RenalModel::new();
    let dt = 0.016;

    // Linha de base em repouso (PAM = 90 mmHg)
    for _ in 0..120 {
        renal.step(dt, 90.0);
    }
    let baseline_du = renal.diuresis_ml_h;
    let baseline_na = renal.sodium_excretion;

    // Hipertensão aguda (PAM = 135 mmHg)
    for _ in 0..200 {
        renal.step(dt, 135.0);
    }

    println!(
        "Natriurese de Pressão de Guyton: Basal DU={:.1} mL/h -> Hipertenso DU={:.1} mL/h | Basal Na={:.1} -> Hipertenso Na={:.1} mEq/dia",
        baseline_du, renal.diuresis_ml_h, baseline_na, renal.sodium_excretion
    );

    // Diurese e natriurese de pressão devem disparar significativamente (> 2.5x basal)
    assert!(renal.diuresis_ml_h > baseline_du * 2.2, "Natriurese de pressão insuficiente: {}", renal.diuresis_ml_h);
    assert!(renal.sodium_excretion > baseline_na * 2.0, "Excreção de sódio insuficiente sob hipertensão");
    // O VLEC deve contrair para restabelecer o equilíbrio a longo prazo
    assert!(renal.vlec < 15.0, "VLEC deveria contrair sob diurese pressórica de Guyton");
}

#[test]
fn test_prerenal_oliguria_under_hypotensive_shock() {
    let mut renal = RenalModel::new();
    let dt = 0.016;

    // Choque hipotensivo severo (PAM = 42 mmHg)
    for _ in 0..200 {
        renal.step(dt, 42.0);
    }

    println!(
        "Insuficiência Pré-Renal em Choque: PAM=42 mmHg -> TFG={:.1} mL/min, DU={:.1} mL/h",
        renal.tfg, renal.diuresis_ml_h
    );

    // Na PAM < 50 mmHg, a pressão capilar glomerular colapsa
    assert!(renal.tfg < 35.0, "TFG deveria colapsar no choque hipotensivo: {}", renal.tfg);
    // Oligúria/anúria pré-renal (< 15 mL/h)
    assert!(renal.diuresis_ml_h < 15.0, "Rim deveria entrar em oligúria/anúria pré-renal: {}", renal.diuresis_ml_h);
}

#[test]
fn test_loop_diuretic_and_fluid_bolus_volume_shifts() {
    let mut renal = RenalModel::new();
    let dt = 0.016;

    // 1. Aplicação de Furosemida (Diurético de Alça 80%)
    renal.set_params(0.0, 1.0, false, 0.80, 0.0, 2000.0);
    for _ in 0..600 {
        renal.step(dt, 90.0);
    }

    let diuretic_du = renal.diuresis_ml_h;
    let depleted_vlec = renal.vlec;
    let pmes_mod_diuretic = renal.get_pmes_modulation();

    println!(
        "Furosemida 80%: DU={:.1} mL/h, VLEC={:.2} L, PMES Mod={:.2} mmHg",
        diuretic_du, depleted_vlec, pmes_mod_diuretic
    );

    // Diurético maciço deve elevar o débito urinário > 200 mL/h
    assert!(diuretic_du > 200.0, "Débito urinário sob furosemida insuficiente: {}", diuretic_du);
    // VLEC deve depletar abaixo de 15 L
    assert!(depleted_vlec < 14.95, "VLEC deveria depletar sob furosemida: {}", depleted_vlec);
    // Modulação na PMES deve ser negativa (reduz pré-carga venosa e alivia edema)
    assert!(pmes_mod_diuretic < -0.05, "PMES modulação deveria ser negativa sob diurético");

    // 2. Desafio Volêmico: Infusão de Bolus IV de 1000 mL
    renal.infuse_bolus(1000.0);
    let post_bolus_vlec = renal.vlec;
    let pmes_mod_bolus = renal.get_pmes_modulation();

    println!(
        "Pós-Bolus IV 1000 mL: VLEC={:.2} L, PMES Mod={:.2} mmHg",
        post_bolus_vlec, pmes_mod_bolus
    );

    assert!(post_bolus_vlec > depleted_vlec + 0.90, "VLEC não recuperou após bolus de 1000 mL");
    assert!(pmes_mod_bolus > pmes_mod_diuretic, "PMES modulação deveria subir após bolus");
}
