import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export type StyleMode = 'professional' | 'casual' | 'sarcasm';

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
  visible: boolean;
  selectedText: string;
  rephrasedText: string;
  currentStyle: StyleMode;
  isLoading: boolean;
  error: string | null;
  config: AppConfig | null;
  
  setVisible: (visible: boolean) => void;
  setSelectedText: (text: string) => void;
  setRephrasedText: (text: string) => void;
  setCurrentStyle: (style: StyleMode) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  setConfig: (config: AppConfig) => void;
  loadConfig: () => Promise<void>;
  rephrase: () => Promise<void>;
  replaceText: () => Promise<void>;
  hidePopup: () => Promise<void>;
}

export const useStore = create<AppState>((set, get) => ({
  visible: false,
  selectedText: '',
  rephrasedText: '',
  currentStyle: 'professional',
  isLoading: false,
  error: null,
  config: null,
  
  setVisible: (visible) => set({ visible }),
  setSelectedText: (text) => set({ selectedText: text }),
  setRephrasedText: (text) => set({ rephrasedText: text }),
  setCurrentStyle: (style) => set({ currentStyle: style }),
  setLoading: (loading) => set({ isLoading: loading }),
  setError: (error) => set({ error }),
  setConfig: (config) => set({ config }),
  
  loadConfig: async () => {
    try {
      const config = await invoke<AppConfig>('load_config');
      set({ config });
    } catch (error) {
      console.error('Failed to load config:', error);
    }
  },
  
  rephrase: async () => {
    const { selectedText, currentStyle, config } = get();
    
    // Only require API key if not using proxy
    if (config?.model_provider !== 'proxy' && !config?.api_key) {
      set({ error: 'API key not configured' });
      return;
    }
    
    set({ isLoading: true, error: null });
    
    try {
      // First, get the selected text from clipboard
      const text = selectedText || await invoke<string>('get_clipboard_text');
      
      if (!text || text.trim() === '') {
        set({ error: 'No text selected', isLoading: false });
        return;
      }
      
      set({ selectedText: text });
      
      const rephrased = await invoke<string>('rephrase_text', {
        text,
        style: currentStyle,
        provider: config.model_provider || 'proxy',
        apiKey: config.api_key || '',
      });
      
      set({ rephrasedText: rephrased, isLoading: false });
    } catch (error) {
      const errorMessage = typeof error === 'string' ? error : 'Failed to rephrase text';
      set({ error: errorMessage, isLoading: false });
    }
  },
  
  replaceText: async () => {
    const { rephrasedText } = get();
    
    if (!rephrasedText) {
      return;
    }
    
    try {
      await invoke('replace_selected_text', { newText: rephrasedText });
      await get().hidePopup();
    } catch (error) {
      console.error('Failed to replace text:', error);
      set({ error: 'Failed to replace text' });
    }
  },
  
  hidePopup: async () => {
    try {
      await invoke('hide_popup');
      set({
        visible: false,
        selectedText: '',
        rephrasedText: '',
        error: null,
        isLoading: false,
      });
    } catch (error) {
      console.error('Failed to hide popup:', error);
    }
  },
}));


