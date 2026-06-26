export interface ThemeTokens {
  housingBg: string;
  housingBorder: string;
  housingShadow: string;
  lensOff: string;
  green: string;
  greenGlow: string;
  yellow: string;
  yellowGlow: string;
  red: string;
  redGlow: string;
}

export const themes: Record<string, ThemeTokens> = {
  classic: {
    housingBg: "#1a1a1a",
    housingBorder: "#2e2e2e",
    housingShadow: "0 8px 24px rgba(0,0,0,0.45)",
    lensOff: "#2a2a2a",
    green: "#3ddc67",
    greenGlow: "0 0 18px rgba(61,220,103,0.85)",
    yellow: "#ffd34d",
    yellowGlow: "0 0 18px rgba(255,211,77,0.85)",
    red: "#ff4d4d",
    redGlow: "0 0 18px rgba(255,77,77,0.85)",
  },
  minimal: {
    housingBg: "#f4f4f5",
    housingBorder: "#d4d4d8",
    housingShadow: "0 4px 12px rgba(0,0,0,0.12)",
    lensOff: "#e4e4e7",
    green: "#16a34a",
    greenGlow: "0 0 10px rgba(22,163,74,0.5)",
    yellow: "#ca8a04",
    yellowGlow: "0 0 10px rgba(202,138,4,0.5)",
    red: "#dc2626",
    redGlow: "0 0 10px rgba(220,38,38,0.5)",
  },
};

export function applyTheme(name: string): void {
  const theme = themes[name] ?? themes.classic;
  const root = document.documentElement;
  root.style.setProperty("--housing-bg", theme.housingBg);
  root.style.setProperty("--housing-border", theme.housingBorder);
  root.style.setProperty("--housing-shadow", theme.housingShadow);
  root.style.setProperty("--lens-off", theme.lensOff);
  root.style.setProperty("--green", theme.green);
  root.style.setProperty("--green-glow", theme.greenGlow);
  root.style.setProperty("--yellow", theme.yellow);
  root.style.setProperty("--yellow-glow", theme.yellowGlow);
  root.style.setProperty("--red", theme.red);
  root.style.setProperty("--red-glow", theme.redGlow);
  document.body.dataset.theme = name;
}
