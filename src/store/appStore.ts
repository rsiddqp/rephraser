import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface AppConfig {
  hotkey: string;
  default_style: string;
  model_provider: string;
  api_key?: string;
  theme: string;
  start_on_login: boolean;
  auto_update: boolean;
}

interface AppState {
  config: AppConfig | null;
  setConfig: (config: AppConfig) => void;
  loadConfig: () => Promise<void>;
}

export const useStore = create<AppState>((set) => ({
  config: null,
  setConfig: (config) => set({ config }),
  
  loadConfig: async () => {
    try {
      const config = await invoke<AppConfig>('load_config');
      set({ config });
    } catch (error) {
      console.error('Failed to load config:', error);
    }
  },
}));


