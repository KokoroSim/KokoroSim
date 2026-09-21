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

    expect(jsErrors).toHaveLength(0);
  });

  test('deve renderizar os 4 canais de osciloscópio (Canvases ativos)', async ({ page }) => {
    await page.waitForTimeout(1500);

    const canvasIds = ['#canvas-pa', '#canvas-ecg', '#canvas-ch3', '#canvas-ch4'];
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
    await expect(totalCanvases).toHaveCount(4);

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
    // Abre a sanfona de Visualização
    const visAccordion = page.locator('summary', { hasText: /Visualização/ });
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
    // Abre a sanfona de Visualização
    const visAccordion = page.locator('summary', { hasText: /Visualização/ });
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
    // Abre a sanfona de Visualização
    const visAccordion = page.locator('summary', { hasText: /Visualização/ });
    await visAccordion.click();

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
    await expect(accordions).toHaveCount(6);

    // 1. Todas iniciam fechadas
    for (let i = 0; i < 6; i++) {
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
});
