export interface ControlItem {
    type: 'slider' | 'checkbox' | 'select';
    id: string;
    label: string;
    min?: number;
    max?: number;
    value?: number | string;
    step?: number;
    unit?: string;
    checked?: boolean;
    options?: { value: string, text: string }[];
    help?: string;
}

export interface ControlGroup {
    id: string;
    title: string;
    items: ControlItem[];
}

export const controlsData: ControlGroup[] = [
    {
        id: "group-visual",
        title: "0. Visualização",
        items: [
            { type: "select", id: "view-mode", options: [
                { value: "continuous", text: "Varredura Contínua" },
                { value: "single", text: "Gatilho Único" },
                { value: "paged", text: "Paginação Longa" }
            ]},
            { type: "checkbox", id: "show-sa", label: "Nó SA (Gatilho)", checked: true },
            { type: "checkbox", id: "show-av", label: "Nó AV (Condução)", checked: true },
            { type: "checkbox", id: "show-vent", label: "Ventrículo (Motor)", checked: true },
            { type: "checkbox", id: "ghost-toggle", label: "Onda Fantasma (Histórico)", checked: true }
        ]
    },
    {
        id: "group-ions",
        title: "1. Íons e Eletrólitos",
        items: [
            { type: "slider", id: "sl-k", label: "Potássio [K+]_o", min: 1.5, max: 9.0, value: 5.4, step: 0.1, unit: " mEq/L", help: "O impacto recai sobre o Potencial de Equilíbrio de Nernst e afeta a força motriz de todas as correntes iônicas. Mínimo (1.5) é letal, máximo (9.0) causa parada em diástole." },
            { type: "slider", id: "sl-ca", label: "Cálcio [Ca2+]_o", min: 0.5, max: 4.0, value: 2.0, step: 0.1, unit: " mmol/L", help: "Afeta a força motriz das correntes de cálcio. Hipocalcemia severa (0.5) até hipercalcemia grave (4.0)." },
            { type: "slider", id: "sl-na", label: "Sódio [Na+]_o", min: 110, max: 160, value: 140, step: 1.0, unit: " mEq/L", help: "Afeta a força motriz das correntes de sódio. Hiponatremia grave (110) até hipernatremia (160)." }
        ]
    },
    {
        id: "group-ans",
        title: "2. Sistema Nervoso Autônomo",
        items: [
            { type: "slider", id: "sl-symp", label: "Tônus Simpático", min: 0, max: 100, value: 0, step: 1, unit: "%", help: "Aumenta a permeabilidade dos canais I_f (funny), I_CaL (cálcio lento) e a velocidade da bomba SERCA." },
            { type: "slider", id: "sl-parasymp", label: "Tônus Parassimpático", min: 0, max: 100, value: 0, step: 1, unit: "%", help: "Ativa imediatamente a corrente I_K,ACh (hiperpolarizando as células nodais) e reduz o I_CaL." }
        ]
    },
    {
        id: "group-drugs",
        title: "3. Fármacos Antiarrítmicos",
        items: [
            { type: "slider", id: "sl-lido", label: "Bloq. Na+ (Lidocaína)", min: 0, max: 100, value: 0, step: 1, unit: "%", help: "Classe I: Alargamento progressivo do complexo QRS; redução violenta do dV/dt_max da Fase 0." },
            { type: "slider", id: "sl-amio", label: "Bloq. K+ (Amiodarona)", min: 0, max: 100, value: 0, step: 1, unit: "%", help: "Classe III: Atraso na repolarização (Fase 3); prolongamento agudo do intervalo QT." },
            { type: "slider", id: "sl-vera", label: "Bloq. Ca2+ (Verapamil)", min: 0, max: 100, value: 0, step: 1, unit: "%", help: "Classe IV: Achatamento do platô ventricular e Bloqueio Atrioventricular." },
            { type: "slider", id: "sl-digo", label: "Inibidor Na+/K+ (Digoxina)", min: 0, max: 100, value: 0, step: 1, unit: "%", help: "Reduz a força da corrente I_NaK. Causa acúmulo intracelular de sódio e cálcio." }
        ]
    },
    {
        id: "group-patho",
        title: "4. Condições Patológicas",
        items: [
            { type: "slider", id: "sl-isch", label: "Nível de Isquemia", min: 0, max: 100, value: 0, step: 1, unit: "%", help: "Falta de ATP induz a abertura dos canais I_K,ATP. Aborta o platô precocemente e causa Supra de ST." }
        ]
    }
];
