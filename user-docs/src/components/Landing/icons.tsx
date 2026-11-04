import React, {type ReactNode} from 'react';
import {ChatGptIcon, ClaudeIcon, CursorIcon} from '../PageActions/icons';

// The four-pointed star of Gemini, simplified (the brand mark is not in
// simple-icons). Claude, Cursor and OpenAI come from PageActions/icons.
export function GeminiIcon({size = 16}: {size?: number}): ReactNode {
  return (
    <svg width={size} height={size} viewBox="0 0 24 24" fill="currentColor" aria-hidden>
      <path d="M12 0c.6 6.4 5.6 11.4 12 12-6.4.6-11.4 5.6-12 12-.6-6.4-5.6-11.4-12-12C6.4 11.4 11.4 6.4 12 0Z" />
    </svg>
  );
}

export {ChatGptIcon, ClaudeIcon, CursorIcon};
