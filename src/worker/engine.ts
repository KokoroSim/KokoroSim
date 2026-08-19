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

            const v_sa_old = S_SEV[0];
            const v_av_old = S_INA[0];

            // 1. Integrar Nó Sinoatrial (Severi 2012 - coelho)
            ratesSeveri(timeSec, C_SEV, R_SEV, S_SEV, A_SEV);
            for (let j = 0; j < 33; j++) S_SEV[j] += R_SEV[j] * dtSec;

            // 2. Integrar Nó Atrioventricular (Inada - coelho)
            ratesInada(timeSec, C_INA, R_INA, S_INA, A_INA);
            for (let j = 0; j < 29; j++) S_INA[j] += R_INA[j] * dtSec;

            // --- ACOPLAMENTO ELETROTÔNICO SA -> AV ---
            // Corrente de junção comunicante (Gap Junction) que flui do SA para o AV
            const g_sa_av = 0.005; // condutância da junção em microS (Ajustável para o PR interval)
            const i_gap_sa_av = g_sa_av * (v_sa_old - v_av_old); // nanoA
            // Adiciona a corrente no AV e subtrai do SA (I/C * dt = dV)
            S_INA[0] += (i_gap_sa_av / C_INA[3]) * dtSec;
            S_SEV[0] -= (i_gap_sa_av / C_SEV[3]) * dtSec;

            // 3. Integrar Ventrículo (Ten Tusscher 2004 - humano) usa Milissegundos
            C_TUS[8] = 0; // DESLIGA o marca-passo artificial/intrínseco do ventrículo!

            // Sub-cycling de 10x (dt=0.001ms) para evitar explosão (NaN) no canal rápido de Na+
            const subSteps = 10;
            const subDT = DT / subSteps;
            for (let k = 0; k < subSteps; k++) {
                const subTime = time + k * subDT;
                ratesTussher(subTime, C_TUS, R_TUS, S_TUS, A_TUS);
                for (let j = 0; j < 17; j++) S_TUS[j] += R_TUS[j] * subDT;

                // --- ACOPLAMENTO ELETROTÔNICO AV -> VENTRÍCULO ---
                const v_av_current = S_INA[0];
                const v_vent_current = S_TUS[0];
                // g_av_v em pA/pF/mV (adimensionalizado por capacitância)
                const g_av_v = 0.05; 
                const i_gap_av_v = g_av_v * (v_av_current - v_vent_current);
                S_TUS[0] += i_gap_av_v * subDT;
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
