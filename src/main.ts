import './style.css';
import { initSidebar } from './ui/sidebar';
import { initCanvas } from './ui/canvas';
import EngineWorker from './worker/engine?worker';

export const worker = new EngineWorker();

document.addEventListener('DOMContentLoaded', () => {
    initSidebar();
    initCanvas();

    // Iniciar o Worker com parâmetros iniciais vazios (ou pegando do DOM depois)
    worker.postMessage({ type: 'INIT', payload: {} });

    // Escutar os pacotes do Integrador de Euler
    worker.onmessage = (e) => {
        if (e.data.type === 'DATA_BATCH') {
            // Em breve: repassar e.data.payload para o canvas.ts
            // console.log("Recebendo batch de dados do Worker:", e.data.payload.length);
        }
    };
});
