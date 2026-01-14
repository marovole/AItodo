import clsx from 'clsx';
import { useTodoStore } from '~/stores/todoStore';

interface TodoItemProps {
  todo: I.Todo;
  isSelected: boolean;
}

const statusIcons: Record<I.TodoStatus, string> = {
  pending: '○',
  researching: '🔄',
  review: '●',
  done: '✓',
  archived: '📁',
};

export default function TodoItem({ todo, isSelected }: TodoItemProps) {
  const { openDetailPanel, updateTodo } = useTodoStore();

  const handleClick = () => {
    openDetailPanel(todo.id);
  };

  const handleStatusToggle = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (todo.status === 'review') {
      updateTodo(todo.id, { status: 'done' });
    } else if (todo.status === 'done') {
      updateTodo(todo.id, { status: 'review' });
    }
  };

  const formatDate = (dateStr: string): string => {
    const date = new Date(dateStr);
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const days = Math.floor(diff / (1000 * 60 * 60 * 24));
    
    if (days === 0) return '今天';
    if (days === 1) return '昨天';
    if (days < 7) return `${days}天前`;
    return date.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' });
  };

  return (
    <div
      className={clsx('todo-item', {
        selected: isSelected,
        'status-done': todo.status === 'done',
        'status-archived': todo.status === 'archived',
        'status-researching': todo.status === 'researching',
      })}
      onClick={handleClick}
    >
      <button
        className={clsx('status-indicator', `status-${todo.status}`)}
        onClick={handleStatusToggle}
        title={todo.status === 'review' ? '标记完成' : ''}
      >
        {statusIcons[todo.status]}
      </button>

      <div className="todo-content">
        <h3 className="todo-title">{todo.title}</h3>
        {(todo.description || todo.url) && (
          <p className="todo-subtitle">
            {todo.description || todo.url}
          </p>
        )}
      </div>

      <div className="todo-meta">
        {todo.status === 'researching' && (
          <span className="research-badge">调研中...</span>
        )}
        <span className="todo-date">{formatDate(todo.created_at)}</span>
      </div>
    </div>
  );
}
