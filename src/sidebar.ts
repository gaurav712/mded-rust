const MINIMUM_SIDEBAR_WIDTH_RATIO = 0.1;
const MAXIMUM_SIDEBAR_WIDTH_RATIO = 0.4;

export function initSidebar() {
    const sidebarDiv = document.createElement('div');
    sidebarDiv.id = 'sidebar';
    sidebarDiv.className = 'sidebar';
    
    // Sidebar content
    
    return sidebarDiv;
}

export function initResizeHandle() {
    const resizeHandle = document.createElement('div');
    resizeHandle.className = 'resize-handle';
    resizeHandle.setAttribute('aria-label', 'Resize sidebar');
    
    const container = document.querySelector('main.container') as HTMLElement;
    let isResizing = false;
    let startX = 0;
    let startWidth = 0;
    
    const getSidebarWidth = (): number => {
        const saved = localStorage.getItem('sidebar-width');
        return saved ? parseFloat(saved) : window.innerWidth * 0.3;
    };
    
    const setSidebarWidth = (width: number) => {
        const min = window.innerWidth * MINIMUM_SIDEBAR_WIDTH_RATIO;
        const max = window.innerWidth * MAXIMUM_SIDEBAR_WIDTH_RATIO;
        const clamped = Math.max(min, Math.min(max, width));
        
        container.style.setProperty('--sidebar-width', `${clamped}px`);
        localStorage.setItem('sidebar-width', clamped.toString());
    };
    
    setSidebarWidth(getSidebarWidth());
    
    resizeHandle.addEventListener('mousedown', (e) => {
        isResizing = true;
        startX = e.clientX;
        const sidebar = document.querySelector('.sidebar') as HTMLElement;
        startWidth = sidebar?.offsetWidth ?? getSidebarWidth();
        container.classList.add('resizing');
        document.body.style.cursor = 'col-resize';
        e.preventDefault();
    });
    
    document.addEventListener('mousemove', (e) => {
        if (!isResizing) return;
        
        const diff = e.clientX - startX;
        const newWidth = startWidth + diff;
        setSidebarWidth(newWidth);
        e.preventDefault();
    });
    
    document.addEventListener('mouseup', () => {
        if (isResizing) {
            isResizing = false;
            container.classList.remove('resizing');
            document.body.style.cursor = '';
        }
    });
    
    window.addEventListener('resize', () => {
        if (!isResizing) {
            const saved = localStorage.getItem('sidebar-width');
            if (saved) setSidebarWidth(parseFloat(saved));
        }
    });
    
    return resizeHandle;
}

