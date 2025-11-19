import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { register, unregister } from '@tauri-apps/plugin-global-shortcut';
import { RefreshCw, Copy, Settings as SettingsIcon } from 'lucide-react';
import Settings from './components/Settings';

type StyleMode = 'professional' | 'casual' | 'sarcasm';

function App() {
  const [inputText, setInputText] = useState('');
  const [rephrasedText, setRephrasedText] = useState('');
  const [currentStyle, setCurrentStyle] = useState<StyleMode>('professional');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showSettings, setShowSettings] = useState(false);
  const [apiKey, setApiKey] = useState('');
  
  const rephrasedSectionRef = useRef<HTMLDivElement>(null);

  const handleRephrase = async () => {
    const trimmedText = inputText.trim();
    if (!trimmedText) {
      setError('Please enter some text');
      return;
    }

    // Check text length
    if (trimmedText.length > 10000) {
      setError('Text is too long. Maximum 10,000 characters allowed.');
      return;
    }

    if (!apiKey) {
      setError('Please configure your OpenAI API key in Settings');
      setShowSettings(true);
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const rephrased = await invoke<string>('rephrase_text', {
        text: trimmedText,
        style: currentStyle,
        apiKey: apiKey,
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
      console.log('✅ Copied to clipboard, minimizing window...');
      
      // Immediately hide window for seamless workflow
      await invoke('hide_popup');
      
      // Reset state for next use
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

  // Handle Cmd+C / Ctrl+C to copy rephrased text and minimize
  useEffect(() => {
    const handleKeyDown = async (e: KeyboardEvent) => {
      // Check if Cmd+C (Mac) or Ctrl+C (Windows/Linux)
      if ((e.metaKey || e.ctrlKey) && e.key === 'c') {
        // Only handle if we have rephrased text and we're not in an input field
        const target = e.target as HTMLElement;
        const isInput = target.tagName === 'TEXTAREA' || target.tagName === 'INPUT';
        
        if (rephrasedText && !isInput) {
          e.preventDefault();
          console.log('⌨️ Cmd+C detected, copying rephrased text...');
          await handleCopy();
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [rephrasedText]);

  // Load API key and register hotkey on mount
  useEffect(() => {
    let hotkeyRegistered = '';
    let isProcessing = false; // Prevent concurrent processing
    
    // Load config and API key
    invoke<any>('load_config').then(async config => {
      if (config.api_key) {
        setApiKey(config.api_key);
      }
      
      // Register global hotkey - try multiple formats
      const hotkeyOptions = [
        'CmdOrCtrl+Shift+R',
        'CommandOrControl+Shift+R',
        'Ctrl+Shift+R',
      ];
      
      let registered = false;
      for (const hotkeyString of hotkeyOptions) {
        try {
          await register(hotkeyString, async () => {
          // Prevent concurrent executions
          if (isProcessing) {
            console.log('⚠️ Already processing a request, skipping...');
            return;
          }
          
          isProcessing = true;
          console.log('🔥 Hotkey triggered!');
          
          try {
            // CRITICAL: Capture text FIRST (while original app has focus)
            console.log('📋 Capturing selected text from focused app...');
            const text = await invoke<string>('get_selected_text');
            
            if (!text || text.trim().length === 0) {
              setError('No text selected. Please select some text and try again.');
              setInputText('');
              console.error('❌ No text was captured');
              isProcessing = false;
              return;
            }
            
            console.log('✅ Text captured:', text.slice(0, 50) + '...');
            
            // NOW show window at cursor position (after capturing text)
            await invoke('show_popup_at_cursor');
            
            setInputText(text);
            setError(null);
            
            // Automatically trigger rephrase if API key exists
            if (config.api_key) {
              setIsLoading(true);
              
              try {
                const rephrased = await invoke<string>('rephrase_text', {
                  text,
                  style: currentStyle,
                  apiKey: config.api_key,
                });
                setRephrasedText(rephrased);
                console.log('✅ Rephrasing complete!');
                
                // Auto-scroll to rephrased text section
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
              setError('Please configure your OpenAI API key in Settings');
              setShowSettings(true);
            }
          } catch (e) {
            const errorMessage = typeof e === 'string' ? e : 'Failed to capture text. Make sure accessibility permissions are enabled in System Preferences → Security & Privacy → Accessibility.';
            setError(errorMessage);
            console.error('❌ Capture error:', e);
          } finally {
            isProcessing = false;
          }
        });
        
        hotkeyRegistered = hotkeyString;
        registered = true;
        console.log(`✅ Hotkey registered: ${hotkeyString}`);
        break;
      } catch (err) {
        console.log(`⚠️ Failed to register ${hotkeyString}, trying next...`, err);
      }
    }
    
    if (!registered) {
      console.error('❌ Failed to register any hotkey format');
      setError('Failed to register keyboard shortcut. The shortcut may be in use by another app.');
    }
    }).catch((err) => {
      console.error('❌ Failed to load config:', err);
    });

    // Cleanup: unregister hotkey on unmount
    return () => {
      if (hotkeyRegistered) {
        unregister(hotkeyRegistered).catch((err: unknown) => {
          console.error('Failed to unregister hotkey:', err);
        });
      }
    };
  }, [currentStyle]);

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
        <div className="flex gap-3 mb-4">
          {(['professional', 'casual', 'sarcasm'] as StyleMode[]).map((style) => (
            <button
              key={style}
              onClick={() => setCurrentStyle(style)}
              className={`flex-1 px-4 py-3 rounded-lg text-sm font-semibold transition-all ${
                currentStyle === style
                  ? 'bg-blue-600 text-white shadow-lg transform scale-105'
                  : 'bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-blue-50 dark:hover:bg-gray-600'
              }`}
            >
              {style.charAt(0).toUpperCase() + style.slice(1)}
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
                Rephrased ({currentStyle})
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
      {showSettings && <Settings onClose={() => setShowSettings(false)} />}
    </div>
  );
}

export default App;
