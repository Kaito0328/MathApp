import '../src/design/styles.css';
  import 'katex/dist/katex.min.css';
import { ThemeProvider } from '../src/design/ThemeProvider';
import { RootShell } from '../src/app-shell/RootShell'

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="ja">
      <body style={{ margin: 0, fontFamily: 'system-ui, sans-serif' }}>
        <ThemeProvider>
          <RootShell>{children}</RootShell>
        </ThemeProvider>
      </body>
    </html>
  )
}
