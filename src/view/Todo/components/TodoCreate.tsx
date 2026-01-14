import { useState, useRef, useEffect } from 'react';
import { useTodoStore } from '~/stores/todoStore';

export default function TodoCreate() {
  const [title, setTitle] = useState('');
  const [isExpanded, setIsExpanded] = useState(false);
  const [description, setDescription] = useState('');
  const [url, setUrl] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);
  const { createTodo, isLoading } = useTodoStore();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!title.trim()) return;

    await createTodo({
      title: title.trim(),
      description: description.trim() || undefined,
      url: url.trim() || undefined,
    });

    setTitle('');
    setDescription('');
    setUrl('');
    setIsExpanded(false);
    inputRef.current?.focus();
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey && !isExpanded) {
      handleSubmit(e);
    }
    if (e.key === 'Escape') {
      setIsExpanded(false);
      setDescription('');
      setUrl('');
    }
  };

  useEffect(() => {
    const handleGlobalKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'n') {
        e.preventDefault();
        inputRef.current?.focus();
      }
    };
    window.addEventListener('keydown', handleGlobalKeyDown);
    return () => window.removeEventListener('keydown', handleGlobalKeyDown);
  }, []);

  return (
    <form className="todo-create" onSubmit={handleSubmit}>
      <div className="create-main">
        <span className="create-icon">+</span>
        <input
          ref={inputRef}
          type="text"
          className="create-input"
          placeholder="添加新任务..."
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          onKeyDown={handleKeyDown}
          onFocus={() => setIsExpanded(true)}
          disabled={isLoading}
        />
        <button
          type="button"
          className="expand-toggle"
          onClick={() => setIsExpanded(!isExpanded)}
          title={isExpanded ? '收起详情' : '添加详情'}
        >
          {isExpanded ? '▲' : '▼'}
        </button>
      </div>

      {isExpanded && (
        <div className="create-expanded">
          <textarea
            className="create-description"
            placeholder="添加描述（可选）..."
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            rows={2}
          />
          <input
            type="url"
            className="create-url"
            placeholder="添加链接（可选）..."
            value={url}
            onChange={(e) => setUrl(e.target.value)}
          />
          <div className="create-actions">
            <button
              type="button"
              className="btn-cancel"
              onClick={() => {
                setIsExpanded(false);
                setDescription('');
                setUrl('');
              }}
            >
              取消
            </button>
            <button
              type="submit"
              className="btn-submit"
              disabled={!title.trim() || isLoading}
            >
              {isLoading ? '添加中...' : '添加任务'}
            </button>
          </div>
        </div>
      )}
    </form>
  );
}
