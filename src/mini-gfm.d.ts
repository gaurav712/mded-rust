declare module '@oblivionocean/minigfm' {
    export class MiniGFM {
        constructor(options?: {
            unsafe?: boolean;
            hljs?: any;
        });
        parse(markdown: string): string;
    }
}

