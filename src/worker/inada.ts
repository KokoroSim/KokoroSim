/*
   There are a total of 81 entries in the algebraic variable array.
   There are a total of 29 entries in each of the rate and state variable arrays.
   There are a total of 58 entries in the constant variable array.
 */
/*
 * VOI is time in component environment (second).
 * STATES[0] is V in component membrane (millivolt).
 * CONSTANTS[0] is R in component membrane (joule_per_kilomole_kelvin).
 * CONSTANTS[1] is T in component membrane (kelvin).
 * CONSTANTS[2] is F in component membrane (coulomb_per_mole).
 * CONSTANTS[3] is C in component membrane (microF).
 * CONSTANTS[48] is RTONF in component membrane (millivolt).
 * ALGEBRAIC[65] is i_Na in component fast_sodium_current (nanoA).
 * ALGEBRAIC[70] is i_CaL in component L_type_calcium_current (nanoA).
 * ALGEBRAIC[66] is i_to in component transient_outward_potassium_current (nanoA).
 * ALGEBRAIC[34] is i_Kr in component rapid_delayed_rectifier_potassium_current (nanoA).
 * ALGEBRAIC[17] is i_f in component hyperpolarising_activated_current (nanoA).
 * ALGEBRAIC[67] is i_st in component sustained_outward_potassium_current (nanoA).
 * ALGEBRAIC[49] is i_K1 in component time_independent_potassium_current (nanoA).
 * ALGEBRAIC[64] is i_NaCa in component sodium_calcium_exchange_current (nanoA).
 * ALGEBRAIC[51] is i_p in component sodium_potassium_pump (nanoA).
 * ALGEBRAIC[50] is i_b in component background_current (nanoA).
 * ALGEBRAIC[69] is i_ACh in component acetylcholine_sensitive_current (nanoA).
 * CONSTANTS[4] is g_f in component hyperpolarising_activated_current (microS).
 * CONSTANTS[5] is ACh in component acetylcholine_sensitive_current (millimolar).
 * STATES[1] is y in component hyperpolarising_activated_current_y_gate (dimensionless).
 * ALGEBRAIC[0] is y_inf in component hyperpolarising_activated_current_y_gate (dimensionless).
 * ALGEBRAIC[19] is tau_y in component hyperpolarising_activated_current_y_gate (second).
 * CONSTANTS[6] is g_Kr in component rapid_delayed_rectifier_potassium_current (microS).
 * CONSTANTS[50] is E_K in component rapid_delayed_rectifier_potassium_current (millivolt).
 * CONSTANTS[7] is Ki in component intracellular_potassium_concentration (millimolar).
 * CONSTANTS[8] is Kc in component extracellular_potassium_concentration (millimolar).
 * STATES[2] is paf in component rapid_delayed_rectifier_potassium_current_paf_gate (dimensionless).
 * STATES[3] is pas in component rapid_delayed_rectifier_potassium_current_pas_gate (dimensionless).
 * STATES[4] is pik in component rapid_delayed_rectifier_potassium_current_pik_gate (dimensionless).
 * ALGEBRAIC[1] is paf_infinity in component rapid_delayed_rectifier_potassium_current_paf_gate (dimensionless).
 * ALGEBRAIC[20] is tau_paf in component rapid_delayed_rectifier_potassium_current_paf_gate (second).
 * ALGEBRAIC[2] is pas_infinity in component rapid_delayed_rectifier_potassium_current_pas_gate (dimensionless).
 * ALGEBRAIC[21] is tau_pas in component rapid_delayed_rectifier_potassium_current_pas_gate (second).
 * ALGEBRAIC[3] is pik_infinity in component rapid_delayed_rectifier_potassium_current_pik_gate (dimensionless).
 * ALGEBRAIC[22] is alpha_pik in component rapid_delayed_rectifier_potassium_current_pik_gate (per_second).
 * ALGEBRAIC[35] is beta_pik in component rapid_delayed_rectifier_potassium_current_pik_gate (per_second).
 * ALGEBRAIC[43] is tau_pik in component rapid_delayed_rectifier_potassium_current_pik_gate (second).
 * CONSTANTS[9] is g_K1 in component time_independent_potassium_current (microS).
 * ALGEBRAIC[42] is g_K1_prime in component time_independent_potassium_current (microS).
 * CONSTANTS[10] is g_b in component background_current (microS).
 * CONSTANTS[11] is E_b in component background_current (millivolt).
 * CONSTANTS[12] is I_p in component sodium_potassium_pump (nanoA).
 * CONSTANTS[13] is Nai in component intracellular_sodium_concentration (millimolar).
 * CONSTANTS[14] is kNaCa in component sodium_calcium_exchange_current (nanoA).
 * ALGEBRAIC[60] is x1 in component sodium_calcium_exchange_current (dimensionless).
 * ALGEBRAIC[61] is x2 in component sodium_calcium_exchange_current (dimensionless).
 * ALGEBRAIC[62] is x3 in component sodium_calcium_exchange_current (dimensionless).
 * ALGEBRAIC[63] is x4 in component sodium_calcium_exchange_current (dimensionless).
 * ALGEBRAIC[57] is k41 in component sodium_calcium_exchange_current (dimensionless).
 * CONSTANTS[49] is k34 in component sodium_calcium_exchange_current (dimensionless).
 * ALGEBRAIC[54] is k23 in component sodium_calcium_exchange_current (dimensionless).
 * ALGEBRAIC[55] is k21 in component sodium_calcium_exchange_current (dimensionless).
 * ALGEBRAIC[53] is k32 in component sodium_calcium_exchange_current (dimensionless).
 * CONSTANTS[53] is k43 in component sodium_calcium_exchange_current (dimensionless).
 * ALGEBRAIC[59] is k12 in component sodium_calcium_exchange_current (dimensionless).
 * ALGEBRAIC[58] is k14 in component sodium_calcium_exchange_current (dimensionless).
 * CONSTANTS[15] is Qci in component sodium_calcium_exchange_current (dimensionless).
 * CONSTANTS[16] is Qn in component sodium_calcium_exchange_current (dimensionless).
 * CONSTANTS[17] is Qco in component sodium_calcium_exchange_current (dimensionless).
 * CONSTANTS[18] is Kci in component sodium_calcium_exchange_current (millimolar).
 * CONSTANTS[19] is K1ni in component sodium_calcium_exchange_current (millimolar).
 * CONSTANTS[20] is K2ni in component sodium_calcium_exchange_current (millimolar).
 * CONSTANTS[21] is K3ni in component sodium_calcium_exchange_current (millimolar).
 * CONSTANTS[22] is Kcni in component sodium_calcium_exchange_current (millimolar).
 * CONSTANTS[23] is K3no in component sodium_calcium_exchange_current (millimolar).
 * CONSTANTS[24] is K1no in component sodium_calcium_exchange_current (millimolar).
 * CONSTANTS[25] is K2no in component sodium_calcium_exchange_current (millimolar).
 * CONSTANTS[26] is Kco in component sodium_calcium_exchange_current (millimolar).
 * ALGEBRAIC[52] is do in component sodium_calcium_exchange_current (dimensionless).
 * ALGEBRAIC[56] is di in component sodium_calcium_exchange_current (dimensionless).
 * CONSTANTS[27] is Cao in component extracellular_calcium_concentration (millimolar).
 * CONSTANTS[28] is Nao in component extracellular_sodium_concentration (millimolar).
 * STATES[5] is Casub in component intracellular_calcium_concentration (millimolar).
 * CONSTANTS[29] is g_Na in component fast_sodium_current (microlitre_per_second).
 * CONSTANTS[51] is E_Na in component fast_sodium_current (millivolt).
 * STATES[6] is m in component fast_sodium_current_m_gate (dimensionless).
 * STATES[7] is h1 in component fast_sodium_current_h1_gate (dimensionless).
 * STATES[8] is h2 in component fast_sodium_current_h2_gate (dimensionless).
 * ALGEBRAIC[23] is alpha_m in component fast_sodium_current_m_gate (per_second).
 * ALGEBRAIC[36] is beta_m in component fast_sodium_current_m_gate (per_second).
 * CONSTANTS[30] is delta_m in component fast_sodium_current_m_gate (millivolt).
 * ALGEBRAIC[4] is E0_m in component fast_sodium_current_m_gate (millivolt).
 * ALGEBRAIC[5] is alpha_h1 in component fast_sodium_current_h1_gate (per_second).
 * ALGEBRAIC[24] is beta_h1 in component fast_sodium_current_h1_gate (per_second).
 * ALGEBRAIC[37] is h1_inf in component fast_sodium_current_h1_gate (dimensionless).
 * ALGEBRAIC[44] is tau_h1 in component fast_sodium_current_h1_gate (second).
 * ALGEBRAIC[6] is alpha_h2 in component fast_sodium_current_h2_gate (per_second).
 * ALGEBRAIC[25] is beta_h2 in component fast_sodium_current_h2_gate (per_second).
 * ALGEBRAIC[38] is h2_inf in component fast_sodium_current_h2_gate (dimensionless).
 * ALGEBRAIC[45] is tau_h2 in component fast_sodium_current_h2_gate (second).
 * CONSTANTS[31] is g_CaL in component L_type_calcium_current (microS).
 * CONSTANTS[32] is E_CaL in component L_type_calcium_current (millivolt).
 * STATES[9] is d in component L_type_calcium_current_d_gate (dimensionless).
 * STATES[10] is f in component L_type_calcium_current_f_gate (dimensionless).
 * STATES[11] is f2 in component L_type_calcium_current_f2_gate (dimensionless).
 * ALGEBRAIC[7] is alpha_d in component L_type_calcium_current_d_gate (per_second).
 * ALGEBRAIC[26] is beta_d in component L_type_calcium_current_d_gate (per_second).
 * ALGEBRAIC[39] is d_inf in component L_type_calcium_current_d_gate (dimensionless).
 * ALGEBRAIC[46] is tau_d in component L_type_calcium_current_d_gate (second).
 * CONSTANTS[33] is act_shift in component L_type_calcium_current_d_gate (millivolt).
 * CONSTANTS[34] is slope_factor_act in component L_type_calcium_current_d_gate (millivolt).
 * ALGEBRAIC[8] is f_inf in component L_type_calcium_current_f_gate (dimensionless).
 * ALGEBRAIC[27] is tau_f in component L_type_calcium_current_f_gate (second).
 * CONSTANTS[35] is inact_shift in component L_type_calcium_current_f_gate (millivolt).
 * ALGEBRAIC[9] is f2_inf in component L_type_calcium_current_f2_gate (dimensionless).
 * ALGEBRAIC[28] is tau_f2 in component L_type_calcium_current_f2_gate (second).
 * CONSTANTS[36] is inact_shift in component L_type_calcium_current_f2_gate (millivolt).
 * CONSTANTS[52] is E_K in component transient_outward_potassium_current (millivolt).
 * CONSTANTS[37] is g_to in component transient_outward_potassium_current (microS).
 * STATES[12] is r in component transient_outward_potassium_current_r_gate (dimensionless).
 * STATES[13] is q_fast in component transient_outward_potassium_current_qfast_gate (dimensionless).
 * STATES[14] is q_slow in component transient_outward_potassium_current_qslow_gate (dimensionless).
 * ALGEBRAIC[29] is tau_r in component transient_outward_potassium_current_r_gate (second).
 * ALGEBRAIC[10] is r_infinity in component transient_outward_potassium_current_r_gate (dimensionless).
 * ALGEBRAIC[30] is tau_qfast in component transient_outward_potassium_current_qfast_gate (second).
 * ALGEBRAIC[11] is qfast_infinity in component transient_outward_potassium_current_qfast_gate (dimensionless).
 * ALGEBRAIC[31] is tau_qslow in component transient_outward_potassium_current_qslow_gate (second).
 * ALGEBRAIC[12] is qslow_infinity in component transient_outward_potassium_current_qslow_gate (dimensionless).
 * CONSTANTS[38] is E_st in component sustained_outward_potassium_current (millivolt).
 * CONSTANTS[39] is g_st in component sustained_outward_potassium_current (microS).
 * STATES[15] is qa in component sustained_outward_potassium_current_qa_gate (dimensionless).
 * STATES[16] is qi in component sustained_outward_potassium_current_qi_gate (dimensionless).
 * ALGEBRAIC[47] is tau_qa in component sustained_outward_potassium_current_qa_gate (second).
 * ALGEBRAIC[13] is qa_infinity in component sustained_outward_potassium_current_qa_gate (dimensionless).
 * ALGEBRAIC[32] is alpha_qa in component sustained_outward_potassium_current_qa_gate (per_second).
 * ALGEBRAIC[40] is beta_qa in component sustained_outward_potassium_current_qa_gate (per_second).
 * ALGEBRAIC[48] is tau_qi in component sustained_outward_potassium_current_qi_gate (second).
 * ALGEBRAIC[14] is alpha_qi in component sustained_outward_potassium_current_qi_gate (per_second).
 * ALGEBRAIC[33] is beta_qi in component sustained_outward_potassium_current_qi_gate (per_second).
 * ALGEBRAIC[41] is qi_infinity in component sustained_outward_potassium_current_qi_gate (dimensionless).
 * ALGEBRAIC[68] is g_ACh in component acetylcholine_sensitive_current (microS).
 * CONSTANTS[40] is g_ACh_max in component acetylcholine_sensitive_current (microS).
 * CONSTANTS[41] is K_ACh in component acetylcholine_sensitive_current (millimolar).
 * STATES[17] is achf in component acetylcholine_sensitive_current_achf_gate (dimensionless).
 * STATES[18] is achs in component acetylcholine_sensitive_current_achs_gate (dimensionless).
 * CONSTANTS[42] is alpha_achf in component acetylcholine_sensitive_current_achf_gate (per_second).
 * ALGEBRAIC[15] is beta_achf in component acetylcholine_sensitive_current_achf_gate (per_second).
 * CONSTANTS[43] is alpha_achs in component acetylcholine_sensitive_current_achs_gate (per_second).
 * ALGEBRAIC[16] is beta_achs in component acetylcholine_sensitive_current_achs_gate (per_second).
 * STATES[19] is Cai in component intracellular_calcium_concentration (millimolar).
 * CONSTANTS[54] is V_up in component intracellular_calcium_concentration (micrometre3).
 * CONSTANTS[55] is V_rel in component intracellular_calcium_concentration (micrometre3).
 * CONSTANTS[56] is V_sub in component intracellular_calcium_concentration (micrometre3).
 * CONSTANTS[57] is Vi in component intracellular_calcium_concentration (micrometre3).
 * CONSTANTS[44] is V_cell in component intracellular_calcium_concentration (micrometre3).
 * ALGEBRAIC[72] is i_up in component intracellular_calcium_concentration (millimolar_per_second).
 * ALGEBRAIC[73] is i_tr in component intracellular_calcium_concentration (millimolar_per_second).
 * ALGEBRAIC[75] is i_rel in component intracellular_calcium_concentration (millimolar_per_second).
 * ALGEBRAIC[71] is i_diff in component intracellular_calcium_concentration (millimolar_per_second).
 * STATES[20] is Ca_up in component intracellular_calcium_concentration (millimolar).
 * STATES[21] is Ca_rel in component intracellular_calcium_concentration (millimolar).
 * CONSTANTS[45] is P_rel in component intracellular_calcium_concentration (per_second).
 * CONSTANTS[46] is K_up in component intracellular_calcium_concentration (millimolar).
 * CONSTANTS[47] is tau_tr in component intracellular_calcium_concentration (second).
 * STATES[22] is f_TC in component intracellular_calcium_concentration (dimensionless).
 * STATES[23] is f_TMC in component intracellular_calcium_concentration (dimensionless).
 * STATES[24] is f_TMM in component intracellular_calcium_concentration (dimensionless).
 * STATES[25] is f_CMi in component intracellular_calcium_concentration (dimensionless).
 * STATES[26] is f_CMs in component intracellular_calcium_concentration (dimensionless).
 * STATES[27] is f_CQ in component intracellular_calcium_concentration (dimensionless).
 * STATES[28] is f_CSL in component intracellular_calcium_concentration (dimensionless).
 * ALGEBRAIC[74] is diff_f_TC in component intracellular_calcium_concentration (per_second).
 * ALGEBRAIC[76] is diff_f_TMC in component intracellular_calcium_concentration (per_second).
 * ALGEBRAIC[18] is diff_f_TMM in component intracellular_calcium_concentration (per_second).
 * ALGEBRAIC[77] is diff_f_CMi in component intracellular_calcium_concentration (per_second).
 * ALGEBRAIC[78] is diff_f_CMs in component intracellular_calcium_concentration (per_second).
 * ALGEBRAIC[79] is diff_f_CQ in component intracellular_calcium_concentration (per_second).
 * ALGEBRAIC[80] is diff_f_CSL in component intracellular_calcium_concentration (per_second).
 * RATES[0] is d/dt V in component membrane (millivolt).
 * RATES[1] is d/dt y in component hyperpolarising_activated_current_y_gate (dimensionless).
 * RATES[2] is d/dt paf in component rapid_delayed_rectifier_potassium_current_paf_gate (dimensionless).
 * RATES[3] is d/dt pas in component rapid_delayed_rectifier_potassium_current_pas_gate (dimensionless).
 * RATES[4] is d/dt pik in component rapid_delayed_rectifier_potassium_current_pik_gate (dimensionless).
 * RATES[6] is d/dt m in component fast_sodium_current_m_gate (dimensionless).
 * RATES[7] is d/dt h1 in component fast_sodium_current_h1_gate (dimensionless).
 * RATES[8] is d/dt h2 in component fast_sodium_current_h2_gate (dimensionless).
 * RATES[9] is d/dt d in component L_type_calcium_current_d_gate (dimensionless).
 * RATES[10] is d/dt f in component L_type_calcium_current_f_gate (dimensionless).
 * RATES[11] is d/dt f2 in component L_type_calcium_current_f2_gate (dimensionless).
 * RATES[12] is d/dt r in component transient_outward_potassium_current_r_gate (dimensionless).
 * RATES[13] is d/dt q_fast in component transient_outward_potassium_current_qfast_gate (dimensionless).
 * RATES[14] is d/dt q_slow in component transient_outward_potassium_current_qslow_gate (dimensionless).
 * RATES[15] is d/dt qa in component sustained_outward_potassium_current_qa_gate (dimensionless).
 * RATES[16] is d/dt qi in component sustained_outward_potassium_current_qi_gate (dimensionless).
 * RATES[17] is d/dt achf in component acetylcholine_sensitive_current_achf_gate (dimensionless).
 * RATES[18] is d/dt achs in component acetylcholine_sensitive_current_achs_gate (dimensionless).
 * RATES[20] is d/dt Ca_up in component intracellular_calcium_concentration (millimolar).
 * RATES[21] is d/dt Ca_rel in component intracellular_calcium_concentration (millimolar).
 * RATES[19] is d/dt Cai in component intracellular_calcium_concentration (millimolar).
 * RATES[5] is d/dt Casub in component intracellular_calcium_concentration (millimolar).
 * RATES[22] is d/dt f_TC in component intracellular_calcium_concentration (dimensionless).
 * RATES[23] is d/dt f_TMC in component intracellular_calcium_concentration (dimensionless).
 * RATES[24] is d/dt f_TMM in component intracellular_calcium_concentration (dimensionless).
 * RATES[25] is d/dt f_CMi in component intracellular_calcium_concentration (dimensionless).
 * RATES[26] is d/dt f_CMs in component intracellular_calcium_concentration (dimensionless).
 * RATES[27] is d/dt f_CQ in component intracellular_calcium_concentration (dimensionless).
 * RATES[28] is d/dt f_CSL in component intracellular_calcium_concentration (dimensionless).
 */
