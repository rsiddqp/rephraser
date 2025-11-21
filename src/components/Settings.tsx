import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../store/appStore';
import { Save, X } from 'lucide-react';

interface SettingsProps {
  onClose: () => void;
}

const Settings = ({ onClose }: SettingsProps) => {
  const { config, setConfig } = useStore();
  const [apiKey, setApiKey] = useState(config?.api_key || '');
  const [modelProvider, setModelProvider] = useState(config?.model_provider || 'openai');
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (config) {
      setApiKey(config.api_key || '');
      setModelProvider(config.model_provider || 'openai');
    }
  }, [config]);

  const handleSave = async () => {
    if (!config) return;

    // Only require API key if not using proxy
    if (modelProvider !== 'proxy' && !apiKey.trim()) {
      alert('Please enter an API key for the selected provider, or use "Proxy Server (Default)" to use the app without your own API key.');
      return;
    }

    setSaving(true);
    try {
      const newConfig = {
        ...config,
        api_key: apiKey,
        model_provider: modelProvider,
      };
      
      await invoke('save_config', { config: newConfig });
      setConfig(newConfig);
      onClose();
    } catch (error) {
      console.error('Failed to save config:', error);
      alert('Failed to save settings');
    } finally {
      setSaving(false);
    }
  };

  const getApiKeyPlaceholder = () => {
    switch (modelProvider) {
      case 'proxy':
        return 'No API key needed (using default)';
      case 'openai':
        return 'sk-...';
      case 'claude':
      case 'anthropic':
        return 'sk-ant-...';
      case 'gemini':
      case 'google':
        return 'AIza...';
      case 'perplexity':
        return 'pplx-...';
      default:
        return 'Enter your API key';
    }
  };

  const getApiKeyLink = () => {
    switch (modelProvider) {
      case 'proxy':
        return '#';
      case 'openai':
        return 'https://platform.openai.com/api-keys';
      case 'claude':
      case 'anthropic':
        return 'https://console.anthropic.com/account/keys';
      case 'gemini':
      case 'google':
        return 'https://makersuite.google.com/app/apikey';
      case 'perplexity':
        return 'https://www.perplexity.ai/settings/api';
      default:
        return '#';
    }
  };

  const getProviderName = () => {
    switch (modelProvider) {
      case 'proxy':
        return 'Proxy Server';
      case 'openai':
        return 'OpenAI';
      case 'claude':
      case 'anthropic':
        return 'Anthropic';
      case 'gemini':
      case 'google':
        return 'Google';
      case 'perplexity':
        return 'Perplexity';
      default:
        return 'Provider';
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow-2xl w-[500px] max-h-[80vh] overflow-y-auto">
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
                  setApiKey(''); // Clear API key when switching to proxy
                }
              }}
              className="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              <option value="proxy">Proxy Server (Default - No API key needed)</option>
              <option value="openai">OpenAI (GPT-4o-mini) - Use your API key</option>
              <option value="claude">Anthropic (Claude 3.5 Sonnet) - Use your API key</option>
              <option value="gemini">Google (Gemini Pro) - Use your API key</option>
              <option value="perplexity">Perplexity (Llama 3.1) - Use your API key</option>
            </select>
            <p className="mt-2 text-xs text-gray-500 dark:text-gray-400">
              Default uses proxy server (free). Advanced users can use their own API keys.
            </p>
          </div>

          {/* API Key */}
          {modelProvider !== 'proxy' && (
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                {getProviderName()} API Key (Optional)
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
                ✅ <strong>Default mode active</strong> - No API key required. The app will work immediately!
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
            </select>
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
            onClick={handleSave}
            disabled={saving}
            className="flex items-center gap-2 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors disabled:opacity-50"
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


