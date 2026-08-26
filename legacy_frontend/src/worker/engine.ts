import init, { HeartSystem } from '../../wasm_engine/pkg/wasm_engine.js';
import wasmUrl from '../../wasm_engine/pkg/wasm_engine_bg.wasm?url';

let params: Record<string, number> = {};
let isRunning = false;
let intervalId: any = null;
let system: any = null;

interface SimPoint {
    time: number;
    sa: number;
    av: number;
    atrium: number;
    ventricle: number;
}

// Keep hud tracking
let hudData = {
    hr: 75,
    pr: 160,
    qt: 400,
    apState: 'Normal',
    ek: -85,
    ena: 60,
    eca: 120,
    vmax: 200,
    vmin: -85
};

async function initEngine() {
    await init(wasmUrl);
    system = new HeartSystem();
    console.log("WASM HeartSystem initialized in Web Worker!");
}

initEngine();

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
    } else if (type === 'RESET') {
        if (system) {
            system = new HeartSystem();
        }
    }
};

function startEngine() {
    if (intervalId) clearInterval(intervalId);
    
    const dt = 0.01;
    const stepsPerFrame = 1600;
    const downsample = 10; 
    let currentTime = 0;

    intervalId = setInterval(() => {
        if (!isRunning || !system) return;

        // Extrai parâmetros do usuário 
        const ko = params['sl-k'] !== undefined ? params['sl-k'] : 5.4;
        const cao = params['sl-ca'] !== undefined ? params['sl-ca'] : 2.0;
        const nao = params['sl-na'] !== undefined ? params['sl-na'] : 140.0;
        
        const blockNa = 1.0 - ((params['sl-lido'] || 0) / 100.0);
        const blockK = 1.0 - ((params['sl-amio'] || 0) / 100.0);
        const blockCa = 1.0 - ((params['sl-vera'] || 0) / 100.0);
        const blockNaK = 1.0 - ((params['sl-digo'] || 0) / 100.0);
        
        const symp = (params['sl-symp'] || 0) / 100.0;
        const parasymp = (params['sl-parasymp'] || 0) / 100.0;
        const isch = (params['sl-isch'] || 0) / 100.0;

        // Atualiza a farmacologia no motor WASM
        system.update_params(
            ko, cao, nao, 
            blockNa, blockK, blockCa, blockNaK, 
            symp, parasymp, isch
        );
        
        // Execute the heavy integration entirely in WebAssembly
        const flatBatch = system.run_batch(dt, stepsPerFrame, downsample);
        
        // Convert the flat Float64Array to Array of Objects for the Chart
        const batchData: SimPoint[] = [];
        for (let i = 0; i < flatBatch.length; i += 4) {
            batchData.push({
                time: currentTime,
                sa: flatBatch[i],
                av: flatBatch[i+1],
                atrium: flatBatch[i+2],
                ventricle: flatBatch[i+3]
            });
            currentTime += (dt * downsample);
        }

        // Post data back to main thread
        postMessage({
            type: 'TICK',
            batch: batchData,
            hud: hudData
        });
        
    }, 16);
}
