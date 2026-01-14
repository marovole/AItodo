import Titlebar from '~view/Titlebar';
import Ask from '~view/Ask';
import Settings from '~view/Settings';
import TodoView from '~view/Todo';

const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

const viewMap = {
  titlebar: <Titlebar />,
  ask: <Ask />,
  settings: <Settings />,
  todo: <TodoView />,
};

function getViewLabel(): string {
  if (!isTauri) {
    const hash = window.location.hash;
    if (hash.includes('/todo')) return 'todo';
    if (hash.includes('/settings')) return 'settings';
    if (hash.includes('/ask')) return 'ask';
    return 'todo';
  }
  
  const { getCurrentWebview } = require('@tauri-apps/api/webview');
  return getCurrentWebview().label;
}

export default function App() {
  const label = getViewLabel();
  return viewMap[label as keyof typeof viewMap] || <TodoView />;
}
