// Configuração do Integrador
// Configuração do Integrador
const DT = 0.01; // ms (passo de tempo menor para manter a estabilidade de Euler)
let time = 0; // ms (tempo total de simulação)

// Estado do Motor
let isRunning = false;
let intervalId: number | null = null;

// Parâmetros Basais e Farmacológicos (recebidos da interface)
let params: Record<string, any> = {};

import { initConsts as initSeveri, computeRates as ratesSeveri, computeVariables as varsSeveri } from './severi';
import { initConsts as initInada, computeRates as ratesInada, computeVariables as varsInada } from './inada';
import { initConsts as initTussher, computeRates as ratesTussher, computeVariables as varsTussher } from './tentussher';

const C_SEV = new Float64Array(104), R_SEV = new Float64Array(33), S_SEV = new Float64Array(33), A_SEV = new Float64Array(90);
const C_INA = new Float64Array(58), R_INA = new Float64Array(29), S_INA = new Float64Array(29), A_INA = new Float64Array(81);
const C_TUS = new Float64Array(46), R_TUS = new Float64Array(17), S_TUS = new Float64Array(17), A_TUS = new Float64Array(69);

// Buffers de Atraso Fisiológico (Delay Lines) para simular propagação espacial do sinal
const delayBufferSA = new Float64Array(20000);
delayBufferSA.fill(-60); // Repouso aproximado do SA
let bufferIdxSA = 0;
const delayBufferAV = new Float64Array(20000);
delayBufferAV.fill(-60); // Repouso aproximado do AV
let bufferIdxAV = 0;

// Variáveis de controle do Sistema de Gatilhos (Pingers de Condução)
let timerAV = -1; 
let timerVent = -1;
let sa_fired = false;
let av_fired = false;

initSeveri(C_SEV, R_SEV, S_SEV);
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

// === TUNING FISIOLÓGICO DO NÓ AV (INADA) PARA EVITAR "ESCAPE" ===
// O coelho tem o Nó AV disparando nativamente a mais de 130 BPM!
// Mesmo desligando a corrente I_f, a corrente de fuga (Background) estava puxando
// a voltagem do AV para o limiar, pois seu potencial de reversão de fábrica é -22.5mV!
C_INA[4] = 0.0; // Desliga g_f (Corrente de Marca-passo) no AV Node
C_INA[11] = -60.0; // Atraca a corrente de fuga em -60mV para forçar repouso plano!
// C_INA[3] removido: escalar a capacitância alarga muito o formato do pulso (APD).

// Variáveis de Estado - Fase 3 (Modelo Diferencial)
// (v_sa e w_sa removidos, usando STATES array)

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

        for (let i = 0; i < stepsPerFrame; i++) {
            
            // -------------------------------------------------------------
            // INTEGRAÇÃO NUMÉRICA (Fase 3) - Ocorre a cada micro-passo DT
            // -------------------------------------------------------------
            
            // Atualizar Constantes do Usuário
            if (params['sl-k'] !== undefined) C_SEV[16] = params['sl-k']; // Ko
            if (params['sl-symp'] !== undefined) C_SEV[12] = params['sl-symp'] / 100; // Iso_1_uM
            if (params['sl-parasymp'] !== undefined) C_SEV[11] = params['sl-parasymp'] / 100; // ACh

            // Tempo em segundos para Severi e Inada
            const timeSec = time / 1000.0;
            const dtSec = DT / 1000.0;

            // 1. Integrar Nó Sinoatrial (Severi 2012 - coelho)
            ratesSeveri(timeSec, C_SEV, R_SEV, S_SEV, A_SEV);
            for (let j = 0; j < 33; j++) S_SEV[j] += R_SEV[j] * dtSec;

            // --- DETECTOR DE DISPARO DO SA ---
            if (S_SEV[0] > -20 && !sa_fired) {
                sa_fired = true;
                timerAV = 100; // Tempo de viagem até o Nó AV (Intervalo PR base)
            } else if (S_SEV[0] < -40) {
                sa_fired = false;
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
            
            // PROTEÇÃO DO INTEGRATOR EULER (Voltage Clamp Diastólico)
            // Se a voltagem cair abaixo de -65mV, o portão 'm' do sódio explode para infinito devido ao dt fixo.
            if (S_INA[0] < -65.0) S_INA[0] = -65.0;

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
            const v_av = S_INA[0];
            const v_vent = S_TUS[0];

            // "Downsampling": salvamos os dados a cada 1ms (100 passos) para UI
            if (i % 100 === 0) {
                const kFactor = (params['sl-k'] || 5.4) / 5.4;

                let sa = v_sa;
                let av = v_av;
                let vent = v_vent;

                // ECG Mock temporário baseado na repolarização (a ser aprimorado)
                let ecg = 0;
                if (vent > 0) ecg = 10;
                else if (vent > -60 && vent < 0) ecg = 20 * (kFactor * kFactor);

                batchData.push({
                    t: time,
                    sa, 
                    av,
                    vent,
                    ecg 
                });
            }

            time += DT;
        }

        // Despacha o pacote de amostras matemáticas prontas para a UI pintar
        self.postMessage({
            type: 'DATA_BATCH',
            payload: batchData
        });

    }, 16); 
}
