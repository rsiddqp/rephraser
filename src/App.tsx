import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { register, unregister } from '@tauri-apps/plugin-global-shortcut';
import { RefreshCw, Copy, Settings as SettingsIcon } from 'lucide-react';
import Settings from './components/Settings';
import type { CustomStyle } from './store/appStore';

const BUILTIN_STYLES = ['professional', 'casual', 'sarcasm'] as const;

function App() {
  const [inputText, setInputText] = useState('');
  const [rephrasedText, setRephrasedText] = useState('');
  const [currentStyle, setCurrentStyle] = useState<string>('professional');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showSettings, setShowSettings] = useState(false);
  const [customStyles, setCustomStyles] = useState<CustomStyle[]>([]);
  
  const getCustomPromptForStyle = (styleId: string): string => {
    const custom = customStyles.find(s => s.id === styleId);
    return custom?.prompt || '';
  };

  const getStyleLabel = (styleId: string): string => {
    if (BUILTIN_STYLES.includes(styleId as any)) {
      return styleId.charAt(0).toUpperCase() + styleId.slice(1);
    }
    const custom = customStyles.find(s => s.id === styleId);
    return custom?.name || styleId;
  };

  const isBuiltinStyle = (styleId: string): boolean => {
    return BUILTIN_STYLES.includes(styleId as any);
  };

  const handleSettingsClose = async () => {
    try {
      const config = await invoke<any>('load_config');
      
      if (config.default_style) {
        setCurrentStyle(config.default_style);
      }
      setCustomStyles(config.custom_styles || []);
    } catch (error) {
      console.error('Failed to reload config:', error);
    }
    setShowSettings(false);
  };
  
  const rephrasedSectionRef = useRef<HTMLDivElement>(null);

  const handleRephrase = async () => {
    const trimmedText = inputText.trim();
    if (!trimmedText) {
      setError('Please enter some text');
      return;
    }

    if (trimmedText.length > 10000) {
      setError('Text is too long. Maximum 10,000 characters allowed.');
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const [freshConfig, currentApiKey] = await Promise.all([
        invoke<any>('load_config'),
        invoke<string | null>('get_api_key'),
      ]);
      const currentProvider = freshConfig.model_provider || 'proxy';
      
      if (currentProvider !== 'proxy' && !currentApiKey) {
        setError('Please configure your API key in Settings or use the default Proxy Server');
        setShowSettings(true);
        setIsLoading(false);
        return;
      }

      const customPrompt = getCustomPromptForStyle(currentStyle);
      
      const rephrased = await invoke<string>('rephrase_text', {
        text: trimmedText,
        style: isBuiltinStyle(currentStyle) ? currentStyle : 'professional',
        provider: currentProvider,
        apiKey: currentApiKey || '',
        customPrompt: customPrompt || null,
      });

      setRephrasedText(rephrased);
    } catch (e) {
      const errorMessage = typeof e === 'string' ? e : 'Failed to rephrase text. Please try again.';
      setError(errorMessage);
      console.error('Rephrase error:', e);
    } finally {
      setIsLoading(false);
    }
  };

  const handleCopy = async () => {
    if (!rephrasedText) return;
    
    try {
      await invoke('copy_to_clipboard', { text: rephrasedText });
      await invoke('hide_popup');
      
      setInputText('');
      setRephrasedText('');
      setError(null);
      setIsLoading(false);
    } catch (e) {
      setError('Failed to copy to clipboard');
      console.error('Copy error:', e);
    }
  };

  const handlePaste = async () => {
    try {
      const text = await invoke<string>('get_clipboard_text');
      const trimmed = text.trim();
      if (trimmed) {
        setInputText(trimmed);
        setError(null);
      } else {
        setError('Clipboard is empty');
      }
    } catch (e) {
      setError('Failed to paste from clipboard');
      console.error('Paste error:', e);
    }
  };

  useEffect(() => {
    const handleKeyDown = async (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'c') {
        const target = e.target as HTMLElement;
        const isInput = target.tagName === 'TEXTAREA' || target.tagName === 'INPUT';
        
        if (rephrasedText && !isInput) {
          e.preventDefault();
          await handleCopy();
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [rephrasedText]);

  useEffect(() => {
    const init = async () => {
      try {
        const config = await invoke<any>('load_config');
        setCurrentStyle(config.default_style || 'professional');
        setCustomStyles(config.custom_styles || []);
      } catch (error) {
        console.error('Failed to load initial config:', error);
      }

      try {
        const trusted = await invoke<boolean>('check_accessibility');
        if (!trusted) {
          setError('Accessibility permission required. Please enable Rephraser in System Settings → Privacy & Security → Accessibility, then restart the app.');
        }
      } catch (error) {
        console.error('Accessibility check failed:', error);
      }
    };
    
    init();
  }, []);

  useEffect(() => {
    let hotkeyRegistered = '';
    let isProcessing = false;
    
    invoke<any>('load_config').then(async config => {
      const hotkeyOptions = [
        'CmdOrCtrl+Shift+R',
        'CommandOrControl+Shift+R',
        'Ctrl+Shift+R',
      ];
      
      let registered = false;
      for (const hotkeyString of hotkeyOptions) {
        try {
          try {
            await unregister(hotkeyString);
          } catch (_e) {
            // Ignore if not registered
          }
          
          await register(hotkeyString, async () => {
          if (isProcessing) return;
          
          isProcessing = true;
          
          try {
            const [freshConfig, freshApiKey] = await Promise.all([
              invoke<any>('load_config'),
              invoke<string | null>('get_api_key'),
            ]);
            
            const text = await invoke<string>('get_selected_text');
            
            if (!text || text.trim().length === 0) {
              setError('No text selected. Please select some text and try again.');
              setInputText('');
              isProcessing = false;
              return;
            }
            
            await invoke('show_popup_at_cursor');
            
            setInputText(text);
            setError(null);
            
            const currentProvider = freshConfig.model_provider || 'proxy';
            const currentApiKey = freshApiKey || '';
            const latestCustomStyles: CustomStyle[] = freshConfig.custom_styles || [];
            setCustomStyles(latestCustomStyles);
            
            if (currentProvider === 'proxy' || currentApiKey) {
              setIsLoading(true);
              
              try {
                const customPrompt = latestCustomStyles.find(s => s.id === currentStyle)?.prompt || '';

                const rephrased = await invoke<string>('rephrase_text', {
                  text,
                  style: BUILTIN_STYLES.includes(currentStyle as any) ? currentStyle : 'professional',
                  provider: currentProvider,
                  apiKey: currentApiKey,
                  customPrompt: customPrompt || null,
                });
                setRephrasedText(rephrased);
                
                setTimeout(() => {
                  rephrasedSectionRef.current?.scrollIntoView({ 
                    behavior: 'smooth', 
                    block: 'nearest' 
                  });
                }, 100);
              } catch (e) {
                const errorMessage = typeof e === 'string' ? e : 'Failed to rephrase text';
                setError(errorMessage);
                console.error('Rephrase error:', e);
              } finally {
                setIsLoading(false);
              }
            } else {
              setError('Please configure your API key in Settings');
              setShowSettings(true);
            }
          } catch (e) {
            const errorMessage = typeof e === 'string' ? e : 'Failed to capture text. Make sure accessibility permissions are enabled in System Preferences → Security & Privacy → Accessibility.';
            setError(errorMessage);
            console.error('Capture error:', e);
          } finally {
            isProcessing = false;
          }
        });
        
        hotkeyRegistered = hotkeyString;
        registered = true;
        break;
      } catch (err) {
        console.log(`Failed to register ${hotkeyString}, trying next...`, err);
      }
    }
    
    if (!registered) {
      console.error('Failed to register any hotkey format');
      setError('Failed to register keyboard shortcut. The shortcut may be in use by another app.');
    }
    }).catch((err) => {
      console.error('Failed to load config:', err);
    });

    return () => {
      if (hotkeyRegistered) {
        unregister(hotkeyRegistered).catch((err: unknown) => {
          console.error('Failed to unregister hotkey:', err);
        });
      }
    };
  }, [currentStyle]);

  const allStyles = [
    ...BUILTIN_STYLES.map(s => ({ id: s, label: s.charAt(0).toUpperCase() + s.slice(1) })),
    ...customStyles.map(s => ({ id: s.id, label: s.name })),
  ];

  return (
    <div className="h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800 p-6">
      <div className="max-w-4xl mx-auto h-full flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between mb-6">
          <div>
            <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Rephraser</h1>
            <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
              Rephrase text in different styles with AI
            </p>
          </div>
          <button
            onClick={() => setShowSettings(true)}
            className="p-2 rounded-lg hover:bg-white/50 dark:hover:bg-gray-700 transition-colors"
          >
            <SettingsIcon size={24} className="text-gray-700 dark:text-gray-300" />
          </button>
        </div>

        {/* Style Selector */}
        <div className="flex gap-2 mb-4 flex-wrap">
          {allStyles.map((style) => (
            <button
              key={style.id}
              onClick={() => setCurrentStyle(style.id)}
              className={`px-4 py-2.5 rounded-lg text-sm font-semibold transition-all ${
                currentStyle === style.id
                  ? 'bg-blue-600 text-white shadow-lg transform scale-105'
                  : 'bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-blue-50 dark:hover:bg-gray-600'
              } ${allStyles.length <= 3 ? 'flex-1' : ''}`}
            >
              {style.label}
            </button>
          ))}
        </div>

        {/* Input Area */}
        <div className="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-4 mb-4">
          <div className="flex items-center justify-between mb-2">
            <label className="text-sm font-semibold text-gray-700 dark:text-gray-300">
              Original Text
            </label>
            <button
              onClick={handlePaste}
              className="text-xs text-blue-600 hover:text-blue-700 dark:text-blue-400 font-medium"
            >
              Paste from Clipboard
            </button>
          </div>
          <textarea
            value={inputText}
            onChange={(e) => setInputText(e.target.value)}
            placeholder="Type or paste your text here..."
            className="w-full h-32 p-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-white resize-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
        </div>

        {/* Rephrase Button */}
        <button
          onClick={handleRephrase}
          disabled={isLoading || !inputText.trim()}
          className="w-full py-3 px-6 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white rounded-lg font-semibold shadow-lg transition-all disabled:cursor-not-allowed flex items-center justify-center gap-2 mb-4"
        >
          <RefreshCw size={20} className={isLoading ? 'animate-spin' : ''} />
          {isLoading ? 'Rephrasing...' : 'Rephrase'}
        </button>

        {/* Error */}
        {error && (
          <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-3 mb-4">
            <p className="text-red-700 dark:text-red-400 text-sm">{error}</p>
          </div>
        )}

        {/* Output Area */}
        {rephrasedText && (
          <div ref={rephrasedSectionRef} className="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-4 flex-1 flex flex-col">
            <div className="flex items-center justify-between mb-2">
              <label className="text-sm font-semibold text-gray-700 dark:text-gray-300">
                Rephrased ({getStyleLabel(currentStyle)})
              </label>
              <button
                onClick={handleCopy}
                className="flex items-center gap-1 text-xs text-blue-600 hover:text-blue-700 dark:text-blue-400 font-medium"
              >
                <Copy size={14} />
                Copy
              </button>
            </div>
            <div className="flex-1 p-3 border border-blue-200 dark:border-blue-800 rounded-lg bg-blue-50 dark:bg-blue-900/20 text-gray-900 dark:text-white overflow-y-auto">
              {rephrasedText}
            </div>
          </div>
        )}
      </div>

      {/* Settings Modal */}
      {showSettings && <Settings onClose={handleSettingsClose} />}
    </div>
  );
}

export default App;
