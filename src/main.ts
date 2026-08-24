import './style.css';
import { initSidebar } from './ui/sidebar';
import { initCanvas, pushData, clearData } from './ui/canvas';
import EngineWorker from './worker/engine?worker';

export const worker = new EngineWorker();
(window as any).worker = worker;

document.addEventListener('DOMContentLoaded', () => {
    initSidebar();
    initCanvas();

    // Iniciar o Worker com parâmetros iniciais vazios (ou pegando do DOM depois)
    worker.postMessage({ type: 'INIT', payload: {} });

    // Escutar os pacotes do Integrador de Euler
    worker.onmessage = (e: MessageEvent) => {
        if (e.data.type === 'DATA_BATCH') {
            pushData(e.data.payload);
            
            if (e.data.hud) {
                const h = e.data.hud;
                const bpmEl = document.getElementById('hud-bpm');
                const prEl = document.getElementById('hud-pr');
                const qrsEl = document.getElementById('hud-qrs');
                const qtEl = document.getElementById('hud-qt');
                const vrestEl = document.getElementById('hud-vrest');
                const prrrEl = document.getElementById('hud-pr-rr');
                
                if (bpmEl) bpmEl.innerText = h.bpm.toString();
                if (prEl) prEl.innerText = h.pr + 'ms';
                if (qrsEl) qrsEl.innerText = h.qrs + 'ms';
                if (qtEl) qtEl.innerText = h.qt + 'ms';
                if (vrestEl) vrestEl.innerText = h.vrest + 'mV';
                if (prrrEl) {
                    let ratio = h.rr > 0 ? (h.pr / h.rr).toFixed(2) : '-';
                    prrrEl.innerText = ratio;
                }
            }
        } else if (e.data.type === 'DATA_CLEAR') {
            clearData();
        }
    };
});
