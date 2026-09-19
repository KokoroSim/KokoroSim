use engine::models::HeartSystem;

// =============================================================================
// TESTE DE RESILIÊNCIA E NÃO-CONGELAMENTO DE CÉLULAS SOB PARÂMETROS EXTREMOS
// Valida que alterações em K+, isquemia, bloqueadores e condutâncias não causam
// perda de automatismo ou aprisionamento de refratariedade nos pingers.
// =============================================================================

#[test]
fn test_potassium_spectrum_conduction_resilience() {
    let dt = 0.01;

    // Testa todo o espectro clínico de Potássio:
    // Normocalemia clínica baixa (4.0 mM), intermediária (4.8 mM), basal (5.4 mM)
    // e hipercalemia (7.0 mM). Em todas as faixas, a condução 1:1 SA -> Átrio -> Ventrículo
    // deve ser mantida sem congelamento do sistema nodal ou bloqueio de refratariedade.
    for &ko in &[4.0, 4.8, 5.4, 7.0] {
        let mut sys = HeartSystem::new();
        sys.update_params(
            ko, 1.8, 140.0,
            1.0, 1.0, 1.0, 1.0,
            0.0, 0.0, 0.0, 0.0,
        );

        let mut sa_beats = 0;
        let mut vent_beats = 0;
        let mut last_sa = sys.get_sa_v();
        let mut last_vent = sys.get_vent_endo_v();

        // 3 segundos (300k passos)
        for _ in 0..300_000 {
            sys.step(dt);
            let sa = sys.get_sa_v();
            let vent = sys.get_vent_endo_v();

            if last_sa <= -20.0 && sa > -20.0 { sa_beats += 1; }
            if last_vent <= -20.0 && vent > -20.0 { vent_beats += 1; }

            last_sa = sa;
            last_vent = vent;
        }

        assert!(
            sa_beats >= 2,
            "Nó SA parou de disparar sob Ko = {:.1} mM ({} batimentos)",
            ko, sa_beats
        );
        assert!(
            vent_beats >= 2,
            "Ventrículo não recebeu condução sob Ko = {:.1} mM ({} batimentos)",
            ko, vent_beats
        );
    }
}

#[test]
fn test_ischemia_severe_maintains_dynamic_conduction() {
    let dt = 0.01;

    // Isquemia aguda moderada a grave (40% e 70%)
    for &isch in &[0.40, 0.70] {
        let mut sys = HeartSystem::new();
        sys.update_params(
            5.4, 1.8, 140.0,
            1.0, 1.0, 1.0, 1.0,
            0.0, 0.0, isch, 0.0,
        );

        let mut vent_beats = 0;
        let mut last_vent = sys.get_vent_endo_v();

        for _ in 0..300_000 {
            sys.step(dt);
            let vent = sys.get_vent_endo_v();
            if last_vent <= -20.0 && vent > -20.0 { vent_beats += 1; }
            last_vent = vent;
        }

        assert!(
            vent_beats >= 2,
            "Isquemia {:.0}% travou condução ventricular ({} batimentos)",
            isch * 100.0, vent_beats
        );
    }
}
