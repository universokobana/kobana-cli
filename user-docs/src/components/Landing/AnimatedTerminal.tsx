import React, {useEffect, useState, type ReactNode} from 'react';
import {Pause, Play} from 'lucide-react';
import styles from './AnimatedTerminal.module.css';

interface Demo {
  command: string;
  lines: string[];
}

// Illustrative output, in the shape the CLI prints (JSON, trimmed with --fields).
const DEMOS: Demo[] = [
  {
    command: 'kobana billing invoices list --fields id,status,total',
    lines: ['{ "data": [', '  { "id": "inv_01", "status": "paid", "total": 1490.00 },', '  { "id": "inv_02", "status": "open", "total": 320.50 }', '] }'],
  },
  {
    command: 'kobana banking bank-billets list --fields id,status,amount',
    lines: ['{ "data": [', '  { "id": 8412, "status": "opened", "amount": 250.00 },', '  { "id": 8413, "status": "paid", "amount": 1200.00 }', '] }'],
  },
  {
    command: 'kobana finance payables list --fields id,status,amount',
    lines: ['{ "data": [', '  { "id": "pay_77", "status": "scheduled", "amount": 890.90 },', '  { "id": "pay_78", "status": "paid", "amount": 45.00 }', '] }'],
  },
  {
    command: 'kobana kadu tasks list --fields id,status',
    lines: ['{ "data": [', '  { "id": "tsk_5", "status": "running" },', '  { "id": "tsk_6", "status": "done" }', '] }'],
  },
];

const TYPE_MS = 45;
const LINE_MS = 280;
const HOLD_MS = 2600;

interface Props {
  title: string;
  pause: string;
  play: string;
}

export default function AnimatedTerminal({title, pause, play}: Props): ReactNode {
  const [demo, setDemo] = useState(0);
  const [typed, setTyped] = useState(0);
  const [shown, setShown] = useState(0);
  const [paused, setPaused] = useState(false);
  const [reduced, setReduced] = useState(false);

  useEffect(() => {
    setReduced(window.matchMedia('(prefers-reduced-motion: reduce)').matches);
  }, []);

  const current = DEMOS[demo];
  const done = typed >= current.command.length && shown >= current.lines.length;
  const frozen = paused || reduced;

  useEffect(() => {
    if (frozen) return undefined;
    let wait: number;
    let step: () => void;
    if (typed < current.command.length) {
      wait = TYPE_MS;
      step = () => setTyped((n) => n + 1);
    } else if (shown < current.lines.length) {
      wait = LINE_MS;
      step = () => setShown((n) => n + 1);
    } else {
      wait = HOLD_MS;
      step = () => {
        setDemo((d) => (d + 1) % DEMOS.length);
        setTyped(0);
        setShown(0);
      };
    }
    const id = window.setTimeout(step, wait);
    return () => window.clearTimeout(id);
  }, [frozen, typed, shown, current]);

  // Reduced motion: show the first demo complete, never animate.
  const command = reduced ? current.command : current.command.slice(0, typed);
  const lines = reduced ? current.lines : current.lines.slice(0, shown);
  const typing = !reduced && !paused && !done && typed < current.command.length;

  return (
    <div className={styles.terminal} role="img" aria-label={current.command}>
      <div className={styles.bar}>
        <span /> <span /> <span />
        <em>{title}</em>
        {reduced ? null : (
          <button type="button" className={styles.toggle} onClick={() => setPaused((p) => !p)} aria-label={paused ? play : pause}>
            {paused ? <Play size={14} aria-hidden /> : <Pause size={14} aria-hidden />}
          </button>
        )}
      </div>
      <pre aria-hidden>
        <span className={styles.prompt}>$</span> {command}
        <span className={typing || (!done && !frozen) ? styles.caretSolid : styles.caret} />
        {lines.map((line, i) => (
          <React.Fragment key={`${demo}-${i}`}>
            {'\n'}
            <span className={styles.line}>{line}</span>
          </React.Fragment>
        ))}
      </pre>
    </div>
  );
}
