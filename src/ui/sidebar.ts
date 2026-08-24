import { controlsData } from '../config/controls';

export function initSidebar() {
    const sidebar = document.getElementById('controls-sidebar');
    if (!sidebar) return;

    // Container para os botões do topo
    const topButtonsDiv = document.createElement('div');
    topButtonsDiv.style.cssText = 'display: flex; gap: 1vw; margin-bottom: 2vh;';

    // Global Reset Button
    const resetAllBtn = document.createElement('button');
    resetAllBtn.className = 'btn-reset-all';
    resetAllBtn.textContent = '☢ RESETAR SIMULAÇÃO ☢';
    resetAllBtn.style.cssText = 'flex: 2; background: #e74c3c; color: #fff; border: none; padding: 1.5vh; font-weight: bold; cursor: pointer; border-radius: 0.5vh; font-size: 1.8vh; transition: 0.2s; box-shadow: 0 0 10px rgba(231, 76, 60, 0.5); display: block;';
    resetAllBtn.onmouseover = () => resetAllBtn.style.background = '#c0392b';
    resetAllBtn.onmouseout = () => resetAllBtn.style.background = '#e74c3c';
    resetAllBtn.addEventListener('click', () => {
        // Clica virtualmente em todos os resets individuais
        document.querySelectorAll('.btn-reset').forEach((btn: any) => btn.click());
        
        // Pede pro motor limpar a memória e resetar o relógio
        if ((window as any).worker) {
            (window as any).worker.postMessage({ type: 'RESET' });
        }
    });

    // About Button
    const aboutBtn = document.createElement('button');
    aboutBtn.textContent = 'ℹ SOBRE';
    aboutBtn.style.cssText = 'flex: 1; background: #3498db; color: #fff; border: none; padding: 1.5vh; font-weight: bold; cursor: pointer; border-radius: 0.5vh; font-size: 1.8vh; transition: 0.2s;';
    aboutBtn.onmouseover = () => aboutBtn.style.background = '#2980b9';
    aboutBtn.onmouseout = () => aboutBtn.style.background = '#3498db';
    aboutBtn.addEventListener('click', async () => {
        const modal = document.getElementById('about-modal');
        const content = document.getElementById('about-content');
        if (modal && content) {
            modal.style.display = 'flex';
            try {
                const response = await fetch('./README.md');
                if (!response.ok) throw new Error('Não foi possível carregar o README.');
                const text = await response.text();
                // Utiliza a lib marked injetada via CDN no index.html
                content.innerHTML = (window as any).marked ? (window as any).marked.parse(text) : `<pre style="white-space: pre-wrap; font-family: monospace;">${text}</pre>`;
            } catch (err) {
                content.innerHTML = 'Erro ao carregar o conteúdo do Sobre.';
            }
        }
    });
    
    // Configurar o botão de fechar do modal
    const closeAboutBtn = document.getElementById('close-about');
    if (closeAboutBtn) {
        closeAboutBtn.addEventListener('click', () => {
            const modal = document.getElementById('about-modal');
            if (modal) modal.style.display = 'none';
        });
    }

    topButtonsDiv.appendChild(resetAllBtn);
    topButtonsDiv.appendChild(aboutBtn);
    
    // Insere no topo
    sidebar.insertBefore(topButtonsDiv, sidebar.firstChild);

    controlsData.forEach((group, index) => {
        const groupDiv = document.createElement('div');
        groupDiv.className = 'control-group';
        
        const header = document.createElement('h3');
        header.innerHTML = group.title + ' <span class="toggle-icon">' + (index === 0 ? '▲' : '▼') + '</span>';
        header.className = 'accordion-header';
        
        const content = document.createElement('div');
        content.className = 'accordion-content';
        if (index === 0) content.classList.add('active');
        
        header.addEventListener('click', () => {
            const isActive = content.classList.contains('active');
            
            // Close all other accordions
            document.querySelectorAll('.accordion-content').forEach(el => el.classList.remove('active'));
            document.querySelectorAll('.accordion-header .toggle-icon').forEach(el => el.textContent = '▼');

            // Toggle current
            if (!isActive) {
                content.classList.add('active');
                header.querySelector('.toggle-icon')!.textContent = '▲';
            }
        });
        
        group.items.forEach(item => {
            const container = document.createElement('div');
            container.className = 'slider-container';
            
            if (item.type === 'slider') {
                container.innerHTML = `
                    <div class="slider-header">
                        <span>${item.label}</span>
                        <span id="val-${item.id}">${item.value}${item.unit}</span>
                    </div>
                    <input type="range" id="${item.id}" min="${item.min}" max="${item.max}" step="${item.step}" value="${item.value}">
                    <div class="slider-actions">
                        <button class="icon-btn btn-reset" data-id="${item.id}" title="Resetar">↺ Reset</button>
                        ${item.help ? `<button class="icon-btn btn-help" data-id="${item.id}" title="Ajuda">? Ajuda</button>` : ''}
                    </div>
                    ${item.help ? `<div class="help-text" id="help-${item.id}">${item.help}</div>` : ''}
                `;
            } else if (item.type === 'checkbox') {
                container.innerHTML = `
                    <label style="font-size: 1.8vh; display: flex; gap: 1vw;">
                        <input type="checkbox" id="${item.id}" ${item.checked ? 'checked' : ''}> ${item.label}
                    </label>
                `;
            } else if (item.type === 'select') {
                let options = item.options?.map(opt => `<option value="${opt.value}">${opt.text}</option>`).join('') || '';
                container.innerHTML = `
                    <select id="${item.id}" style="width: 100%; background: #000; color: #fff; padding: 0.5vh; border: 1px solid var(--glass-border); font-size: 1.8vh;">
                        ${options}
                    </select>
                `;
            }
            content.appendChild(container);
        });
        
        groupDiv.appendChild(header);
        groupDiv.appendChild(content);
        sidebar.appendChild(groupDiv);
    });

    bindControls();
}

