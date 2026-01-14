import { useEffect, useState } from 'react';
import { useTodoStore } from '~/stores/todoStore';
import TodoLayout from './components/TodoLayout';
import TodoSidebar from './components/TodoSidebar';
import TodoList from './components/TodoList';
import TodoDetail from './components/TodoDetail';
import Toast from './components/Toast';
import '~/styles/todo.css';

const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

interface ResearchCompletePayload {
  todoId: string;
  content: string;
  citations?: { url: string; title: string }[];
  source: string;
  startedAt: string;
}

export default function TodoView() {
  const { 
    fetchTodos, 
    fetchStatusCounts, 
    error,
    clearError,
    openDetailPanel
  } = useTodoStore();

  const [isOnline, setIsOnline] = useState(navigator.onLine);

  useEffect(() => {
    fetchTodos();
    fetchStatusCounts();
  }, [fetchTodos, fetchStatusCounts]);

  useEffect(() => {
    if (!isTauri) return;

    let unlisten: (() => void) | null = null;

    const setupListener = async () => {
      const { listen } = await import('@tauri-apps/api/event');
      const { invoke } = await import('@tauri-apps/api/core');
      const { isPermissionGranted, requestPermission, sendNotification } = await import('@tauri-apps/plugin-notification');

      const unlistenFn = await listen<ResearchCompletePayload>('research_complete', async (event) => {
        const { todoId, content, source, startedAt } = event.payload;
        
        try {
          await invoke('cmd_save_research_result', {
            todoId,
            source,
            content,
            rawHtml: null,
            startedAt
          });

          await fetchTodos();
          await fetchStatusCounts();
          
          const todoDetail = await invoke<{ todo: { title: string } } | null>('cmd_get_todo_detail', { id: todoId });
          const title = todoDetail?.todo?.title || '调研任务';

          const hasPermission = await isPermissionGranted();
          if (!hasPermission) {
            const permission = await requestPermission();
            if (permission !== 'granted') return;
          }

          sendNotification({
            title: '调研完成',
            body: `"${title}" 的调研已完成，点击查看结果`,
            extra: { todoId }
          });
        } catch (err) {
          console.error('Failed to save research result:', err);
        }
      });
      unlisten = unlistenFn;
    };

    setupListener();

    return () => {
      if (unlisten) unlisten();
    };
  }, [fetchTodos, fetchStatusCounts]);

  useEffect(() => {
    if (!isTauri) return;

    let unsubscribe: (() => void) | null = null;

    const setupHandler = async () => {
      const { onAction } = await import('@tauri-apps/plugin-notification');
      const listener = await onAction((notification) => {
        const todoId = notification.extra?.todoId as string | undefined;
        if (todoId) {
          openDetailPanel(todoId);
        }
      });
      unsubscribe = () => listener.unregister();
    };

    setupHandler();

    return () => {
      if (unsubscribe) unsubscribe();
    };
  }, [openDetailPanel]);

  useEffect(() => {
    const handleOnline = () => setIsOnline(true);
    const handleOffline = () => setIsOnline(false);

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  return (
    <>
      {!isOnline && (
        <div className="network-offline-banner">
          <span>⚠️ 网络已断开，调研功能暂时不可用</span>
        </div>
      )}
      
      <TodoLayout 
        sidebar={<TodoSidebar />}
        list={<TodoList />}
        detail={<TodoDetail />}
      />

      {error && (
        <Toast 
          message={error} 
          type="error" 
          onClose={clearError} 
        />
      )}
    </>
  );
}
