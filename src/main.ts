import './style.css';
import { initSidebar } from './ui/sidebar';
import { initCanvas, pushData } from './ui/canvas';
import EngineWorker from './worker/engine?worker';

export const worker = new EngineWorker();
(window as any).worker = worker;

document.addEventListener('DOMContentLoaded', () => {
    initSidebar();
    initCanvas();

    // Iniciar o Worker com parâmetros iniciais vazios (ou pegando do DOM depois)
    worker.postMessage({ type: 'INIT', payload: {} });

    // Escutar os pacotes do Integrador de Euler
    worker.onmessage = (e) => {
        if (e.data.type === 'DATA_BATCH') {
            pushData(e.data.payload);
        }
    };
});
