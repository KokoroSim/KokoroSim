// @ts-nocheck
/*
   There are a total of 90 entries in the algebraic variable array.
   There are a total of 33 entries in each of the rate and state variable arrays.
   There are a total of 104 entries in the constant variable array.
 */
/*
 * VOI is time in component environment (second).
 * CONSTANTS[0] is R in component Membrane (joule_per_kilomole_kelvin).
 * CONSTANTS[1] is T in component Membrane (kelvin).
 * CONSTANTS[2] is F in component Membrane (coulomb_per_mole).
 * CONSTANTS[3] is C in component Membrane (microF).
 * CONSTANTS[85] is RTONF in component Membrane (millivolt).
 * ALGEBRAIC[51] is i_f in component i_f (nanoA).
 * ALGEBRAIC[52] is i_NaK in component i_NaK (nanoA).
 * ALGEBRAIC[66] is i_NaCa in component i_NaCa (nanoA).
 * ALGEBRAIC[68] is i_Na in component i_Na (nanoA).
 * ALGEBRAIC[76] is i_Kr in component i_Kr (nanoA).
 * ALGEBRAIC[80] is i_Ks in component i_Ks (nanoA).
 * ALGEBRAIC[74] is i_to in component i_to (nanoA).
 * ALGEBRAIC[72] is i_CaL in component i_CaL (nanoA).
 * ALGEBRAIC[73] is i_CaT in component i_CaT (nanoA).
 * ALGEBRAIC[83] is i_KACh in component i_KACh (nanoA).
 * ALGEBRAIC[7] is V in component Membrane (millivolt).
 * CONSTANTS[4] is clamp_mode in component Membrane (dimensionless).
 * ALGEBRAIC[3] is V_clamp in component Voltage_clamp (millivolt).
 * STATES[0] is V_ode in component Membrane (millivolt).
 * ALGEBRAIC[86] is i_tot in component Membrane (nanoA).
 * CONSTANTS[5] is t_holding in component Voltage_clamp (second).
 * CONSTANTS[6] is t_test in component Voltage_clamp (second).
 * CONSTANTS[7] is V_test in component Voltage_clamp (millivolt).
 * CONSTANTS[8] is V_holding in component Voltage_clamp (millivolt).
 * CONSTANTS[9] is Iva_3_uM in component Rate_modulation_experiments (dimensionless).
 * CONSTANTS[10] is Cs_5_mM in component Rate_modulation_experiments (dimensionless).
 * CONSTANTS[11] is ACh in component Rate_modulation_experiments (millimolar).
 * CONSTANTS[12] is Iso_1_uM in component Rate_modulation_experiments (dimensionless).
 * CONSTANTS[13] is BAPTA_10_mM in component Rate_modulation_experiments (dimensionless).
 * ALGEBRAIC[16] is Nai in component Nai_concentration (millimolar).
 * CONSTANTS[14] is Nao in component Ionic_values (millimolar).
 * CONSTANTS[15] is Ki in component Ionic_values (millimolar).
 * CONSTANTS[16] is Ko in component Ionic_values (millimolar).
 * STATES[1] is Ca_sub in component Ca_dynamics (millimolar).
 * CONSTANTS[17] is Cao in component Ionic_values (millimolar).
 * ALGEBRAIC[32] is E_Na in component Ionic_values (millivolt).
 * CONSTANTS[87] is E_K in component Ionic_values (millivolt).
 * ALGEBRAIC[0] is E_Ca in component Ionic_values (millivolt).
 * CONSTANTS[98] is V_sub in component Cell_parameters (millimetre3).
 * CONSTANTS[100] is V_i in component Cell_parameters (millimetre3).
 * ALGEBRAIC[46] is i_fNa in component i_f (nanoA).
 * ALGEBRAIC[71] is i_siNa in component i_CaL (nanoA).
 * STATES[2] is Nai_ in component Nai_concentration (millimolar).
 * ALGEBRAIC[42] is ICs_on_Icontrol in component i_f (dimensionless).
 * ALGEBRAIC[49] is i_fK in component i_f (nanoA).
 * CONSTANTS[82] is g_f_Na in component i_f (microS).
 * CONSTANTS[83] is g_f_K in component i_f (microS).
 * CONSTANTS[18] is Km_f in component i_f (millimolar).
 * STATES[3] is y in component i_f_y_gate (dimensionless).
 * ALGEBRAIC[8] is tau_y in component i_f_y_gate (second).
 * ALGEBRAIC[25] is y_infinity in component i_f_y_gate (dimensionless).
 * CONSTANTS[89] is ACh_shift in component i_f_y_gate (millivolt).
 * CONSTANTS[90] is Iso_shift in component i_f_y_gate (millivolt).
 * CONSTANTS[19] is Km_Kp in component i_NaK (millimolar).
 * CONSTANTS[20] is Km_Nap in component i_NaK (millimolar).
 * CONSTANTS[21] is i_NaK_max in component i_NaK (nanoA).
 * CONSTANTS[91] is Iso_increase in component i_NaK (dimensionless).
 * CONSTANTS[22] is K_NaCa in component i_NaCa (nanoA).
 * ALGEBRAIC[63] is x1 in component i_NaCa (dimensionless).
 * ALGEBRAIC[59] is x2 in component i_NaCa (dimensionless).
 * ALGEBRAIC[64] is x3 in component i_NaCa (dimensionless).
 * ALGEBRAIC[65] is x4 in component i_NaCa (dimensionless).
 * ALGEBRAIC[54] is k41 in component i_NaCa (dimensionless).
 * CONSTANTS[92] is k34 in component i_NaCa (dimensionless).
 * ALGEBRAIC[62] is k23 in component i_NaCa (dimensionless).
 * ALGEBRAIC[61] is k21 in component i_NaCa (dimensionless).
 * ALGEBRAIC[58] is k32 in component i_NaCa (dimensionless).
 * ALGEBRAIC[53] is k43 in component i_NaCa (dimensionless).
 * ALGEBRAIC[56] is k12 in component i_NaCa (dimensionless).
 * ALGEBRAIC[57] is k14 in component i_NaCa (dimensionless).
 * CONSTANTS[23] is Qci in component i_NaCa (dimensionless).
 * CONSTANTS[24] is Qn in component i_NaCa (dimensionless).
 * CONSTANTS[25] is Qco in component i_NaCa (dimensionless).
 * CONSTANTS[26] is K3ni in component i_NaCa (millimolar).
 * CONSTANTS[27] is Kci in component i_NaCa (millimolar).
 * CONSTANTS[28] is K1ni in component i_NaCa (millimolar).
 * CONSTANTS[29] is K2ni in component i_NaCa (millimolar).
 * CONSTANTS[30] is Kcni in component i_NaCa (millimolar).
 * CONSTANTS[31] is K3no in component i_NaCa (millimolar).
 * CONSTANTS[32] is K1no in component i_NaCa (millimolar).
 * CONSTANTS[33] is K2no in component i_NaCa (millimolar).
 * CONSTANTS[34] is Kco in component i_NaCa (millimolar).
 * ALGEBRAIC[60] is do in component i_NaCa (dimensionless).
 * ALGEBRAIC[55] is di in component i_NaCa (dimensionless).
 * CONSTANTS[35] is g_Na in component i_Na (microS).
 * ALGEBRAIC[67] is E_mh in component i_Na (millivolt).
 * STATES[4] is m in component i_Na_m_gate (dimensionless).
 * STATES[5] is h in component i_Na_h_gate (dimensionless).
 * ALGEBRAIC[26] is alpha_m in component i_Na_m_gate (per_second).
 * ALGEBRAIC[40] is beta_m in component i_Na_m_gate (per_second).
 * CONSTANTS[36] is delta_m in component i_Na_m_gate (millivolt).
 * ALGEBRAIC[9] is E0_m in component i_Na_m_gate (millivolt).
 * ALGEBRAIC[10] is alpha_h in component i_Na_h_gate (per_second).
 * ALGEBRAIC[27] is beta_h in component i_Na_h_gate (per_second).
 * ALGEBRAIC[69] is i_siCa in component i_CaL (nanoA).
 * ALGEBRAIC[70] is i_siK in component i_CaL (nanoA).
 * CONSTANTS[94] is ACh_block in component i_CaL (dimensionless).
 * CONSTANTS[37] is P_CaL in component i_CaL (nanoA_per_millimolar).
 * STATES[6] is dL in component i_CaL_dL_gate (dimensionless).
 * STATES[7] is fL in component i_CaL_fL_gate (dimensionless).
 * STATES[8] is fCa in component i_CaL_fCa_gate (dimensionless).
 * CONSTANTS[93] is Iso_increase in component i_CaL (dimensionless).
 * ALGEBRAIC[11] is dL_infinity in component i_CaL_dL_gate (dimensionless).
 * ALGEBRAIC[50] is tau_dL in component i_CaL_dL_gate (second).
 * ALGEBRAIC[41] is alpha_dL in component i_CaL_dL_gate (per_second).
 * ALGEBRAIC[48] is beta_dL in component i_CaL_dL_gate (per_second).
 * ALGEBRAIC[28] is adVm in component i_CaL_dL_gate (millivolt).
 * ALGEBRAIC[45] is bdVm in component i_CaL_dL_gate (millivolt).
 * CONSTANTS[95] is Iso_shift in component i_CaL_dL_gate (millivolt).
 * CONSTANTS[96] is Iso_slope in component i_CaL_dL_gate (dimensionless).
 * ALGEBRAIC[12] is fL_infinity in component i_CaL_fL_gate (dimensionless).
 * ALGEBRAIC[29] is tau_fL in component i_CaL_fL_gate (second).
 * CONSTANTS[38] is alpha_fCa in component i_CaL_fCa_gate (per_second).
 * ALGEBRAIC[1] is fCa_infinity in component i_CaL_fCa_gate (dimensionless).
 * ALGEBRAIC[5] is tau_fCa in component i_CaL_fCa_gate (second).
 * CONSTANTS[39] is Km_fCa in component i_CaL_fCa_gate (millimolar).
 * CONSTANTS[40] is P_CaT in component i_CaT (nanoA_per_millimolar).
 * STATES[9] is dT in component i_CaT_dT_gate (dimensionless).
 * STATES[10] is fT in component i_CaT_fT_gate (dimensionless).
 * ALGEBRAIC[13] is dT_infinity in component i_CaT_dT_gate (dimensionless).
 * ALGEBRAIC[30] is tau_dT in component i_CaT_dT_gate (second).
 * ALGEBRAIC[14] is fT_infinity in component i_CaT_fT_gate (dimensionless).
 * ALGEBRAIC[31] is tau_fT in component i_CaT_fT_gate (second).
 * ALGEBRAIC[75] is j_SRCarel in component Ca_SR_release (millimolar_per_second).
 * STATES[11] is R in component Ca_SR_release (dimensionless).
 * STATES[12] is O in component Ca_SR_release (dimensionless).
 * STATES[13] is I in component Ca_SR_release (dimensionless).
 * STATES[14] is RI in component Ca_SR_release (dimensionless).
 * CONSTANTS[41] is ks in component Ca_SR_release (per_second).
 * CONSTANTS[42] is MaxSR in component Ca_SR_release (dimensionless).
 * CONSTANTS[43] is MinSR in component Ca_SR_release (dimensionless).
 * CONSTANTS[44] is EC50_SR in component Ca_SR_release (millimolar).
 * CONSTANTS[45] is HSR in component Ca_SR_release (dimensionless).
 * ALGEBRAIC[6] is koSRCa in component Ca_SR_release (per_millimolar2_second).
 * ALGEBRAIC[15] is kiSRCa in component Ca_SR_release (per_millimolar_second).
 * CONSTANTS[46] is koCa in component Ca_SR_release (per_millimolar2_second).
 * CONSTANTS[47] is kiCa in component Ca_SR_release (per_millimolar_second).
 * ALGEBRAIC[2] is kCaSR in component Ca_SR_release (dimensionless).
 * CONSTANTS[48] is kim in component Ca_SR_release (per_second).
 * CONSTANTS[49] is kom in component Ca_SR_release (per_second).
 * STATES[15] is Ca_jsr in component Ca_dynamics (millimolar).
 * ALGEBRAIC[77] is j_Ca_dif in component Ca_intracellular_fluxes (millimolar_per_second).
 * ALGEBRAIC[79] is j_up in component Ca_intracellular_fluxes (millimolar_per_second).
 * ALGEBRAIC[82] is j_tr in component Ca_intracellular_fluxes (millimolar_per_second).
 * CONSTANTS[50] is tau_dif_Ca in component Ca_intracellular_fluxes (second).
 * CONSTANTS[51] is tau_tr in component Ca_intracellular_fluxes (second).
 * CONSTANTS[88] is P_up in component Ca_intracellular_fluxes (millimolar_per_second).
 * CONSTANTS[52] is P_up_basal in component Ca_intracellular_fluxes (millimolar_per_second).
 * CONSTANTS[84] is b_up in component Ca_intracellular_fluxes (dimensionless).
 * CONSTANTS[53] is K_up in component Ca_intracellular_fluxes (millimolar).
 * STATES[16] is Ca_nsr in component Ca_dynamics (millimolar).
 * STATES[17] is Cai in component Ca_dynamics (millimolar).
 * CONSTANTS[54] is TC_tot in component Ca_buffering (millimolar).
 * CONSTANTS[55] is TMC_tot in component Ca_buffering (millimolar).
 * CONSTANTS[56] is CM_tot in component Ca_buffering (millimolar).
 * CONSTANTS[57] is CQ_tot in component Ca_buffering (millimolar).
 * ALGEBRAIC[81] is delta_fTC in component Ca_buffering (per_second).
 * ALGEBRAIC[84] is delta_fTMC in component Ca_buffering (per_second).
 * ALGEBRAIC[88] is delta_fCMs in component Ca_buffering (per_second).
 * ALGEBRAIC[87] is delta_fCMi in component Ca_buffering (per_second).
 * ALGEBRAIC[85] is delta_fCQ in component Ca_buffering (per_second).
 * ALGEBRAIC[4] is delta_fTMM in component Ca_buffering (per_second).
 * STATES[18] is fTMM in component Ca_buffering (dimensionless).
 * STATES[19] is fCMi in component Ca_buffering (dimensionless).
 * STATES[20] is fCMs in component Ca_buffering (dimensionless).
 * STATES[21] is fTC in component Ca_buffering (dimensionless).
 * STATES[22] is fTMC in component Ca_buffering (dimensionless).
 * STATES[23] is fCQ in component Ca_buffering (dimensionless).
 * CONSTANTS[58] is kf_TC in component Ca_buffering (per_millimolar_second).
 * CONSTANTS[59] is kf_TMM in component Ca_buffering (per_millimolar_second).
 * CONSTANTS[60] is kf_TMC in component Ca_buffering (per_millimolar_second).
 * CONSTANTS[61] is kf_CM in component Ca_buffering (per_millimolar_second).
 * CONSTANTS[62] is kf_CQ in component Ca_buffering (per_millimolar_second).
 * CONSTANTS[63] is kb_TC in component Ca_buffering (per_second).
 * CONSTANTS[64] is kb_TMC in component Ca_buffering (per_second).
 * CONSTANTS[65] is kb_TMM in component Ca_buffering (per_second).
 * CONSTANTS[66] is kb_CM in component Ca_buffering (per_second).
 * CONSTANTS[67] is kb_CQ in component Ca_buffering (per_second).
 * CONSTANTS[68] is Mgi in component Ca_buffering (millimolar).
 * CONSTANTS[99] is V_jsr in component Cell_parameters (millimetre3).
 * CONSTANTS[101] is V_nsr in component Cell_parameters (millimetre3).
 * ALGEBRAIC[89] is BAPTA in component Ca_dynamics (millimolar).
 * CONSTANTS[69] is kfBAPTA in component Ca_dynamics (per_millimolar_second).
 * CONSTANTS[70] is kbBAPTA in component Ca_dynamics (per_second).
 * STATES[24] is fBAPTA in component Ca_dynamics (millimolar).
 * STATES[25] is fBAPTA_sub in component Ca_dynamics (millimolar).
 * CONSTANTS[71] is T in component Ca_dynamics (second).
 * CONSTANTS[97] is V_cell in component Cell_parameters (millimetre3).
 * CONSTANTS[72] is V_jsr_part in component Cell_parameters (dimensionless).
 * CONSTANTS[73] is V_i_part in component Cell_parameters (dimensionless).
 * CONSTANTS[74] is V_nsr_part in component Cell_parameters (dimensionless).
 * CONSTANTS[75] is R_cell in component Cell_parameters (micrometre).
 * CONSTANTS[76] is L_cell in component Cell_parameters (micrometre).
 * CONSTANTS[77] is L_sub in component Cell_parameters (micrometre).
 * CONSTANTS[78] is g_to in component i_to (microS).
 * STATES[26] is q in component i_to_q_gate (dimensionless).
 * STATES[27] is r in component i_to_r_gate (dimensionless).
 * ALGEBRAIC[17] is q_infinity in component i_to_q_gate (dimensionless).
 * ALGEBRAIC[33] is tau_q in component i_to_q_gate (second).
 * ALGEBRAIC[18] is r_infinity in component i_to_r_gate (dimensionless).
 * ALGEBRAIC[34] is tau_r in component i_to_r_gate (second).
 * CONSTANTS[79] is g_Kr in component i_Kr (microS).
 * STATES[28] is paS in component i_Kr_pa_gate (dimensionless).
 * STATES[29] is paF in component i_Kr_pa_gate (dimensionless).
 * STATES[30] is piy in component i_Kr_pi_gate (dimensionless).
 * ALGEBRAIC[19] is pa_infinity in component i_Kr_pa_gate (dimensionless).
 * ALGEBRAIC[20] is alfapaF in component i_Kr_pa_gate (per_second).
 * ALGEBRAIC[21] is betapaF in component i_Kr_pa_gate (per_second).
 * ALGEBRAIC[35] is tau_paS in component i_Kr_pa_gate (second).
 * ALGEBRAIC[36] is tau_paF in component i_Kr_pa_gate (second).
 * ALGEBRAIC[37] is pi_infinity in component i_Kr_pi_gate (dimensionless).
 * ALGEBRAIC[22] is tau_pi in component i_Kr_pi_gate (second).
 * CONSTANTS[86] is g_Ks in component i_Ks (microS).
 * ALGEBRAIC[78] is E_Ks in component i_Ks (millivolt).
 * STATES[31] is n in component i_Ks_n_gate (dimensionless).
 * ALGEBRAIC[23] is n_infinity in component i_Ks_n_gate (dimensionless).
 * ALGEBRAIC[47] is tau_n in component i_Ks_n_gate (second).
 * CONSTANTS[102] is Iso_shift in component i_Ks_n_gate (millivolt).
 * CONSTANTS[80] is shift in component i_Ks_n_gate (millivolt).
 * ALGEBRAIC[38] is alpha_n in component i_Ks_n_gate (per_second).
 * ALGEBRAIC[43] is beta_n in component i_Ks_n_gate (per_second).
 * CONSTANTS[81] is g_KACh in component i_KACh (microS).
 * STATES[32] is a in component i_KACh_a_gate (dimensionless).
 * CONSTANTS[103] is alpha_a in component i_KACh_a_gate (per_second).
 * ALGEBRAIC[24] is beta_a in component i_KACh_a_gate (per_second).
 * ALGEBRAIC[39] is a_infinity in component i_KACh_a_gate (dimensionless).
 * ALGEBRAIC[44] is tau_a in component i_KACh_a_gate (second).
 * RATES[0] is d/dt V_ode in component Membrane (millivolt).
 * RATES[2] is d/dt Nai_ in component Nai_concentration (millimolar).
 * RATES[3] is d/dt y in component i_f_y_gate (dimensionless).
 * RATES[4] is d/dt m in component i_Na_m_gate (dimensionless).
 * RATES[5] is d/dt h in component i_Na_h_gate (dimensionless).
 * RATES[6] is d/dt dL in component i_CaL_dL_gate (dimensionless).
 * RATES[7] is d/dt fL in component i_CaL_fL_gate (dimensionless).
 * RATES[8] is d/dt fCa in component i_CaL_fCa_gate (dimensionless).
 * RATES[9] is d/dt dT in component i_CaT_dT_gate (dimensionless).
 * RATES[10] is d/dt fT in component i_CaT_fT_gate (dimensionless).
 * RATES[11] is d/dt R in component Ca_SR_release (dimensionless).
 * RATES[12] is d/dt O in component Ca_SR_release (dimensionless).
 * RATES[13] is d/dt I in component Ca_SR_release (dimensionless).
 * RATES[14] is d/dt RI in component Ca_SR_release (dimensionless).
 * RATES[21] is d/dt fTC in component Ca_buffering (dimensionless).
 * RATES[22] is d/dt fTMC in component Ca_buffering (dimensionless).
 * RATES[18] is d/dt fTMM in component Ca_buffering (dimensionless).
 * RATES[19] is d/dt fCMi in component Ca_buffering (dimensionless).
 * RATES[20] is d/dt fCMs in component Ca_buffering (dimensionless).
 * RATES[23] is d/dt fCQ in component Ca_buffering (dimensionless).
 * RATES[17] is d/dt Cai in component Ca_dynamics (millimolar).
 * RATES[24] is d/dt fBAPTA in component Ca_dynamics (millimolar).
 * RATES[1] is d/dt Ca_sub in component Ca_dynamics (millimolar).
 * RATES[25] is d/dt fBAPTA_sub in component Ca_dynamics (millimolar).
 * RATES[16] is d/dt Ca_nsr in component Ca_dynamics (millimolar).
 * RATES[15] is d/dt Ca_jsr in component Ca_dynamics (millimolar).
 * RATES[26] is d/dt q in component i_to_q_gate (dimensionless).
 * RATES[27] is d/dt r in component i_to_r_gate (dimensionless).
 * RATES[28] is d/dt paS in component i_Kr_pa_gate (dimensionless).
 * RATES[29] is d/dt paF in component i_Kr_pa_gate (dimensionless).
 * RATES[30] is d/dt piy in component i_Kr_pi_gate (dimensionless).
 * RATES[31] is d/dt n in component i_Ks_n_gate (dimensionless).
 * RATES[32] is d/dt a in component i_KACh_a_gate (dimensionless).
 */
