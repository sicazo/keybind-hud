import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useRef, useState } from "react";

interface WorkStatus {
  clocked_in: boolean;
  start_ts: number | null;
  elapsed_secs: number;
  remaining_secs: number;
}

function fmt(secs: number): string {
  const abs = Math.abs(secs);
  const h = Math.floor(abs / 3600);
  const m = Math.floor((abs % 3600) / 60);
  return `${h}h ${m.toString().padStart(2, "0")}m`;
}

export function WorkTimer() {
  const [status, setStatus] = useState<WorkStatus | null>(null);
  const [settingTime, setSettingTime] = useState(false);
  const [timeInput, setTimeInput] = useState("");
  const [error, setError] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  const refresh = useCallback(() => {
    invoke<WorkStatus>("work_get_status").then(setStatus).catch(console.error);
  }, []);

  useEffect(() => {
    refresh();
    const id = setInterval(refresh, 30_000);
    return () => clearInterval(id);
  }, [refresh]);

  useEffect(() => {
    if (settingTime) inputRef.current?.focus();
  }, [settingTime]);

  const clockIn = () => invoke("work_clock_in").then(refresh).catch(console.error);
  const clockOut = () => invoke("work_clock_out").then(refresh).catch(console.error);

  const submitTime = () => {
    if (!/^\d{1,2}:\d{2}$/.test(timeInput.trim())) {
      setError("use HH:MM (24h)");
      return;
    }
    invoke("work_set_time", { time: timeInput.trim() })
      .then(() => { setSettingTime(false); setTimeInput(""); setError(""); refresh(); })
      .catch(() => setError("invalid time"));
  };

  const handleTimeKey = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") submitTime();
    if (e.key === "Escape") { setSettingTime(false); setTimeInput(""); setError(""); }
    e.stopPropagation();
  };

  if (!status) return null;

  const over = status.remaining_secs < 0;
  const pct = Math.min(100, (status.elapsed_secs / (7 * 3600 + 45 * 60)) * 100);

  return (
    <div className="work-timer">
      <div className="wt-header">
        <span className="wt-icon">󱎫</span>
        <span className="wt-label">work timer</span>
        {status.clocked_in && (
          <span className={`wt-time ${over ? "over" : ""}`}>
            {over ? `+${fmt(status.remaining_secs)} over` : `${fmt(status.remaining_secs)} left`}
          </span>
        )}
        {!status.clocked_in && <span className="wt-time muted">off</span>}
      </div>

      {status.clocked_in && (
        <div className="wt-bar-wrap">
          <div className="wt-bar" style={{ width: `${pct}%`, background: over ? "var(--green)" : "var(--accent)" }} />
        </div>
      )}

      {settingTime ? (
        <div className="wt-row">
          <input
            ref={inputRef}
            className="wt-input"
            placeholder="09:00"
            value={timeInput}
            onChange={(e) => { setTimeInput(e.target.value); setError(""); }}
            onKeyDown={handleTimeKey}
          />
          <button className="wt-btn" onClick={submitTime}>set</button>
          <button className="wt-btn muted" onClick={() => { setSettingTime(false); setTimeInput(""); setError(""); }}>cancel</button>
          {error && <span className="wt-error">{error}</span>}
        </div>
      ) : (
        <div className="wt-row">
          {!status.clocked_in && <button className="wt-btn accent" onClick={clockIn}>clock in now</button>}
          {status.clocked_in && <button className="wt-btn" onClick={clockOut}>clock out</button>}
          <button className="wt-btn" onClick={() => setSettingTime(true)}>set start time</button>
        </div>
      )}
    </div>
  );
}
