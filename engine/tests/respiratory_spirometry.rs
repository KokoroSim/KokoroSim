use engine::models::HeartSystem;

#[test]
fn test_respiratory_eupnea_volumes_and_pressures() {
    let mut system = HeartSystem::new();
    let dt = 0.01; // ms
    let steps = 50000; // 500 segundos simulados (5 segundos em passos rápidos)

    for _ in 0..steps {
        system.step(dt);
        let vol = system.get_resp_vol();
        let p_pl = system.get_resp_ppl();

        // Volume em repouso deve oscilar entre CRF (2.3 L) e CPT (não ultrapassando 3.0 L em repouso)
        assert!(vol >= 2.25 && vol <= 2.85, "Volume de repouso fora dos limites: {:.2} L", vol);

        // Pressão intrapleural deve oscilar entre -5.0 e -8.0 cmH2O
        assert!(p_pl >= -8.2 && p_pl <= -4.8, "Pressão intrapleural anormal em repouso: {:.2} cmH2O", p_pl);
    }
}

#[test]
fn test_forced_spirometry_maneuver_and_tiffeneau() {
    let mut system = HeartSystem::new();
    let dt = 1.0; // passos de 1 ms

    // 1. Simula 2 segundos de repouso
    for _ in 0..2000 {
        system.step(dt);
    }

    // 2. Dispara manobra de espirometria forçada
    system.trigger_spirometry();
    assert!(system.is_in_spirometry(), "Deveria estar em manobra de espirometria");

    let mut max_vol = 0.0;
    // Avança 7 segundos para completar inspiração máxima + expiração forçada
    for _ in 0..7000 {
        system.step(dt);
        let v = system.get_resp_vol();
        if v > max_vol {
            max_vol = v;
        }
    }

    // O volume máximo deve atingir a Capacidade Pulmonar Total (CPT ~5.8 L)
    assert!(max_vol >= 5.5, "Inspiração máxima não atingiu CPT: {:.2} L", max_vol);

    let vef1 = system.get_vef1();
    let cvf = system.get_cvf();
    let tiffeneau = system.get_tiffeneau();

    println!("Espirometria Normal: VEF1 = {:.2} L, CVF = {:.2} L, Tiffeneau = {:.1}%", vef1, cvf, tiffeneau);

    assert!(cvf >= 4.0 && cvf <= 4.8, "CVF fora da faixa fisiológica: {:.2} L", cvf);
    assert!(vef1 >= 3.0 && vef1 <= 4.2, "VEF1 fora da faixa fisiológica: {:.2} L", vef1);
    assert!(tiffeneau >= 70.0 && tiffeneau <= 95.0, "Índice de Tiffeneau normal deve ser >= 70%: {:.1}%", tiffeneau);
}

#[test]
fn test_obstructive_disorder_reduces_tiffeneau() {
    let mut system = HeartSystem::new();
    // Aumenta resistência de vias aéreas simulando broncoespasmo severo (Raw = 6.5 cmH2O/(L/s))
    system.set_respiratory_params(15.0, 6.5, 0.10);

    let dt = 1.0;
    system.trigger_spirometry();

    for _ in 0..8000 {
        system.step(dt);
    }

    let tiffeneau = system.get_tiffeneau();
    println!("Espirometria Obstrutiva (Asma/DPOC): Tiffeneau = {:.1}%", tiffeneau);

    // No padrão obstrutivo, o Tiffeneau é patológico (< 70%)
    assert!(tiffeneau < 70.0, "Broncoespasmo deve reduzir o Tiffeneau abaixo de 70%: {:.1}%", tiffeneau);
}
