import '../src/design/styles.css';
  import 'katex/dist/katex.min.css';
import { ThemeProvider } from '../src/design/ThemeProvider';
import { RootShell } from '../src/app-shell/RootShell'
import AppPage from '../src/app-shell/AppPage'
import Footer from '../src/app-shell/Footer'

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="ja">
  <body className="bg-surface text-on-surface" style={{ margin: 0, fontFamily: 'system-ui, sans-serif', minHeight: '100vh' }}>
        <ThemeProvider>
          <RootShell>
            <AppPage footer={<Footer />}>{children}</AppPage>
          </RootShell>
        </ThemeProvider>
      </body>
    </html>
  )
}
