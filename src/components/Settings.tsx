import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../store/appStore';
import { Save, X, Plus, Trash2, Pencil, Check } from 'lucide-react';
import type { CustomStyle } from '../store/appStore';

interface SettingsProps {
  onClose: () => void;
}

const Settings = ({ onClose }: SettingsProps) => {
  const { config, setConfig } = useStore();
  const [apiKey, setApiKey] = useState('');
  const [modelProvider, setModelProvider] = useState(config?.model_provider || 'proxy');
  const [saving, setSaving] = useState(false);
  const [loading, setLoading] = useState(true);
  const [customStyles, setCustomStyles] = useState<CustomStyle[]>([]);
  const [editingStyle, setEditingStyle] = useState<CustomStyle | null>(null);
  const [newStyleName, setNewStyleName] = useState('');
  const [newStylePrompt, setNewStylePrompt] = useState('');
  const [showAddForm, setShowAddForm] = useState(false);

  useEffect(() => {
    const initConfig = async () => {
      try {
        const [loadedConfig, storedKey] = await Promise.all([
          invoke<any>('load_config'),
          invoke<string | null>('get_api_key'),
        ]);
        
        setConfig(loadedConfig);
        setApiKey(storedKey || '');
        setModelProvider(loadedConfig.model_provider || 'proxy');
        setCustomStyles(loadedConfig.custom_styles || []);
        setLoading(false);
      } catch (error) {
        console.error('Failed to load config:', error);
        setLoading(false);
      }
    };
    
    initConfig();
  }, []);

  useEffect(() => {
    if (config) {
      setModelProvider(config.model_provider || 'proxy');
    }
  }, [config]);

  const handleSave = async () => {
    if (!config) return;

    if (modelProvider !== 'proxy' && !apiKey.trim()) {
      alert('Please enter an API key for the selected provider, or use "Proxy Server (Default)" to use the app without your own API key.');
      return;
    }

    setSaving(true);
    try {
      const newConfig = {
        ...config,
        model_provider: modelProvider,
        custom_styles: customStyles,
      };
      delete (newConfig as any).api_key;
      
      await Promise.all([
        invoke('save_config', { config: newConfig }),
        invoke('set_api_key', { key: apiKey.trim() }),
      ]);
      
      setConfig(newConfig);
      onClose();
    } catch (error) {
      console.error('Failed to save settings:', error);
      alert(`Failed to save settings: ${error}`);
    } finally {
      setSaving(false);
    }
  };

  const handleAddStyle = () => {
    if (!newStyleName.trim() || !newStylePrompt.trim()) return;
    
    const style: CustomStyle = {
      id: `custom_${Date.now()}`,
      name: newStyleName.trim(),
      prompt: newStylePrompt.trim(),
    };
    
    setCustomStyles(prev => [...prev, style]);
    setNewStyleName('');
    setNewStylePrompt('');
    setShowAddForm(false);
  };

  const handleDeleteStyle = (id: string) => {
    setCustomStyles(prev => prev.filter(s => s.id !== id));
  };

  const handleStartEdit = (style: CustomStyle) => {
    setEditingStyle({ ...style });
  };

  const handleSaveEdit = () => {
    if (!editingStyle || !editingStyle.name.trim() || !editingStyle.prompt.trim()) return;
    
    setCustomStyles(prev =>
      prev.map(s => s.id === editingStyle.id ? editingStyle : s)
    );
    setEditingStyle(null);
  };

  const getApiKeyPlaceholder = () => {
    switch (modelProvider) {
      case 'proxy': return 'No API key needed (using default)';
      case 'openai': return 'sk-...';
      case 'claude': case 'anthropic': return 'sk-ant-...';
      case 'gemini': case 'google': return 'AIza...';
      case 'perplexity': return 'pplx-...';
      default: return 'Enter your API key';
    }
  };

  const getApiKeyLink = () => {
    switch (modelProvider) {
      case 'proxy': return '#';
      case 'openai': return 'https://platform.openai.com/api-keys';
      case 'claude': case 'anthropic': return 'https://console.anthropic.com/account/keys';
      case 'gemini': case 'google': return 'https://aistudio.google.com/app/apikey';
      case 'perplexity': return 'https://www.perplexity.ai/settings/api';
      default: return '#';
    }
  };

  const getProviderName = () => {
    switch (modelProvider) {
      case 'proxy': return 'Proxy Server';
      case 'openai': return 'OpenAI';
      case 'claude': case 'anthropic': return 'Anthropic';
      case 'gemini': case 'google': return 'Google';
      case 'perplexity': return 'Perplexity';
      default: return 'Provider';
    }
  };

  if (loading) {
    return (
      <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div className="bg-white dark:bg-gray-800 rounded-xl shadow-2xl w-[500px] p-8 text-center">
          <p className="text-gray-700 dark:text-gray-300">Loading settings...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow-2xl w-[540px] max-h-[85vh] overflow-y-auto">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
          <h2 className="text-lg font-bold text-gray-900 dark:text-white">Settings</h2>
          <button
            onClick={onClose}
            className="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
          >
            <X size={20} />
          </button>
        </div>

        {/* Content */}
        <div className="p-6 space-y-6">
          {/* Model Provider Selection */}
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              AI Model Provider
            </label>
            <select
              value={modelProvider}
              onChange={(e) => {
                setModelProvider(e.target.value);
                if (e.target.value === 'proxy') {
                  setApiKey('');
                }
              }}
              className="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              <option value="proxy">Proxy Server (Default - No API key needed)</option>
              <option value="openai">OpenAI (GPT-4o mini) - Use your API key</option>
              <option value="claude">Anthropic (Claude Sonnet 4.6) - Use your API key</option>
              <option value="gemini">Google (Gemini 2.5 Flash) - Use your API key</option>
              <option value="perplexity">Perplexity (Sonar) - Use your API key</option>
            </select>
            <p className="mt-2 text-xs text-gray-500 dark:text-gray-400">
              Default uses proxy server (free). Advanced users can use their own API keys.
            </p>
          </div>

          {/* API Key */}
          {modelProvider !== 'proxy' && (
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                {getProviderName()} API Key
              </label>
              <input
                type="password"
                value={apiKey}
                onChange={(e) => setApiKey(e.target.value)}
                placeholder={getApiKeyPlaceholder()}
                className="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              />
              <p className="mt-2 text-xs text-gray-500 dark:text-gray-400">
                Get your API key from{' '}
                <a
                  href={getApiKeyLink()}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-blue-500 hover:underline"
                >
                  {getProviderName()} Platform
                </a>
              </p>
            </div>
          )}
          
          {modelProvider === 'proxy' && (
            <div className="p-4 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg">
              <p className="text-sm text-green-800 dark:text-green-400">
                <strong>Default mode active</strong> — No API key required. The app will work immediately!
              </p>
            </div>
          )}

          {/* Hotkey Display */}
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Global Hotkey
            </label>
            <div className="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-900 text-gray-700 dark:text-gray-300 font-mono">
              <span className="inline-block px-2 py-1 bg-gray-200 dark:bg-gray-700 rounded">Cmd</span>
              {' + '}
              <span className="inline-block px-2 py-1 bg-gray-200 dark:bg-gray-700 rounded">Shift</span>
              {' + '}
              <span className="inline-block px-2 py-1 bg-gray-200 dark:bg-gray-700 rounded">R</span>
            </div>
            <p className="mt-2 text-xs text-gray-500 dark:text-gray-400">
              Press this combination to activate Rephraser (Ctrl+Shift+R on Windows)
            </p>
          </div>

          {/* Default Style */}
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Default Style
            </label>
            <select
              value={config?.default_style || 'professional'}
              onChange={(e) => {
                if (config) {
                  setConfig({ ...config, default_style: e.target.value });
                }
              }}
              className="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              <option value="professional">Professional</option>
              <option value="casual">Casual</option>
              <option value="sarcasm">Sarcasm</option>
              {customStyles.map(s => (
                <option key={s.id} value={s.id}>{s.name}</option>
              ))}
            </select>
          </div>

          {/* Custom Styles */}
          <div>
            <div className="flex items-center justify-between mb-3">
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Custom Rephrase Styles
              </label>
              {!showAddForm && (
                <button
                  onClick={() => setShowAddForm(true)}
                  className="flex items-center gap-1 text-xs text-blue-600 hover:text-blue-700 dark:text-blue-400 font-medium"
                >
                  <Plus size={14} />
                  Add Style
                </button>
              )}
            </div>

            <p className="text-xs text-gray-500 dark:text-gray-400 mb-3">
              Create your own rephrasing tones. Describe how you want the AI to rephrase your text.
            </p>

            {/* Existing Custom Styles */}
            {customStyles.length > 0 && (
              <div className="space-y-2 mb-3">
                {customStyles.map(style => (
                  <div
                    key={style.id}
                    className="border border-gray-200 dark:border-gray-600 rounded-lg p-3 bg-gray-50 dark:bg-gray-900"
                  >
                    {editingStyle?.id === style.id ? (
                      <div className="space-y-2">
                        <input
                          type="text"
                          value={editingStyle.name}
                          onChange={(e) => setEditingStyle({ ...editingStyle, name: e.target.value })}
                          className="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                          placeholder="Style name"
                        />
                        <textarea
                          value={editingStyle.prompt}
                          onChange={(e) => setEditingStyle({ ...editingStyle, prompt: e.target.value })}
                          className="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white resize-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                          rows={3}
                          placeholder="Describe the tone and style..."
                        />
                        <div className="flex justify-end gap-2">
                          <button
                            onClick={() => setEditingStyle(null)}
                            className="px-2 py-1 text-xs text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700 rounded"
                          >
                            Cancel
                          </button>
                          <button
                            onClick={handleSaveEdit}
                            className="flex items-center gap-1 px-2 py-1 text-xs text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-900/30 rounded"
                          >
                            <Check size={12} /> Save
                          </button>
                        </div>
                      </div>
                    ) : (
                      <div className="flex items-start justify-between gap-2">
                        <div className="flex-1 min-w-0">
                          <p className="text-sm font-medium text-gray-900 dark:text-white">{style.name}</p>
                          <p className="text-xs text-gray-500 dark:text-gray-400 mt-0.5 line-clamp-2">{style.prompt}</p>
                        </div>
                        <div className="flex items-center gap-1 shrink-0">
                          <button
                            onClick={() => handleStartEdit(style)}
                            className="p-1.5 rounded hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-500 dark:text-gray-400 transition-colors"
                          >
                            <Pencil size={14} />
                          </button>
                          <button
                            onClick={() => handleDeleteStyle(style.id)}
                            className="p-1.5 rounded hover:bg-red-100 dark:hover:bg-red-900/30 text-gray-500 dark:text-gray-400 hover:text-red-600 dark:hover:text-red-400 transition-colors"
                          >
                            <Trash2 size={14} />
                          </button>
                        </div>
                      </div>
                    )}
                  </div>
                ))}
              </div>
            )}

            {/* Add New Style Form */}
            {showAddForm && (
              <div className="border border-blue-200 dark:border-blue-800 rounded-lg p-3 bg-blue-50/50 dark:bg-blue-900/10 space-y-2">
                <input
                  type="text"
                  value={newStyleName}
                  onChange={(e) => setNewStyleName(e.target.value)}
                  className="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  placeholder="Style name (e.g., Formal Email, Gen Z, Pirate)"
                />
                <textarea
                  value={newStylePrompt}
                  onChange={(e) => setNewStylePrompt(e.target.value)}
                  className="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white resize-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  rows={3}
                  placeholder="Describe how the AI should rephrase text (e.g., 'Rephrase in a warm, empathetic tone as if writing to a close friend who needs encouragement')"
                />
                <div className="flex justify-end gap-2">
                  <button
                    onClick={() => {
                      setShowAddForm(false);
                      setNewStyleName('');
                      setNewStylePrompt('');
                    }}
                    className="px-3 py-1.5 text-xs text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-lg transition-colors"
                  >
                    Cancel
                  </button>
                  <button
                    onClick={handleAddStyle}
                    disabled={!newStyleName.trim() || !newStylePrompt.trim()}
                    className="flex items-center gap-1 px-3 py-1.5 text-xs bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    <Plus size={12} /> Add Style
                  </button>
                </div>
              </div>
            )}

            {customStyles.length === 0 && !showAddForm && (
              <div className="border border-dashed border-gray-300 dark:border-gray-600 rounded-lg p-4 text-center">
                <p className="text-xs text-gray-500 dark:text-gray-400">
                  No custom styles yet. Click "Add Style" to create your own rephrase tone.
                </p>
              </div>
            )}
          </div>
        </div>

        {/* Footer */}
        <div className="flex justify-end gap-3 p-4 border-t border-gray-200 dark:border-gray-700">
          <button
            onClick={onClose}
            className="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={(e) => {
              e.preventDefault();
              e.stopPropagation();
              handleSave();
            }}
            disabled={saving}
            type="button"
            className="flex items-center gap-2 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <Save size={16} />
            {saving ? 'Saving...' : 'Save'}
          </button>
        </div>
      </div>
    </div>
  );
};

export default Settings;
