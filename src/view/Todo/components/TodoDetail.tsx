import { useState } from 'react';
import { useTodoStore } from '~/stores/todoStore';
import ResearchProgress from './ResearchProgress';
import MarkdownViewer from './MarkdownViewer';

export default function TodoDetail() {
  const { selectedTodo, closeDetailPanel, updateTodo, deleteTodo, startResearch, cancelResearch, isLoading } = useTodoStore();
  const [isEditing, setIsEditing] = useState(false);
  const [editTitle, setEditTitle] = useState('');
  const [editDescription, setEditDescription] = useState('');
  const [editUrl, setEditUrl] = useState('');
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);

  if (!selectedTodo) {
    return (
      <div className="detail-empty">
        <span className="empty-icon">👈</span>
        <p>选择一个任务查看详情</p>
      </div>
    );
  }

  const { todo, research } = selectedTodo;

  const handleStartEdit = () => {
    setEditTitle(todo.title);
    setEditDescription(todo.description || '');
    setEditUrl(todo.url || '');
    setIsEditing(true);
  };

  const handleSaveEdit = async () => {
    await updateTodo(todo.id, {
      title: editTitle.trim() || todo.title,
      description: editDescription.trim() || undefined,
      url: editUrl.trim() || undefined,
    });
    setIsEditing(false);
  };

  const handleCancelEdit = () => {
    setIsEditing(false);
  };

  const handleDelete = async () => {
    await deleteTodo(todo.id);
    setShowDeleteConfirm(false);
  };

  const handleStartResearch = () => {
    startResearch(todo.id);
  };

  const handleCancelResearch = () => {
    cancelResearch(todo.id);
  };

  const handleMarkDone = () => {
    updateTodo(todo.id, { status: 'done' });
  };

  const handleArchive = () => {
    updateTodo(todo.id, { status: 'archived' });
  };

  const copyToClipboard = async (text: string) => {
    await navigator.clipboard.writeText(text);
  };

  const formatDuration = (seconds: number): string => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}分${secs}秒`;
  };

  return (
    <div className="todo-detail-inner">
      <header className="detail-header">
        <button className="close-btn" onClick={closeDetailPanel} title="关闭">
          ✕
        </button>
        <div className="header-actions">
          {!isEditing && (
            <button className="edit-btn" onClick={handleStartEdit} title="编辑">
              ✏️
            </button>
          )}
          <button 
            className="delete-btn" 
            onClick={() => setShowDeleteConfirm(true)} 
            title="删除"
          >
            🗑️
          </button>
        </div>
      </header>

      <div className="detail-content">
        {isEditing ? (
          <div className="edit-form">
            <input
              type="text"
              className="edit-title"
              value={editTitle}
              onChange={(e) => setEditTitle(e.target.value)}
              placeholder="任务标题"
              autoFocus
            />
            <textarea
              className="edit-description"
              value={editDescription}
              onChange={(e) => setEditDescription(e.target.value)}
              placeholder="任务描述（可选）"
              rows={3}
            />
            <input
              type="url"
              className="edit-url"
              value={editUrl}
              onChange={(e) => setEditUrl(e.target.value)}
              placeholder="相关链接（可选）"
            />
            <div className="edit-actions">
              <button className="btn-cancel" onClick={handleCancelEdit}>取消</button>
              <button className="btn-save" onClick={handleSaveEdit} disabled={isLoading}>
                保存
              </button>
            </div>
          </div>
        ) : (
          <>
            <h2 className="detail-title">{todo.title}</h2>
            
            <div className="detail-status">
              <span className={`status-badge status-${todo.status}`}>
                {todo.status === 'pending' && '待调研'}
                {todo.status === 'researching' && '调研中'}
                {todo.status === 'review' && '待审阅'}
                {todo.status === 'done' && '已完成'}
                {todo.status === 'archived' && '已归档'}
              </span>
            </div>

            {todo.description && (
              <p className="detail-description">{todo.description}</p>
            )}

            {todo.url && (
              <a 
                href={todo.url} 
                className="detail-url" 
                target="_blank" 
                rel="noopener noreferrer"
              >
                🔗 {todo.url}
              </a>
            )}
          </>
        )}

        <div className="detail-section research-section">
          <h3 className="section-title">调研结果</h3>
          
          {todo.status === 'researching' && (
            <ResearchProgress 
              startedAt={research?.started_at || new Date().toISOString()}
              onCancel={handleCancelResearch}
            />
          )}

          {todo.status === 'pending' && (
            <div className="research-empty">
              <p>点击下方按钮开始 AI 调研</p>
              <button 
                className="btn-research" 
                onClick={handleStartResearch}
                disabled={isLoading}
              >
                🔬 开始调研
              </button>
            </div>
          )}

          {research?.content && (
            <div className="research-result">
              <div className="result-header">
                <span className="result-source">来源: {research.source}</span>
                {research.duration_seconds && (
                  <span className="result-duration">
                    耗时: {formatDuration(research.duration_seconds)}
                  </span>
                )}
                <button 
                  className="copy-btn" 
                  onClick={() => copyToClipboard(research.content || '')}
                  title="复制内容"
                >
                  📋 复制
                </button>
              </div>
              <div className="result-content">
                <MarkdownViewer content={research.content} />
              </div>
            </div>
          )}
        </div>

        <div className="detail-actions">
          {todo.status === 'review' && (
            <button className="btn-done" onClick={handleMarkDone}>
              ✅ 标记完成
            </button>
          )}
          {todo.status !== 'archived' && (
            <button className="btn-archive" onClick={handleArchive}>
              📁 归档
            </button>
          )}
        </div>

        <div className="detail-meta">
          <p>创建于: {new Date(todo.created_at).toLocaleString('zh-CN')}</p>
          <p>更新于: {new Date(todo.updated_at).toLocaleString('zh-CN')}</p>
        </div>
      </div>

      {showDeleteConfirm && (
        <div className="delete-confirm-overlay">
          <div className="delete-confirm-dialog">
            <p>确定要删除这个任务吗？</p>
            <p className="warning">此操作不可恢复</p>
            <div className="confirm-actions">
              <button className="btn-cancel" onClick={() => setShowDeleteConfirm(false)}>
                取消
              </button>
              <button className="btn-delete" onClick={handleDelete}>
                删除
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
