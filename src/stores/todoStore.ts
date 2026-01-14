import { create } from 'zustand';
import { invoke as tauriInvoke } from '@tauri-apps/api/core';

const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

type InvokeFn = <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;

const invoke: InvokeFn = isTauri ? tauriInvoke : createMockInvoke();

function createMockInvoke() {
  const mockTodos: I.Todo[] = [];
  let todoIdCounter = 1;

  return async <T>(cmd: string, args?: Record<string, unknown>): Promise<T> => {
    console.log('[Mock] invoke:', cmd, args);
    
    switch (cmd) {
      case 'cmd_get_todos': {
        const status = args?.status as string | null;
        if (status) {
          return mockTodos.filter(t => t.status === status) as T;
        }
        return [...mockTodos] as T;
      }
      
      case 'cmd_get_status_counts': {
        return {
          pending: mockTodos.filter(t => t.status === 'pending').length,
          researching: mockTodos.filter(t => t.status === 'researching').length,
          review: mockTodos.filter(t => t.status === 'review').length,
          done: mockTodos.filter(t => t.status === 'done').length,
          archived: mockTodos.filter(t => t.status === 'archived').length,
          total: mockTodos.length,
        } as T;
      }
      
      case 'cmd_create_todo': {
        const input = args?.input as I.CreateTodoInput;
        const todo: I.Todo = {
          id: `mock-${todoIdCounter++}`,
          title: input.title,
          description: input.description || null,
          url: input.url || null,
          status: 'pending',
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        };
        mockTodos.push(todo);
        return todo as T;
      }
      
      case 'cmd_update_todo': {
        const id = args?.id as string;
        const input = args?.input as I.UpdateTodoInput;
        const todo = mockTodos.find(t => t.id === id);
        if (todo) {
          if (input.title) todo.title = input.title;
          if (input.description !== undefined) todo.description = input.description;
          if (input.url !== undefined) todo.url = input.url;
          if (input.status) todo.status = input.status;
          todo.updated_at = new Date().toISOString();
          return todo as T;
        }
        return null as T;
      }
      
      case 'cmd_delete_todo': {
        const id = args?.id as string;
        const index = mockTodos.findIndex(t => t.id === id);
        if (index >= 0) {
          mockTodos.splice(index, 1);
          return true as T;
        }
        return false as T;
      }
      
      case 'cmd_get_todo_detail': {
        const id = args?.id as string;
        const todo = mockTodos.find(t => t.id === id);
        if (todo) {
          return { todo, research: null } as T;
        }
        return null as T;
      }
      
      case 'cmd_start_research':
      case 'cmd_cancel_research': {
        const id = args?.id as string;
        const todo = mockTodos.find(t => t.id === id);
        if (todo) {
          todo.status = cmd === 'cmd_start_research' ? 'researching' : 'pending';
          todo.updated_at = new Date().toISOString();
          return todo as T;
        }
        return null as T;
      }
      
      default:
        console.warn('[Mock] Unknown command:', cmd);
        return null as T;
    }
  };
}

interface TodoState {
  todos: I.Todo[];
  selectedTodo: I.TodoWithResearch | null;
  statusCounts: I.StatusCounts;
  currentFilter: I.TodoStatus | 'all';
  sidebarCollapsed: boolean;
  detailPanelOpen: boolean;
  isLoading: boolean;
  error: string | null;

  fetchTodos: (status?: I.TodoStatus | 'all') => Promise<void>;
  fetchTodoDetail: (id: string) => Promise<void>;
  fetchStatusCounts: () => Promise<void>;
  createTodo: (input: I.CreateTodoInput) => Promise<I.Todo | null>;
  updateTodo: (id: string, input: I.UpdateTodoInput) => Promise<void>;
  deleteTodo: (id: string) => Promise<void>;
  startResearch: (id: string) => Promise<void>;
  cancelResearch: (id: string) => Promise<void>;
  setCurrentFilter: (filter: I.TodoStatus | 'all') => void;
  toggleSidebar: () => void;
  openDetailPanel: (id: string) => void;
  closeDetailPanel: () => void;
  clearError: () => void;
}

