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
    const speedX = 1.5; // Comprime horizontalmente para caber ~3 ciclos na tela
    const beatPeriod = 300; // Shorter beat period so more waves fit
    let lastBeatCount = 0;
    let lastYs: Record<string, number> = {};

    let dataBuffer: any[] = [];
    
    // Injetado pelo main.ts
    // Injetado pelo main.ts
    (window as any).pushData = function(batch: any[]) {
        dataBuffer.push(...batch);
    };

    (window as any).clearData = function() {
        dataBuffer = [];
        x = 0;
        contexts.forEach(ctx => {
            if (!ctx) return;
            ctx.fillStyle = '#000';
            ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);
            ctx.beginPath();
        });
    };
    
    function drawMockup() {
        if (canvases[0].width === 0) resizeCanvases();

        if (dataBuffer.length === 0) {
            requestAnimationFrame(drawMockup);
            return;
        }

        const viewMode = (document.getElementById('view-mode') as HTMLSelectElement)?.value || 'continuous';
        const showSA = (document.getElementById('show-sa') as HTMLInputElement)?.checked;
        const showAtr = (document.getElementById('show-atr') as HTMLInputElement)?.checked;
        const showAV = (document.getElementById('show-av') as HTMLInputElement)?.checked;
        const showVent = (document.getElementById('show-vent') as HTMLInputElement)?.checked;
        const ghost = (document.getElementById('ghost-toggle') as HTMLInputElement)?.checked;

        const localBuffer = [...dataBuffer];
        dataBuffer.length = 0; 
        
        // speedX base: 1.5 pixels per frame (16ms)
        // We now have packets every 0.1ms. So each packet represents 0.1/16 of a frame.
        const speedXPerSample = speedX * (0.1 / 16.0);

        // Pre-calculate Total X Advance to clear the screen ahead
        let totalXAdvance = speedXPerSample * localBuffer.length;

        // Process triggers for "single" mode
        let triggered = false;
        if (viewMode === 'single') {
            for (const sig of localBuffer) {
                const currentBeatCount = Math.floor(sig.t / beatPeriod);
                if (currentBeatCount > lastBeatCount) {
                    lastBeatCount = currentBeatCount;
                    triggered = true;
                    x = 0; // Trigger reset
                }
            }
        }

        // Handle Background Wiping
        if (viewMode === 'continuous') {
            contexts.forEach((ctx, i) => {
                ctx!.fillStyle = '#000';
                // Apaga um bloco à frente da caneta (largura dependente do avanço + margem)
                ctx!.fillRect(x + 1, 0, totalXAdvance + 10, canvases[i].height);
                if (x === 0) ctx!.fillRect(0, 0, totalXAdvance + 10, canvases[i].height);
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

        // Batch Draw Function
        const drawChannel = (ctx: CanvasRenderingContext2D, dataKey: string, color: string, id: string, isEcg = false, isMock = false, mockFn?: (t:number)=>number) => {
            ctx.beginPath();
            let currentX = x;
            
            // Pega o último Y salvo para começar a linha sem quebra
            let startY = lastYs[id];
            if (startY === undefined) {
                const firstVal = isMock ? mockFn!(localBuffer[0].t) : (localBuffer[0] as any)[dataKey];
                startY = isEcg ? (ctx.canvas.height / 2) - firstVal : ctx.canvas.height - ((firstVal + 100) / 150) * ctx.canvas.height;
            }
            
            ctx.moveTo(currentX, startY);

            for (let i = 0; i < localBuffer.length; i++) {
                const sig = localBuffer[i];
                if (currentX >= ctx.canvas.width && viewMode !== 'single') {
                    // Wrap-around
                    ctx.stroke(); // Fecha o traço atual
                    if (ghost) {
                        ctx.beginPath();
                        ctx.moveTo(currentX - 20, startY);
                        // logica ghost ignorada no meio do wrap para simplicidade
                    }
                    ctx.beginPath();
                    currentX = 0;
                    
                    const val = isMock ? mockFn!(sig.t) : (sig as any)[dataKey];
                    const rawY = isEcg ? (ctx.canvas.height / 2) - val : ctx.canvas.height - ((val + 100) / 150) * ctx.canvas.height;
                    ctx.moveTo(currentX, rawY);
                }

                currentX += speedXPerSample;
                const val = isMock ? mockFn!(sig.t) : (sig as any)[dataKey];
                const y = isEcg ? (ctx.canvas.height / 2) - val : ctx.canvas.height - ((val + 100) / 150) * ctx.canvas.height;
                
                ctx.lineTo(currentX, y);
                lastYs[id] = y;
            }

            ctx.strokeStyle = color;
            ctx.lineWidth = 1.5;
            ctx.lineCap = 'round';
            ctx.lineJoin = 'round';
            ctx.stroke();
        };

        // Render channels
        if (showSA) drawChannel(contexts[0]!, 'sa', '#e74c3c', 'sa');
        if (showAtr) drawChannel(contexts[0]!, 'atr', '#3498db', 'atr');
        if (showAV) drawChannel(contexts[0]!, 'av', '#f1c40f', 'av');
        if (showVent) drawChannel(contexts[0]!, 'vent', '#2ecc71', 'vent');
        
        drawChannel(contexts[1]!, 'ecg', '#00ffff', 'ecg', true);
        
        const selCh3 = (document.getElementById('sel-ch3') as HTMLSelectElement)?.value || 'corrente_na';
        const selCh4 = (document.getElementById('sel-ch4') as HTMLSelectElement)?.value || 'ca_int';
        
        // As correntes podem ter amplitudes bizarras, então a função drawChannel pode precisar normalizar.
        // A princípio o drawChannel atual apenas subtrai de Y, então se as amplitudes forem gigantes, podem sair da tela.
        // Como o original mockava com sin*20, a gente pode passar um mockFn vazio e usar a string real da chave!
        drawChannel(contexts[2]!, selCh3, '#9b59b6', selCh3, false, false);
        drawChannel(contexts[3]!, selCh4, '#e67e22', selCh4, false, false);

        // Atualiza a posição X global da caneta
        for (let i = 0; i < localBuffer.length; i++) {
            x += speedXPerSample;
            if (x >= canvases[0].width && viewMode !== 'single') x = 0;
        }

        requestAnimationFrame(drawMockup);
    }

    drawMockup();
}

export function pushData(batch: any[]) {
    if ((window as any).pushData) {
        (window as any).pushData(batch);
    }
}

export function clearData() {
    if ((window as any).clearData) {
        (window as any).clearData();
    }
}
