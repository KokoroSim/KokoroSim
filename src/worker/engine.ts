// Configuração do Integrador
// Configuração do Integrador
const DT = 0.01; // ms (passo de tempo menor para manter a estabilidade de Euler)
let time = 0; // ms (tempo total de simulação)

// Estado do Motor
let isRunning = false;
let intervalId: number | null = null;

// Parâmetros Basais e Farmacológicos (recebidos da interface)
let params: Record<string, any> = {};

import { initConsts as initSeveri, computeRates as ratesSeveri } from './severi';
import { initConsts as initAtrium, computeRates as ratesAtrium } from './courtemanche';
import { initConsts as initInada, computeRates as ratesInada } from './inada';
import { initConsts as initTussher, computeRates as ratesTussher } from './tentussher';

const C_SEV = new Float64Array(104), R_SEV = new Float64Array(33), S_SEV = new Float64Array(33), A_SEV = new Float64Array(90);
const C_ATR = new Float64Array(49), R_ATR = new Float64Array(21), S_ATR = new Float64Array(21), A_ATR = new Float64Array(75);
const C_INA = new Float64Array(58), R_INA = new Float64Array(29), S_INA = new Float64Array(29), A_INA = new Float64Array(81);
const C_TUS = new Float64Array(46), R_TUS = new Float64Array(17), S_TUS = new Float64Array(17), A_TUS = new Float64Array(69);

// Variáveis de controle do Sistema de Gatilhos (Pingers de Condução)
let timerAtrium = -1;
let timerAV = -1; 
let timerVent = -1;
let sa_fired = false;
let atrium_fired = false;
let av_fired = false;

initSeveri(C_SEV, R_SEV, S_SEV);
initAtrium(C_ATR, R_ATR, S_ATR);
C_ATR[8] = 0; // Desliga o marcapasso nativo do Músculo Atrial (obedece apenas ao SA)
initInada(C_INA, R_INA, S_INA);
initTussher(C_TUS, R_TUS, S_TUS);

// === TUNING FISIOLÓGICO DO NÓ SA (SEVERI) PARA ~96 BPM ===
// Reduz as correntes de entrada da Fase 4 e aumenta a de saída para deitar a inclinação (BPM humano)
// Essa configuração mantém uma margem segura para a Acetilcolina (Simpático/Parassimpático) não matar a célula.
C_SEV[82] *= 0.25; // Corta 75% da g_f_Na (Corrente Funny)
C_SEV[83] *= 0.25; // Corta 75% da g_f_K (Corrente Funny)
C_SEV[37] *= 0.75; // Corta 25% de P_CaL (Cálcio Lento)
C_SEV[79] *= 1.25; // Aumenta 25% de g_Kr (Potássio Rápido, empurra pra baixo)
C_SEV[35] *= 0.50; // Corta 50% de g_Na (Sódio background/rápido)
// Aumenta o tamanho (Capacitância) da célula para níveis humanos, atrasando a carga/descarga em 40%
// Isso empurra a frequência final de 96 BPM para redondos 75 BPM sem quebrar a dinâmica do modelo!
C_SEV[3] *= 1.40;

// === TUNING FISIOLÓGICO DO NÓ AV (INADA) PARA FASE 4 LENTA ===
// Nós reduzimos as correntes de marca-passo nativas (I_f e Fuga) a uma fração bem pequena.
// Isso impede que o AV atinja o limiar sozinho antes de 800ms (não escapa do SA),
// mas permite que o gráfico mostre a clássica rampa lenta ascendente da Fase 4.
C_INA[4] *= 0.15; // Mantém apenas 15% da Corrente Funny original
C_INA[10] *= 0.45; // Sweet-spot: 40% da Corrente de Fuga cria a rampa sem escapar
// (C_INA[11] = -60 removido: a voltagem agora vai flutuar livre e suavemente)

