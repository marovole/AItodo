import { useState, useEffect } from 'react';

interface ResearchProgressProps {
  startedAt: string;
  onCancel: () => void;
  timeoutMinutes?: number;
}

const DEFAULT_TIMEOUT_MINUTES = 10;

export default function ResearchProgress({ 
  startedAt, 
  onCancel,
  timeoutMinutes = DEFAULT_TIMEOUT_MINUTES 
}: ResearchProgressProps) {
  const [elapsed, setElapsed] = useState(0);

  const timeoutSeconds = timeoutMinutes * 60;
  const isTimedOut = elapsed >= timeoutSeconds;
  const isNearTimeout = elapsed >= timeoutSeconds * 0.8;

  useEffect(() => {
    const startTime = new Date(startedAt).getTime();
    
    const updateElapsed = () => {
      const now = Date.now();
      setElapsed(Math.floor((now - startTime) / 1000));
    };

    updateElapsed();
    const interval = setInterval(updateElapsed, 1000);

    return () => clearInterval(interval);
  }, [startedAt]);

  const formatTime = (seconds: number): string => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  return (
    <div className="research-progress">
      <div className="progress-animation">
        <div className="spinner"></div>
      </div>
      <div className="progress-info">
        <p className="progress-status">AI 正在调研中...</p>
        <p className="progress-time">已用时间: {formatTime(elapsed)}</p>
      </div>
      
      {isNearTimeout && !isTimedOut && (
        <div className="research-timeout-warning">
          <span className="warning-icon">⚠️</span>
          <span>调研时间较长，可能遇到问题</span>
        </div>
      )}
      
      {isTimedOut && (
        <div className="research-timeout-warning">
          <span className="warning-icon">⏰</span>
          <span>调研已超时（{timeoutMinutes}分钟），建议取消后重试</span>
        </div>
      )}
      
      <button className="btn-cancel-research" onClick={onCancel}>
        取消调研
      </button>
    </div>
  );
}
