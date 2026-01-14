declare namespace I {
  export type AppConf = {
    theme: 'light' | 'dark' | 'system';
    stay_on_top: boolean;
    ask_mode: boolean;
    mac_titlebar_hidden: boolean;
  }

  export interface SVG extends React.SVGProps<SVGSVGElement> {
    children?: React.ReactNode;
    size?: number;
    title?: string;
    action?: boolean;
    onClick?: (e: React.MouseEvent) => void;
  }

  export type TodoStatus = 'pending' | 'researching' | 'review' | 'done' | 'archived';

  export interface Todo {
    id: string;
    title: string;
    description: string | null;
    url: string | null;
    status: TodoStatus;
    created_at: string;
    updated_at: string;
  }

  export interface CreateTodoInput {
    title: string;
    description?: string;
    url?: string;
  }

  export interface UpdateTodoInput {
    title?: string;
    description?: string;
    url?: string;
    status?: TodoStatus;
  }

  export interface ResearchResult {
    id: string;
    todo_id: string;
    source: string;
    content: string | null;
    raw_html: string | null;
    started_at: string | null;
    completed_at: string | null;
    duration_seconds: number | null;
  }

  export interface TodoWithResearch {
    todo: Todo;
    research: ResearchResult | null;
  }

  export interface StatusCounts {
    pending: number;
    researching: number;
    review: number;
    done: number;
    archived: number;
    total: number;
  }
}