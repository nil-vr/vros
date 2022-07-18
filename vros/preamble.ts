declare global {
  interface Vros {
    expandPath(path: string): string;
  }

  interface Window {
    vros: Vros;
  }
}
