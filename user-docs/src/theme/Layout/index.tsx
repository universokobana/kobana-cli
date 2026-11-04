import React, {type ReactNode} from 'react';
import Layout from '@theme-original/Layout';
import type {Props} from '@theme/Layout';
import BrowserOnly from '@docusaurus/BrowserOnly';

// The assistant panel renders inside the layout: its code blocks use the
// theme's color-mode provider, which theme/Root sits above. The conversation
// itself lives in Root (components/Assistant/context).
export default function LayoutWithAssistant(props: Props): ReactNode {
  return (
    <Layout {...props}>
      {props.children}
      <BrowserOnly>
        {() => {
          const Panel = require('../../components/Assistant/Panel').default;
          return <Panel />;
        }}
      </BrowserOnly>
    </Layout>
  );
}
