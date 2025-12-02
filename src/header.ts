import { cycleTheme, getTheme } from './theme';

export function initHeader() {
    const headerDiv = document.createElement('div');
    headerDiv.id = 'header';
    headerDiv.className = 'header';
    
    // Create theme toggle button
    const themeButton = document.createElement('button');
    themeButton.id = 'theme-toggle';
    themeButton.className = 'theme-toggle';
    themeButton.setAttribute('aria-label', 'Toggle theme');
    
    const updateButton = () => {
        const theme = getTheme();
        const icons: Record<string, string> = {
            system: '🌓',
            light: '☀️',
            dark: '🌙',
        };
        themeButton.textContent = icons[theme] || '🌓';
        themeButton.title = `Theme: ${theme} (click to cycle)`;
    };
    
    themeButton.addEventListener('click', () => {
        cycleTheme();
        updateButton();
    });
    
    updateButton();
    
    headerDiv.appendChild(themeButton);
    
    return headerDiv;
}

