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
    let t = 0;
    const speedX = 2; // Slower so wave is shorter visually
    const speedT = 4;
    const beatPeriod = 300; // Shorter beat period so more waves fit
    let lastBeatCount = 0;
    
    function getSignal(time: number) {
        const localT = time % beatPeriod;
        
        // Mock SA
        let sa = -60 + (localT / beatPeriod) * 20; // phase 4
        if (localT > 250) sa = 10 - ((localT - 250) / 50) * 70; // phase 3
        else if (localT > 230) sa = -40 + ((localT - 230) / 20) * 50; // phase 0
        
        // Mock AV (delayed)
        let av = -60;
        const avT = (localT + 20) % beatPeriod; // 20 units behind
        if (avT > 250) av = 10 - ((avT - 250) / 50) * 70;
        else if (avT > 240) av = -60 + ((avT - 240) / 10) * 70;

        // Mock Vent (delayed more)
        let vent = -80;
        const ventT = (localT + 60) % beatPeriod; // 60 units behind SA
        if (ventT > 250) vent = -80;
        else if (ventT > 180) vent = 0 - ((ventT - 180) / 70) * 80;
        else if (ventT > 50) vent = 20 - ((ventT - 50) / 130) * 20;
        else if (ventT > 40) vent = -80 + ((ventT - 40) / 10) * 100;

        // Mock ECG
        let ecg = 0;
        if (localT > 220 && localT < 240) ecg = Math.sin((localT - 220)/20 * Math.PI) * 10; // P
        else if (ventT > 40 && ventT < 50) ecg = Math.sin((ventT - 40)/10 * Math.PI) * 40; // R
        else if (ventT > 160 && ventT < 200) ecg = Math.sin((ventT - 160)/40 * Math.PI) * 15; // T

        return { sa, av, vent, ecg };
    }
    
    function drawMockup() {
        if (canvases[0].width === 0) resizeCanvases();

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
            const sig = getSignal(t);
            
            // Checkboxes
            const showSA = (document.getElementById('show-sa') as HTMLInputElement)?.checked;
            const showAV = (document.getElementById('show-av') as HTMLInputElement)?.checked;
            const showVent = (document.getElementById('show-vent') as HTMLInputElement)?.checked;
            const ghost = (document.getElementById('ghost-toggle') as HTMLInputElement)?.checked;

            const drawPoint = (ctx: CanvasRenderingContext2D, yVal: number, color: string) => {
                const y = ctx.canvas.height - ((yVal + 100) / 150) * ctx.canvas.height;
                ctx.fillStyle = color;
                ctx.fillRect(x, y, speedX, 2);
                if (ghost) {
                    ctx.fillStyle = `rgba(${color === '#e74c3c' ? '231,76,60' : color === '#f1c40f' ? '241,196,15' : '46,204,113'}, 0.2)`;
                    ctx.fillRect(x - speedX * 20, y, speedX, 2);
                }
            };

            // Linha 1: PA (Sobrepostos)
            if (showSA) drawPoint(contexts[0]!, sig.sa, '#e74c3c'); // Red
            if (showAV) drawPoint(contexts[0]!, sig.av, '#f1c40f'); // Yellow
            if (showVent) drawPoint(contexts[0]!, sig.vent, '#2ecc71'); // Green

            // Linha 2: ECG
            const ecgY = (canvases[1].height / 2) - sig.ecg;
            contexts[1]!.fillStyle = '#00ffff';
            contexts[1]!.fillRect(x, ecgY, speedX, 2);

            // Linha 3 & 4 (Mock lines with arbitrary signals from vent phase)
            const sig3 = Math.sin(t * 0.05) * 20;
            const sig4 = Math.cos(t * 0.02) * 10;
            drawPoint(contexts[2]!, sig3, '#9b59b6'); // Purple
            drawPoint(contexts[3]!, sig4, '#e67e22'); // Orange

            x += speedX;
        }

        if (x >= canvases[0].width && viewMode !== 'single') x = 0;
        t += speedT;

        requestAnimationFrame(drawMockup);
    }

    drawMockup();
}