// === TUNING FISIOLÓGICO DO VENTRÍCULO (TEN TUSSCHER 2004) ===
// O artigo original define 3 tipos de células mudando apenas 2 condutâncias:
// - Epicárdica: g_to = 0.294
// - M-Cell: g_to = 0.294, g_Ks = 0.098
// - Endocárdica: g_to = 0.073
// Aqui definimos o valor oficial da célula Endocárdica:
C_TUS[20] = 0.073; 

// Salva o estado basal das constantes após o tuning para podermos aplicar 
// modificadores farmacológicos e fisiológicos dinamicamente
const C_SEV_BASE = new Float64Array(C_SEV);
const C_ATR_BASE = new Float64Array(C_ATR);
const C_INA_BASE = new Float64Array(C_INA);
const C_TUS_BASE = new Float64Array(C_TUS);

// Variáveis de Estado - Fase 3 (Modelo Diferencial)
// (v_sa e w_sa removidos, usando STATES array)

let lastBeatCount = -1;

// --- Variáveis de Tracking do HUD ---
let v_sa_prev = -80;
let v_vent_prev = -85;
let t_sa = 0;
let t_vent = 0;
let t_vent_prev = 0;
let bpm = 75;
let pr = 160;
let qt = 400;
let v_rest = -85;
let current_v_min = 0;
let in_ap = false;
let t_ap_start = 0;
let last_ecg_vent = -85;
let v_atr_prev = -80;
let t_atr = 0;
let t_t_wave = 0;
// ------------------------------------

// Escuta mensagens vindas do Main Thread (Interface)
self.onmessage = (e: MessageEvent) => {
    const { type, payload } = e.data;

    if (type === 'INIT') {
        params = payload;
        isRunning = true;
        startEngine();
    } else if (type === 'UPDATE_PARAMS') {
        params = { ...params, ...payload };
    } else if (type === 'PAUSE') {
        isRunning = false;
        if (intervalId) {
            clearInterval(intervalId);
            intervalId = null;
        }
    } else if (type === 'RESUME') {
        isRunning = true;
        startEngine();
    }
};

