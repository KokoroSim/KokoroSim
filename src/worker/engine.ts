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

            // 2. Integrar Nó Atrioventricular (Inada - coelho)
            ratesInada(timeSec, C_INA, R_INA, S_INA, A_INA);
            for (let j = 0; j < 29; j++) S_INA[j] += R_INA[j] * dtSec;

            // 3. Integrar Ventrículo (Ten Tusscher 2004 - humano) usa Milissegundos
            // Sub-cycling de 10x (dt=0.001ms) para evitar explosão (NaN) no canal rápido de Na+
            const subSteps = 10;
            const subDT = DT / subSteps;
            for (let k = 0; k < subSteps; k++) {
                const subTime = time + k * subDT;
                ratesTussher(subTime, C_TUS, R_TUS, S_TUS, A_TUS);
                for (let j = 0; j < 17; j++) S_TUS[j] += R_TUS[j] * subDT;
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
