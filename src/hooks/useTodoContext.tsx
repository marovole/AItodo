import React, { createContext, useContext, useReducer, ReactNode } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Todo, TodoStatus, TodoPriority, CreateTodoRequest, UpdateTodoRequest, TodoFilter } from '../models';

interface TodoState {
    todos: Todo[];
    loading: boolean;
    error: string | null;
    selectedTodoId: string | null;
    filter: TodoFilter;
}

type TodoAction =
    | { type: 'SET_LOADING'; payload: boolean }
    | { type: 'SET_ERROR'; payload: string | null }
    | { type: 'SET_TODOS'; payload: Todo[] }
    | { type: 'ADD_TODO'; payload: Todo }
    | { type: 'UPDATE_TODO'; payload: { id: string; todo: Partial<Todo> } }
    | { type: 'DELETE_TODO'; payload: string }
    | { type: 'SET_SELECTED_TODO'; payload: string | null }
    | { type: 'SET_FILTER'; payload: TodoFilter };

const todoReducer = (state: TodoState, action: TodoAction): TodoState => {
    switch (action.type) {
        case 'SET_LOADING':
            return { ...state, loading: action.payload };
        
        case 'SET_ERROR':
            return { ...state, error: action.payload };
        
        case 'SET_TODOS':
            return { ...state, todos: action.payload, loading: false, error: null };
        
        case 'ADD_TODO':
            return { ...state, todos: [...state.todos, action.payload] };
        
        case 'UPDATE_TODO':
            return {
                ...state,
                todos: state.todos.map(todo =>
                    todo.id === action.payload.id
                        ? { ...todo, ...action.payload.todo }
                        : todo
                )
            };
        
        case 'DELETE_TODO':
            return {
                ...state,
                todos: state.todos.filter(todo => todo.id !== action.payload),
                selectedTodoId: state.selectedTodoId === action.payload ? null : state.selectedTodoId
            };
        
        case 'SET_SELECTED_TODO':
            return { ...state, selectedTodoId: action.payload };
        
        case 'SET_FILTER':
            return { ...state, filter: action.payload };
        
        default:
            return state;
    }
};

const TodoContext = createContext<{
    state: TodoState;
    dispatch: React.Dispatch<TodoAction>;
} | undefined);

export const useTodoContext = () => {
    const context = useContext(TodoContext);
    if (!context) {
        throw new Error('useTodoContext must be used within TodoProvider');
    }
    return context;
};

export const TodoProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
    const [state, dispatch] = useReducer(todoReducer, {
        todos: [],
        loading: false,
        error: null,
        selectedTodoId: null,
        filter: {
            status: undefined,
            search: '',
            priority: undefined,
            limit: undefined,
            offset: undefined,
        }
    });

    React.useEffect(() => {
        loadTodos(dispatch);
    }, []);

    const value = { state, dispatch };
    return <TodoContext.Provider value={value}>{children}</TodoContext.Provider>;
};

async function loadTodos(dispatch: React.Dispatch<TodoAction>) {
    try {
        dispatch({ type: 'SET_LOADING', payload: true });
        const todos = await invoke<Todo[]>('list_todos', {
            filter: {
                status: undefined,
                search: undefined,
                priority: undefined,
                limit: undefined,
                offset: undefined,
            }
        });
        dispatch({ type: 'SET_TODOS', payload: todos });
    } catch (error) {
        dispatch({ type: 'SET_ERROR', payload: error as string });
        dispatch({ type: 'SET_LOADING', payload: false });
    }
}

export const useTodos = () => {
    const { state } = useTodoContext();
    return {
        todos: state.todos,
        loading: state.loading,
        error: state.error,
        selectedTodoId: state.selectedTodoId,
        filter: state.filter,
    };
};

export const useTodoActions = () => {
    const { dispatch } = useTodoContext();
    
    const createTodo = async (request: CreateTodoRequest) => {
        try {
            dispatch({ type: 'SET_LOADING', payload: true });
            const todo = await invoke<Todo>('create_todo', request);
            dispatch({ type: 'ADD_TODO', payload: todo });
        } catch (error) {
            dispatch({ type: 'SET_ERROR', payload: error as string });
        }
    };

    const updateTodo = async (id: string, request: UpdateTodoRequest) => {
        try {
            const updatedTodo = await invoke<Todo>('update_todo', { id, ...request });
            dispatch({ type: 'UPDATE_TODO', payload: { id, todo: updatedTodo } });
        } catch (error) {
            dispatch({ type: 'SET_ERROR', payload: error as string });
        }
    };

    const deleteTodo = async (id: string) => {
        try {
            await invoke<boolean>('delete_todo', { id });
            dispatch({ type: 'DELETE_TODO', payload: id });
        } catch (error) {
            dispatch({ type: 'SET_ERROR', payload: error as string });
        }
    };

    const getTodo = async (id: string) => {
        try {
            return await invoke<Todo>('get_todo', { id });
        } catch (error) {
            dispatch({ type: 'SET_ERROR', payload: error as string });
            return null;
        }
    };

    const setFilter = (filter: TodoFilter) => {
        dispatch({ type: 'SET_FILTER', payload: filter });
    };

    return {
        createTodo,
        updateTodo,
        deleteTodo,
        getTodo,
        setFilter,
    };
};