// Loop Principal do Integrador Numérico (Método de Euler)
function startEngine() {
    if (intervalId) clearInterval(intervalId);
    
    // O setInterval roda a cada 16ms (~60 FPS) no tempo real
    // Mas dentro dele, calculamos múltiplos passos matemáticos para acompanhar o tempo
    intervalId = setInterval(() => {
        if (!isRunning) return;

        // 16ms / 0.01ms = 1600 passos de Euler para altíssima estabilidade numérica.
        const stepsPerFrame = 1600; 
        const batchData = [];

        // Extrai parâmetros do usuário 1x por frame para altíssima performance
        const Ko = params['sl-k'] !== undefined ? params['sl-k'] : 5.4;
        const Cao = params['sl-ca'] !== undefined ? params['sl-ca'] : 2.0;
        const Nao = params['sl-na'] !== undefined ? params['sl-na'] : 140.0;
        
        const blockNa = 1.0 - ((params['sl-lido'] || 0) / 100.0);
        const blockK = 1.0 - ((params['sl-amio'] || 0) / 100.0);
        const blockCa = 1.0 - ((params['sl-vera'] || 0) / 100.0);
        const blockNaK = 1.0 - ((params['sl-digo'] || 0) / 100.0);
        
        const symp = (params['sl-symp'] || 0) / 100.0;
        const parasymp = (params['sl-parasymp'] || 0) / 100.0;
        const iso = symp;
        const ach = parasymp * 1e-3; // Max 1 uM ACh (dose-resposta fisiológica)

        // 3. Sistema Nervoso Autônomo (SNA)
        // Severi usa ACh e Iso para calcular várias constantes de gating e condutâncias.
        // O original era binário para Iso, então interpolamos para ficar suave!
        C_SEV[12] = iso; 
        C_SEV[11] = ach; 

        // Efeitos competitivos na constante 84
        const effIso84 = iso * -0.25;
        const effAch84 = ach > 0 ? (0.7 * ach) / (9e-5 + ach) : 0;
        C_SEV[84] = effIso84 + effAch84; 

        C_SEV[86] = 0.00165760 * (1.0 + 0.2 * iso);
        C_SEV[89] = ach > 0 ? -1.0 - (9.898 * Math.pow(ach, 0.618)) / (Math.pow(ach, 0.618) + 0.00122423) : 0;
        C_SEV[90] = iso * 7.5;
        C_SEV[91] = 1.0 + 0.2 * iso;
        C_SEV[93] = 1.0 + 0.23 * iso;
        C_SEV[94] = (0.31 * ach) / (ach + 9e-5);
        C_SEV[95] = iso * -8.0;
        C_SEV[96] = 1.0 - 0.31 * iso;
        C_SEV[102] = iso * -14.0;
        C_SEV[103] = ach > 0 ? (3.59880 - 0.0256410) / (1.0 + 1.21550e-06 / Math.pow(ach, 1.69510)) + 0.0256410 : 0.0256410;

        // Modificador conservador (10%) para os outros tecidos para evitar bloqueio AV por falha de I_CaL
        const ansCaModifier = 1.0 + (symp * 0.1) - (parasymp * 0.1);

        const isch = (params['sl-isch'] || 0) / 100.0;
        const ischKo = Ko + (isch * 6.0);
        const ischBlock = 1.0 - (isch * 0.5);

        // Atualiza os potenciais de Nernst cacheados de Severi (SA) e Inada (AV)
        // Severi tem E_K em C_SEV[87]. Inada tem E_K em C_INA[50/52] e E_Na em C_INA[51]
        // Se isquemia ativa, usa o Ko isquêmico
        const effectiveKo = isch > 0 ? ischKo : Ko;
        
        // E_K Severi = C_SEV[85] * ln(Ko / Ki) onde Ki = C_SEV[15]
        C_SEV[87] = C_SEV_BASE[85] * Math.log(effectiveKo / C_SEV_BASE[15]);
        
        // E_K Inada = C_INA[48] * ln(Ko / Ki) onde Ki = C_INA[7]
        const inadaEK = C_INA_BASE[48] * Math.log(effectiveKo / C_INA_BASE[7]);
        C_INA[50] = inadaEK;
        C_INA[52] = inadaEK;

        // E_Na Inada = C_INA[48] * ln(Nao / Nai) onde Nai = C_INA[13]
        C_INA[51] = C_INA_BASE[48] * Math.log(Nao / C_INA_BASE[13]);

        for (let i = 0; i < stepsPerFrame; i++) {
            
            // -------------------------------------------------------------
            // INTEGRAÇÃO NUMÉRICA E FARMACOLOGIA - A cada micro-passo DT
            // -------------------------------------------------------------
            
            // 1. Eletrólitos
            C_SEV[16] = Ko; C_SEV[17] = Cao; C_SEV[14] = Nao;
            C_ATR[12] = Ko; C_ATR[24] = Cao; C_ATR[10] = Nao;
            C_INA[8] = Ko;  C_INA[27] = Cao; C_INA[28] = Nao;
            C_TUS[10] = Ko; C_TUS[12] = Cao; C_TUS[11] = Nao;

            // 2. Fármacos Antiarrítmicos
            C_SEV[35] = C_SEV_BASE[35] * blockNa; C_ATR[9]  = C_ATR_BASE[9] * blockNa;
            C_INA[29] = C_INA_BASE[29] * blockNa; C_TUS[16] = C_TUS_BASE[16] * blockNa;

            C_SEV[79] = C_SEV_BASE[79] * blockK; C_SEV[86] = C_SEV_BASE[86] * blockK;
            C_ATR[15] = C_ATR_BASE[15] * blockK; C_ATR[16] = C_ATR_BASE[16] * blockK;
            C_INA[6]  = C_INA_BASE[6] * blockK; 
            C_TUS[14] = C_TUS_BASE[14] * blockK; C_TUS[15] = C_TUS_BASE[15] * blockK;

            C_SEV[37] = C_SEV_BASE[37] * blockCa; C_ATR[17] = C_ATR_BASE[17] * blockCa;
            C_INA[31] = C_INA_BASE[31] * blockCa; C_TUS[18] = C_TUS_BASE[18] * blockCa;

            C_SEV[21] = C_SEV_BASE[21] * blockNaK; C_ATR[20] = C_ATR_BASE[20] * blockNaK;
            C_TUS[21] = C_TUS_BASE[21] * blockNaK;

            // 3. Sistema Nervoso Autônomo
            C_SEV[12] = symp; // SA tem receptor embutido
            C_SEV[11] = parasymp; 

            C_ATR[17] *= ansCaModifier; C_INA[31] *= ansCaModifier; C_TUS[18] *= ansCaModifier;

            // 4. Isquemia (Efeito drástico: Hipercalemia Local + Hipóxia)
            if (isch > 0) {
                C_SEV[16] = ischKo; C_ATR[12] = ischKo; C_INA[8] = ischKo; C_TUS[10] = ischKo;
                C_SEV[35] *= ischBlock; C_ATR[9] *= ischBlock; C_INA[29] *= ischBlock; C_TUS[16] *= ischBlock;
                C_SEV[37] *= ischBlock; C_ATR[17] *= ischBlock; C_INA[31] *= ischBlock; C_TUS[18] *= ischBlock;
            }

            // Tempo em segundos para Severi e Inada
            const timeSec = time / 1000.0;
            const dtSec = DT / 1000.0;

            // 1. Integrar Nó Sinoatrial (Severi 2012 - coelho)
            ratesSeveri(timeSec, C_SEV, R_SEV, S_SEV, A_SEV);
            for (let j = 0; j < 33; j++) S_SEV[j] += R_SEV[j] * dtSec;

            // --- DETECTOR DE DISPARO DO SA ---
            if (S_SEV[0] > -20 && !sa_fired) {
                sa_fired = true;
                timerAtrium = 15; // Tempo curto para espalhar para o Átrio
            } else if (S_SEV[0] < -40) {
                sa_fired = false;
            }

            // --- INTEGRAR MÚSCULO ATRIAL (Courtemanche 1998 - humano) ---
            let stim_atrium = 0;
            if (timerAtrium > -1.0) {
                timerAtrium -= DT;
                if (timerAtrium > -1.0 && timerAtrium <= 0) {
                    stim_atrium = 50; 
                }
            }

            // Courtemanche tem canais rápidos de Sódio que explodem se o DT for muito grande.
            // Precisamos dividir a integração em micro-passos (subSteps).
            const subStepsAtr = 10;
            const subDTAtr = DT / subStepsAtr;
            for (let k = 0; k < subStepsAtr; k++) {
                const subTimeAtr = time + k * subDTAtr;
                ratesAtrium(subTimeAtr, C_ATR, R_ATR, S_ATR, A_ATR);
                for (let j = 0; j < 21; j++) S_ATR[j] += R_ATR[j] * subDTAtr;
                S_ATR[0] += stim_atrium * subDTAtr;
            }

            // --- DETECTOR DE DISPARO DO ÁTRIO ---
            if (S_ATR[0] > -10 && !atrium_fired) {
                atrium_fired = true;
                timerAV = 80; // Tempo de viagem do Átrio até o Nó AV (Onda P -> Nó AV)
            } else if (S_ATR[0] < -40) {
                atrium_fired = false;
            }

            // --- ESTIMULADOR DO AV ---
            let stim_av = 0;
            if (timerAV > -1.0) {
                timerAV -= DT;
                if (timerAV > -1.0 && timerAV <= 0) {
                    stim_av = 2; // ~40mV de amplitude ao longo de 1ms
                }
            }

            // 2. Integrar Nó Atrioventricular (Inada - coelho)
            ratesInada(timeSec, C_INA, R_INA, S_INA, A_INA);
            for (let j = 0; j < 29; j++) S_INA[j] += R_INA[j] * dtSec;

            if (stim_av > 0) {
                // Injeta a corrente por 1ms para cruzar o limiar suavemente
                S_INA[0] += (stim_av / C_INA[3]) * dtSec;
            }

            // --- DETECTOR DE DISPARO DO AV ---
            if (S_INA[0] > -10 && !av_fired) {
                av_fired = true;
                timerVent = 60; // Tempo de viagem His-Purkinje até Ventrículo (Intervalo PQ/QRS)
            } else if (S_INA[0] < -40) {
                av_fired = false;
            }

            // 3. Integrar Ventrículo (Ten Tusscher 2004 - humano)
            C_TUS[8] = 0; // Marca-passo nativo SEMPRE desligado
            
            if (timerVent > -1.0) {
                timerVent -= DT;
                if (timerVent > -1.0 && timerVent <= 0) {
                    C_TUS[8] = -52; 
                }
            }

            const subSteps = 10;
            const subDT = DT / subSteps;
            for (let k = 0; k < subSteps; k++) {
                const subTime = time + k * subDT;
                ratesTussher(subTime, C_TUS, R_TUS, S_TUS, A_TUS);
                
                let manual_stim = 0;
                if (timerVent > -1.0 && timerVent <= 0) {
                    manual_stim = 50; 
                }
                
                for (let j = 0; j < 17; j++) S_TUS[j] += R_TUS[j] * subDT;
                S_TUS[0] += manual_stim * subDT; 
            }

            const v_sa = S_SEV[0];
            const v_atr = S_ATR[0];
            const v_av = S_INA[0];
            const v_vent = S_TUS[0];

            // --- HUD Metrics Calculation (alta resolução) ---
            if (v_sa > -20 && v_sa_prev <= -20) {
                t_sa = timeSec * 1000.0;
            }
            if (v_atr > -20 && v_atr_prev <= -20) {
                t_atr = timeSec * 1000.0;
            }
            if (v_vent < -10 && v_vent_prev >= -10) {
                t_t_wave = timeSec * 1000.0; // Início da repolarização rápida (Fase 3)
            }
            if (v_vent > -20 && v_vent_prev <= -20) {
                t_vent_prev = t_vent;
                t_vent = timeSec * 1000.0;
                let rr = t_vent - t_vent_prev;
                if (rr > 100 && rr < 5000) {
                    bpm = (bpm * 0.7) + ((60000.0 / rr) * 0.3); // Média móvel suave
                }
                pr = t_vent - t_sa;
                if (pr < 0 || pr > 600) pr = 0; // PR inválido se SA não bateu antes
                
                in_ap = true;
                t_ap_start = timeSec * 1000.0;
                v_rest = current_v_min;
                current_v_min = v_vent;
            }
            if (in_ap && v_vent < -60) {
                in_ap = false;
                qt = (timeSec * 1000.0) - t_ap_start;
            }
            if (v_vent < current_v_min) {
                current_v_min = v_vent;
            }
            v_sa_prev = v_sa;
            v_atr_prev = v_atr;
            v_vent_prev = v_vent;
            // ------------------------------------------------

            // "Downsampling": salvamos os dados a cada 0.1ms (10 passos) para UI
            if (i % 10 === 0) {
                const kFactor = (params['sl-k'] || 5.4) / 5.4;

                let sa = v_sa;
                let atr = v_atr;
                let av = v_av;
                let vent = v_vent;

                // --- Geração do Pseudo-ECG (Híbrido Espaço-Temporal) ---
                let ecg = 0;
                let t_ms = timeSec * 1000.0;

                // 1. Onda P (Ativação Atrial). Ocorre quando o átrio despolariza.
                // Usamos um envelope de 80ms para simular a propagação espacial.
                let pWave = 0;
                let timeSinceAtr = t_ms - t_atr;
                if (timeSinceAtr > 0 && timeSinceAtr < 80) {
                    pWave = Math.sin((timeSinceAtr / 80) * Math.PI) * 12.0; 
                }

                // 2. Complexo QRS (Ativação Ventricular)
                let qrs = 0;
                let timeSinceVent = t_ms - t_vent;
                let qrsWidth = 90.0 / (blockNa || 1.0); // Alarga com bloqueadores de Sódio
                if (timeSinceVent > 0 && timeSinceVent < qrsWidth) {
                    // QRS bifásico (Onda R e S)
                    let phase = timeSinceVent / qrsWidth; 
                    if (phase < 0.4) {
                        qrs = Math.sin((phase / 0.4) * Math.PI) * 60.0; // Onda R
                    } else if (phase < 0.7) {
                        qrs = -Math.sin(((phase - 0.4) / 0.3) * Math.PI) * 15.0; // Onda S
                    }
                }

                // 3. Onda T (Repolarização Ventricular)
                // Como uma única célula repolariza muito rápido gerando um pico (T apiculada falsa), 
                // usamos um envelope de ~160ms disparado pela queda da voltagem (-10mV) para recriar o "morrinho".
                // Fisiologia Pura:
                // - Amiodarona (blockK diminui): Alarga a onda T
                // - Hipercalemia (kFactor aumenta): Estreita e eleva a onda T (T apiculada verdadeira!)
                let tWave = 0;
                let timeSinceT = t_ms - t_t_wave;
                
                let tWaveWidth = 160.0 / (blockK * kFactor);
                let tWaveHeight = 20.0 * (kFactor / Math.sqrt(blockK)); // Fator estético realista
                
                if (timeSinceT > 0 && timeSinceT < tWaveWidth) { 
                    // Onda T = metade de um seno ("morrinho")
                    tWave = Math.sin((timeSinceT / tWaveWidth) * Math.PI) * tWaveHeight;
                }

                ecg = pWave + qrs + tWave;
                // --------------------------------------------------------------

                // Extrai as correntes e íons para os monitores inferiores e os normaliza
                // O canvas espera valores no range de -100 a +50 para desenhar bem na tela.
                const corrente_na = A_TUS[53] / 3.0; // I_Na chega a -300 pA/pF (reduzir)
                const corrente_ca = A_TUS[55] * 10.0; // I_CaL chega a -8 pA/pF (aumentar)
                const corrente_k = (A_TUS[50] + A_TUS[51] + A_TUS[52] + A_TUS[57]) * 10.0;
                
                // Cálcio intracelular é pequenininho (~0.0001 a 0.001 mM). Escalando por 50000 -> 5 a 50
                const ca_int = S_TUS[3] * 50000.0;
                
                // Sódio intracelular é bem estável em ~10mM. Tira o offset e escala as variações.
                const na_int = (S_TUS[2] - 10.0) * 10.0;
                
                const corrente_f = A_SEV[51] * 20.0; // I_f é minúsculo no Nó SA

                batchData.push({
                    t: time,
                    sa, 
                    atr,
                    av,
                    vent,
                    ecg,
                    corrente_na,
                    corrente_ca,
                    corrente_k,
                    ca_int,
                    na_int,
                    corrente_f
                });
            }

            time += DT;
        }

        // Despacha o pacote de amostras matemáticas prontas para a UI pintar
        self.postMessage({ 
            type: 'DATA_BATCH', 
            payload: batchData,
            hud: {
                bpm: Math.round(bpm),
                pr: Math.round(pr),
                qrs: Math.round(90 / (blockNa || 1)), // Simulação de alargamento de QRS se bloquear Sódio
                qt: Math.round(qt),
                vrest: Math.round(v_rest),
                rr: Math.round(t_vent - t_vent_prev)
            }
        });

    }, 16); 
}