function updateSliderColor(slider: HTMLInputElement, min: number, max: number, defaultVal: number) {
    const value = parseFloat(slider.value);
    const percentage = ((value - min) / (max - min)) * 100;
    slider.style.setProperty('--val', `${percentage}%`);
    
    // Calcula a severidade baseada na distância do valor padrão
    const maxDiff = Math.max(Math.abs(max - defaultVal), Math.abs(min - defaultVal));
    const diff = Math.abs(value - defaultVal);
    const severity = maxDiff === 0 ? 0 : diff / maxDiff; // 0 (padrão) a 1 (desvio máximo)
    
    // Transição de Verde (120) para Vermelho (0) com base na severidade
    const hue = 120 - (severity * 120);
    slider.style.setProperty('--track-color', `hsl(${hue}, 100%, 50%)`);
}

function bindControls() {
    controlsData.forEach(group => {
        group.items.forEach(item => {
            if (item.type === 'slider') {
                const slider = document.getElementById(item.id) as HTMLInputElement;
                const val = document.getElementById(`val-${item.id}`);
                const min = item.min || 0;
                const max = item.max || 100;
                const defaultVal = Number(item.value) || 0;
                
                // Envia para o Worker inicialmente
                if ((window as any).worker) {
                    (window as any).worker.postMessage({
                        type: 'UPDATE_PARAMS',
                        payload: { [item.id]: defaultVal }
                    });
                }

                // Initialize color
                updateSliderColor(slider, min, max, defaultVal);

                slider.addEventListener('input', (e) => {
                    const target = e.target as HTMLInputElement;
                    if(val) val.textContent = `${target.value}${item.unit}`;
                    updateSliderColor(target, min, max, defaultVal);
                    
                    if ((window as any).worker) {
                        (window as any).worker.postMessage({
                            type: 'UPDATE_PARAMS',
                            payload: { [item.id]: parseFloat(target.value) }
                        });
                    }
                });

                // Reset button
                const btnReset = document.querySelector(`.btn-reset[data-id="${item.id}"]`);
                if (btnReset) {
                    btnReset.addEventListener('click', () => {
                        slider.value = String(item.value);
                        if(val) val.textContent = `${item.value}${item.unit}`;
                        updateSliderColor(slider, min, max, defaultVal);
                        
                        if ((window as any).worker) {
                            (window as any).worker.postMessage({
                                type: 'UPDATE_PARAMS',
                                payload: { [item.id]: defaultVal }
                            });
                        }
                    });
                }

                // Help button
                const btnHelp = document.querySelector(`.btn-help[data-id="${item.id}"]`);
                if (btnHelp) {
                    btnHelp.addEventListener('click', () => {
                        const helpText = document.getElementById(`help-${item.id}`);
                        if (helpText) {
                            helpText.classList.toggle('show');
                        }
                    });
                }
            }
        });
    });

    // Global click listener to close help texts
    document.addEventListener('click', (e) => {
        const target = e.target as HTMLElement;
        if (!target.classList.contains('btn-help') && !target.closest('.help-text')) {
            document.querySelectorAll('.help-text').forEach(el => el.classList.remove('show'));
        }
    });
}
