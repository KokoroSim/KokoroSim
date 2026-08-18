# Simulador Biofísico de Eletrofisiologia Cardíaca

Este projeto é um simulador computacional do potencial de ação cardíaco e eletrocardiograma (ECG), modelado a partir de equações biofísicas puras. Ele simula a atividade iônica em tempo real de três tecidos acoplados: Nó Sinoatrial, Nó Atrioventricular e Miócito Ventricular.

## Objetivo
Demonstrar de forma orgânica e matemática como distúrbios hidroeletrolíticos, modulação autonômica e fármacos antiarrítmicos afetam a morfologia celular e o ritmo cardíaco, sem o uso de regras condicionais ("if/else") para desenhar os gráficos. As arritmias e bloqueios emergem naturalmente das equações diferenciais dos canais iônicos.

## Modelos Matemáticos Utilizados
*   **Nó Sinoatrial (Gatilho):** Severi et al. (2012)
*   **Nó Atrioventricular (Condução):** Inada et al. (2009) - Célula Nodal (N-cell)
*   **Ventrículo (Motor):** Ten Tusscher (2004) - Célula Epicárdica

## 📜 Licença

Este projeto é licenciado sob a **GNU General Public License v3.0 (GPLv3)**.

Isso significa que este software é livre (Copyleft). Você pode usar, estudar, compartilhar e modificar o código à vontade, inclusive para fins comerciais. No entanto, qualquer projeto derivado ou modificado que seja distribuído **obrigatoriamente** deverá ser licenciado sob a mesma GPLv3 e ter seu código-fonte disponibilizado de forma aberta e gratuita.

