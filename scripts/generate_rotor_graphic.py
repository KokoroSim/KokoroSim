import numpy as np
import matplotlib.pyplot as plt
from matplotlib.colors import LinearSegmentedColormap

# Configuração da figura com fundo tecnológico escuro do KokoroSim
fig = plt.figure(figsize=(11, 5.5), dpi=300, facecolor='#0a0c12')

# Subplot 1: Rotor Eletrofisiológico (Onda Espiral em Meio Excitável)
ax1 = plt.subplot(1, 2, 1, facecolor='#0a0c12')
N = 400
x = np.linspace(-3, 3, N)
y = np.linspace(-3, 3, N)
X, Y = np.meshgrid(x, y)
R = np.sqrt(X**2 + Y**2)
Theta = np.arctan2(Y, X)

# Espiral de Arquimedes / Logarítmica representando a frente de onda de despolarização (Rotor)
spiral_phase = np.cos(Theta - 2.2 * np.log(R + 0.1))
# Atenuação no núcleo (singularidade de fase)
membrane_voltage = spiral_phase * np.tanh(R * 1.5)

# Colormap: Ciano (-85 mV repouso) -> Azul Escuro -> Carmesim (+30 mV despolarização)
colors = ['#050811', '#00f2fe', '#ffffff', '#ff1754', '#8b0028']
cmap = LinearSegmentedColormap.from_list('cardiac_wave', colors, N=256)

im1 = ax1.imshow(membrane_voltage, extent=[-3, 3, -3, 3], origin='lower', cmap=cmap)
ax1.set_title("ROTOR ELETROFISIOLÓGICO\n(Onda Espiral // Singularidade de Fase)", 
              color='#00f2fe', fontsize=11, fontweight='bold', pad=16, fontfamily='sans-serif')
ax1.axis('off')

# Subplot 2: Vórtice Hemodinâmico (Dinâmica de Fluidos Transmitral)
ax2 = plt.subplot(1, 2, 2, facecolor='#0a0c12')
# Linhas de corrente e vorticidade de um anel de vórtice (Hill's spherical vortex / Lamb-Oseen)
vorticity = np.exp(-((X - 0.5)**2 + Y**2)/0.6) - 0.6 * np.exp(-((X + 0.8)**2 + Y**2)/1.2)
stream_U = -Y / (R**2 + 0.2) + 0.3 * np.sin(Theta)
stream_V =  X / (R**2 + 0.2) - 0.4 * np.cos(Theta)

colors_hemo = ['#0a0c12', '#00cec9', '#00f2fe', '#ff1754']
cmap_hemo = LinearSegmentedColormap.from_list('vortex_flow', colors_hemo, N=256)

ax2.imshow(vorticity, extent=[-3, 3, -3, 3], origin='lower', cmap=cmap_hemo, alpha=0.85)
strm = ax2.streamplot(x, y, stream_U, stream_V, color='#ffffff', density=1.2, linewidth=0.8, arrowsize=0.9)

ax2.set_title("VÓRTEX HEMODINÂMICO\n(Recirculação e Enchimento Diastólico)", 
              color='#ff1754', fontsize=11, fontweight='bold', pad=16, fontfamily='sans-serif')
ax2.axis('off')

plt.subplots_adjust(top=0.82, bottom=0.08, left=0.06, right=0.94, wspace=0.18)
plt.savefig('ui/assets/brand/rotor_vortex.png', facecolor='#0a0c12', edgecolor='none')
print("✅ Imagem biofísica salva com margens ajustadas em ui/assets/brand/rotor_vortex.png")
