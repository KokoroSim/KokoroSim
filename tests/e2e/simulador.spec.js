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
    await page.waitForTimeout(1000);

    const canvasIds = ['#canvas-pa', '#canvas-ecg', '#canvas-ch3', '#canvas-ch4'];
    for (const id of canvasIds) {
      const canvas = page.locator(id);
      await expect(canvas).toBeVisible();
      const box = await canvas.boundingBox();
      expect(box).not.toBeNull();
      expect(box.width).toBeGreaterThan(100);
      expect(box.height).toBeGreaterThan(50);
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
    const ghostCaptureBtn = page.locator('button', { hasText: /Capturar/ });
    await expect(ghostCaptureBtn).toBeVisible();

    // Dispara a captura da onda fantasma
    await ghostCaptureBtn.click();
    await page.waitForTimeout(1500); // Aguarda o ciclo de gravação do Nó SA

    // O checkbox da onda fantasma deve estar ativado
    const ghostCheckbox = page.locator('.slider-container', { hasText: /Onda Fantasma/ }).locator('input[type="checkbox"]');
    await expect(ghostCheckbox).toBeChecked();

    expect(jsErrors).toHaveLength(0);
  });

  test('deve alternar para modo Single-Shot e exibir botão de disparo único', async ({ page }) => {
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
});
