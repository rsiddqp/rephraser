import { useEffect } from 'react';
import { useStore, StyleMode } from '../store/appStore';
import { X, RefreshCw, Check } from 'lucide-react';

const Popup = () => {
  const {
    visible,
    selectedText,
    rephrasedText,
    currentStyle,
    isLoading,
    error,
    setCurrentStyle,
    rephrase,
    replaceText,
    hidePopup,
  } = useStore();

  useEffect(() => {
    if (visible && !rephrasedText && !isLoading) {
      rephrase();
    }
  }, [visible, rephrasedText, isLoading, rephrase]);

  useEffect(() => {
    if (visible && currentStyle) {
      rephrase();
    }
  }, [currentStyle]);

  const handleStyleChange = (style: StyleMode) => {
    setCurrentStyle(style);
  };

  const handleRephrase = () => {
    rephrase();
  };

  const handleReplace = () => {
    replaceText();
  };

  const handleClose = () => {
    hidePopup();
  };

  if (!visible) {
    return null;
  }

  return (
    <div className="fixed inset-0 flex items-center justify-center bg-transparent">
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow-2xl border border-gray-200 dark:border-gray-700 w-[400px] animate-scale-in">
        {/* Header with close button */}
        <div className="flex items-center justify-between p-3 border-b border-gray-200 dark:border-gray-700">
          <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300">
            Rephraser
          </h3>
          <button
            onClick={handleClose}
            className="p-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
            aria-label="Close"
          >
            <X size={16} className="text-gray-500 dark:text-gray-400" />
          </button>
        </div>

        {/* Style selector */}
        <div className="flex gap-2 p-3 border-b border-gray-200 dark:border-gray-700">
          {(['professional', 'casual', 'sarcasm'] as StyleMode[]).map((style) => (
            <button
              key={style}
              onClick={() => handleStyleChange(style)}
              className={`flex-1 px-3 py-2 rounded-lg text-sm font-medium transition-all ${
                currentStyle === style
                  ? 'bg-blue-500 text-white shadow-md'
                  : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600'
              }`}
            >
              {style.charAt(0).toUpperCase() + style.slice(1)}
            </button>
          ))}
        </div>

        {/* Text preview */}
        <div className="p-4">
          {isLoading ? (
            <div className="flex items-center justify-center py-8">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
            </div>
          ) : error ? (
            <div className="text-red-500 text-sm py-4 text-center">{error}</div>
          ) : (
            <>
              {/* Original text */}
              <div className="mb-3">
                <label className="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide">
                  Original
                </label>
                <div className="mt-1 text-sm text-gray-700 dark:text-gray-300 bg-gray-50 dark:bg-gray-900 p-3 rounded-lg max-h-24 overflow-y-auto">
                  {selectedText || 'No text selected'}
                </div>
              </div>

              {/* Rephrased text */}
              <div>
                <label className="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide">
                  Rephrased ({currentStyle})
                </label>
                <div className="mt-1 text-sm text-gray-900 dark:text-gray-100 bg-blue-50 dark:bg-blue-900/20 p-3 rounded-lg max-h-32 overflow-y-auto border border-blue-200 dark:border-blue-800">
                  {rephrasedText || 'Generating...'}
                </div>
              </div>
            </>
          )}
        </div>

        {/* Action buttons */}
        {!error && (
          <div className="flex gap-2 p-3 border-t border-gray-200 dark:border-gray-700">
            <button
              onClick={handleRephrase}
              disabled={isLoading}
              className="flex items-center justify-center gap-2 flex-1 px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <RefreshCw size={16} className={isLoading ? 'animate-spin' : ''} />
              <span className="text-sm font-medium">Rephrase</span>
            </button>
            <button
              onClick={handleReplace}
              disabled={isLoading || !rephrasedText}
              className="flex items-center justify-center gap-2 flex-1 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <Check size={16} />
              <span className="text-sm font-medium">Replace</span>
            </button>
          </div>
        )}
      </div>
    </div>
  );
};

export default Popup;


