// Configuração do Integrador
const DT = 0.05; // ms (passo de tempo para o Método de Euler)
let time = 0; // ms (tempo total de simulação)

// Estado do Motor
let isRunning = false;
let intervalId: number | null = null;

// Parâmetros Basais e Farmacológicos (recebidos da interface)
let params: Record<string, any> = {};

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

        // Vamos calcular 16ms de simulação por quadro.
        // 16ms / 0.05ms = 320 passos de Euler.
        const stepsPerFrame = 320; 
        const batchData = [];

        for (let i = 0; i < stepsPerFrame; i++) {
            
            // -------------------------------------------------------------
            // AQUI ENTRARÃO AS EQUAÇÕES DIFERENCIAIS DOS 3 TECIDOS (Fase 3)
            // V(t + dt) = V(t) + dV/dt * dt
            // -------------------------------------------------------------
            
            // Por enquanto, criamos um MOCK matemático dependente dos 'params' 
            // apenas para validar o tráfego do Web Worker para o Canvas.
            
            // Para não enviar todos os 320 passos (muita memória/CPU no PostMessage),
            // fazemos um "downsampling" salvando os dados a cada 1ms (a cada 20 passos).
            if (i % 20 === 0) {
                const beatPeriod = 300;
                const localT = time % beatPeriod;
                const kFactor = (params['sl-k'] || 5.4) / 5.4;
                
                // SA Mock com influência do K+
                let sa = -60 + (localT / beatPeriod) * 20; 
                if (localT > 250) sa = 10 - ((localT - 250) / 50) * 70;
                else if (localT > 230) sa = -40 + ((localT - 230) / 20) * 50;
                sa *= kFactor;

                // AV Mock
                let av = -60;
                const avT = (localT + 20) % beatPeriod;
                if (avT > 250) av = 10 - ((avT - 250) / 50) * 70;
                else if (avT > 240) av = -60 + ((avT - 240) / 10) * 70;

                // Vent Mock (Potássio alto sobe o potencial de repouso)
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
