import React, {createContext, useContext, useEffect, useMemo, useState, type ReactNode} from 'react';
import {useChat} from '@ai-sdk/react';
import {DefaultChatTransport} from 'ai';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import {TOOL_LOCALE, toLocale} from './labels';

type Chat = ReturnType<typeof useChat>;

type AssistantState = {
  open: boolean;
  setOpen: (open: boolean) => void;
  chat: Chat;
};

const AssistantContext = createContext<AssistantState | null>(null);

// Mounted in theme/Root, above the page layout: the conversation and the open
// state survive client-side navigation. The panel itself renders inside the
// layout (theme/Layout), where the theme providers its code blocks need exist.
export function AssistantProvider({children}: {children: ReactNode}): ReactNode {
  const {
    i18n: {currentLocale},
  } = useDocusaurusContext();
  const locale = TOOL_LOCALE[toLocale(currentLocale)];
  const [open, setOpen] = useState(false);

  const transport = useMemo(() => new DefaultChatTransport({api: '/api/assistant', body: {locale}}), [locale]);
  const chat = useChat({transport});

  // The page actions menu opens the panel without importing this module.
  useEffect(() => {
    const onAsk = () => setOpen(true);
    window.addEventListener('kobana:ask-ai', onAsk);
    return () => window.removeEventListener('kobana:ask-ai', onAsk);
  }, []);

  const value = useMemo(() => ({open, setOpen, chat}), [open, chat]);
  return <AssistantContext.Provider value={value}>{children}</AssistantContext.Provider>;
}

export function useAssistant(): AssistantState {
  const ctx = useContext(AssistantContext);
  if (!ctx) throw new Error('useAssistant outside AssistantProvider');
  return ctx;
}
