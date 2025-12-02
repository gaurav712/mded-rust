import { MiniGFM } from '@oblivionocean/minigfm';
import hljs from 'highlight.js';
import { initTheme } from './theme';
import { initHeader } from './header';
import { initSidebar, initResizeHandle } from './sidebar';

// Create configured instance
const md = new MiniGFM({
    unsafe: true, // Allow raw HTML rendering
    hljs: hljs,   // Use highlight.js for code blocks
});

// Parse Markdown
const sampleMarkdown = '# Hello World';
const html = md.parse(sampleMarkdown);

window.addEventListener('DOMContentLoaded', () => {
    initTheme();
    
    const container = document.querySelector('main.container');
    if (!container) return;
    
    container.appendChild(initHeader());
    
    const mainContent = document.createElement('div');
    mainContent.className = 'main-content';
    mainContent.append(
        initSidebar(),
        initResizeHandle(),
        Object.assign(document.createElement('div'), {
            id: 'content',
            className: 'content',
            innerHTML: html,
        })
    );
    
    container.appendChild(mainContent);
});

