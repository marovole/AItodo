import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { useState } from 'react';
import { openUrl } from '@tauri-apps/plugin-opener';

const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

interface MarkdownViewerProps {
  content: string;
}

export default function MarkdownViewer({ content }: MarkdownViewerProps) {
  const [isCollapsed, setIsCollapsed] = useState(content.length > 1000);

  const handleLinkClick = async (e: React.MouseEvent<HTMLAnchorElement>) => {
    const href = e.currentTarget.getAttribute('href');
    if (!href || (!href.startsWith('http') && !href.startsWith('https'))) return;

    if (isTauri) {
      e.preventDefault();
      await openUrl(href);
    }
  };

  return (
    <div className="markdown-viewer-container">
      <div className={`markdown-content ${isCollapsed ? 'collapsed' : ''}`}>
        <ReactMarkdown 
          remarkPlugins={[remarkGfm]}
          components={{
            a: ({ node, ...props }) => (
              <a {...props} onClick={handleLinkClick} target="_blank" rel="noopener noreferrer" />
            ),
            code: ({ node, inline, className, children, ...props }: any) => {
              const match = /language-(\w+)/.exec(className || '');
              return !inline && match ? (
                <pre className={className}>
                  <code {...props} className={className}>
                    {children}
                  </code>
                </pre>
              ) : (
                <code {...props} className={className}>
                  {children}
                </code>
              );
            }
          }}
        >
          {content}
        </ReactMarkdown>
      </div>
      
      {content.length > 1000 && (
        <button 
          className="expand-collapse-btn" 
          onClick={() => setIsCollapsed(!isCollapsed)}
        >
          {isCollapsed ? '展开全部' : '收起内容'}
        </button>
      )}
    </div>
  );
}
