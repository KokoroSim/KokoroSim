export function initCanvas() {
    const canvases = ['canvas-pa', 'canvas-ecg', 'canvas-ch3', 'canvas-ch4'].map(id => document.getElementById(id) as HTMLCanvasElement);
    const contexts = canvases.map(c => c?.getContext('2d'));
    
    if (contexts.some(ctx => !ctx)) return;

    const resizeCanvases = () => {
        canvases.forEach(canvas => {
            const parent = canvas.parentElement;
            if (parent) {
                canvas.width = parent.clientWidth * 2;
                canvas.height = parent.clientHeight * 2;
            }
        });
    };

    window.addEventListener('resize', resizeCanvases);

    let x = 0;
    const speedX = 2; // Slower so wave is shorter visually
    const beatPeriod = 300; // Shorter beat period so more waves fit
    let lastBeatCount = 0;
    let lastYs: Record<string, number> = {};

    let dataBuffer: any[] = [];
    
    // Injetado pelo main.ts
    (window as any).pushData = function(batch: any[]) {
        dataBuffer.push(...batch);
    };
    
    function drawMockup() {
        if (canvases[0].width === 0) resizeCanvases();

        // Só desenha se houver dados do Worker no buffer
        if (dataBuffer.length < speedX) {
            requestAnimationFrame(drawMockup);
            return;
        }

        // Puxa o ponto mais recente do buffer para garantir sincronia em TEMPO REAL
        const sig = dataBuffer[dataBuffer.length - 1]; 
        const t = sig.t;
        dataBuffer.length = 0; // Esvazia o buffer consumindo todos os pacotes pendentes

        const viewMode = (document.getElementById('view-mode') as HTMLSelectElement)?.value || 'continuous';
        const currentBeatCount = Math.floor(t / beatPeriod);
        let triggered = false;

        if (viewMode === 'single') {
            if (currentBeatCount > lastBeatCount) {
                x = 0;
                lastBeatCount = currentBeatCount;
                triggered = true;
            }
        }

        if (viewMode === 'continuous') {
            contexts.forEach((ctx, i) => {
                ctx!.fillStyle = '#000';
                ctx!.fillRect(x + speedX, 0, speedX * 10, canvases[i].height);
                if (x === 0) ctx!.fillRect(0, 0, speedX * 10, canvases[i].height);
            });
        } else if (viewMode === 'single' && triggered) {
            contexts.forEach((ctx, i) => {
                ctx!.fillStyle = '#000';
                ctx!.fillRect(0, 0, canvases[i].width, canvases[i].height);
            });
        } else if (viewMode === 'paged' && x === 0) {
            contexts.forEach((ctx, i) => {
                ctx!.fillStyle = '#000';
                ctx!.fillRect(0, 0, canvases[i].width, canvases[i].height);
            });
        }

        if (x < canvases[0].width) {
            // Checkboxes
            const showSA = (document.getElementById('show-sa') as HTMLInputElement)?.checked;
            const showAV = (document.getElementById('show-av') as HTMLInputElement)?.checked;
            const showVent = (document.getElementById('show-vent') as HTMLInputElement)?.checked;
            const ghost = (document.getElementById('ghost-toggle') as HTMLInputElement)?.checked;

            const drawPoint = (ctx: CanvasRenderingContext2D, yVal: number, color: string, id: string) => {
                const y = ctx.canvas.height - ((yVal + 100) / 150) * ctx.canvas.height;

                if (x === 0) lastYs[id] = y;
                const lastY = lastYs[id] !== undefined ? lastYs[id] : y;
                
                ctx.beginPath();
                ctx.moveTo(x - speedX, lastY);
                ctx.lineTo(x, y);
                ctx.strokeStyle = color;
                ctx.lineWidth = 1.0;
                ctx.lineCap = 'round';
                ctx.lineJoin = 'round';
                ctx.stroke();

                if (ghost) {
                    ctx.beginPath();
                    ctx.moveTo(x - speedX * 20 - speedX, lastY);
                    ctx.lineTo(x - speedX * 20, y);
                    ctx.strokeStyle = `rgba(${color === '#e74c3c' ? '231,76,60' : color === '#f1c40f' ? '241,196,15' : '46,204,113'}, 0.2)`;
                    ctx.lineWidth = 1.0;
                    ctx.lineCap = 'round';
                    ctx.lineJoin = 'round';
                    ctx.stroke();
                }
                lastYs[id] = y;
            };

            // Linha 1: PA (Sobrepostos)
            if (showSA) drawPoint(contexts[0]!, sig.sa, '#e74c3c', 'sa'); // Red
            if (showAV) drawPoint(contexts[0]!, sig.av, '#f1c40f', 'av'); // Yellow
            if (showVent) drawPoint(contexts[0]!, sig.vent, '#2ecc71', 'vent'); // Green

            // Linha 2: ECG
            const ecgY = (canvases[1].height / 2) - sig.ecg;
            if (x === 0) lastYs['ecg'] = ecgY;
            const lastEcgY = lastYs['ecg'] !== undefined ? lastYs['ecg'] : ecgY;
            
            contexts[1]!.beginPath();
            contexts[1]!.moveTo(x - speedX, lastEcgY);
            contexts[1]!.lineTo(x, ecgY);
            contexts[1]!.strokeStyle = '#00ffff';
            contexts[1]!.lineWidth = 1.0;
            contexts[1]!.lineCap = 'round';
            contexts[1]!.lineJoin = 'round';
            contexts[1]!.stroke();
            lastYs['ecg'] = ecgY;

            // Linha 3 & 4 (Mock lines with arbitrary signals from vent phase)
            const sig3 = Math.sin(t * 0.05) * 20;
            const sig4 = Math.cos(t * 0.02) * 10;
            drawPoint(contexts[2]!, sig3, '#9b59b6', 'ch3'); // Purple
            drawPoint(contexts[3]!, sig4, '#e67e22', 'ch4'); // Orange

            x += speedX;
        }

        if (x >= canvases[0].width && viewMode !== 'single') x = 0;

        requestAnimationFrame(drawMockup);
    }

    drawMockup();
}

export function pushData(batch: any[]) {
    if ((window as any).pushData) {
        (window as any).pushData(batch);
    }
}