export function initConsts(CONSTANTS: Float64Array, RATES: Float64Array, STATES: Float64Array)
{
STATES[0] = -49.7094187908202;
CONSTANTS[0] = 8314.472;
CONSTANTS[1] = 310;
CONSTANTS[2] = 96485.3415;
CONSTANTS[3] = 2.9e-5;
CONSTANTS[4] = 0.001;
CONSTANTS[5] = 0;
STATES[1] = 0.0462303183096481;
CONSTANTS[6] = 0.0035;
CONSTANTS[7] = 140;
CONSTANTS[8] = 5.4;
STATES[2] = 0.192515363116553;
STATES[3] = 0.0797182955833868;
STATES[4] = 0.949023698965401;
CONSTANTS[9] = 0;
CONSTANTS[10] = 0.0012;
CONSTANTS[11] = -22.5;
CONSTANTS[12] = 0.14268;
CONSTANTS[13] = 8;
CONSTANTS[14] = 2.14455;
CONSTANTS[15] = 0.1369;
CONSTANTS[16] = 0.4315;
CONSTANTS[17] = 0;
CONSTANTS[18] = 0.0207;
CONSTANTS[19] = 395.3;
CONSTANTS[20] = 2.289;
CONSTANTS[21] = 26.44;
CONSTANTS[22] = 26.44;
CONSTANTS[23] = 4.663;
CONSTANTS[24] = 1628;
CONSTANTS[25] = 561.4;
CONSTANTS[26] = 3.663;
CONSTANTS[27] = 2;
CONSTANTS[28] = 140;
STATES[5] = 0.000160310601192365;
CONSTANTS[29] = 0;
STATES[6] = 0.143642247226618;
STATES[7] = 0.0243210273637729;
STATES[8] = 0.0157156121147801;
CONSTANTS[30] = 1e-5;
CONSTANTS[31] = 0.009;
CONSTANTS[32] = 62;
STATES[9] = 0.00179250298710316;
STATES[10] = 0.975550840189597;
STATES[11] = 0.774394220125623;
CONSTANTS[33] = -15;
CONSTANTS[34] = -5;
CONSTANTS[35] = -5;
CONSTANTS[36] = -5;
CONSTANTS[37] = 0;
STATES[12] = 0.0296516611999521;
STATES[13] = 0.899732315818241;
STATES[14] = 0.190111737767474;
CONSTANTS[38] = -37.4;
CONSTANTS[39] = 0.0001;
STATES[15] = 0.476404610622697;
STATES[16] = 0.542303657353244;
CONSTANTS[40] = 0.0198;
CONSTANTS[41] = 0.00035;
STATES[17] = 0.550559577208797;
STATES[18] = 0.567277036232041;
CONSTANTS[42] = 73.1;
CONSTANTS[43] = 3.7;
STATES[19] = 0.000184969821581882;
CONSTANTS[44] = 3.18872e-6;
STATES[20] = 1.11092514657408;
STATES[21] = 0.296249516481577;
CONSTANTS[45] = 1500;
CONSTANTS[46] = 0.0006;
CONSTANTS[47] = 0.06;
STATES[22] = 0.0356473236675985;
STATES[23] = 0.443317425115817;
STATES[24] = 0.491718960234865;
STATES[25] = 0.0723007987059414;
STATES[26] = 0.0630771339141488;
STATES[27] = 0.261430602900137;
STATES[28] = 4.1497704886823e-5;
CONSTANTS[48] = ( CONSTANTS[0]*CONSTANTS[1])/CONSTANTS[2];
CONSTANTS[49] = CONSTANTS[28]/(CONSTANTS[23]+CONSTANTS[28]);
CONSTANTS[50] =  CONSTANTS[48]*Math.log(CONSTANTS[8]/CONSTANTS[7]);
CONSTANTS[51] =  CONSTANTS[48]*Math.log(CONSTANTS[28]/CONSTANTS[13]);
CONSTANTS[52] =  CONSTANTS[48]*Math.log(CONSTANTS[8]/CONSTANTS[7]);
CONSTANTS[53] = CONSTANTS[13]/(CONSTANTS[21]+CONSTANTS[13]);
CONSTANTS[54] =  0.0116000*CONSTANTS[44];
CONSTANTS[55] =  0.00120000*CONSTANTS[44];
CONSTANTS[56] =  0.0100000*CONSTANTS[44];
CONSTANTS[57] =  0.460000*CONSTANTS[44] - CONSTANTS[56];
}
export function computeRates(VOI: number, CONSTANTS: Float64Array, RATES: Float64Array, STATES: Float64Array, ALGEBRAIC: Float64Array)
{
ALGEBRAIC[15] = 120.000/(1.00000+Math.exp(- (STATES[0]+50.0000)/15.0000));
RATES[17] =  CONSTANTS[42]*(1.00000 - STATES[17]) -  ALGEBRAIC[15]*STATES[17];
ALGEBRAIC[16] = 5.82000/(1.00000+Math.exp(- (STATES[0]+50.0000)/15.0000));
RATES[18] =  CONSTANTS[43]*(1.00000 - STATES[18]) -  ALGEBRAIC[16]*STATES[18];
ALGEBRAIC[18] =  2277.00*2.50000*((1.00000 - STATES[23]) - STATES[24]) -  751.000*STATES[24];
RATES[24] = ALGEBRAIC[18];
ALGEBRAIC[0] = 1.00000/(1.00000+Math.exp(((STATES[0]+83.1900) - ( - 7.20000*Math.pow(CONSTANTS[5], 0.690000))/(Math.pow(1.26000e-05, 0.690000)+Math.pow(CONSTANTS[5], 0.690000)))/13.5600));
ALGEBRAIC[19] = 0.250000+ 2.00000*Math.exp(- Math.pow(STATES[0]+70.0000, 2.00000)/500.000);
RATES[1] = (ALGEBRAIC[0] - STATES[1])/ALGEBRAIC[19];
ALGEBRAIC[1] = 1.00000/(1.00000+Math.exp((STATES[0]+10.2200)/- 8.50000));
ALGEBRAIC[20] = 1.00000/( 17.0000*Math.exp( 0.0398000*STATES[0])+ 0.211000*Math.exp( - 0.0510000*STATES[0]));
RATES[2] = (ALGEBRAIC[1] - STATES[2])/ALGEBRAIC[20];
ALGEBRAIC[2] = 1.00000/(1.00000+Math.exp((STATES[0]+10.2200)/- 8.50000));
ALGEBRAIC[21] = 0.335810+ 0.906730*Math.exp(- Math.pow(STATES[0]+10.0000, 2.00000)/988.050);
RATES[3] = (ALGEBRAIC[2] - STATES[3])/ALGEBRAIC[21];
ALGEBRAIC[8] = 1.00000/(1.00000+Math.exp((STATES[0] - (- 24.0000+CONSTANTS[35]))/6.31000));
ALGEBRAIC[27] = 0.0100000+ 0.153900*Math.exp(- Math.pow(STATES[0]+40.0000, 2.00000)/185.670);
RATES[10] = (ALGEBRAIC[8] - STATES[10])/ALGEBRAIC[27];
ALGEBRAIC[9] = 1.00000/(1.00000+Math.exp((STATES[0] - (- 24.0000+CONSTANTS[36]))/6.31000));
ALGEBRAIC[28] = 0.0600000+ 0.480760*2.25000*Math.exp(- Math.pow(STATES[0] - - 40.0000, 2.00000)/138.040);
RATES[11] = (ALGEBRAIC[9] - STATES[11])/ALGEBRAIC[28];
ALGEBRAIC[29] = 0.000596000+0.00311800/( 1.03700*Math.exp( 0.0900000*(STATES[0]+30.6100))+ 0.396000*Math.exp( - 0.120000*(STATES[0]+23.8400)));
ALGEBRAIC[10] = 1.00000/(1.00000+Math.exp((STATES[0] - 7.44000)/- 16.4000));
RATES[12] = (ALGEBRAIC[10] - STATES[12])/ALGEBRAIC[29];
ALGEBRAIC[30] = 0.0126600+4.72716/(1.00000+Math.exp((STATES[0]+154.500)/23.9600));
ALGEBRAIC[11] = 1.00000/(1.00000+Math.exp((STATES[0]+33.8000)/6.12000));
RATES[13] = (ALGEBRAIC[11] - STATES[13])/ALGEBRAIC[30];
ALGEBRAIC[31] = 0.100000+ 4.00000*Math.exp(- Math.pow(STATES[0]+65.0000, 2.00000)/500.000);
ALGEBRAIC[12] = 1.00000/(1.00000+Math.exp((STATES[0]+33.8000)/6.12000));
RATES[14] = (ALGEBRAIC[12] - STATES[14])/ALGEBRAIC[31];
ALGEBRAIC[4] = STATES[0]+44.4000;
ALGEBRAIC[23] = (Math.abs(ALGEBRAIC[4])<CONSTANTS[30] ? ( - 460.000*- 12.6730)/Math.exp(ALGEBRAIC[4]/- 12.6730) : ( - 460.000*ALGEBRAIC[4])/(Math.exp(ALGEBRAIC[4]/- 12.6730) - 1.00000));
ALGEBRAIC[36] =  18400.0*Math.exp(ALGEBRAIC[4]/- 12.6730);
RATES[6] =  ALGEBRAIC[23]*(1.00000 - STATES[6]) -  ALGEBRAIC[36]*STATES[6];
ALGEBRAIC[3] =  (1.00000/(1.00000+Math.exp((STATES[0]+4.90000)/15.1400)))*(1.00000 -  0.300000*Math.exp(- Math.pow(STATES[0], 2.00000)/500.000));
ALGEBRAIC[22] =  92.0100*Math.exp( - 0.0183000*STATES[0]);
ALGEBRAIC[35] =  603.600*Math.exp( 0.00942000*STATES[0]);
ALGEBRAIC[43] = 1.00000/(ALGEBRAIC[22]+ALGEBRAIC[35]);
RATES[4] = (ALGEBRAIC[3] - STATES[4])/ALGEBRAIC[43];
ALGEBRAIC[5] =  44.9000*Math.exp((STATES[0]+66.9000)/- 5.57000);
ALGEBRAIC[24] = 1491.00/(1.00000+ 323.300*Math.exp((STATES[0]+94.6000)/- 12.9000));
ALGEBRAIC[37] = ALGEBRAIC[5]/(ALGEBRAIC[5]+ALGEBRAIC[24]);
ALGEBRAIC[44] = 0.0300000/(1.00000+Math.exp((STATES[0]+40.0000)/6.00000))+0.000350000;
RATES[7] = (ALGEBRAIC[37] - STATES[7])/ALGEBRAIC[44];
ALGEBRAIC[6] =  44.9000*Math.exp((STATES[0]+66.9000)/- 5.57000);
ALGEBRAIC[25] = 1491.00/(1.00000+ 323.300*Math.exp((STATES[0]+94.6000)/- 12.9000));
ALGEBRAIC[38] = ALGEBRAIC[6]/(ALGEBRAIC[6]+ALGEBRAIC[25]);
ALGEBRAIC[45] = 0.120000/(1.00000+Math.exp((STATES[0]+60.0000)/2.00000))+0.00295000;
RATES[8] = (ALGEBRAIC[38] - STATES[8])/ALGEBRAIC[45];
ALGEBRAIC[39] = 1.00000/(1.00000+Math.exp((STATES[0] - (- 3.20000+CONSTANTS[33]))/CONSTANTS[34]));
ALGEBRAIC[7] = ( - 26.1200*(STATES[0]+35.0000))/(Math.exp((STATES[0]+35.0000)/- 2.50000) - 1.00000)+( - 78.1100*STATES[0])/(Math.exp( - 0.208000*STATES[0]) - 1.00000);
ALGEBRAIC[26] = ( 10.5200*(STATES[0] - 5.00000))/(Math.exp( 0.400000*(STATES[0] - 5.00000)) - 1.00000);
ALGEBRAIC[46] = 1.00000/(ALGEBRAIC[7]+ALGEBRAIC[26]);
RATES[9] = (ALGEBRAIC[39] - STATES[9])/ALGEBRAIC[46];
ALGEBRAIC[32] = 1.00000/( 0.150000*Math.exp(- STATES[0]/11.0000)+ 0.200000*Math.exp(- STATES[0]/700.000));
ALGEBRAIC[40] = 1.00000/( 16.0000*Math.exp(STATES[0]/8.00000)+ 15.0000*Math.exp(STATES[0]/50.0000));
ALGEBRAIC[47] = 0.00100000/(ALGEBRAIC[32]+ALGEBRAIC[40]);
ALGEBRAIC[13] = 1.00000/(1.00000+Math.exp((STATES[0] - - 49.1000)/- 8.98000));
RATES[15] = (ALGEBRAIC[13] - STATES[15])/ALGEBRAIC[47];
ALGEBRAIC[14] = 0.150400/( 3100.00*Math.exp(STATES[0]/13.0000)+ 700.000*Math.exp(STATES[0]/70.0000));
ALGEBRAIC[33] = 0.150400/( 95.0000*Math.exp(- STATES[0]/10.0000)+ 50.0000*Math.exp(- STATES[0]/700.000))+0.000229000/(1.00000+Math.exp(- STATES[0]/5.00000));
ALGEBRAIC[48] = 0.00100000/(ALGEBRAIC[14]+ALGEBRAIC[33]);
ALGEBRAIC[41] = ALGEBRAIC[14]/(ALGEBRAIC[14]+ALGEBRAIC[33]);
RATES[16] = (ALGEBRAIC[41] - STATES[16])/ALGEBRAIC[48];
ALGEBRAIC[65] = ( (( CONSTANTS[29]*Math.pow(STATES[6], 3.00000)*( 0.635000*STATES[7]+ 0.365000*STATES[8])*CONSTANTS[28]*STATES[0]*CONSTANTS[2])/CONSTANTS[48])*(Math.exp((STATES[0] - CONSTANTS[51])/CONSTANTS[48]) - 1.00000))/(Math.exp(STATES[0]/CONSTANTS[48]) - 1.00000);
ALGEBRAIC[68] = ( CONSTANTS[40]*STATES[17]*STATES[18]*Math.pow(CONSTANTS[5], 1.50000))/(Math.pow(CONSTANTS[41], 1.50000)+Math.pow(CONSTANTS[5], 1.50000));
ALGEBRAIC[69] = ( (( ALGEBRAIC[68]*CONSTANTS[8])/(10.0000+CONSTANTS[8]))*(STATES[0] - CONSTANTS[50]))/(1.00000+Math.exp(((STATES[0] - CONSTANTS[50]) - 140.000)/( 2.50000*CONSTANTS[48])));
ALGEBRAIC[70] =  CONSTANTS[31]*STATES[9]*( 0.675000*STATES[10]+ 0.325000*STATES[11])*(STATES[0] - CONSTANTS[32])*(1.00000 - (( ALGEBRAIC[69]*CONSTANTS[5])/(9.00000e-05+CONSTANTS[5]))/1.00000);
ALGEBRAIC[66] =  CONSTANTS[37]*STATES[12]*( 0.450000*STATES[13]+ 0.550000*STATES[14])*(STATES[0] - CONSTANTS[52]);
ALGEBRAIC[34] =  CONSTANTS[6]*( 0.900000*STATES[2]+ 0.100000*STATES[3])*STATES[4]*(STATES[0] - CONSTANTS[50]);
ALGEBRAIC[17] =  STATES[1]*CONSTANTS[4]*(STATES[0] - - 30.0000);
ALGEBRAIC[67] =  CONSTANTS[39]*STATES[15]*STATES[16]*(STATES[0] - CONSTANTS[38]);
ALGEBRAIC[42] =  CONSTANTS[9]*(0.500000+0.500000/(1.00000+Math.exp((STATES[0]+30.0000)/5.00000)));
ALGEBRAIC[49] = ( ALGEBRAIC[42]*Math.pow(CONSTANTS[8]/(CONSTANTS[8]+0.590000), 3.00000)*(STATES[0]+81.9000))/(1.00000+Math.exp(( 1.39300*(STATES[0]+81.9000+3.60000))/CONSTANTS[48]));
ALGEBRAIC[57] = Math.exp(( - CONSTANTS[16]*STATES[0])/( 2.00000*CONSTANTS[48]));
ALGEBRAIC[52] = 1.00000+ (CONSTANTS[27]/CONSTANTS[26])*(1.00000+Math.exp(( CONSTANTS[17]*STATES[0])/CONSTANTS[48]))+CONSTANTS[28]/CONSTANTS[24]+Math.pow(CONSTANTS[28], 2.00000)/( CONSTANTS[24]*CONSTANTS[25])+Math.pow(CONSTANTS[28], 3.00000)/( CONSTANTS[24]*CONSTANTS[25]*CONSTANTS[23]);
ALGEBRAIC[54] = ( (Math.pow(CONSTANTS[28], 2.00000)/( CONSTANTS[24]*CONSTANTS[25])+Math.pow(CONSTANTS[28], 3.00000)/( CONSTANTS[24]*CONSTANTS[25]*CONSTANTS[23]))*Math.exp(( - CONSTANTS[16]*STATES[0])/( 2.00000*CONSTANTS[48])))/ALGEBRAIC[52];
ALGEBRAIC[55] = ( (CONSTANTS[27]/CONSTANTS[26])*Math.exp(( - CONSTANTS[17]*STATES[0])/CONSTANTS[48]))/ALGEBRAIC[52];
ALGEBRAIC[53] = Math.exp(( CONSTANTS[16]*STATES[0])/( 2.00000*CONSTANTS[48]));
ALGEBRAIC[60] =  ALGEBRAIC[57]*CONSTANTS[49]*(ALGEBRAIC[54]+ALGEBRAIC[55])+ ALGEBRAIC[55]*ALGEBRAIC[53]*(CONSTANTS[53]+ALGEBRAIC[57]);
ALGEBRAIC[56] = 1.00000+ (STATES[5]/CONSTANTS[18])*(1.00000+Math.exp(( - CONSTANTS[15]*STATES[0])/CONSTANTS[48])+CONSTANTS[13]/CONSTANTS[22])+CONSTANTS[13]/CONSTANTS[19]+Math.pow(CONSTANTS[13], 2.00000)/( CONSTANTS[19]*CONSTANTS[20])+Math.pow(CONSTANTS[13], 3.00000)/( CONSTANTS[19]*CONSTANTS[20]*CONSTANTS[21]);
ALGEBRAIC[59] = ( (STATES[5]/CONSTANTS[18])*Math.exp(( - CONSTANTS[15]*STATES[0])/CONSTANTS[48]))/ALGEBRAIC[56];
ALGEBRAIC[58] = ( (Math.pow(CONSTANTS[13], 2.00000)/( CONSTANTS[19]*CONSTANTS[20])+Math.pow(CONSTANTS[13], 3.00000)/( CONSTANTS[19]*CONSTANTS[20]*CONSTANTS[21]))*Math.exp(( CONSTANTS[16]*STATES[0])/( 2.00000*CONSTANTS[48])))/ALGEBRAIC[56];
ALGEBRAIC[61] =  ALGEBRAIC[53]*CONSTANTS[53]*(ALGEBRAIC[58]+ALGEBRAIC[59])+ ALGEBRAIC[57]*ALGEBRAIC[59]*(CONSTANTS[49]+ALGEBRAIC[53]);
ALGEBRAIC[62] =  ALGEBRAIC[58]*CONSTANTS[53]*(ALGEBRAIC[54]+ALGEBRAIC[55])+ ALGEBRAIC[59]*ALGEBRAIC[54]*(CONSTANTS[53]+ALGEBRAIC[57]);
ALGEBRAIC[63] =  ALGEBRAIC[54]*CONSTANTS[49]*(ALGEBRAIC[58]+ALGEBRAIC[59])+ ALGEBRAIC[58]*ALGEBRAIC[55]*(CONSTANTS[49]+ALGEBRAIC[53]);
ALGEBRAIC[64] = ( CONSTANTS[14]*( ALGEBRAIC[61]*ALGEBRAIC[55] -  ALGEBRAIC[60]*ALGEBRAIC[59]))/(ALGEBRAIC[60]+ALGEBRAIC[61]+ALGEBRAIC[62]+ALGEBRAIC[63]);
ALGEBRAIC[51] = ( CONSTANTS[12]*Math.pow(CONSTANTS[13]/(5.64000+CONSTANTS[13]), 3.00000)*Math.pow(CONSTANTS[8]/(0.621000+CONSTANTS[8]), 2.00000)*1.60000)/(1.50000+Math.exp(- (STATES[0]+60.0000)/40.0000));
ALGEBRAIC[50] =  CONSTANTS[10]*(STATES[0] - CONSTANTS[11]);
RATES[0] = - (ALGEBRAIC[65]+ALGEBRAIC[70]+ALGEBRAIC[66]+ALGEBRAIC[34]+ALGEBRAIC[17]+ALGEBRAIC[67]+ALGEBRAIC[49]+ALGEBRAIC[64]+ALGEBRAIC[51]+ALGEBRAIC[50]+ALGEBRAIC[69])/CONSTANTS[3];
ALGEBRAIC[72] = 5.00000/(1.00000+CONSTANTS[46]/STATES[19]);
ALGEBRAIC[73] = (STATES[20] - STATES[21])/CONSTANTS[47];
RATES[20] = ALGEBRAIC[72] - ( ALGEBRAIC[73]*CONSTANTS[55])/CONSTANTS[54];
ALGEBRAIC[74] =  88800.0*STATES[19]*(1.00000 - STATES[22]) -  446.000*STATES[22];
RATES[22] = ALGEBRAIC[74];
ALGEBRAIC[76] =  227700.*STATES[19]*((1.00000 - STATES[23]) - STATES[24]) -  7.51000*STATES[23];
RATES[23] = ALGEBRAIC[76];
ALGEBRAIC[75] = ( CONSTANTS[45]*(STATES[21] - STATES[5]))/(1.00000+Math.pow(0.00120000/STATES[5], 2.00000));
ALGEBRAIC[79] =  534.000*STATES[21]*(1.00000 - STATES[27]) -  445.000*STATES[27];
RATES[21] = (ALGEBRAIC[73] - ALGEBRAIC[75]) -  10.0000*ALGEBRAIC[79];
ALGEBRAIC[71] = (STATES[5] - STATES[19])/4.00000e-05;
ALGEBRAIC[77] =  227700.*STATES[19]*(1.00000 - STATES[25]) -  542.000*STATES[25];
RATES[19] = ( ALGEBRAIC[71]*CONSTANTS[56] -  ALGEBRAIC[72]*CONSTANTS[54])/CONSTANTS[57] - ( 0.0450000*ALGEBRAIC[77]+ 0.0310000*ALGEBRAIC[74]+ 0.0620000*ALGEBRAIC[76]);
RATES[25] = ALGEBRAIC[77];
ALGEBRAIC[78] =  227700.*STATES[5]*(1.00000 - STATES[26]) -  542.000*STATES[26];
RATES[26] = ALGEBRAIC[78];
RATES[27] = ALGEBRAIC[79];
ALGEBRAIC[80] =  0.00100000*( 115.000*STATES[5]*(1.00000 - STATES[28]) -  1000.00*STATES[28]);
RATES[5] = (((- (ALGEBRAIC[70] -  2.00000*ALGEBRAIC[64])/( 2.00000*CONSTANTS[2])+ ALGEBRAIC[75]*CONSTANTS[55])/CONSTANTS[56] - ALGEBRAIC[71]) -  0.0450000*ALGEBRAIC[78]) -  (0.0310000/1.20000)*ALGEBRAIC[80];
RATES[28] = ALGEBRAIC[80];
}
export function computeVariables(VOI: number, CONSTANTS: Float64Array, RATES: Float64Array, STATES: Float64Array, ALGEBRAIC: Float64Array)
{
ALGEBRAIC[15] = 120.000/(1.00000+Math.exp(- (STATES[0]+50.0000)/15.0000));
ALGEBRAIC[16] = 5.82000/(1.00000+Math.exp(- (STATES[0]+50.0000)/15.0000));
ALGEBRAIC[18] =  2277.00*2.50000*((1.00000 - STATES[23]) - STATES[24]) -  751.000*STATES[24];
ALGEBRAIC[0] = 1.00000/(1.00000+Math.exp(((STATES[0]+83.1900) - ( - 7.20000*Math.pow(CONSTANTS[5], 0.690000))/(Math.pow(1.26000e-05, 0.690000)+Math.pow(CONSTANTS[5], 0.690000)))/13.5600));
ALGEBRAIC[19] = 0.250000+ 2.00000*Math.exp(- Math.pow(STATES[0]+70.0000, 2.00000)/500.000);
ALGEBRAIC[1] = 1.00000/(1.00000+Math.exp((STATES[0]+10.2200)/- 8.50000));
ALGEBRAIC[20] = 1.00000/( 17.0000*Math.exp( 0.0398000*STATES[0])+ 0.211000*Math.exp( - 0.0510000*STATES[0]));
ALGEBRAIC[2] = 1.00000/(1.00000+Math.exp((STATES[0]+10.2200)/- 8.50000));
ALGEBRAIC[21] = 0.335810+ 0.906730*Math.exp(- Math.pow(STATES[0]+10.0000, 2.00000)/988.050);
ALGEBRAIC[8] = 1.00000/(1.00000+Math.exp((STATES[0] - (- 24.0000+CONSTANTS[35]))/6.31000));
ALGEBRAIC[27] = 0.0100000+ 0.153900*Math.exp(- Math.pow(STATES[0]+40.0000, 2.00000)/185.670);
ALGEBRAIC[9] = 1.00000/(1.00000+Math.exp((STATES[0] - (- 24.0000+CONSTANTS[36]))/6.31000));
ALGEBRAIC[28] = 0.0600000+ 0.480760*2.25000*Math.exp(- Math.pow(STATES[0] - - 40.0000, 2.00000)/138.040);
ALGEBRAIC[29] = 0.000596000+0.00311800/( 1.03700*Math.exp( 0.0900000*(STATES[0]+30.6100))+ 0.396000*Math.exp( - 0.120000*(STATES[0]+23.8400)));
ALGEBRAIC[10] = 1.00000/(1.00000+Math.exp((STATES[0] - 7.44000)/- 16.4000));
ALGEBRAIC[30] = 0.0126600+4.72716/(1.00000+Math.exp((STATES[0]+154.500)/23.9600));
ALGEBRAIC[11] = 1.00000/(1.00000+Math.exp((STATES[0]+33.8000)/6.12000));
ALGEBRAIC[31] = 0.100000+ 4.00000*Math.exp(- Math.pow(STATES[0]+65.0000, 2.00000)/500.000);
ALGEBRAIC[12] = 1.00000/(1.00000+Math.exp((STATES[0]+33.8000)/6.12000));
ALGEBRAIC[4] = STATES[0]+44.4000;
ALGEBRAIC[23] = (Math.abs(ALGEBRAIC[4])<CONSTANTS[30] ? ( - 460.000*- 12.6730)/Math.exp(ALGEBRAIC[4]/- 12.6730) : ( - 460.000*ALGEBRAIC[4])/(Math.exp(ALGEBRAIC[4]/- 12.6730) - 1.00000));
ALGEBRAIC[36] =  18400.0*Math.exp(ALGEBRAIC[4]/- 12.6730);
ALGEBRAIC[3] =  (1.00000/(1.00000+Math.exp((STATES[0]+4.90000)/15.1400)))*(1.00000 -  0.300000*Math.exp(- Math.pow(STATES[0], 2.00000)/500.000));
ALGEBRAIC[22] =  92.0100*Math.exp( - 0.0183000*STATES[0]);
ALGEBRAIC[35] =  603.600*Math.exp( 0.00942000*STATES[0]);
ALGEBRAIC[43] = 1.00000/(ALGEBRAIC[22]+ALGEBRAIC[35]);
ALGEBRAIC[5] =  44.9000*Math.exp((STATES[0]+66.9000)/- 5.57000);
ALGEBRAIC[24] = 1491.00/(1.00000+ 323.300*Math.exp((STATES[0]+94.6000)/- 12.9000));
ALGEBRAIC[37] = ALGEBRAIC[5]/(ALGEBRAIC[5]+ALGEBRAIC[24]);
ALGEBRAIC[44] = 0.0300000/(1.00000+Math.exp((STATES[0]+40.0000)/6.00000))+0.000350000;
ALGEBRAIC[6] =  44.9000*Math.exp((STATES[0]+66.9000)/- 5.57000);
ALGEBRAIC[25] = 1491.00/(1.00000+ 323.300*Math.exp((STATES[0]+94.6000)/- 12.9000));
ALGEBRAIC[38] = ALGEBRAIC[6]/(ALGEBRAIC[6]+ALGEBRAIC[25]);
ALGEBRAIC[45] = 0.120000/(1.00000+Math.exp((STATES[0]+60.0000)/2.00000))+0.00295000;
ALGEBRAIC[39] = 1.00000/(1.00000+Math.exp((STATES[0] - (- 3.20000+CONSTANTS[33]))/CONSTANTS[34]));
ALGEBRAIC[7] = ( - 26.1200*(STATES[0]+35.0000))/(Math.exp((STATES[0]+35.0000)/- 2.50000) - 1.00000)+( - 78.1100*STATES[0])/(Math.exp( - 0.208000*STATES[0]) - 1.00000);
ALGEBRAIC[26] = ( 10.5200*(STATES[0] - 5.00000))/(Math.exp( 0.400000*(STATES[0] - 5.00000)) - 1.00000);
ALGEBRAIC[46] = 1.00000/(ALGEBRAIC[7]+ALGEBRAIC[26]);
ALGEBRAIC[32] = 1.00000/( 0.150000*Math.exp(- STATES[0]/11.0000)+ 0.200000*Math.exp(- STATES[0]/700.000));
ALGEBRAIC[40] = 1.00000/( 16.0000*Math.exp(STATES[0]/8.00000)+ 15.0000*Math.exp(STATES[0]/50.0000));
ALGEBRAIC[47] = 0.00100000/(ALGEBRAIC[32]+ALGEBRAIC[40]);
ALGEBRAIC[13] = 1.00000/(1.00000+Math.exp((STATES[0] - - 49.1000)/- 8.98000));
ALGEBRAIC[14] = 0.150400/( 3100.00*Math.exp(STATES[0]/13.0000)+ 700.000*Math.exp(STATES[0]/70.0000));
ALGEBRAIC[33] = 0.150400/( 95.0000*Math.exp(- STATES[0]/10.0000)+ 50.0000*Math.exp(- STATES[0]/700.000))+0.000229000/(1.00000+Math.exp(- STATES[0]/5.00000));
ALGEBRAIC[48] = 0.00100000/(ALGEBRAIC[14]+ALGEBRAIC[33]);
ALGEBRAIC[41] = ALGEBRAIC[14]/(ALGEBRAIC[14]+ALGEBRAIC[33]);
ALGEBRAIC[65] = ( (( CONSTANTS[29]*Math.pow(STATES[6], 3.00000)*( 0.635000*STATES[7]+ 0.365000*STATES[8])*CONSTANTS[28]*STATES[0]*CONSTANTS[2])/CONSTANTS[48])*(Math.exp((STATES[0] - CONSTANTS[51])/CONSTANTS[48]) - 1.00000))/(Math.exp(STATES[0]/CONSTANTS[48]) - 1.00000);
ALGEBRAIC[68] = ( CONSTANTS[40]*STATES[17]*STATES[18]*Math.pow(CONSTANTS[5], 1.50000))/(Math.pow(CONSTANTS[41], 1.50000)+Math.pow(CONSTANTS[5], 1.50000));
ALGEBRAIC[69] = ( (( ALGEBRAIC[68]*CONSTANTS[8])/(10.0000+CONSTANTS[8]))*(STATES[0] - CONSTANTS[50]))/(1.00000+Math.exp(((STATES[0] - CONSTANTS[50]) - 140.000)/( 2.50000*CONSTANTS[48])));
ALGEBRAIC[70] =  CONSTANTS[31]*STATES[9]*( 0.675000*STATES[10]+ 0.325000*STATES[11])*(STATES[0] - CONSTANTS[32])*(1.00000 - (( ALGEBRAIC[69]*CONSTANTS[5])/(9.00000e-05+CONSTANTS[5]))/1.00000);
ALGEBRAIC[66] =  CONSTANTS[37]*STATES[12]*( 0.450000*STATES[13]+ 0.550000*STATES[14])*(STATES[0] - CONSTANTS[52]);
ALGEBRAIC[34] =  CONSTANTS[6]*( 0.900000*STATES[2]+ 0.100000*STATES[3])*STATES[4]*(STATES[0] - CONSTANTS[50]);
ALGEBRAIC[17] =  STATES[1]*CONSTANTS[4]*(STATES[0] - - 30.0000);
ALGEBRAIC[67] =  CONSTANTS[39]*STATES[15]*STATES[16]*(STATES[0] - CONSTANTS[38]);
ALGEBRAIC[42] =  CONSTANTS[9]*(0.500000+0.500000/(1.00000+Math.exp((STATES[0]+30.0000)/5.00000)));
ALGEBRAIC[49] = ( ALGEBRAIC[42]*Math.pow(CONSTANTS[8]/(CONSTANTS[8]+0.590000), 3.00000)*(STATES[0]+81.9000))/(1.00000+Math.exp(( 1.39300*(STATES[0]+81.9000+3.60000))/CONSTANTS[48]));
ALGEBRAIC[57] = Math.exp(( - CONSTANTS[16]*STATES[0])/( 2.00000*CONSTANTS[48]));
ALGEBRAIC[52] = 1.00000+ (CONSTANTS[27]/CONSTANTS[26])*(1.00000+Math.exp(( CONSTANTS[17]*STATES[0])/CONSTANTS[48]))+CONSTANTS[28]/CONSTANTS[24]+Math.pow(CONSTANTS[28], 2.00000)/( CONSTANTS[24]*CONSTANTS[25])+Math.pow(CONSTANTS[28], 3.00000)/( CONSTANTS[24]*CONSTANTS[25]*CONSTANTS[23]);
ALGEBRAIC[54] = ( (Math.pow(CONSTANTS[28], 2.00000)/( CONSTANTS[24]*CONSTANTS[25])+Math.pow(CONSTANTS[28], 3.00000)/( CONSTANTS[24]*CONSTANTS[25]*CONSTANTS[23]))*Math.exp(( - CONSTANTS[16]*STATES[0])/( 2.00000*CONSTANTS[48])))/ALGEBRAIC[52];
ALGEBRAIC[55] = ( (CONSTANTS[27]/CONSTANTS[26])*Math.exp(( - CONSTANTS[17]*STATES[0])/CONSTANTS[48]))/ALGEBRAIC[52];
ALGEBRAIC[53] = Math.exp(( CONSTANTS[16]*STATES[0])/( 2.00000*CONSTANTS[48]));
ALGEBRAIC[60] =  ALGEBRAIC[57]*CONSTANTS[49]*(ALGEBRAIC[54]+ALGEBRAIC[55])+ ALGEBRAIC[55]*ALGEBRAIC[53]*(CONSTANTS[53]+ALGEBRAIC[57]);
ALGEBRAIC[56] = 1.00000+ (STATES[5]/CONSTANTS[18])*(1.00000+Math.exp(( - CONSTANTS[15]*STATES[0])/CONSTANTS[48])+CONSTANTS[13]/CONSTANTS[22])+CONSTANTS[13]/CONSTANTS[19]+Math.pow(CONSTANTS[13], 2.00000)/( CONSTANTS[19]*CONSTANTS[20])+Math.pow(CONSTANTS[13], 3.00000)/( CONSTANTS[19]*CONSTANTS[20]*CONSTANTS[21]);
ALGEBRAIC[59] = ( (STATES[5]/CONSTANTS[18])*Math.exp(( - CONSTANTS[15]*STATES[0])/CONSTANTS[48]))/ALGEBRAIC[56];
ALGEBRAIC[58] = ( (Math.pow(CONSTANTS[13], 2.00000)/( CONSTANTS[19]*CONSTANTS[20])+Math.pow(CONSTANTS[13], 3.00000)/( CONSTANTS[19]*CONSTANTS[20]*CONSTANTS[21]))*Math.exp(( CONSTANTS[16]*STATES[0])/( 2.00000*CONSTANTS[48])))/ALGEBRAIC[56];
ALGEBRAIC[61] =  ALGEBRAIC[53]*CONSTANTS[53]*(ALGEBRAIC[58]+ALGEBRAIC[59])+ ALGEBRAIC[57]*ALGEBRAIC[59]*(CONSTANTS[49]+ALGEBRAIC[53]);
ALGEBRAIC[62] =  ALGEBRAIC[58]*CONSTANTS[53]*(ALGEBRAIC[54]+ALGEBRAIC[55])+ ALGEBRAIC[59]*ALGEBRAIC[54]*(CONSTANTS[53]+ALGEBRAIC[57]);
ALGEBRAIC[63] =  ALGEBRAIC[54]*CONSTANTS[49]*(ALGEBRAIC[58]+ALGEBRAIC[59])+ ALGEBRAIC[58]*ALGEBRAIC[55]*(CONSTANTS[49]+ALGEBRAIC[53]);
ALGEBRAIC[64] = ( CONSTANTS[14]*( ALGEBRAIC[61]*ALGEBRAIC[55] -  ALGEBRAIC[60]*ALGEBRAIC[59]))/(ALGEBRAIC[60]+ALGEBRAIC[61]+ALGEBRAIC[62]+ALGEBRAIC[63]);
ALGEBRAIC[51] = ( CONSTANTS[12]*Math.pow(CONSTANTS[13]/(5.64000+CONSTANTS[13]), 3.00000)*Math.pow(CONSTANTS[8]/(0.621000+CONSTANTS[8]), 2.00000)*1.60000)/(1.50000+Math.exp(- (STATES[0]+60.0000)/40.0000));
ALGEBRAIC[50] =  CONSTANTS[10]*(STATES[0] - CONSTANTS[11]);
ALGEBRAIC[72] = 5.00000/(1.00000+CONSTANTS[46]/STATES[19]);
ALGEBRAIC[73] = (STATES[20] - STATES[21])/CONSTANTS[47];
ALGEBRAIC[74] =  88800.0*STATES[19]*(1.00000 - STATES[22]) -  446.000*STATES[22];
ALGEBRAIC[76] =  227700.*STATES[19]*((1.00000 - STATES[23]) - STATES[24]) -  7.51000*STATES[23];
ALGEBRAIC[75] = ( CONSTANTS[45]*(STATES[21] - STATES[5]))/(1.00000+Math.pow(0.00120000/STATES[5], 2.00000));
ALGEBRAIC[79] =  534.000*STATES[21]*(1.00000 - STATES[27]) -  445.000*STATES[27];
ALGEBRAIC[71] = (STATES[5] - STATES[19])/4.00000e-05;
ALGEBRAIC[77] =  227700.*STATES[19]*(1.00000 - STATES[25]) -  542.000*STATES[25];
ALGEBRAIC[78] =  227700.*STATES[5]*(1.00000 - STATES[26]) -  542.000*STATES[26];
ALGEBRAIC[80] =  0.00100000*( 115.000*STATES[5]*(1.00000 - STATES[28]) -  1000.00*STATES[28]);
}
