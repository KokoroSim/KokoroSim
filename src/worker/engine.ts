// Configuração do Integrador
// Configuração do Integrador
const DT = 0.01; // ms (passo de tempo menor para manter a estabilidade de Euler)
let time = 0; // ms (tempo total de simulação)

// Estado do Motor
let isRunning = false;
let intervalId: number | null = null;

// Parâmetros Basais e Farmacológicos (recebidos da interface)
let params: Record<string, any> = {};

import { initConsts, computeRates, computeVariables } from './severi';

const CONSTANTS = new Float64Array(104);
const RATES = new Float64Array(33);
const STATES = new Float64Array(33);
const ALGEBRAIC = new Float64Array(90);

initConsts(CONSTANTS, RATES, STATES);

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
            if (params['sl-k'] !== undefined) CONSTANTS[16] = params['sl-k']; // Ko
            if (params['sl-symp'] !== undefined) CONSTANTS[12] = params['sl-symp'] / 100; // Iso_1_uM
            if (params['sl-parasymp'] !== undefined) CONSTANTS[11] = params['sl-parasymp'] / 100; // ACh

            // Tempo em segundos para o modelo
            const timeSec = time / 1000.0;
            const dtSec = DT / 1000.0;

            // Calcular Derivadas (Severi 2012)
            computeRates(timeSec, CONSTANTS, RATES, STATES, ALGEBRAIC);
            
            // Aplicar passo de integração Euler para todas as 33 variáveis de estado
            for (let j = 0; j < 33; j++) {
                STATES[j] += RATES[j] * dtSec;
            }

            // O Potencial de Ação é o STATES[0] (V_ode)
            const v_sa = STATES[0];

            // "Downsampling": salvamos os dados a cada 1ms (100 passos) para UI
            if (i % 100 === 0) {
                const beatPeriod = 300;
                const localT = time % beatPeriod;

                // Variáveis para plotagem
                let sa = v_sa;

                // AV Mock
                let av = -60;
                const avT = (localT + 20) % beatPeriod;
                if (avT > 250) av = 10 - ((avT - 250) / 50) * 70;
                else if (avT > 240) av = -60 + ((avT - 240) / 10) * 70;

                // Vent Mock (Potássio alto sobe o potencial de repouso)
                const kFactor = (params['sl-k'] || 5.4) / 5.4;
                let vent = -80 * (1 / kFactor);
                const ventT = (localT + 60) % beatPeriod;
                if (ventT > 250) vent = -80 * (1 / kFactor);
                else if (ventT > 180) vent = 0 - ((ventT - 180) / 70) * 80;
                else if (ventT > 50) vent = 20 - ((ventT - 50) / 130) * 20;
                else if (ventT > 40) vent = (-80 * (1 / kFactor)) + ((ventT - 40) / 10) * 100;

                // ECG Mock (Potássio alto = Onda T apiculada, Potássio baixo = Onda T achatada/invertida)
                let ecg = 0;
                if (localT > 220 && localT < 240) ecg = Math.sin((localT - 220)/20 * Math.PI) * 10;
                else if (ventT > 40 && ventT < 50) ecg = Math.sin((ventT - 40)/10 * Math.PI) * 40;
                else if (ventT > 160 && ventT < 200) {
                    const tWaveBase = Math.sin((ventT - 160)/40 * Math.PI) * 15;
                    // kFactor > 1: T wave apiculada. kFactor < 1: achatada
                    ecg = tWaveBase * (kFactor * kFactor);
                }

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