export const useTodoStore = create<TodoState>((set, get) => ({
  todos: [],
  selectedTodo: null,
  statusCounts: { pending: 0, researching: 0, review: 0, done: 0, archived: 0, total: 0 },
  currentFilter: 'all',
  sidebarCollapsed: false,
  detailPanelOpen: false,
  isLoading: false,
  error: null,

  fetchTodos: async (status?: I.TodoStatus | 'all') => {
    set({ isLoading: true, error: null });
    try {
      const statusParam = status === 'all' ? null : status;
      const todos = await invoke<I.Todo[]>('cmd_get_todos', { status: statusParam });
      set({ todos, isLoading: false });
    } catch (error) {
      set({ error: String(error), isLoading: false });
    }
  },

  fetchTodoDetail: async (id: string) => {
    set({ isLoading: true, error: null });
    try {
      const detail = await invoke<I.TodoWithResearch | null>('cmd_get_todo_detail', { id });
      set({ selectedTodo: detail, isLoading: false, detailPanelOpen: !!detail });
    } catch (error) {
      set({ error: String(error), isLoading: false });
    }
  },

  fetchStatusCounts: async () => {
    try {
      const counts = await invoke<I.StatusCounts>('cmd_get_status_counts');
      set({ statusCounts: counts });
    } catch (error) {
      console.error('Failed to fetch status counts:', error);
    }
  },

  createTodo: async (input: I.CreateTodoInput) => {
    set({ isLoading: true, error: null });
    try {
      const todo = await invoke<I.Todo>('cmd_create_todo', { input });
      const { fetchTodos, currentFilter, fetchStatusCounts } = get();
      await fetchTodos(currentFilter);
      await fetchStatusCounts();
      set({ isLoading: false });
      return todo;
    } catch (error) {
      set({ error: String(error), isLoading: false });
      return null;
    }
  },

  updateTodo: async (id: string, input: I.UpdateTodoInput) => {
    set({ isLoading: true, error: null });
    try {
      await invoke<I.Todo | null>('cmd_update_todo', { id, input });
      const { fetchTodos, currentFilter, fetchStatusCounts, selectedTodo } = get();
      await fetchTodos(currentFilter);
      await fetchStatusCounts();
      if (selectedTodo?.todo.id === id) {
        const detail = await invoke<I.TodoWithResearch | null>('cmd_get_todo_detail', { id });
        set({ selectedTodo: detail });
      }
      set({ isLoading: false });
    } catch (error) {
      set({ error: String(error), isLoading: false });
    }
  },

  deleteTodo: async (id: string) => {
    set({ isLoading: true, error: null });
    try {
      await invoke<boolean>('cmd_delete_todo', { id });
      const { fetchTodos, currentFilter, fetchStatusCounts, selectedTodo } = get();
      await fetchTodos(currentFilter);
      await fetchStatusCounts();
      if (selectedTodo?.todo.id === id) {
        set({ selectedTodo: null, detailPanelOpen: false });
      }
      set({ isLoading: false });
    } catch (error) {
      set({ error: String(error), isLoading: false });
    }
  },

  startResearch: async (id: string) => {
    set({ isLoading: true, error: null });
    try {
      await invoke<I.Todo | null>('cmd_start_research', { id });
      const { fetchTodos, currentFilter, fetchStatusCounts, fetchTodoDetail } = get();
      await fetchTodos(currentFilter);
      await fetchStatusCounts();
      await fetchTodoDetail(id);
      set({ isLoading: false });
    } catch (error) {
      set({ error: String(error), isLoading: false });
    }
  },

  cancelResearch: async (id: string) => {
    set({ isLoading: true, error: null });
    try {
      await invoke<I.Todo | null>('cmd_cancel_research', { id });
      const { fetchTodos, currentFilter, fetchStatusCounts, fetchTodoDetail } = get();
      await fetchTodos(currentFilter);
      await fetchStatusCounts();
      await fetchTodoDetail(id);
      set({ isLoading: false });
    } catch (error) {
      set({ error: String(error), isLoading: false });
    }
  },

  setCurrentFilter: (filter: I.TodoStatus | 'all') => {
    set({ currentFilter: filter });
    get().fetchTodos(filter);
  },

  toggleSidebar: () => {
    set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed }));
  },

  openDetailPanel: (id: string) => {
    get().fetchTodoDetail(id);
  },

  closeDetailPanel: () => {
    set({ detailPanelOpen: false, selectedTodo: null });
  },

  clearError: () => {
    set({ error: null });
  },
}));
