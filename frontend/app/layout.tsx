import '../src/design/styles.css';
  import 'katex/dist/katex.min.css';
import { ThemeProvider } from '../src/design/ThemeProvider';

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="ja">
      <body style={{ margin: 0, padding: 16, fontFamily: 'system-ui, sans-serif' }}>
        <ThemeProvider>{children}</ThemeProvider>
      </body>
    </html>
  )
}
