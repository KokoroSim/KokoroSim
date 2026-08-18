import './style.css';
import { initSidebar } from './ui/sidebar';
import { initCanvas } from './ui/canvas';

document.addEventListener('DOMContentLoaded', () => {
    initSidebar();
    initCanvas();
});
