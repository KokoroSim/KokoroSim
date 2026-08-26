export class AudioSystem {
    private ctx: AudioContext | null = null;
    private enabled = false;

    constructor() {
        // AudioContext is created on first user interaction to bypass autoplay restrictions
    }

    public async init() {
        if (!this.ctx) {
            this.ctx = new (window.AudioContext || (window as any).webkitAudioContext)();
        }
        if (this.ctx.state === 'suspended') {
            await this.ctx.resume();
        }
        this.enabled = true;
    }

    public toggle(state: boolean) {
        this.enabled = state;
        if (state && this.ctx?.state === 'suspended') {
            this.ctx.resume();
        }
    }

    // Classic ECG Monitor Bip
    public playBip(pitchOffset: number = 0) {
        if (!this.enabled || !this.ctx) return;
        const osc = this.ctx.createOscillator();
        const gain = this.ctx.createGain();
        
        osc.type = 'sine';
        osc.frequency.setValueAtTime(800 + pitchOffset, this.ctx.currentTime); // 800Hz classic bip
        
        gain.gain.setValueAtTime(0, this.ctx.currentTime);
        gain.gain.linearRampToValueAtTime(0.5, this.ctx.currentTime + 0.01);
        gain.gain.exponentialRampToValueAtTime(0.01, this.ctx.currentTime + 0.1);
        
        osc.connect(gain);
        gain.connect(this.ctx.destination);
        
        osc.start();
        osc.stop(this.ctx.currentTime + 0.1);
    }

    // B1 - "Tum" (Fechamento das Válvulas AV - Início da Sístole)
    public playB1() {
        if (!this.enabled || !this.ctx) return;
        this.playLowThud(60, 0.15, 0.8);
    }

    // B2 - "Tá" (Fechamento das Válvulas Semilunares - Fim da Sístole)
    public playB2() {
        if (!this.enabled || !this.ctx) return;
        this.playLowThud(90, 0.10, 0.6); // Slightly higher pitch, shorter, softer
    }

    private playLowThud(freq: number, duration: number, vol: number) {
        const osc = this.ctx!.createOscillator();
        const gain = this.ctx!.createGain();
        
        // Heart sound is a low frequency thump
        osc.type = 'sine';
        osc.frequency.setValueAtTime(freq, this.ctx!.currentTime);
        osc.frequency.exponentialRampToValueAtTime(30, this.ctx!.currentTime + duration); // Pitch drop
        
        gain.gain.setValueAtTime(0, this.ctx!.currentTime);
        gain.gain.linearRampToValueAtTime(vol, this.ctx!.currentTime + 0.02);
        gain.gain.exponentialRampToValueAtTime(0.01, this.ctx!.currentTime + duration);
        
        osc.connect(gain);
        gain.connect(this.ctx!.destination);
        
        osc.start();
        osc.stop(this.ctx!.currentTime + duration);
    }
}

export const audioSystem = new AudioSystem();
