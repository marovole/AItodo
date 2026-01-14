import clsx from 'clsx';
import { useTodoStore } from '~/stores/todoStore';

const statusItems: { key: I.TodoStatus | 'all'; icon: string; label: string }[] = [
  { key: 'all', icon: '📋', label: '全部' },
  { key: 'pending', icon: '📥', label: '待调研' },
  { key: 'researching', icon: '🔍', label: '调研中' },
  { key: 'review', icon: '📝', label: '待审阅' },
  { key: 'done', icon: '✅', label: '已完成' },
  { key: 'archived', icon: '📁', label: '归档' },
];

export default function TodoSidebar() {
  const { currentFilter, setCurrentFilter, statusCounts, toggleSidebar, sidebarCollapsed } = useTodoStore();

  const getCount = (key: I.TodoStatus | 'all'): number => {
    if (key === 'all') return statusCounts.total - statusCounts.archived;
    return statusCounts[key as keyof I.StatusCounts] as number;
  };

  return (
    <nav className={clsx('todo-sidebar', { collapsed: sidebarCollapsed })}>
      <div className="sidebar-header">
        <h1 className={clsx('sidebar-title', { hidden: sidebarCollapsed })}>AI Todo</h1>
        <button 
          className="sidebar-toggle" 
          onClick={toggleSidebar}
          title={sidebarCollapsed ? '展开侧边栏' : '收起侧边栏'}
        >
          {sidebarCollapsed ? '›' : '‹'}
        </button>
      </div>

      <ul className="sidebar-nav">
        {statusItems.map((item) => (
          <li key={item.key}>
            <button
              className={clsx('nav-item', { active: currentFilter === item.key })}
              onClick={() => setCurrentFilter(item.key)}
              title={item.label}
            >
              <span className="nav-icon">{item.icon}</span>
              {!sidebarCollapsed && (
                <>
                  <span className="nav-label">{item.label}</span>
                  <span className="nav-count">{getCount(item.key)}</span>
                </>
              )}
            </button>
          </li>
        ))}
      </ul>

      <div className="sidebar-footer">
        <button className="nav-item settings-btn" title="设置">
          <span className="nav-icon">⚙️</span>
          {!sidebarCollapsed && <span className="nav-label">设置</span>}
        </button>
      </div>
    </nav>
  );
}
