// @ts-check
const { defineConfig } = require('@playwright/test');

module.exports = defineConfig({
  testDir: './tests/e2e',
  timeout: 30000,
  expect: {
    timeout: 5000,
  },
  // Executa sequencialmente em um único worker para reusar uma única janela/sessão
  // e não abrir janelas concorrentes que atrapalhem o usuário
  workers: 1,
  fullyParallel: false,
  retries: 0,
  reporter: [['list'], ['html', { open: 'never' }]],

  use: {
    // Por padrão roda headless (invisível em background, 0 janelas na tela)
    headless: true,
    baseURL: 'http://localhost:8081',
    viewport: { width: 1280, height: 800 },
    actionTimeout: 10000,
    ignoreHTTPSErrors: true,
    
    // Configurações de isolamento e docking fixo quando executado com --headed
    launchOptions: {
      channel: 'chrome', // Usa o Google Chrome nativo do sistema
      args: [
        '--no-first-run',
        '--no-default-browser-check',
        '--disable-infobars',
        '--window-size=1280,800',
        '--window-position=50,50', // Janela ancorada em posição fixa
      ],
    },
  },

  // Reusa o servidor local que já estiver rodando, ou sobe o server.py se necessário
  webServer: {
    command: 'python3 ui/server.py',
    url: 'http://localhost:8081/app.html',
    reuseExistingServer: true,
    timeout: 10000,
  },
});
