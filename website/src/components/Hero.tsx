import { ArrowRight, BookOpen } from 'lucide-react'
import { GithubIcon } from './ui/Icons'

const terminalLines = [
  { text: 'pickledb init && pickledb insert user.json', delay: 0, type: 'input' as const },
  { text: '✓ Database initialized', delay: 0.8, type: 'success' as const },
  { text: '✓ Record encrypted (AES-256-GCM)', delay: 1.6, type: 'success' as const },
  { text: '✓ Search index created', delay: 2.4, type: 'success' as const },
  { text: '', delay: 0, type: 'spacer' as const },
  { text: 'pickledb search email=alice@example.com', delay: 3.2, type: 'input' as const },
  { text: '1 encrypted record found', delay: 4.4, type: 'output' as const },
  { text: 'Query time: 0.42ms', delay: 5.2, type: 'highlight' as const },
]

export function Hero() {
  return (
    <section className="relative min-h-screen flex items-center overflow-hidden">
      <div className="absolute inset-0">
        {/* Depth orbs */}
        <div className="orb w-[500px] h-[500px] bg-emerald-500 top-[15%] left-[10%] animate-drift" />
        <div className="orb w-[350px] h-[350px] bg-cyan-500 bottom-[20%] right-[15%] animate-drift" style={{ animationDelay: '3s' }} />
        <div className="orb w-[200px] h-[200px] bg-violet-500 top-[60%] left-[50%] animate-drift" style={{ animationDelay: '5s' }} />

        {/* Grid */}
        <div className="absolute inset-0 opacity-[0.025]">
          <div className="h-full w-full" style={{
            backgroundImage: 'radial-gradient(circle at 1px 1px, rgba(255,255,255,0.5) 1px, transparent 0)',
            backgroundSize: '40px 40px',
          }} />
        </div>
        <div className="absolute bottom-0 left-0 right-0 h-48 bg-gradient-to-t from-pickle-950 to-transparent" />
      </div>

      <div className="relative mx-auto max-w-7xl px-6 lg:px-8 pt-32 pb-20 lg:pt-40 lg:pb-32 w-full">
        <div className="grid lg:grid-cols-2 gap-12 lg:gap-16 items-center">
          <div className="space-y-6">
            <div className="space-y-4">
              <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-emerald-500/20 bg-emerald-500/10">
                <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse" />
                <span className="text-xs font-medium text-emerald-400">v0.4.0 — Pre-release</span>
              </div>
              <h1 className="font-heading text-[clamp(2.5rem,5vw,4.5rem)] font-bold leading-[1.05] tracking-tight text-white">
                Zero-Trust Embedded{' '}
                <span className="text-emerald-400">Database</span>
                <br />
                for Modern Apps
              </h1>
              <p className="text-lg text-pickle-400 max-w-md leading-relaxed">
                Encrypted by design. Searchable without exposing plaintext. 
                Built in Rust for performance and security at the edge.
              </p>
            </div>

            <div className="flex flex-wrap gap-3">
              <a
                href="#"
                className="inline-flex items-center gap-2 px-5 py-2.5 text-sm font-medium rounded-xl bg-emerald-500 text-white hover:bg-emerald-400 transition-all hover:shadow-lg hover:shadow-emerald-500/25 active:scale-[0.98]"
              >
                Get Started
                <ArrowRight className="w-3.5 h-3.5" />
              </a>
              <a
                href="https://github.com/seladb/pickledb"
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center gap-2 px-5 py-2.5 text-sm font-medium rounded-xl border border-border text-pickle-300 hover:text-white hover:border-pickle-400 transition-all active:scale-[0.98]"
              >
                <GithubIcon className="w-4 h-4" />
                GitHub
              </a>
              <a
                href="#"
                className="inline-flex items-center gap-2 px-5 py-2.5 text-sm font-medium rounded-xl text-pickle-400 hover:text-white transition-colors"
              >
                <BookOpen className="w-4 h-4" />
                Docs
              </a>
            </div>

            <div className="flex items-center gap-4 pt-2">
              <div className="flex -space-x-2">
                {['#10b960', '#3b82f6', '#8b5cf6'].map((color, i) => (
                  <div
                    key={i}
                    className="w-7 h-7 rounded-full border-2 border-pickle-950 flex items-center justify-center text-[10px] font-bold text-white"
                    style={{ backgroundColor: color }}
                  >
                    {['S', 'M', 'R'][i]}
                  </div>
                ))}
              </div>
              <div className="text-sm text-pickle-400">
                <span className="text-white font-semibold">1,200+</span> developers trust PickleDB
              </div>
            </div>
          </div>

          <div className="hidden lg:block perspective-card">
            <div className="relative terminal-depth">
              <div className="absolute -inset-4 bg-gradient-to-r from-emerald-500/8 via-cyan-500/5 to-violet-500/8 rounded-3xl blur-xl opacity-60" />
              <div className="relative rounded-2xl border border-border/60 bg-pickle-900/90 backdrop-blur-sm overflow-hidden shadow-[0_8px_60px_-12px_rgba(0,0,0,0.7)]">
                <div className="flex items-center gap-1.5 px-4 py-3 border-b border-border bg-pickle-800/50">
                  <div className="w-2.5 h-2.5 rounded-full bg-red-500/70" />
                  <div className="w-2.5 h-2.5 rounded-full bg-yellow-500/70" />
                  <div className="w-2.5 h-2.5 rounded-full bg-green-500/70" />
                  <span className="ml-3 text-[11px] text-pickle-400 font-mono">terminal — pickledb</span>
                </div>
                <div className="p-5 font-mono text-sm leading-relaxed min-h-[240px]">
                  {terminalLines.map((line, i) => {
                    if (line.type === 'spacer') return <div key={i} className="h-2" />
                    return (
                      <div
                        key={i}
                        className="terminal-line"
                        style={{ animationDelay: `${line.delay}s`, animationFillMode: 'backwards' }}
                      >
                        {line.type === 'input' && (
                          <div className="flex items-center">
                            <span className="text-emerald-400 mr-2 shrink-0">$</span>
                            <span className="text-white">{line.text}</span>
                          </div>
                        )}
                        {line.type === 'success' && (
                          <div className="text-emerald-400/80 ml-5 flex items-center gap-2">
                            <span>◆</span>
                            {line.text}
                          </div>
                        )}
                        {line.type === 'output' && (
                          <div className="text-pickle-300 ml-5">{line.text}</div>
                        )}
                        {line.type === 'highlight' && (
                          <div className="text-blue-400 ml-5">{line.text}</div>
                        )}
                      </div>
                    )
                  })}
                  <div className="flex items-center mt-2 ml-5">
                    <span className="inline-block w-2 h-4 bg-emerald-400 cursor-blink" />
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
