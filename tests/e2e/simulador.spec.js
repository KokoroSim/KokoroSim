// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('KokoroSim — Testes E2E do Simulador Web (Wasm + Dioxus)', () => {
  let jsErrors = [];

  test.beforeEach(async ({ page }) => {
    jsErrors = [];
    page.on('pageerror', (exception) => {
      jsErrors.push(`[PAGE ERROR] ${exception.message}`);
    });
    page.on('console', (msg) => {
      if (msg.type() === 'error') {
        jsErrors.push(`[CONSOLE ERROR] ${msg.text()}`);
      }
    });

    await page.goto('/app.html');
    await expect(page.locator('#app-grid')).toBeVisible({ timeout: 15000 });
  });

  test('deve inicializar o Wasm e exibir telemetria do HUD estabilizada (~76 BPM)', async ({ page }) => {
    await page.waitForTimeout(2500);

    const bpmLocator = page.locator('#hud-bpm');
    await expect(bpmLocator).toBeVisible();

    const bpmText = await bpmLocator.innerText();
    const bpmNum = parseFloat(bpmText.replace(' BPM', '').trim());

    // Frequência fisiológica sinusal sem salto inicial
    expect(bpmNum).toBeGreaterThanOrEqual(72.0);
    expect(bpmNum).toBeLessThanOrEqual(82.0);

    // Valida pods de telemetria
    await expect(page.locator('#hud-pr')).toContainText('ms');
    await expect(page.locator('#hud-qrs')).toContainText('ms');
    await expect(page.locator('#hud-qt')).toContainText('ms');
    await expect(page.locator('#hud-vrest')).toContainText('mV');
    await expect(page.locator('#hud-ef')).toContainText('%');
    await expect(page.locator('#hud-sv')).toContainText('mL');
    await expect(page.locator('#hud-co')).toContainText('L/min');

    expect(jsErrors).toHaveLength(0);
  });

  test('deve renderizar os canais de osciloscópio e a alça P x V (Canvases ativos)', async ({ page }) => {
    await page.waitForTimeout(1500);

    const canvasIds = ['#canvas-pa', '#canvas-ecg', '#canvas-ch3', '#canvas-ch4', '#canvas-pv'];
    for (const id of canvasIds) {
      const canvas = page.locator(id);
      await expect(canvas).toBeVisible();
      await canvas.scrollIntoViewIfNeeded();
      const box = await canvas.boundingBox();
      expect(box).not.toBeNull();
      expect(box.width).toBeGreaterThan(50);
      expect(box.height).toBeGreaterThan(30);
    }

    const totalCanvases = page.locator('.canvas-wrapper canvas');
    await expect(totalCanvases).toHaveCount(5);

    expect(jsErrors).toHaveLength(0);
  });

  test('deve controlar o congelamento e retomada da simulação (Pausa)', async ({ page }) => {
    const pauseBtn = page.locator('.btn-pause, .btn-resume');
    await expect(pauseBtn).toBeVisible();
    await expect(pauseBtn).toContainText('CONGELAR');

    // Clica para pausar/congelar
    await pauseBtn.click();
    await expect(pauseBtn).toContainText('CONTINUAR');
    await expect(pauseBtn).toHaveClass(/btn-resume/);

    // Clica para continuar
    await pauseBtn.click();
    await expect(pauseBtn).toContainText('CONGELAR');
    await expect(pauseBtn).toHaveClass(/btn-pause/);

    expect(jsErrors).toHaveLength(0);
  });

  test('deve capturar a Onda Fantasma bio-sincronizada', async ({ page }) => {
    // Abre a sanfona de Osciloscópio & Fantasma
    const visAccordion = page.locator('summary', { hasText: /Osciloscópio/ });
    await visAccordion.click();

    const ghostCaptureBtn = page.locator('button', { hasText: /Capturar/ });
    await expect(ghostCaptureBtn).toBeVisible();

    // Dispara a captura da onda fantasma
    await ghostCaptureBtn.click();
    await page.waitForTimeout(2000); // Aguarda o ciclo de gravação do Nó SA

    // O checkbox da onda fantasma deve estar ativado
    const ghostCheckbox = page.locator('.slider-container', { hasText: /Onda Fantasma/ }).locator('input[type="checkbox"]');
    await expect(ghostCheckbox).toBeChecked();

    expect(jsErrors).toHaveLength(0);
  });

  test('deve alternar para modo Single-Shot e exibir botão de disparo único', async ({ page }) => {
    // Abre a sanfona de Osciloscópio & Fantasma
    const visAccordion = page.locator('summary', { hasText: /Osciloscópio/ });
    await visAccordion.click();

    const modeSelect = page.locator('select').first();
    await expect(modeSelect).toBeVisible();

    // Seleciona modo Single-Shot pelo valor do option
    await modeSelect.selectOption('triggered_single');

    // Botão de armar disparo deve aparecer
    const singleShotBtn = page.locator('button', { hasText: /DISPARAR \/ ARMAR/ });
    await expect(singleShotBtn).toBeVisible();
    await singleShotBtn.click();

    // Retorna para o modo padrão
    await modeSelect.selectOption('rolling');
    await expect(singleShotBtn).not.toBeVisible();

    expect(jsErrors).toHaveLength(0);
  });

  test('deve resetar parâmetros alterados com o botão Resetar', async ({ page }) => {
    const resetBtn = page.locator('.btn-reset-all');
    await expect(resetBtn).toBeVisible();

    await resetBtn.click();
    await page.waitForTimeout(1000);

    const bpmLocator = page.locator('#hud-bpm');
    await expect(bpmLocator).toBeVisible();

    expect(jsErrors).toHaveLength(0);
  });

  test('deve exibir padrão limpo de 3 camadas ativas no gráfico de PA e legenda humanizada na hemodinâmica', async ({ page }) => {
    // Abre a sanfona de Camadas Ativas
    const layersAccordion = page.locator('summary', { hasText: /Camadas Ativas/ });
    await layersAccordion.click();

    // 1. Valida checkboxes das camadas celulares no painel
    const saCheckbox = page.locator('.slider-container', { hasText: /Nó SA/ }).locator('input[type="checkbox"]');
    const atriumCheckbox = page.locator('.slider-container', { hasText: /Átrio/ }).locator('input[type="checkbox"]');
    const epiCheckbox = page.locator('.slider-container', { hasText: /Epicárdio/ }).locator('input[type="checkbox"]');
    const avCheckbox = page.locator('.slider-container', { hasText: /Nó AV/ }).locator('input[type="checkbox"]');
    const purkCheckbox = page.locator('.slider-container', { hasText: /Purkinje/ }).locator('input[type="checkbox"]');
    const endoCheckbox = page.locator('.slider-container', { hasText: /Endocárdio/ }).locator('input[type="checkbox"]');
    const fibCheckbox = page.locator('.slider-container', { hasText: /Fibroblasto/ }).locator('input[type="checkbox"]');

    // Apenas 3 ativas por padrão para visual limpo
    await expect(saCheckbox).toBeChecked();
    await expect(atriumCheckbox).toBeChecked();
    await expect(epiCheckbox).toBeChecked();

    // As demais desmarcadas por padrão
    await expect(avCheckbox).not.toBeChecked();
    await expect(purkCheckbox).not.toBeChecked();
    await expect(endoCheckbox).not.toBeChecked();
    await expect(fibCheckbox).not.toBeChecked();

    // 2. Valida rótulo do CH-04 (Hemodinâmica): humano e sem hex cru
    const hemoLabel = page.locator('.canvas-wrapper', { has: page.locator('#canvas-ch4') }).locator('.canvas-label');
    await expect(hemoLabel).toBeVisible();
    const labelText = (await hemoLabel.innerText()).toUpperCase();
    expect(labelText).toContain('LVP');
    expect(labelText).toContain('AOP');
    expect(labelText).not.toContain('#00F2FE');
    expect(labelText).not.toContain('#FF1754');

    expect(jsErrors).toHaveLength(0);
  });

  test('deve iniciar com todas as sanfonas fechadas e permitir apenas uma aberta por vez', async ({ page }) => {
    const accordions = page.locator('details.control-group');
    await expect(accordions).toHaveCount(10);

    // 1. Todas iniciam fechadas
    for (let i = 0; i < 10; i++) {
      await expect(accordions.nth(i)).not.toHaveAttribute('open');
    }

    // 2. Abre a primeira sanfona (Visualização)
    const summary0 = accordions.nth(0).locator('summary');
    await summary0.click();
    await expect(accordions.nth(0)).toHaveAttribute('open');

    // 3. Abre a segunda sanfona (Íons e Eletrólitos) -> a primeira deve fechar automaticamente
    const summary1 = accordions.nth(1).locator('summary');
    await summary1.click();
    await expect(accordions.nth(1)).toHaveAttribute('open');
    await expect(accordions.nth(0)).not.toHaveAttribute('open');

    // 4. Clica novamente na segunda sanfona -> ela deve fechar
    await summary1.click();
    await expect(accordions.nth(1)).not.toHaveAttribute('open');

    expect(jsErrors).toHaveLength(0);
  });

  test('deve manipular os controles de valvopatias e manter estabilidade hemodinâmica', async ({ page }) => {
    // 1. Abre a sanfona de Valvopatias (Sanfona 8)
    const valvoAccordion = page.locator('details.control-group', { hasText: 'Valvopatias' });
    await expect(valvoAccordion).toBeVisible();
    await valvoAccordion.locator('summary').click();
    await expect(valvoAccordion).toHaveAttribute('open');

    // 2. Valida a presença dos 4 sliders de valvopatias
    const sliders = valvoAccordion.locator('input[type="range"]');
    await expect(sliders).toHaveCount(4);

    // 3. Altera a Estenose Aórtica para 80%
    const eaSlider = sliders.nth(0);
    await eaSlider.fill('80');
    await page.waitForTimeout(1500);

    // 4. Valida que a telemetria do HUD permanece responsiva e estável
    const efLocator = page.locator('#hud-ef');
    await expect(efLocator).toBeVisible();
    await expect(efLocator).toContainText('%');

    const coLocator = page.locator('#hud-co');
    await expect(coLocator).toBeVisible();
    await expect(coLocator).toContainText('L/min');

    // 5. Canvas da alça P x V continua ativo
    const pvCanvas = page.locator('#canvas-pv');
    await expect(pvCanvas).toBeVisible();

    expect(jsErrors).toHaveLength(0);
  });

  test('deve alternar entre Cardio Lab e Pulmo Lab mantendo o motor contínuo e telemetria funcional', async ({ page }) => {
    await page.waitForTimeout(1000);

    // 1. Alterna para o Pulmo Lab
    const pulmoTabBtn = page.locator('.lab-tab', { hasText: 'PULMO LAB' });
    await expect(pulmoTabBtn).toBeVisible();
    await pulmoTabBtn.click();
    await expect(pulmoTabBtn).toHaveClass(/active/);

    // 2. Valida os 4 canvases respiratórios
    const pulmoCanvasIds = ['#canvas-resp-vol', '#canvas-resp-flow', '#canvas-resp-ppl', '#canvas-resp-rsa'];
    for (const id of pulmoCanvasIds) {
      const canvas = page.locator(id);
      await expect(canvas).toBeVisible();
      const box = await canvas.boundingBox();
      expect(box).not.toBeNull();
      expect(box.width).toBeGreaterThan(50);
      expect(box.height).toBeGreaterThan(30);
    }

    // 3. Valida pods de telemetria respiratória no HUD
    await expect(page.locator('#hud-rr')).toContainText('irpm');
    await expect(page.locator('#hud-vef1')).toContainText('L');
    await expect(page.locator('#hud-cvf')).toContainText('L');
    await expect(page.locator('#hud-tiff')).toContainText('%');
    await expect(page.locator('#hud-pef')).toContainText('L/s');

    // 4. Abre sanfona de espirometria forçada e dispara a manobra
    const spiroAccordion = page.locator('details.control-group', { hasText: 'Espirometria Forçada' });
    await spiroAccordion.locator('summary').click();
    await expect(spiroAccordion).toHaveAttribute('open');

    const triggerBtn = page.locator('.btn-spiro-trigger');
    await expect(triggerBtn).toBeVisible();
    await triggerBtn.click();

    // Aguarda conclusão da manobra e valida laudo
    await page.waitForTimeout(1200);
    const diagBanner = page.locator('.spiro-diag');
    await expect(diagBanner).toBeVisible();

    // 5. Retorna para o Cardio Lab e valida restauração dos canvases cardíacos
    const cardioTabBtn = page.locator('.lab-tab', { hasText: 'CARDIO LAB' });
    await cardioTabBtn.click();
    await expect(page.locator('#canvas-pa')).toBeVisible();
    await expect(page.locator('#canvas-ecg')).toBeVisible();

    expect(jsErrors).toHaveLength(0);
  });

  test('deve manipular os controles de Guyton e ortostase com telemetria no HUD', async ({ page }) => {
    // 1. Valida pods de PVC e PMES no HUD
    const cvpHud = page.locator('#hud-cvp');
    const pmesHud = page.locator('#hud-pmes');
    await expect(cvpHud).toBeVisible();
    await expect(pmesHud).toBeVisible();
    await expect(cvpHud).toContainText('mmHg');
    await expect(pmesHud).toContainText('mmHg');

    // 2. Abre a sanfona 9 (Retorno Venoso & Guyton)
    const guytonAccordion = page.locator('details.control-group', { hasText: 'Retorno Venoso & Guyton' });
    await expect(guytonAccordion).toBeVisible();
    await guytonAccordion.locator('summary').click();
    await expect(guytonAccordion).toHaveAttribute('open');

    // 3. Testa o toggle de Ortostase
    const orthoContainer = guytonAccordion.locator('.slider-container', { hasText: 'Ortostase' });
    const orthoCheckbox = orthoContainer.locator('input[type="checkbox"]');
    await expect(orthoCheckbox).toBeVisible();
    await expect(orthoCheckbox).not.toBeChecked();

    await orthoContainer.click();
    await expect(orthoCheckbox).toBeChecked();
    await page.waitForTimeout(500);

    // Desmarca ortostase
    await orthoContainer.click();
    await expect(orthoCheckbox).not.toBeChecked();

    expect(jsErrors).toHaveLength(0);
  });

  test('deve comutar entre Alça P×V e Diagrama de Guyton (DC × RV) no canal 2D', async ({ page }) => {
    // 1. Inicialmente no modo PV loop: #canvas-pv visível, #canvas-guyton oculto
    const pvCanvas = page.locator('#canvas-pv');
    const guytonCanvas = page.locator('#canvas-guyton');
    const modeSelect = page.locator('#select-2d-mode');

    await expect(pvCanvas).toBeVisible();
    await expect(guytonCanvas).not.toBeVisible();
    await expect(modeSelect).toHaveValue('pv');

    // 2. Comuta pelo dropdown para Diagrama de Guyton
    await modeSelect.selectOption('guyton');
    await expect(pvCanvas).not.toBeVisible();
    await expect(guytonCanvas).toBeVisible();

    // Valida dimensões do canvas de Guyton
    const box = await guytonCanvas.boundingBox();
    expect(box).not.toBeNull();
    expect(box.width).toBeGreaterThan(50);
    expect(box.height).toBeGreaterThan(30);

    // 3. Abre a Sanfona 9 e usa o botão de atalho para alternar de volta para PxV
    const guytonAccordion = page.locator('details.control-group', { hasText: 'Retorno Venoso & Guyton' });
    await guytonAccordion.locator('summary').click();
    await expect(guytonAccordion).toHaveAttribute('open');

    const toggleBtn = guytonAccordion.locator('.btn-toggle-guyton-2d');
    await expect(toggleBtn).toBeVisible();
    await expect(toggleBtn).toContainText('Voltar para Alça P×V');

    await toggleBtn.click();
    await expect(pvCanvas).toBeVisible();
    await expect(guytonCanvas).not.toBeVisible();
    await expect(modeSelect).toHaveValue('pv');
    await expect(toggleBtn).toContainText('Exibir Diagrama de Guyton');

    // 4. Alterna novamente para Guyton pelo botão da sanfona
    await toggleBtn.click();
    await expect(guytonCanvas).toBeVisible();
    await expect(pvCanvas).not.toBeVisible();
    await expect(modeSelect).toHaveValue('guyton');

    expect(jsErrors).toHaveLength(0);
  });

  test('deve manipular Albumina e Permeabilidade na Sanfona 6 e refletir na dinâmica de Starling e Edema', async ({ page }) => {
    // 1. Valida a presença da telemetria de Starling no Cardio HUD inicial
    const pcpLocator = page.locator('#hud-pcp');
    await expect(pcpLocator).toBeVisible();
    await expect(pcpLocator).toContainText('mmHg');

    const picLocator = page.locator('#hud-pic');
    await expect(picLocator).toBeVisible();
    await expect(picLocator).toContainText('mmHg');

    const godetLocator = page.locator('#hud-godet');
    await expect(godetLocator).toBeVisible();
    await expect(godetLocator).toContainText('0');

    // 2. Abre a Sanfona 6 (Condições Patológicas)
    const pathoAccordion = page.locator('details.control-group', { hasText: 'Condições Patológicas' });
    await expect(pathoAccordion).toBeVisible();
    await pathoAccordion.locator('summary').click();
    await expect(pathoAccordion).toHaveAttribute('open');

    // Valida que agora contém 4 sliders (Isquemia, Fibrose, Albumina, Permeabilidade)
    const sliders = pathoAccordion.locator('input[type="range"]');
    await expect(sliders).toHaveCount(4);

    // 3. Reduz Albumina para 1.5 g/dL (Hipoalbuminemia severa / Síndrome Nefrótica / Cirrose)
    const albContainer = pathoAccordion.locator('.slider-container', { hasText: 'Albumina Sérica' });
    const albSlider = albContainer.locator('input[type="range"]');
    await albSlider.fill('1.5');
    await page.waitForTimeout(2000);

    // 4. Verifica colapso da pressão oncótica pi_c e desenvolvimento de edema sistêmico (Cacifo / Godet)
    await expect(picLocator).toContainText(/^(7\.|8\.|9\.)/); // ~7.5 mmHg
    const godetVal = await godetLocator.innerText();
    expect(godetVal).not.toBe('0');

    // 5. Alterna para o Pulmo Lab e valida a telemetria respiratória de edema e SpO2
    const pulmoTabBtn = page.locator('.lab-tab', { hasText: 'PULMO LAB' });
    await pulmoTabBtn.click();
    await expect(pulmoTabBtn).toHaveClass(/active/);

    const edemaPulmLocator = page.locator('#hud-edema-pulm');
    await expect(edemaPulmLocator).toBeVisible();
    await expect(edemaPulmLocator).toContainText('L');

    const spo2Locator = page.locator('#hud-spo2');
    await expect(spo2Locator).toBeVisible();
    await expect(spo2Locator).toContainText('%');

    expect(jsErrors).toHaveLength(0);
  });

  test('deve alternar para o Nefro Lab, exibir os 4 canais renais e responder à Furosemida com aumento de diurese', async ({ page }) => {
    // 1. Alterna para a aba NEFRO LAB
    const nefroTabBtn = page.locator('.lab-tab', { hasText: 'NEFRO LAB' });
    await expect(nefroTabBtn).toBeVisible();
    await nefroTabBtn.click();
    await expect(nefroTabBtn).toHaveClass(/active/);

    // 2. Valida a presença dos 4 canvases especializados do Nefro Lab
    await expect(page.locator('#canvas-nefro-gfr')).toBeVisible();
    await expect(page.locator('#canvas-nefro-uout')).toBeVisible();
    await expect(page.locator('#canvas-nefro-rbf')).toBeVisible();
    await expect(page.locator('#canvas-nefro-vlec')).toBeVisible();

    // 3. Valida a telemetria do HUD Nefro (TFG, DU, VLEC)
    const tfgLocator = page.locator('#hud-tfg');
    await expect(tfgLocator).toBeVisible();
    await expect(tfgLocator).toContainText('mL/min');

    const duLocator = page.locator('#hud-du');
    await expect(duLocator).toBeVisible();
    await expect(duLocator).toContainText('mL/h');

    const vlecLocator = page.locator('#hud-vlec');
    await expect(vlecLocator).toBeVisible();
    await expect(vlecLocator).toContainText('L');

    // 4. Abre a Sanfona 21 (Farmacologia Tubular & Diuréticos)
    const pharmAccordion = page.locator('details.control-group', { hasText: 'Farmacologia Tubular' });
    await expect(pharmAccordion).toBeVisible();
    await pharmAccordion.locator('summary').click();
    await expect(pharmAccordion).toHaveAttribute('open');

    // 5. Eleva Diurético de Alça (Furosemida) para 80%
    const furosemidaContainer = pharmAccordion.locator('.slider-container', { hasText: 'Diurético de Alça (Furosemida)' });
    const furosemidaSlider = furosemidaContainer.locator('input[type="range"]');
    await furosemidaSlider.fill('80');

    // Aguarda o efeito farmacológico e a atualização da telemetria (10 Hz)
    await page.waitForTimeout(2000);

    // 6. Confirma que o débito urinário instantâneo aumentou significativamente (> 120 mL/h, mais que o dobro do basal de 60 mL/h)
    const duText = await duLocator.innerText();
    const duVal = parseFloat(duText.replace(/[^\d.]/g, ''));
    expect(duVal).toBeGreaterThan(120.0);

    // 7. Abre a Sanfona 22 (Balanço Hidrossalino) e aciona Bolus IV
    const balanceAccordion = page.locator('details.control-group', { hasText: 'Balanço Hidrossalino' });
    await expect(balanceAccordion).toBeVisible();
    await balanceAccordion.locator('summary').click();
    await expect(balanceAccordion).toHaveAttribute('open');

    const bolusBtn = balanceAccordion.locator('button', { hasText: 'INFUNDIR BOLUS IV' });
    await expect(bolusBtn).toBeVisible();
    await bolusBtn.click();

    await page.waitForTimeout(1000);
    expect(jsErrors).toHaveLength(0);
  });
});


