export class CardiacModel {
  public V: number = -85.0;
  public w: number = 0.0;
  public cellType: 'ventricular' | 'nodal' = 'ventricular';

  // Parâmetros fisiológicos
  public potassium: number = 4.0; // 3.5 a 5.0 é normal
  public adrenaline: number = 0.0;
  public caBlocker: number = 1.0;

  private timer: number = 0;
  public isBeeping: boolean = false; // Gatilho visual/sonoro

  // O pulso "Atrial" que usamos para desenhar a onda P do ECG
  public atrialV: number = 0; 

  public step(dt: number): { v: number, ecg: number } {
    this.timer += dt;

    // 1. Efeitos Iônicos
    // Hipercalemia despolariza o repouso.
    const K_shift = (this.potassium - 4.0) * 8.0; 
    let vRest = this.cellType === 'ventricular' ? -85.0 + K_shift : -60.0;
    
    // 2. Frequência Cardíaca Básica (Adrenalina acelera, Nodal acelera)
    const baseCycleLength = this.cellType === 'nodal' ? 700 : 850;
    const cycleLength = baseCycleLength - (this.adrenaline * 200);

    // 3. Estímulo (Nodal tem automatismo intrínseco, Ventricular precisa de estímulo)
    let I_stim = 0;
    this.atrialV = 0;

    // Gatilho atrial ocorre um pouco antes da despolarização ventricular
    if (this.timer % cycleLength > cycleLength - 80) {
      this.atrialV = Math.sin(((this.timer % cycleLength) - (cycleLength - 80)) * Math.PI / 80);
    }

    if (this.timer >= cycleLength) {
      this.timer = 0;
      if (this.cellType === 'ventricular') I_stim = 30.0; // Gatilho de Purkinje
    }

    // Nodal tem despolarização diastólica fase 4 (I_f) aumentada por adrenalina
    if (this.cellType === 'nodal' && this.V < -40.0) {
      I_stim += 0.1 + (this.adrenaline * 0.2); 
    }

    // 4. Dinâmica do Potencial (Aliev-Panfilov modificado)
    const vPeak = 30.0;
    const vNorm = Math.max(0, Math.min(1, (this.V - vRest) / (vPeak - vRest)));
    const threshold = this.cellType === 'ventricular' ? 0.2 : 0.4;
    
    // Corrente Rápida (Fase 0)
    let f_v = 4.0 * vNorm * (1.0 - vNorm) * (vNorm - threshold);
    if (this.cellType === 'nodal') f_v *= 0.2; // Nó SA não tem canais rápidos de Na+

    // Corrente de Repolarização (Fase 3) / Platô (Fase 2)
    // Cálcio mantém o platô. Se caBlocker < 1.0, o platô cai rápido.
    const k_repol = 0.8 * (this.potassium / 4.0); 
    const k_plateau = 0.03 * this.caBlocker * (1.0 + this.adrenaline * 0.5);

    const dV = (f_v * 100.0 - k_repol * this.w * (vNorm + 0.1) + I_stim);
    const dw = k_plateau * (vNorm - 0.7 * this.w);

    this.V += dV * dt;
    this.w += dw * dt;

    if (this.V < vRest - 5) this.V = vRest - 5; // Trava inferior

    // 5. Síntese do Vetor de ECG
    // O ECG depende da derivada dV/dt (Fase 0 = QRS, Fase 3 = Onda T)
    const dVdt = dV;
    const isQRS = dVdt > 10.0;
    const isTWave = this.w > 0.1 && dVdt < -0.1;
    
    this.isBeeping = isQRS; // Sinaliza para o Audio tocar

    // Mistura Atrial (P) + Ventricular Rápido (QRS) + Repolarização (T)
    const ecgRaw = (this.atrialV * 1.5) + (isQRS ? dVdt * 0.1 : 0) + (isTWave ? -dVdt * 0.5 : 0);

    return { v: this.V, ecg: ecgRaw };
  }
}
