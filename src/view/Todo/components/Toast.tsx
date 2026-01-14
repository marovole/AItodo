import { useEffect } from 'react';

interface ToastProps {
  message: string;
  type?: 'error' | 'success' | 'info';
  onClose: () => void;
  duration?: number;
}

export default function Toast({ 
  message, 
  type = 'info', 
  onClose, 
  duration = 5000 
}: ToastProps) {
  useEffect(() => {
    const timer = setTimeout(onClose, duration);
    return () => clearTimeout(timer);
  }, [onClose, duration]);

  return (
    <div className={`todo-toast todo-toast-${type}`} onClick={onClose}>
      <span className="toast-message">{message}</span>
      <button className="toast-close">&times;</button>
    </div>
  );
}
