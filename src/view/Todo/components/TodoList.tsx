import { useTodoStore } from '~/stores/todoStore';
import TodoItem from './TodoItem';
import TodoCreate from './TodoCreate';

export default function TodoList() {
  const { todos, selectedTodo, isLoading, currentFilter } = useTodoStore();

  const getFilterTitle = (): string => {
    const titles: Record<string, string> = {
      all: '全部任务',
      pending: '待调研',
      researching: '调研中',
      review: '待审阅',
      done: '已完成',
      archived: '归档',
    };
    return titles[currentFilter] || '全部任务';
  };

  return (
    <div className="todo-list-inner">
      <header className="list-header">
        <h2 className="list-title">{getFilterTitle()}</h2>
        {isLoading && <span className="loading-indicator">加载中...</span>}
      </header>

      <div className="todo-list">
        {todos.length === 0 ? (
          <div className="empty-state">
            <span className="empty-icon">📭</span>
            <p className="empty-text">暂无任务</p>
            <p className="empty-hint">在下方添加新任务开始</p>
          </div>
        ) : (
          todos.map((todo) => (
            <TodoItem
              key={todo.id}
              todo={todo}
              isSelected={selectedTodo?.todo.id === todo.id}
            />
          ))
        )}
      </div>

      <TodoCreate />
    </div>
  );
}
