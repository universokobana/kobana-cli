import React, {type ReactNode} from 'react';
import {AssistantProvider} from '../components/Assistant/context';

// Holds the docs assistant's conversation above every page, so it survives
// client-side navigation. The panel renders in theme/Layout.
export default function Root({children}: {children: ReactNode}): ReactNode {
  return <AssistantProvider>{children}</AssistantProvider>;
}