export function initConsts(CONSTANTS: Float64Array, RATES: Float64Array, STATES: Float64Array)
{
CONSTANTS[0] = 8314.472;
CONSTANTS[1] = 310;
CONSTANTS[2] = 96485.3415;
CONSTANTS[3] = 3.2e-5;
CONSTANTS[4] = 0;
STATES[0] = -52;
CONSTANTS[5] = 0.5;
CONSTANTS[6] = 0.5;
CONSTANTS[7] = -35;
CONSTANTS[8] = -45;
CONSTANTS[9] = 0;
CONSTANTS[10] = 0;
CONSTANTS[11] = 0;
CONSTANTS[12] = 0;
CONSTANTS[13] = 0;
CONSTANTS[14] = 140;
CONSTANTS[15] = 140;
CONSTANTS[16] = 5.4;
STATES[1] = 1e-5;
CONSTANTS[17] = 1.8;
STATES[2] = 7.5;
CONSTANTS[18] = 45;
STATES[3] = 0.181334538702451;
CONSTANTS[19] = 1.4;
CONSTANTS[20] = 14;
CONSTANTS[21] = 0.063;
CONSTANTS[22] = 4;
CONSTANTS[23] = 0.1369;
CONSTANTS[24] = 0.4315;
CONSTANTS[25] = 0;
CONSTANTS[26] = 26.44;
CONSTANTS[27] = 0.0207;
CONSTANTS[28] = 395.3;
CONSTANTS[29] = 2.289;
CONSTANTS[30] = 26.44;
CONSTANTS[31] = 4.663;
CONSTANTS[32] = 1628;
CONSTANTS[33] = 561.4;
CONSTANTS[34] = 3.663;
CONSTANTS[35] = 0.0125;
STATES[4] = 0.440131579215766;
STATES[5] = 1.3676940140066e-5;
CONSTANTS[36] = 1e-5;
CONSTANTS[37] = 0.2;
STATES[6] = 0;
STATES[7] = 0.497133507285601;
STATES[8] = 0.697998543259722;
CONSTANTS[38] = 0.01;
CONSTANTS[39] = 0.00035;
CONSTANTS[40] = 0.02;
STATES[9] = 0;
STATES[10] = 0;
STATES[11] = 0.912317231017262;
STATES[12] = 1.7340201253e-7;
STATES[13] = 7.86181717518e-8;
STATES[14] = 0.211148145512825;
CONSTANTS[41] = 250000000;
CONSTANTS[42] = 15;
CONSTANTS[43] = 1;
CONSTANTS[44] = 0.45;
CONSTANTS[45] = 2.5;
CONSTANTS[46] = 10000;
CONSTANTS[47] = 500;
CONSTANTS[48] = 5;
CONSTANTS[49] = 60;
STATES[15] = 0.316762674605;
CONSTANTS[50] = 4e-5;
CONSTANTS[51] = 0.04;
CONSTANTS[52] = 12;
CONSTANTS[53] = 0.0006;
STATES[16] = 1.05386465080816;
STATES[17] = 1e-5;
CONSTANTS[54] = 0.031;
CONSTANTS[55] = 0.062;
CONSTANTS[56] = 0.045;
CONSTANTS[57] = 10;
STATES[18] = 0.501049376634;
STATES[19] = 0.0373817991524254;
STATES[20] = 0.054381370046;
STATES[21] = 0.0180519400676086;
STATES[22] = 0.281244308217086;
STATES[23] = 0.299624275428735;
CONSTANTS[58] = 88800;
CONSTANTS[59] = 2277;
CONSTANTS[60] = 227700;
CONSTANTS[61] = 227700;
CONSTANTS[62] = 534;
CONSTANTS[63] = 446;
CONSTANTS[64] = 7.51;
CONSTANTS[65] = 751;
CONSTANTS[66] = 542;
CONSTANTS[67] = 445;
CONSTANTS[68] = 2.5;
CONSTANTS[69] = 940000;
CONSTANTS[70] = 119.38;
STATES[24] = 0;
STATES[25] = 0;
CONSTANTS[71] = 6.928;
CONSTANTS[72] = 0.0012;
CONSTANTS[73] = 0.46;
CONSTANTS[74] = 0.0116;
CONSTANTS[75] = 4;
CONSTANTS[76] = 70;
CONSTANTS[77] = 0.02;
CONSTANTS[78] = 0.002;
STATES[26] = 0.506139850982478;
STATES[27] = 0.0144605370597924;
CONSTANTS[79] = 0.0021637;
STATES[28] = 0.322999177802891;
STATES[29] = 0.0990510403258968;
STATES[30] = 0.705410877258545;
STATES[31] = 0;
CONSTANTS[80] = 0;
CONSTANTS[81] = 0.00864;
STATES[32] = 0;
CONSTANTS[82] = (CONSTANTS[9]>=1.00000 ?  0.0300000*(1.00000 - 0.660000) : 0.0300000);
CONSTANTS[83] = (CONSTANTS[9]>=1.00000 ?  0.0300000*(1.00000 - 0.660000) : 0.0300000);
CONSTANTS[84] = (CONSTANTS[12]>0.00000 ? - 0.250000 : CONSTANTS[11]>0.00000 ? ( 0.700000*CONSTANTS[11])/(9.00000e-05+CONSTANTS[11]) : 0.00000);
CONSTANTS[85] = ( CONSTANTS[0]*CONSTANTS[1])/CONSTANTS[2];
CONSTANTS[86] = (CONSTANTS[12]>0.00000 ?  1.20000*0.00165760 : 0.00165760);
CONSTANTS[87] =  CONSTANTS[85]*Math.log(CONSTANTS[16]/CONSTANTS[15]);
CONSTANTS[88] =  CONSTANTS[52]*(1.00000 - CONSTANTS[84]);
CONSTANTS[89] = (CONSTANTS[11]>0.00000 ? - 1.00000 - ( 9.89800*Math.pow( 1.00000*CONSTANTS[11], 0.618000))/(Math.pow( 1.00000*CONSTANTS[11], 0.618000)+0.00122423) : 0.00000);
CONSTANTS[90] = (CONSTANTS[12]>0.00000 ? 7.50000 : 0.00000);
CONSTANTS[91] = (CONSTANTS[12]>0.00000 ? 1.20000 : 1.00000);
CONSTANTS[92] = CONSTANTS[14]/(CONSTANTS[31]+CONSTANTS[14]);
CONSTANTS[93] = (CONSTANTS[12]>0.00000 ? 1.23000 : 1.00000);
CONSTANTS[94] = ( 0.310000*CONSTANTS[11])/(CONSTANTS[11]+9.00000e-05);
CONSTANTS[95] = (CONSTANTS[12]>0.00000 ? - 8.00000 : 0.00000);
CONSTANTS[96] = (CONSTANTS[12]>0.00000 ? 0.690000 : 1.00000);
CONSTANTS[97] =  1.00000e-09* 3.14159265358979*Math.pow(CONSTANTS[75], 2.00000)*CONSTANTS[76];
CONSTANTS[98] =  1.00000e-09*2.00000* 3.14159265358979*CONSTANTS[77]*(CONSTANTS[75] - CONSTANTS[77]/2.00000)*CONSTANTS[76];
CONSTANTS[99] =  CONSTANTS[72]*CONSTANTS[97];
CONSTANTS[100] =  CONSTANTS[73]*CONSTANTS[97] - CONSTANTS[98];
CONSTANTS[101] =  CONSTANTS[74]*CONSTANTS[97];
CONSTANTS[102] = (CONSTANTS[12]>0.00000 ? - 14.0000 : 0.00000);
CONSTANTS[103] = (3.59880 - 0.0256410)/(1.00000+1.21550e-06/Math.pow( 1.00000*CONSTANTS[11], 1.69510))+0.0256410;
}
export function computeRates(VOI: number, CONSTANTS: Float64Array, RATES: Float64Array, STATES: Float64Array, ALGEBRAIC: Float64Array)
{
ALGEBRAIC[4] =  CONSTANTS[59]*CONSTANTS[68]*(1.00000 - (STATES[22]+STATES[18])) -  CONSTANTS[65]*STATES[18];
RATES[18] = ALGEBRAIC[4];
ALGEBRAIC[1] = CONSTANTS[39]/(CONSTANTS[39]+STATES[1]);
ALGEBRAIC[5] = ( 0.00100000*ALGEBRAIC[1])/CONSTANTS[38];
RATES[8] = (ALGEBRAIC[1] - STATES[8])/ALGEBRAIC[5];
ALGEBRAIC[2] = CONSTANTS[42] - (CONSTANTS[42] - CONSTANTS[43])/(1.00000+Math.pow(CONSTANTS[44]/STATES[15], CONSTANTS[45]));
ALGEBRAIC[6] = CONSTANTS[46]/ALGEBRAIC[2];
ALGEBRAIC[15] =  CONSTANTS[47]*ALGEBRAIC[2];
RATES[11] = ( CONSTANTS[48]*STATES[14] -  ALGEBRAIC[15]*STATES[1]*STATES[11]) - ( ALGEBRAIC[6]*Math.pow(STATES[1], 2.00000)*STATES[11] -  CONSTANTS[49]*STATES[12]);
RATES[12] = ( ALGEBRAIC[6]*Math.pow(STATES[1], 2.00000)*STATES[11] -  CONSTANTS[49]*STATES[12]) - ( ALGEBRAIC[15]*STATES[1]*STATES[12] -  CONSTANTS[48]*STATES[13]);
RATES[13] = ( ALGEBRAIC[15]*STATES[1]*STATES[12] -  CONSTANTS[48]*STATES[13]) - ( CONSTANTS[49]*STATES[13] -  ALGEBRAIC[6]*Math.pow(STATES[1], 2.00000)*STATES[14]);
RATES[14] = ( CONSTANTS[49]*STATES[13] -  ALGEBRAIC[6]*Math.pow(STATES[1], 2.00000)*STATES[14]) - ( CONSTANTS[48]*STATES[14] -  ALGEBRAIC[15]*STATES[1]*STATES[11]);
ALGEBRAIC[3] = (VOI>CONSTANTS[5]&&VOI<CONSTANTS[5]+CONSTANTS[6] ? CONSTANTS[7] : CONSTANTS[8]);
ALGEBRAIC[7] = (CONSTANTS[4]>=1.00000 ? ALGEBRAIC[3] : STATES[0]);
ALGEBRAIC[8] = 0.716653/( 0.0708000*Math.exp(- (((ALGEBRAIC[7]+5.00000) - CONSTANTS[89]) - CONSTANTS[90])/20.2791)+ 10.6000*Math.exp(((ALGEBRAIC[7] - CONSTANTS[89]) - CONSTANTS[90])/18.0000));
ALGEBRAIC[25] = 1.00000/(1.00000+Math.exp((((ALGEBRAIC[7]+52.5000) - CONSTANTS[89]) - CONSTANTS[90])/9.00000));
RATES[3] = (ALGEBRAIC[25] - STATES[3])/ALGEBRAIC[8];
ALGEBRAIC[10] =  20.0000*Math.exp( - 0.125000*(ALGEBRAIC[7]+75.0000));
ALGEBRAIC[27] = 2000.00/( 320.000*Math.exp( - 0.100000*(ALGEBRAIC[7]+75.0000))+1.00000);
RATES[5] =  ALGEBRAIC[10]*(1.00000 - STATES[5]) -  ALGEBRAIC[27]*STATES[5];
ALGEBRAIC[12] = 1.00000/(1.00000+Math.exp((ALGEBRAIC[7]+37.4000)/5.30000));
ALGEBRAIC[29] =  0.00100000*(44.3000+ 230.000*Math.exp(- Math.pow((ALGEBRAIC[7]+36.0000)/10.0000, 2.00000)));
RATES[7] = (ALGEBRAIC[12] - STATES[7])/ALGEBRAIC[29];
ALGEBRAIC[13] = 1.00000/(1.00000+Math.exp(- (ALGEBRAIC[7]+38.3000)/5.50000));
ALGEBRAIC[30] = 0.00100000/( 1.06800*Math.exp((ALGEBRAIC[7]+38.3000)/30.0000)+ 1.06800*Math.exp(- (ALGEBRAIC[7]+38.3000)/30.0000));
RATES[9] = (ALGEBRAIC[13] - STATES[9])/ALGEBRAIC[30];
ALGEBRAIC[14] = 1.00000/(1.00000+Math.exp((ALGEBRAIC[7]+58.7000)/3.80000));
ALGEBRAIC[31] = 1.00000/( 16.6700*Math.exp(- (ALGEBRAIC[7]+75.0000)/83.3000)+ 16.6700*Math.exp((ALGEBRAIC[7]+75.0000)/15.3800));
RATES[10] = (ALGEBRAIC[14] - STATES[10])/ALGEBRAIC[31];
ALGEBRAIC[17] = 1.00000/(1.00000+Math.exp((ALGEBRAIC[7]+49.0000)/13.0000));
ALGEBRAIC[33] =  0.00100000*0.600000*(65.1700/( 0.570000*Math.exp( - 0.0800000*(ALGEBRAIC[7]+44.0000))+ 0.0650000*Math.exp( 0.100000*(ALGEBRAIC[7]+45.9300)))+10.1000);
RATES[26] = (ALGEBRAIC[17] - STATES[26])/ALGEBRAIC[33];
ALGEBRAIC[18] = 1.00000/(1.00000+Math.exp(- (ALGEBRAIC[7] - 19.3000)/15.0000));
ALGEBRAIC[34] =  0.00100000*0.660000*1.40000*(15.5900/( 1.03700*Math.exp( 0.0900000*(ALGEBRAIC[7]+30.6100))+ 0.369000*Math.exp( - 0.120000*(ALGEBRAIC[7]+23.8400)))+2.98000);
RATES[27] = (ALGEBRAIC[18] - STATES[27])/ALGEBRAIC[34];
ALGEBRAIC[19] = 1.00000/(1.00000+Math.exp(- (ALGEBRAIC[7]+14.8000)/8.50000));
ALGEBRAIC[35] = 0.846554/( 4.20000*Math.exp(ALGEBRAIC[7]/17.0000)+ 0.150000*Math.exp(- ALGEBRAIC[7]/21.6000));
RATES[28] = (ALGEBRAIC[19] - STATES[28])/ALGEBRAIC[35];
ALGEBRAIC[36] = 1.00000/( 30.0000*Math.exp(ALGEBRAIC[7]/10.0000)+Math.exp(- ALGEBRAIC[7]/12.0000));
RATES[29] = (ALGEBRAIC[19] - STATES[29])/ALGEBRAIC[36];
ALGEBRAIC[37] = 1.00000/(1.00000+Math.exp((ALGEBRAIC[7]+28.6000)/17.1000));
ALGEBRAIC[22] = 1.00000/( 100.000*Math.exp(- ALGEBRAIC[7]/54.6450)+ 656.000*Math.exp(ALGEBRAIC[7]/106.157));
RATES[30] = (ALGEBRAIC[37] - STATES[30])/ALGEBRAIC[22];
ALGEBRAIC[9] = ALGEBRAIC[7]+41.0000;
ALGEBRAIC[26] = (Math.abs(ALGEBRAIC[9])<CONSTANTS[36] ? 2000.00 : ( 200.000*ALGEBRAIC[9])/(1.00000 - Math.exp( - 0.100000*ALGEBRAIC[9])));
ALGEBRAIC[40] =  8000.00*Math.exp( - 0.0560000*(ALGEBRAIC[7]+66.0000));
RATES[4] =  ALGEBRAIC[26]*(1.00000 - STATES[4]) -  ALGEBRAIC[40]*STATES[4];
ALGEBRAIC[24] =  10.0000*Math.exp( 0.0133000*(ALGEBRAIC[7]+40.0000));
ALGEBRAIC[39] = CONSTANTS[103]/(CONSTANTS[103]+ALGEBRAIC[24]);
ALGEBRAIC[44] = 1.00000/(CONSTANTS[103]+ALGEBRAIC[24]);
RATES[32] = (ALGEBRAIC[39] - STATES[32])/ALGEBRAIC[44];
ALGEBRAIC[23] = (14.0000/(1.00000+Math.exp(- ((ALGEBRAIC[7] - 40.0000) - CONSTANTS[102])/12.0000)))/(14.0000/(1.00000+Math.exp(- ((ALGEBRAIC[7] - 40.0000) - CONSTANTS[102])/12.0000))+ 1.00000*Math.exp(- (ALGEBRAIC[7] - CONSTANTS[102])/45.0000));
ALGEBRAIC[38] = 28.0000/(1.00000+Math.exp(- ((ALGEBRAIC[7] - 40.0000) - CONSTANTS[102])/3.00000));
ALGEBRAIC[43] =  1.00000*Math.exp(- (((ALGEBRAIC[7] - CONSTANTS[102]) - CONSTANTS[80]) - 5.00000)/25.0000);
ALGEBRAIC[47] = 1.00000/(ALGEBRAIC[38]+ALGEBRAIC[43]);
RATES[31] = (ALGEBRAIC[23] - STATES[31])/ALGEBRAIC[47];
ALGEBRAIC[11] = 1.00000/(1.00000+Math.exp(- ((ALGEBRAIC[7]+20.3000) - CONSTANTS[95])/( CONSTANTS[96]*4.20000)));
ALGEBRAIC[28] = (ALGEBRAIC[7]==- 41.8000 ? - 41.8000 : ALGEBRAIC[7]==0.00000 ? 0.00000 : ALGEBRAIC[7]==- 6.80000 ? - 6.80001 : ALGEBRAIC[7]);
ALGEBRAIC[41] = ( - 0.0283900*((ALGEBRAIC[28]+41.8000) - CONSTANTS[95]))/(Math.exp(- ((ALGEBRAIC[28]+41.8000) - CONSTANTS[95])/2.50000) - 1.00000) - ( 0.0849000*((ALGEBRAIC[28]+6.80000) - CONSTANTS[95]))/(Math.exp(- ((ALGEBRAIC[28]+6.80000) - CONSTANTS[95])/4.80000) - 1.00000);
ALGEBRAIC[45] = (ALGEBRAIC[7]==- 1.80000 ? - 1.80001 : ALGEBRAIC[7]);
ALGEBRAIC[48] = ( 0.0114300*((ALGEBRAIC[45]+1.80000) - CONSTANTS[95]))/(Math.exp(((ALGEBRAIC[45]+1.80000) - CONSTANTS[95])/2.50000) - 1.00000);
ALGEBRAIC[50] = 0.00100000/(ALGEBRAIC[41]+ALGEBRAIC[48]);
RATES[6] = (ALGEBRAIC[11] - STATES[6])/ALGEBRAIC[50];
ALGEBRAIC[16] = (CONSTANTS[13]>0.00000 ? 7.50000 : STATES[2]);
ALGEBRAIC[32] =  CONSTANTS[85]*Math.log(CONSTANTS[14]/ALGEBRAIC[16]);
ALGEBRAIC[52] =  CONSTANTS[91]*CONSTANTS[21]*Math.pow(1.00000+Math.pow(CONSTANTS[19]/CONSTANTS[16], 1.20000), - 1.00000)*Math.pow(1.00000+Math.pow(CONSTANTS[20]/ALGEBRAIC[16], 1.30000), - 1.00000)*Math.pow(1.00000+Math.exp(- ((ALGEBRAIC[7] - ALGEBRAIC[32])+110.000)/20.0000), - 1.00000);
ALGEBRAIC[54] = Math.exp(( - CONSTANTS[24]*ALGEBRAIC[7])/( 2.00000*CONSTANTS[85]));
ALGEBRAIC[60] = 1.00000+ (CONSTANTS[17]/CONSTANTS[34])*(1.00000+Math.exp(( CONSTANTS[25]*ALGEBRAIC[7])/CONSTANTS[85]))+ (CONSTANTS[14]/CONSTANTS[32])*(1.00000+ (CONSTANTS[14]/CONSTANTS[33])*(1.00000+CONSTANTS[14]/CONSTANTS[31]));
ALGEBRAIC[62] = ( (( (CONSTANTS[14]/CONSTANTS[32])*CONSTANTS[14])/CONSTANTS[33])*(1.00000+CONSTANTS[14]/CONSTANTS[31])*Math.exp(( - CONSTANTS[24]*ALGEBRAIC[7])/( 2.00000*CONSTANTS[85])))/ALGEBRAIC[60];
ALGEBRAIC[61] = ( (CONSTANTS[17]/CONSTANTS[34])*Math.exp(( CONSTANTS[25]*ALGEBRAIC[7])/CONSTANTS[85]))/ALGEBRAIC[60];
ALGEBRAIC[58] = Math.exp(( CONSTANTS[24]*ALGEBRAIC[7])/( 2.00000*CONSTANTS[85]));
ALGEBRAIC[53] = ALGEBRAIC[16]/(CONSTANTS[26]+ALGEBRAIC[16]);
ALGEBRAIC[63] =  ALGEBRAIC[54]*CONSTANTS[92]*(ALGEBRAIC[62]+ALGEBRAIC[61])+ ALGEBRAIC[61]*ALGEBRAIC[58]*(ALGEBRAIC[53]+ALGEBRAIC[54]);
ALGEBRAIC[55] = 1.00000+ (STATES[1]/CONSTANTS[27])*(1.00000+Math.exp(( - CONSTANTS[23]*ALGEBRAIC[7])/CONSTANTS[85])+ALGEBRAIC[16]/CONSTANTS[30])+ (ALGEBRAIC[16]/CONSTANTS[28])*(1.00000+ (ALGEBRAIC[16]/CONSTANTS[29])*(1.00000+ALGEBRAIC[16]/CONSTANTS[26]));
ALGEBRAIC[56] = ( (STATES[1]/CONSTANTS[27])*Math.exp(( - CONSTANTS[23]*ALGEBRAIC[7])/CONSTANTS[85]))/ALGEBRAIC[55];
ALGEBRAIC[57] = ( (( (ALGEBRAIC[16]/CONSTANTS[28])*ALGEBRAIC[16])/CONSTANTS[29])*(1.00000+ALGEBRAIC[16]/CONSTANTS[26])*Math.exp(( CONSTANTS[24]*ALGEBRAIC[7])/( 2.00000*CONSTANTS[85])))/ALGEBRAIC[55];
ALGEBRAIC[59] =  ALGEBRAIC[58]*ALGEBRAIC[53]*(ALGEBRAIC[57]+ALGEBRAIC[56])+ ALGEBRAIC[54]*ALGEBRAIC[56]*(CONSTANTS[92]+ALGEBRAIC[58]);
ALGEBRAIC[64] =  ALGEBRAIC[57]*ALGEBRAIC[53]*(ALGEBRAIC[62]+ALGEBRAIC[61])+ ALGEBRAIC[56]*ALGEBRAIC[62]*(ALGEBRAIC[53]+ALGEBRAIC[54]);
ALGEBRAIC[65] =  ALGEBRAIC[62]*CONSTANTS[92]*(ALGEBRAIC[57]+ALGEBRAIC[56])+ ALGEBRAIC[57]*ALGEBRAIC[61]*(CONSTANTS[92]+ALGEBRAIC[58]);
ALGEBRAIC[66] = ( CONSTANTS[22]*( ALGEBRAIC[59]*ALGEBRAIC[61] -  ALGEBRAIC[63]*ALGEBRAIC[56]))/(ALGEBRAIC[63]+ALGEBRAIC[59]+ALGEBRAIC[64]+ALGEBRAIC[65]);
ALGEBRAIC[67] =  CONSTANTS[85]*Math.log((CONSTANTS[14]+ 0.120000*CONSTANTS[16])/(ALGEBRAIC[16]+ 0.120000*CONSTANTS[15]));
ALGEBRAIC[68] =  CONSTANTS[35]*Math.pow(STATES[4], 3.00000)*STATES[5]*(ALGEBRAIC[7] - ALGEBRAIC[67]);
ALGEBRAIC[42] = (CONSTANTS[10]>=1.00000 ? (10.6015/5.00000)/(10.6015/5.00000+Math.exp(( - 0.710000*ALGEBRAIC[7])/25.0000)) : 1.00000);
ALGEBRAIC[46] =  (( Math.pow(STATES[3], 2.00000)*CONSTANTS[16])/(CONSTANTS[16]+CONSTANTS[18]))*CONSTANTS[82]*(ALGEBRAIC[7] - ALGEBRAIC[32])*ALGEBRAIC[42];
ALGEBRAIC[71] =  (( 1.85000e-05*CONSTANTS[37]*(ALGEBRAIC[7] - 0.00000))/( CONSTANTS[85]*(1.00000 - Math.exp(( - 1.00000*(ALGEBRAIC[7] - 0.00000))/CONSTANTS[85]))))*(ALGEBRAIC[16] -  CONSTANTS[14]*Math.exp(( - 1.00000*(ALGEBRAIC[7] - 0.00000))/CONSTANTS[85]))*STATES[6]*STATES[7]*STATES[8];
RATES[2] = ( - 1.00000*(ALGEBRAIC[68]+ALGEBRAIC[46]+ALGEBRAIC[71]+ 3.00000*ALGEBRAIC[52]+ 3.00000*ALGEBRAIC[66]))/( 1.00000*(CONSTANTS[100]+CONSTANTS[98])*CONSTANTS[2]);
ALGEBRAIC[81] =  CONSTANTS[58]*STATES[17]*(1.00000 - STATES[21]) -  CONSTANTS[63]*STATES[21];
RATES[21] = ALGEBRAIC[81];
ALGEBRAIC[79] = CONSTANTS[88]/(1.00000+CONSTANTS[53]/STATES[17]);
ALGEBRAIC[82] = (STATES[16] - STATES[15])/CONSTANTS[51];
RATES[16] = ALGEBRAIC[79] - ( ALGEBRAIC[82]*CONSTANTS[99])/CONSTANTS[101];
ALGEBRAIC[84] =  CONSTANTS[60]*STATES[17]*(1.00000 - (STATES[22]+STATES[18])) -  CONSTANTS[64]*STATES[22];
RATES[22] = ALGEBRAIC[84];
ALGEBRAIC[85] =  CONSTANTS[62]*STATES[15]*(1.00000 - STATES[23]) -  CONSTANTS[67]*STATES[23];
RATES[23] = ALGEBRAIC[85];
ALGEBRAIC[75] =  CONSTANTS[41]*STATES[12]*(STATES[15] - STATES[1]);
RATES[15] = ALGEBRAIC[82] - (ALGEBRAIC[75]+ CONSTANTS[57]*ALGEBRAIC[85]);
ALGEBRAIC[49] =  (( Math.pow(STATES[3], 2.00000)*CONSTANTS[16])/(CONSTANTS[16]+CONSTANTS[18]))*CONSTANTS[83]*(ALGEBRAIC[7] - CONSTANTS[87])*ALGEBRAIC[42];
ALGEBRAIC[51] = ALGEBRAIC[46]+ALGEBRAIC[49];
ALGEBRAIC[76] =  CONSTANTS[79]*(ALGEBRAIC[7] - CONSTANTS[87])*( 0.900000*STATES[29]+ 0.100000*STATES[28])*STATES[30];
ALGEBRAIC[78] =  CONSTANTS[85]*Math.log((CONSTANTS[16]+ 0.00000*CONSTANTS[14])/(CONSTANTS[15]+ 0.00000*ALGEBRAIC[16]));
ALGEBRAIC[80] =  CONSTANTS[86]*(ALGEBRAIC[7] - ALGEBRAIC[78])*Math.pow(STATES[31], 2.00000);
ALGEBRAIC[74] =  CONSTANTS[78]*(ALGEBRAIC[7] - CONSTANTS[87])*STATES[26]*STATES[27];
ALGEBRAIC[69] =  (( 2.00000*CONSTANTS[37]*(ALGEBRAIC[7] - 0.00000))/( CONSTANTS[85]*(1.00000 - Math.exp(( - 1.00000*(ALGEBRAIC[7] - 0.00000)*2.00000)/CONSTANTS[85]))))*(STATES[1] -  CONSTANTS[17]*Math.exp(( - 2.00000*(ALGEBRAIC[7] - 0.00000))/CONSTANTS[85]))*STATES[6]*STATES[7]*STATES[8];
ALGEBRAIC[70] =  (( 0.000365000*CONSTANTS[37]*(ALGEBRAIC[7] - 0.00000))/( CONSTANTS[85]*(1.00000 - Math.exp(( - 1.00000*(ALGEBRAIC[7] - 0.00000))/CONSTANTS[85]))))*(CONSTANTS[15] -  CONSTANTS[16]*Math.exp(( - 1.00000*(ALGEBRAIC[7] - 0.00000))/CONSTANTS[85]))*STATES[6]*STATES[7]*STATES[8];
ALGEBRAIC[72] =  (ALGEBRAIC[69]+ALGEBRAIC[70]+ALGEBRAIC[71])*(1.00000 - CONSTANTS[94])*1.00000*CONSTANTS[93];
ALGEBRAIC[73] =  (( 2.00000*CONSTANTS[40]*ALGEBRAIC[7])/( CONSTANTS[85]*(1.00000 - Math.exp(( - 1.00000*ALGEBRAIC[7]*2.00000)/CONSTANTS[85]))))*(STATES[1] -  CONSTANTS[17]*Math.exp(( - 2.00000*ALGEBRAIC[7])/CONSTANTS[85]))*STATES[9]*STATES[10];
ALGEBRAIC[83] = (CONSTANTS[11]>0.00000 ?  CONSTANTS[81]*(ALGEBRAIC[7] - CONSTANTS[87])*(1.00000+Math.exp((ALGEBRAIC[7]+20.0000)/20.0000))*STATES[32] : 0.00000);
ALGEBRAIC[86] = ALGEBRAIC[51]+ALGEBRAIC[76]+ALGEBRAIC[80]+ALGEBRAIC[74]+ALGEBRAIC[52]+ALGEBRAIC[66]+ALGEBRAIC[68]+ALGEBRAIC[72]+ALGEBRAIC[73]+ALGEBRAIC[83];
RATES[0] = - ALGEBRAIC[86]/CONSTANTS[3];
ALGEBRAIC[87] =  CONSTANTS[61]*STATES[17]*(1.00000 - STATES[19]) -  CONSTANTS[66]*STATES[19];
RATES[19] = ALGEBRAIC[87];
ALGEBRAIC[88] =  CONSTANTS[61]*STATES[1]*(1.00000 - STATES[20]) -  CONSTANTS[66]*STATES[20];
RATES[20] = ALGEBRAIC[88];
ALGEBRAIC[77] = (STATES[1] - STATES[17])/CONSTANTS[50];
ALGEBRAIC[89] = (CONSTANTS[13]>0.00000&&VOI>CONSTANTS[71] ? 10.0000 : 0.00000);
RATES[17] = (( 1.00000*( ALGEBRAIC[77]*CONSTANTS[98] -  ALGEBRAIC[79]*CONSTANTS[101]))/CONSTANTS[100] - ( CONSTANTS[56]*ALGEBRAIC[87]+ CONSTANTS[54]*ALGEBRAIC[81]+ CONSTANTS[55]*ALGEBRAIC[84])) - ( CONSTANTS[69]*STATES[17]*(ALGEBRAIC[89] - STATES[24]) -  CONSTANTS[70]*STATES[24]);
RATES[24] =  CONSTANTS[69]*STATES[17]*(ALGEBRAIC[89] - STATES[24]) -  CONSTANTS[70]*STATES[24];
RATES[1] = (( ALGEBRAIC[75]*CONSTANTS[99])/CONSTANTS[98] - (((ALGEBRAIC[69]+ALGEBRAIC[73]) -  2.00000*ALGEBRAIC[66])/( 2.00000*CONSTANTS[2]*CONSTANTS[98])+ALGEBRAIC[77]+ CONSTANTS[56]*ALGEBRAIC[88])) - ( CONSTANTS[69]*STATES[1]*(ALGEBRAIC[89] - STATES[25]) -  CONSTANTS[70]*STATES[25]);
RATES[25] =  CONSTANTS[69]*STATES[1]*(ALGEBRAIC[89] - STATES[25]) -  CONSTANTS[70]*STATES[25];
}
export function computeVariables(VOI: number, CONSTANTS: Float64Array, RATES: Float64Array, STATES: Float64Array, ALGEBRAIC: Float64Array)
{
ALGEBRAIC[4] =  CONSTANTS[59]*CONSTANTS[68]*(1.00000 - (STATES[22]+STATES[18])) -  CONSTANTS[65]*STATES[18];
ALGEBRAIC[1] = CONSTANTS[39]/(CONSTANTS[39]+STATES[1]);
ALGEBRAIC[5] = ( 0.00100000*ALGEBRAIC[1])/CONSTANTS[38];
ALGEBRAIC[2] = CONSTANTS[42] - (CONSTANTS[42] - CONSTANTS[43])/(1.00000+Math.pow(CONSTANTS[44]/STATES[15], CONSTANTS[45]));
ALGEBRAIC[6] = CONSTANTS[46]/ALGEBRAIC[2];
ALGEBRAIC[15] =  CONSTANTS[47]*ALGEBRAIC[2];
ALGEBRAIC[3] = (VOI>CONSTANTS[5]&&VOI<CONSTANTS[5]+CONSTANTS[6] ? CONSTANTS[7] : CONSTANTS[8]);
ALGEBRAIC[7] = (CONSTANTS[4]>=1.00000 ? ALGEBRAIC[3] : STATES[0]);
ALGEBRAIC[8] = 0.716653/( 0.0708000*Math.exp(- (((ALGEBRAIC[7]+5.00000) - CONSTANTS[89]) - CONSTANTS[90])/20.2791)+ 10.6000*Math.exp(((ALGEBRAIC[7] - CONSTANTS[89]) - CONSTANTS[90])/18.0000));
ALGEBRAIC[25] = 1.00000/(1.00000+Math.exp((((ALGEBRAIC[7]+52.5000) - CONSTANTS[89]) - CONSTANTS[90])/9.00000));
ALGEBRAIC[10] =  20.0000*Math.exp( - 0.125000*(ALGEBRAIC[7]+75.0000));
ALGEBRAIC[27] = 2000.00/( 320.000*Math.exp( - 0.100000*(ALGEBRAIC[7]+75.0000))+1.00000);
ALGEBRAIC[12] = 1.00000/(1.00000+Math.exp((ALGEBRAIC[7]+37.4000)/5.30000));
ALGEBRAIC[29] =  0.00100000*(44.3000+ 230.000*Math.exp(- Math.pow((ALGEBRAIC[7]+36.0000)/10.0000, 2.00000)));
ALGEBRAIC[13] = 1.00000/(1.00000+Math.exp(- (ALGEBRAIC[7]+38.3000)/5.50000));
ALGEBRAIC[30] = 0.00100000/( 1.06800*Math.exp((ALGEBRAIC[7]+38.3000)/30.0000)+ 1.06800*Math.exp(- (ALGEBRAIC[7]+38.3000)/30.0000));
ALGEBRAIC[14] = 1.00000/(1.00000+Math.exp((ALGEBRAIC[7]+58.7000)/3.80000));
ALGEBRAIC[31] = 1.00000/( 16.6700*Math.exp(- (ALGEBRAIC[7]+75.0000)/83.3000)+ 16.6700*Math.exp((ALGEBRAIC[7]+75.0000)/15.3800));
ALGEBRAIC[17] = 1.00000/(1.00000+Math.exp((ALGEBRAIC[7]+49.0000)/13.0000));
ALGEBRAIC[33] =  0.00100000*0.600000*(65.1700/( 0.570000*Math.exp( - 0.0800000*(ALGEBRAIC[7]+44.0000))+ 0.0650000*Math.exp( 0.100000*(ALGEBRAIC[7]+45.9300)))+10.1000);
ALGEBRAIC[18] = 1.00000/(1.00000+Math.exp(- (ALGEBRAIC[7] - 19.3000)/15.0000));
ALGEBRAIC[34] =  0.00100000*0.660000*1.40000*(15.5900/( 1.03700*Math.exp( 0.0900000*(ALGEBRAIC[7]+30.6100))+ 0.369000*Math.exp( - 0.120000*(ALGEBRAIC[7]+23.8400)))+2.98000);
ALGEBRAIC[19] = 1.00000/(1.00000+Math.exp(- (ALGEBRAIC[7]+14.8000)/8.50000));
ALGEBRAIC[35] = 0.846554/( 4.20000*Math.exp(ALGEBRAIC[7]/17.0000)+ 0.150000*Math.exp(- ALGEBRAIC[7]/21.6000));
ALGEBRAIC[36] = 1.00000/( 30.0000*Math.exp(ALGEBRAIC[7]/10.0000)+Math.exp(- ALGEBRAIC[7]/12.0000));
ALGEBRAIC[37] = 1.00000/(1.00000+Math.exp((ALGEBRAIC[7]+28.6000)/17.1000));
ALGEBRAIC[22] = 1.00000/( 100.000*Math.exp(- ALGEBRAIC[7]/54.6450)+ 656.000*Math.exp(ALGEBRAIC[7]/106.157));
ALGEBRAIC[9] = ALGEBRAIC[7]+41.0000;
ALGEBRAIC[26] = (Math.abs(ALGEBRAIC[9])<CONSTANTS[36] ? 2000.00 : ( 200.000*ALGEBRAIC[9])/(1.00000 - Math.exp( - 0.100000*ALGEBRAIC[9])));
ALGEBRAIC[40] =  8000.00*Math.exp( - 0.0560000*(ALGEBRAIC[7]+66.0000));
ALGEBRAIC[24] =  10.0000*Math.exp( 0.0133000*(ALGEBRAIC[7]+40.0000));
ALGEBRAIC[39] = CONSTANTS[103]/(CONSTANTS[103]+ALGEBRAIC[24]);
ALGEBRAIC[44] = 1.00000/(CONSTANTS[103]+ALGEBRAIC[24]);
ALGEBRAIC[23] = (14.0000/(1.00000+Math.exp(- ((ALGEBRAIC[7] - 40.0000) - CONSTANTS[102])/12.0000)))/(14.0000/(1.00000+Math.exp(- ((ALGEBRAIC[7] - 40.0000) - CONSTANTS[102])/12.0000))+ 1.00000*Math.exp(- (ALGEBRAIC[7] - CONSTANTS[102])/45.0000));
ALGEBRAIC[38] = 28.0000/(1.00000+Math.exp(- ((ALGEBRAIC[7] - 40.0000) - CONSTANTS[102])/3.00000));
ALGEBRAIC[43] =  1.00000*Math.exp(- (((ALGEBRAIC[7] - CONSTANTS[102]) - CONSTANTS[80]) - 5.00000)/25.0000);
ALGEBRAIC[47] = 1.00000/(ALGEBRAIC[38]+ALGEBRAIC[43]);
ALGEBRAIC[11] = 1.00000/(1.00000+Math.exp(- ((ALGEBRAIC[7]+20.3000) - CONSTANTS[95])/( CONSTANTS[96]*4.20000)));
ALGEBRAIC[28] = (ALGEBRAIC[7]==- 41.8000 ? - 41.8000 : ALGEBRAIC[7]==0.00000 ? 0.00000 : ALGEBRAIC[7]==- 6.80000 ? - 6.80001 : ALGEBRAIC[7]);
ALGEBRAIC[41] = ( - 0.0283900*((ALGEBRAIC[28]+41.8000) - CONSTANTS[95]))/(Math.exp(- ((ALGEBRAIC[28]+41.8000) - CONSTANTS[95])/2.50000) - 1.00000) - ( 0.0849000*((ALGEBRAIC[28]+6.80000) - CONSTANTS[95]))/(Math.exp(- ((ALGEBRAIC[28]+6.80000) - CONSTANTS[95])/4.80000) - 1.00000);
ALGEBRAIC[45] = (ALGEBRAIC[7]==- 1.80000 ? - 1.80001 : ALGEBRAIC[7]);
ALGEBRAIC[48] = ( 0.0114300*((ALGEBRAIC[45]+1.80000) - CONSTANTS[95]))/(Math.exp(((ALGEBRAIC[45]+1.80000) - CONSTANTS[95])/2.50000) - 1.00000);
ALGEBRAIC[50] = 0.00100000/(ALGEBRAIC[41]+ALGEBRAIC[48]);
ALGEBRAIC[16] = (CONSTANTS[13]>0.00000 ? 7.50000 : STATES[2]);
ALGEBRAIC[32] =  CONSTANTS[85]*Math.log(CONSTANTS[14]/ALGEBRAIC[16]);
ALGEBRAIC[52] =  CONSTANTS[91]*CONSTANTS[21]*Math.pow(1.00000+Math.pow(CONSTANTS[19]/CONSTANTS[16], 1.20000), - 1.00000)*Math.pow(1.00000+Math.pow(CONSTANTS[20]/ALGEBRAIC[16], 1.30000), - 1.00000)*Math.pow(1.00000+Math.exp(- ((ALGEBRAIC[7] - ALGEBRAIC[32])+110.000)/20.0000), - 1.00000);
ALGEBRAIC[54] = Math.exp(( - CONSTANTS[24]*ALGEBRAIC[7])/( 2.00000*CONSTANTS[85]));
ALGEBRAIC[60] = 1.00000+ (CONSTANTS[17]/CONSTANTS[34])*(1.00000+Math.exp(( CONSTANTS[25]*ALGEBRAIC[7])/CONSTANTS[85]))+ (CONSTANTS[14]/CONSTANTS[32])*(1.00000+ (CONSTANTS[14]/CONSTANTS[33])*(1.00000+CONSTANTS[14]/CONSTANTS[31]));
ALGEBRAIC[62] = ( (( (CONSTANTS[14]/CONSTANTS[32])*CONSTANTS[14])/CONSTANTS[33])*(1.00000+CONSTANTS[14]/CONSTANTS[31])*Math.exp(( - CONSTANTS[24]*ALGEBRAIC[7])/( 2.00000*CONSTANTS[85])))/ALGEBRAIC[60];
ALGEBRAIC[61] = ( (CONSTANTS[17]/CONSTANTS[34])*Math.exp(( CONSTANTS[25]*ALGEBRAIC[7])/CONSTANTS[85]))/ALGEBRAIC[60];
ALGEBRAIC[58] = Math.exp(( CONSTANTS[24]*ALGEBRAIC[7])/( 2.00000*CONSTANTS[85]));
ALGEBRAIC[53] = ALGEBRAIC[16]/(CONSTANTS[26]+ALGEBRAIC[16]);
ALGEBRAIC[63] =  ALGEBRAIC[54]*CONSTANTS[92]*(ALGEBRAIC[62]+ALGEBRAIC[61])+ ALGEBRAIC[61]*ALGEBRAIC[58]*(ALGEBRAIC[53]+ALGEBRAIC[54]);
ALGEBRAIC[55] = 1.00000+ (STATES[1]/CONSTANTS[27])*(1.00000+Math.exp(( - CONSTANTS[23]*ALGEBRAIC[7])/CONSTANTS[85])+ALGEBRAIC[16]/CONSTANTS[30])+ (ALGEBRAIC[16]/CONSTANTS[28])*(1.00000+ (ALGEBRAIC[16]/CONSTANTS[29])*(1.00000+ALGEBRAIC[16]/CONSTANTS[26]));
ALGEBRAIC[56] = ( (STATES[1]/CONSTANTS[27])*Math.exp(( - CONSTANTS[23]*ALGEBRAIC[7])/CONSTANTS[85]))/ALGEBRAIC[55];
ALGEBRAIC[57] = ( (( (ALGEBRAIC[16]/CONSTANTS[28])*ALGEBRAIC[16])/CONSTANTS[29])*(1.00000+ALGEBRAIC[16]/CONSTANTS[26])*Math.exp(( CONSTANTS[24]*ALGEBRAIC[7])/( 2.00000*CONSTANTS[85])))/ALGEBRAIC[55];
ALGEBRAIC[59] =  ALGEBRAIC[58]*ALGEBRAIC[53]*(ALGEBRAIC[57]+ALGEBRAIC[56])+ ALGEBRAIC[54]*ALGEBRAIC[56]*(CONSTANTS[92]+ALGEBRAIC[58]);
ALGEBRAIC[64] =  ALGEBRAIC[57]*ALGEBRAIC[53]*(ALGEBRAIC[62]+ALGEBRAIC[61])+ ALGEBRAIC[56]*ALGEBRAIC[62]*(ALGEBRAIC[53]+ALGEBRAIC[54]);
ALGEBRAIC[65] =  ALGEBRAIC[62]*CONSTANTS[92]*(ALGEBRAIC[57]+ALGEBRAIC[56])+ ALGEBRAIC[57]*ALGEBRAIC[61]*(CONSTANTS[92]+ALGEBRAIC[58]);
ALGEBRAIC[66] = ( CONSTANTS[22]*( ALGEBRAIC[59]*ALGEBRAIC[61] -  ALGEBRAIC[63]*ALGEBRAIC[56]))/(ALGEBRAIC[63]+ALGEBRAIC[59]+ALGEBRAIC[64]+ALGEBRAIC[65]);
ALGEBRAIC[67] =  CONSTANTS[85]*Math.log((CONSTANTS[14]+ 0.120000*CONSTANTS[16])/(ALGEBRAIC[16]+ 0.120000*CONSTANTS[15]));
ALGEBRAIC[68] =  CONSTANTS[35]*Math.pow(STATES[4], 3.00000)*STATES[5]*(ALGEBRAIC[7] - ALGEBRAIC[67]);
ALGEBRAIC[42] = (CONSTANTS[10]>=1.00000 ? (10.6015/5.00000)/(10.6015/5.00000+Math.exp(( - 0.710000*ALGEBRAIC[7])/25.0000)) : 1.00000);
ALGEBRAIC[46] =  (( Math.pow(STATES[3], 2.00000)*CONSTANTS[16])/(CONSTANTS[16]+CONSTANTS[18]))*CONSTANTS[82]*(ALGEBRAIC[7] - ALGEBRAIC[32])*ALGEBRAIC[42];
ALGEBRAIC[71] =  (( 1.85000e-05*CONSTANTS[37]*(ALGEBRAIC[7] - 0.00000))/( CONSTANTS[85]*(1.00000 - Math.exp(( - 1.00000*(ALGEBRAIC[7] - 0.00000))/CONSTANTS[85]))))*(ALGEBRAIC[16] -  CONSTANTS[14]*Math.exp(( - 1.00000*(ALGEBRAIC[7] - 0.00000))/CONSTANTS[85]))*STATES[6]*STATES[7]*STATES[8];
ALGEBRAIC[81] =  CONSTANTS[58]*STATES[17]*(1.00000 - STATES[21]) -  CONSTANTS[63]*STATES[21];
ALGEBRAIC[79] = CONSTANTS[88]/(1.00000+CONSTANTS[53]/STATES[17]);
ALGEBRAIC[82] = (STATES[16] - STATES[15])/CONSTANTS[51];
ALGEBRAIC[84] =  CONSTANTS[60]*STATES[17]*(1.00000 - (STATES[22]+STATES[18])) -  CONSTANTS[64]*STATES[22];
ALGEBRAIC[85] =  CONSTANTS[62]*STATES[15]*(1.00000 - STATES[23]) -  CONSTANTS[67]*STATES[23];
ALGEBRAIC[75] =  CONSTANTS[41]*STATES[12]*(STATES[15] - STATES[1]);
ALGEBRAIC[49] =  (( Math.pow(STATES[3], 2.00000)*CONSTANTS[16])/(CONSTANTS[16]+CONSTANTS[18]))*CONSTANTS[83]*(ALGEBRAIC[7] - CONSTANTS[87])*ALGEBRAIC[42];
ALGEBRAIC[51] = ALGEBRAIC[46]+ALGEBRAIC[49];
ALGEBRAIC[76] =  CONSTANTS[79]*(ALGEBRAIC[7] - CONSTANTS[87])*( 0.900000*STATES[29]+ 0.100000*STATES[28])*STATES[30];
ALGEBRAIC[78] =  CONSTANTS[85]*Math.log((CONSTANTS[16]+ 0.00000*CONSTANTS[14])/(CONSTANTS[15]+ 0.00000*ALGEBRAIC[16]));
ALGEBRAIC[80] =  CONSTANTS[86]*(ALGEBRAIC[7] - ALGEBRAIC[78])*Math.pow(STATES[31], 2.00000);
ALGEBRAIC[74] =  CONSTANTS[78]*(ALGEBRAIC[7] - CONSTANTS[87])*STATES[26]*STATES[27];
ALGEBRAIC[69] =  (( 2.00000*CONSTANTS[37]*(ALGEBRAIC[7] - 0.00000))/( CONSTANTS[85]*(1.00000 - Math.exp(( - 1.00000*(ALGEBRAIC[7] - 0.00000)*2.00000)/CONSTANTS[85]))))*(STATES[1] -  CONSTANTS[17]*Math.exp(( - 2.00000*(ALGEBRAIC[7] - 0.00000))/CONSTANTS[85]))*STATES[6]*STATES[7]*STATES[8];
ALGEBRAIC[70] =  (( 0.000365000*CONSTANTS[37]*(ALGEBRAIC[7] - 0.00000))/( CONSTANTS[85]*(1.00000 - Math.exp(( - 1.00000*(ALGEBRAIC[7] - 0.00000))/CONSTANTS[85]))))*(CONSTANTS[15] -  CONSTANTS[16]*Math.exp(( - 1.00000*(ALGEBRAIC[7] - 0.00000))/CONSTANTS[85]))*STATES[6]*STATES[7]*STATES[8];
ALGEBRAIC[72] =  (ALGEBRAIC[69]+ALGEBRAIC[70]+ALGEBRAIC[71])*(1.00000 - CONSTANTS[94])*1.00000*CONSTANTS[93];
ALGEBRAIC[73] =  (( 2.00000*CONSTANTS[40]*ALGEBRAIC[7])/( CONSTANTS[85]*(1.00000 - Math.exp(( - 1.00000*ALGEBRAIC[7]*2.00000)/CONSTANTS[85]))))*(STATES[1] -  CONSTANTS[17]*Math.exp(( - 2.00000*ALGEBRAIC[7])/CONSTANTS[85]))*STATES[9]*STATES[10];
ALGEBRAIC[83] = (CONSTANTS[11]>0.00000 ?  CONSTANTS[81]*(ALGEBRAIC[7] - CONSTANTS[87])*(1.00000+Math.exp((ALGEBRAIC[7]+20.0000)/20.0000))*STATES[32] : 0.00000);
ALGEBRAIC[86] = ALGEBRAIC[51]+ALGEBRAIC[76]+ALGEBRAIC[80]+ALGEBRAIC[74]+ALGEBRAIC[52]+ALGEBRAIC[66]+ALGEBRAIC[68]+ALGEBRAIC[72]+ALGEBRAIC[73]+ALGEBRAIC[83];
ALGEBRAIC[87] =  CONSTANTS[61]*STATES[17]*(1.00000 - STATES[19]) -  CONSTANTS[66]*STATES[19];
ALGEBRAIC[88] =  CONSTANTS[61]*STATES[1]*(1.00000 - STATES[20]) -  CONSTANTS[66]*STATES[20];
ALGEBRAIC[77] = (STATES[1] - STATES[17])/CONSTANTS[50];
ALGEBRAIC[89] = (CONSTANTS[13]>0.00000&&VOI>CONSTANTS[71] ? 10.0000 : 0.00000);
ALGEBRAIC[0] =  0.500000*CONSTANTS[85]*Math.log(CONSTANTS[17]/STATES[1]);
ALGEBRAIC[20] = (1.00000/(1.00000+Math.exp(- (ALGEBRAIC[7]+23.2000)/6.60000)))/(0.846554/( 37.2000*Math.exp(ALGEBRAIC[7]/11.9000)+ 0.960000*Math.exp(- ALGEBRAIC[7]/18.5000)));
ALGEBRAIC[21] =  4.00000*(( 37.2000*Math.exp(ALGEBRAIC[7]/15.9000)+ 0.960000*Math.exp(- ALGEBRAIC[7]/22.5000))/0.846554 - (1.00000/(1.00000+Math.exp(- (ALGEBRAIC[7]+23.2000)/10.6000)))/(0.846554/( 37.2000*Math.exp(ALGEBRAIC[7]/15.9000)+ 0.960000*Math.exp(- ALGEBRAIC[7]/22.5000))));
}
