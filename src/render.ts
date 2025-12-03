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
const sampleMarkdown = `
# Hello World

Welcome to **MiniGFM** demo!

---

## Features Showcase

### 1. Lists

- Unordered item 1
- Unordered item 2  
  - Nested item A
  - Nested item B

1. Ordered list item one
2. Ordered list item two
    1. Nested ordered sub-item

### 2. Task List

- [x] Completed task
- [ ] Incomplete task

### 3. Table

| Syntax   | Description        | Example     |
|----------|-------------------|-------------|
| Header 1 | Header 2          | Header 3    |
| Cell 1   | _Italic_          | \`inline\`  |
| Cell 2   | **Bold**          | [Link](#)   |

### 4. Code

Inline code: \`console.log('Hello!');\`

\`\`\`js
// JavaScript example
function greet(name) {
    console.log('Hello, ' + name + '!');
}
greet('World');
\`\`\`

### 5. Blockquotes

> "This is a blockquote.  
> It can span multiple lines."

### 6. Strikethrough

~~This text is struck through~~

### 7. Images

![MiniGFM Logo](https://placehold.co/32x32 "Logo")

### 8. Link

[Visit MiniGFM](https://github.com/oblivionocean/minigfm)

### 9. Horizontal Rule

---

`;


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

