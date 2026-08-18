import { CardiacModel } from './model';
import { AudioSystem } from './audio';
import { Renderer } from './renderer';

const model = new CardiacModel();
const audio = new AudioSystem();
let renderer: Renderer;
let selectedLead = 'DII';

// UI Elements
const btnStart = document.getElementById('btn-start')!;
const overlay = document.getElementById('overlay')!;
const appDiv = document.getElementById('app')!;

const selCell = document.getElementById('sel-cell') as HTMLSelectElement;
const selLead = document.getElementById('sel-lead') as HTMLSelectElement;
const slAdr = document.getElementById('sl-adr') as HTMLInputElement;
const slK = document.getElementById('sl-k') as HTMLInputElement;
const slCa = document.getElementById('sl-ca') as HTMLInputElement;

// Labels
const valAdr = document.getElementById('val-adr')!;
const valK = document.getElementById('val-k')!;
const valCa = document.getElementById('val-ca')!;

// Inicialização e desbloqueio de Áudio
btnStart.addEventListener('click', () => {
  audio.init();
  overlay.style.display = 'none';
  appDiv.style.display = 'flex';
  
  const cvsAP = document.getElementById('cvs-ap') as HTMLCanvasElement;
  const cvsECG = document.getElementById('cvs-ecg') as HTMLCanvasElement;
  renderer = new Renderer(cvsAP, cvsECG);
  
  requestAnimationFrame(loop);
});

// Bindings da UI
selCell.addEventListener('change', (e) => model.cellType = (e.target as any).value);
selLead.addEventListener('change', (e) => selectedLead = (e.target as any).value);

slAdr.addEventListener('input', (e) => {
  model.adrenaline = parseFloat((e.target as HTMLInputElement).value);
  valAdr.innerText = model.adrenaline.toFixed(2);
});

slK.addEventListener('input', (e) => {
  model.potassium = parseFloat((e.target as HTMLInputElement).value);
  valK.innerText = model.potassium.toFixed(1) + ' mEq/L';
});

slCa.addEventListener('input', (e) => {
  model.caBlocker = parseFloat((e.target as HTMLInputElement).value);
  valCa.innerText = Math.round(model.caBlocker * 100) + '%';
});

const btnReset = document.getElementById('btn-reset') as HTMLButtonElement;
btnReset.addEventListener('click', () => {
  // Reset Model
  model.cellType = 'ventricular';
  model.potassium = 4.0;
  model.adrenaline = 0.0;
  model.caBlocker = 1.0;
  selectedLead = 'DII';

  // Reset UI inputs
  selCell.value = 'ventricular';
  selLead.value = 'DII';
  slAdr.value = '0';
  slK.value = '4.0';
  slCa.value = '1.0';

  // Reset UI labels
  valAdr.innerText = '0.00';
  valK.innerText = '4.0 mEq/L';
  valCa.innerText = '100%';
});

// Loop Principal
let lastTime = 0;
let wasBeeping = false;

function loop(time: number) {
  if (!lastTime) lastTime = time;
  const deltaMs = Math.min(time - lastTime, 30); // Trava contra travamentos da tab
  lastTime = time;

  // Passos de integração Euler (0.5ms para estabilidade numérica)
  const dt = 0.5;
  const steps = Math.floor(deltaMs / dt);

  for (let i = 0; i < steps; i++) {
    const { v, ecg } = model.step(dt);
    
    // Atualiza o gráfico 1x por equivalente de milissegundo real (aproximadamente 1 pixel = 1ms)
    if (i % 2 === 0) { 
      renderer.pushData(v, ecg, selectedLead);
    }

    // Gatilho sonoro (garante que toque apenas 1 vez por QRS)
    if (model.isBeeping && !wasBeeping) {
      audio.playQRSBeep();
    }
    wasBeeping = model.isBeeping;
  }

  renderer.draw();
  requestAnimationFrame(loop);
}
