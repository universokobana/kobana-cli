import React, {useEffect, useRef, useState, type ReactNode} from 'react';
import {GripVertical} from 'lucide-react';

import styles from './styles.module.css';

const STORAGE_KEY = 'kobana-assistant-width';
const DEFAULT_WIDTH = 448; // 28rem
const MIN_WIDTH = 320;
const KEY_STEP = 32;

// Never wider than 60% of the window, so the article keeps room.
const clamp = (w: number) => Math.round(Math.min(Math.max(w, MIN_WIDTH), Math.max(MIN_WIDTH, window.innerWidth * 0.6)));

const readStored = (): number => {
  try {
    const v = Number(window.localStorage.getItem(STORAGE_KEY));
    return Number.isFinite(v) && v > 0 ? v : DEFAULT_WIDTH;
  } catch {
    return DEFAULT_WIDTH;
  }
};

const store = (w: number) => {
  try {
    window.localStorage.setItem(STORAGE_KEY, String(w));
  } catch {
    // Private mode or blocked storage: the width just isn't remembered.
  }
};

// Drag handle on the panel's left edge. The width lives in the
// `--assistant-width` CSS variable on <html>, which both the panel and the
// space the page makes for it (styles.module.css) read.
export default function ResizeHandle({label}: {label: string}): ReactNode {
  const [width, setWidth] = useState(() => clamp(readStored()));
  const [dragging, setDragging] = useState(false);
  // Latest width for the pointer/keyboard handlers, kept in sync after render.
  const widthRef = useRef(width);

  useEffect(() => {
    widthRef.current = width;
    document.documentElement.style.setProperty('--assistant-width', `${width}px`);
  }, [width]);

  useEffect(() => {
    const onResize = () => setWidth((w) => clamp(w));
    window.addEventListener('resize', onResize);
    return () => {
      window.removeEventListener('resize', onResize);
      document.documentElement.style.removeProperty('--assistant-width');
    };
  }, []);

  const onPointerDown = (e: React.PointerEvent<HTMLDivElement>) => {
    if (e.button !== 0) return;
    e.preventDefault();
    e.currentTarget.setPointerCapture(e.pointerId);
    setDragging(true);
  };

  const onPointerMove = (e: React.PointerEvent<HTMLDivElement>) => {
    if (!dragging) return;
    // The panel is pinned to the right edge: its width is the distance from
    // the pointer to the right of the window.
    setWidth(clamp(window.innerWidth - e.clientX));
  };

  const endDrag = (e: React.PointerEvent<HTMLDivElement>) => {
    if (!dragging) return;
    e.currentTarget.releasePointerCapture(e.pointerId);
    setDragging(false);
    store(widthRef.current);
  };

  const resizeBy = (delta: number) => {
    const w = clamp(widthRef.current + delta);
    setWidth(w);
    store(w);
  };

  return (
    <div
      role="separator"
      aria-orientation="vertical"
      aria-label={label}
      aria-valuenow={width}
      aria-valuemin={MIN_WIDTH}
      tabIndex={0}
      title={label}
      className={styles.resize}
      data-dragging={dragging || undefined}
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={endDrag}
      onPointerCancel={endDrag}
      onDoubleClick={() => {
        setWidth(clamp(DEFAULT_WIDTH));
        store(DEFAULT_WIDTH);
      }}
      onKeyDown={(e) => {
        // Left widens (the panel grows toward the article), right narrows.
        if (e.key === 'ArrowLeft') resizeBy(KEY_STEP);
        else if (e.key === 'ArrowRight') resizeBy(-KEY_STEP);
        else return;
        e.preventDefault();
      }}>
      <span className={styles.grip}>
        <GripVertical size={12} aria-hidden />
      </span>
    </div>
  );
}
