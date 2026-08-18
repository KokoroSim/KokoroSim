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
                batchData.push({
                    t: time,
                    // Mock dependente do K_o (só pra provar comunicação)
                    sa: -60 + Math.sin(time * 0.01) * (params['sl-k'] || 5.4) * 2, 
                    av: -60,
                    vent: -80,
                    ecg: 0 
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
