export type Theme = 'dark' | 'light' | 'system';

const gruvboxDark = {
    background: '#282828',
    foreground: '#ebdbb2',
    color0: '#282828',
    color8: '#928374',
    color1: '#cc241d',
    color9: '#fb4934',
    color2: '#98971a',
    color10: '#b8bb26',
    color3: '#d79921',
    color11: '#fabd2f',
    color4: '#458588',
    color12: '#83a598',
    color5: '#b16286',
    color13: '#d3869b',
    color6: '#689d6a',
    color14: '#8ec07c',
    color7: '#a89984',
    color15: '#ebdbb2',
};

const gruvboxLight = {
    background: '#fbf1c7',
    foreground: '#3c3836',
    color0: '#fdf4c1',
    color8: '#928374',
    color1: '#cc241d',
    color9: '#9d0006',
    color2: '#98971a',
    color10: '#79740e',
    color3: '#d79921',
    color11: '#b57614',
    color4: '#458588',
    color12: '#076678',
    color5: '#b16286',
    color13: '#8f3f71',
    color6: '#689d6a',
    color14: '#427b58',
    color7: '#7c6f64',
    color15: '#3c3836',
};

function getSystemTheme(): 'dark' | 'light' {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

function getEffectiveTheme(theme: Theme): 'dark' | 'light' {
    return theme === 'system' ? getSystemTheme() : theme;
}

function applyTheme(theme: 'dark' | 'light') {
    const colors = theme === 'dark' ? gruvboxDark : gruvboxLight;
    const root = document.documentElement;
    
    root.style.setProperty('--bg', colors.background);
    root.style.setProperty('--fg', colors.foreground);
    Object.entries(colors).forEach(([key, value]) => {
        if (key.startsWith('color')) {
            root.style.setProperty(`--${key}`, value);
        }
    });
    
    root.setAttribute('data-theme', theme);
}


export function initTheme() {
    applyTheme(getEffectiveTheme(currentTheme));
    
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
        if (currentTheme === 'system') {
            applyTheme(getEffectiveTheme(currentTheme));
        }
    });
}

let currentTheme: Theme = (localStorage.getItem('theme') as Theme) || 'system';

// Get the current theme
export function getTheme(): Theme {
    return currentTheme;
}

// Handler for the theme toggle button
export function cycleTheme() {
    const themes: Theme[] = ['system', 'light', 'dark'];
    currentTheme = themes[(themes.indexOf(currentTheme) + 1) % themes.length];
    localStorage.setItem('theme', currentTheme);
    applyTheme(getEffectiveTheme(currentTheme));
}

