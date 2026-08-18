export class Renderer {
  private ctxAP: CanvasRenderingContext2D;
  private ctxECG: CanvasRenderingContext2D;
  private bufferAP: Float32Array;
  private bufferECG: Float32Array;
  private head: number = 0;
  private width: number;

  constructor(canvasAP: HTMLCanvasElement, canvasECG: HTMLCanvasElement) {
    this.ctxAP = canvasAP.getContext('2d')!;
    this.ctxECG = canvasECG.getContext('2d')!;
    this.width = canvasAP.width;
    
    this.bufferAP = new Float32Array(this.width).fill(-85);
    this.bufferECG = new Float32Array(this.width).fill(0);
  }

  public pushData(v: number, ecg: number, lead: string) {
    this.bufferAP[this.head] = v;
    
    // Projeção do ECG baseada na Derivação selecionada
    let projEcg = ecg;
    if (lead === 'aVR') projEcg = -ecg * 0.8;
    if (lead === 'V1') projEcg = -ecg * 0.5 + (Math.random()*0.1); // V1 é predominantemente negativo

    this.bufferECG[this.head] = projEcg;
    
    this.head = (this.head + 1) % this.width;
  }

  private mapY(val: number, min: number, max: number, h: number): number {
    const norm = Math.max(0, Math.min(1, (val - min) / (max - min)));
    return h - norm * h;
  }

  public draw() {
    this.drawGraph(this.ctxAP, this.bufferAP, -100, 50, '#34d399'); // Verde AP
    this.drawGraph(this.ctxECG, this.bufferECG, -5, 5, '#60a5fa');  // Azul ECG
  }

  private drawGraph(ctx: CanvasRenderingContext2D, buffer: Float32Array, min: number, max: number, color: string) {
    const h = ctx.canvas.height;
    ctx.clearRect(0, 0, this.width, h);

    // Linha de varredura (Sweep bar)
    ctx.fillStyle = 'rgba(255,0,0,0.5)';
    ctx.fillRect(this.head, 0, 3, h);

    ctx.strokeStyle = color;
    ctx.lineWidth = 2;
    ctx.beginPath();

    for (let x = 0; x < this.width; x++) {
      const idx = (this.head + x) % this.width;
      const y = this.mapY(buffer[idx], min, max, h);
      if (x === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.stroke();
  }
}
