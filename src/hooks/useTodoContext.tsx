import { useEffect } from 'react';
import { useTodoStore } from '~/stores/todoStore';

export const TodoProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const fetchTodos = useTodoStore((state) => state.fetchTodos);
  const fetchStatusCounts = useTodoStore((state) => state.fetchStatusCounts);

  useEffect(() => {
    fetchTodos();
    fetchStatusCounts();
  }, [fetchTodos, fetchStatusCounts]);

  return <>{children}</>;
};

export const useTodoContext = () => {
  return useTodoStore();
};

export const useTodos = () => {
  const { todos, isLoading, error, selectedTodo, currentFilter } = useTodoStore();
  return {
    todos,
    loading: isLoading,
    error,
    selectedTodoId: selectedTodo?.todo.id ?? null,
    filter: currentFilter,
  };
};

export const useTodoActions = () => {
  const {
    createTodo,
    updateTodo,
    deleteTodo,
    fetchTodoDetail,
    setCurrentFilter,
  } = useTodoStore();

  return {
    createTodo,
    updateTodo,
    deleteTodo,
    getTodo: fetchTodoDetail,
    setFilter: setCurrentFilter,
  };
};
