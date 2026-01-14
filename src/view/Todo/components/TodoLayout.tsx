import React from 'react';
import clsx from 'clsx';
import { useTodoStore } from '~/stores/todoStore';

interface TodoLayoutProps {
  sidebar: React.ReactNode;
  list: React.ReactNode;
  detail: React.ReactNode;
}

export default function TodoLayout({ sidebar, list, detail }: TodoLayoutProps) {
  const { sidebarCollapsed, detailPanelOpen } = useTodoStore();

  return (
    <div className={clsx('todo-layout', { 
      'sidebar-collapsed': sidebarCollapsed,
      'detail-open': detailPanelOpen 
    })}>
      <aside className="todo-sidebar-container">
        {sidebar}
      </aside>
      <main className="todo-list-container">
        {list}
      </main>
      <aside className={clsx('todo-detail-container', { open: detailPanelOpen })}>
        {detail}
      </aside>
    </div>
  );
}
