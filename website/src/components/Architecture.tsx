import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { Lock, Search, Database, FileText, HardDrive, Cpu, Layers, Zap, ArrowDown, ArrowRight } from 'lucide-react'

const colorMap = {
  emerald: { bg: 'bg-emerald-500/10', text: 'text-emerald-400', border: 'border-emerald-500/20', ring: 'ring-emerald-500/20' },
  cyan: { bg: 'bg-cyan-500/10', text: 'text-cyan-400', border: 'border-cyan-500/20', ring: 'ring-cyan-500/20' },
  violet: { bg: 'bg-violet-500/10', text: 'text-violet-400', border: 'border-violet-500/20', ring: 'ring-violet-500/20' },
}

const flowData = {
  write: {
    trusted: [
      { icon: Cpu, label: 'Application', desc: 'Your code. Plaintext lives only here.' },
      { icon: Lock, label: 'PickleClient', desc: 'Encrypts with AES-256-GCM. Derives HMAC search tokens.' },
    ],
    untrusted: [
      { icon: Database, label: 'PickleEngine', desc: 'Orchestrates all components. Never sees keys.', color: 'cyan' as const },
      { icon: FileText, label: 'WAL Log', desc: 'Crash-safe append-only journal.', color: 'violet' as const },
      { icon: Layers, label: 'Page Manager', desc: 'Writes to 4KB slotted pages.', color: 'cyan' as const },
      { icon: Database, label: 'Buffer Pool', desc: 'Dirty page cache with FIFO eviction.', color: 'cyan' as const },
      { icon: HardDrive, label: 'Encrypted Storage', desc: 'Only ciphertext on disk.', color: 'emerald' as const },
    ],
    flow: ['Application', 'PickleClient', 'PickleEngine', 'WAL Log', 'Page Manager', 'Buffer Pool', 'Encrypted Storage'],
  },
  search: {
    trusted: [
      { icon: Cpu, label: 'Application', desc: 'Calls db.search("email").eq(...).' },
      { icon: Lock, label: 'PickleClient', desc: 'Derives HMAC-SHA256 search token.' },
    ],
    untrusted: [
      { icon: Database, label: 'PickleEngine', desc: 'Forwards token to index.', color: 'cyan' as const },
      { icon: Search, label: 'HashIndex', desc: 'Returns RecordIds — no decryption.', color: 'violet' as const },
      { icon: HardDrive, label: 'Storage', desc: 'Reads encrypted payloads.', color: 'cyan' as const },
      { icon: Lock, label: 'PickleClient', desc: 'Decrypts client-side.', color: 'emerald' as const },
      { icon: Cpu, label: 'Application', desc: 'Receives plaintext.', color: 'emerald' as const },
    ],
    flow: ['Application', 'PickleClient', 'PickleEngine', 'HashIndex', 'Storage', 'PickleClient', 'Application'],
  },
}

export function Architecture() {
  const [mode, setMode] = useState<'write' | 'search'>('write')
  const data = flowData[mode]

  return (
    <section id="architecture" className="relative py-16 lg:py-24">
      <div className="mx-auto max-w-5xl px-6 lg:px-8">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          className="max-w-2xl mb-10"
        >
          <span className="text-xs uppercase tracking-widest text-emerald-400/70 font-medium">Architecture</span>
          <h2 className="mt-4 font-heading text-3xl sm:text-4xl lg:text-5xl font-bold text-white tracking-tight">
            How it <span className="text-emerald-400">works</span>
          </h2>
          <p className="mt-3 text-pickle-400 text-lg max-w-lg">
            Data is encrypted before it leaves your application. The engine never sees plaintext.
          </p>
        </motion.div>

        {/* Mode toggle */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          className="flex items-center gap-2 bg-surface rounded-xl border border-border p-1 w-fit mb-10"
        >
          <button
            onClick={() => setMode('write')}
            className={`flex items-center gap-2 px-4 py-2 text-xs font-medium rounded-lg transition-all ${
              mode === 'write' ? 'bg-emerald-500/20 text-emerald-400 shadow-sm' : 'text-pickle-400 hover:text-white'
            }`}
          >
            <Zap className="w-3.5 h-3.5" />
            Write Flow
          </button>
          <button
            onClick={() => setMode('search')}
            className={`flex items-center gap-2 px-4 py-2 text-xs font-medium rounded-lg transition-all ${
              mode === 'search' ? 'bg-cyan-500/20 text-cyan-400 shadow-sm' : 'text-pickle-400 hover:text-white'
            }`}
          >
            <Search className="w-3.5 h-3.5" />
            Search Flow
          </button>
        </motion.div>

        {/* Diagram */}
        <div className="relative">
          <div className="absolute -inset-6 bg-gradient-to-br from-emerald-500/5 via-transparent to-cyan-500/5 rounded-3xl blur-2xl opacity-50" />
          <div className="relative rounded-2xl border border-border bg-surface/50 overflow-hidden shadow-[0_4px_60px_-20px_rgba(0,0,0,0.5)]">
          <div className="p-6 lg:p-8">
            <AnimatePresence mode="wait">
              <motion.div
                key={mode}
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                transition={{ duration: 0.15 }}
                className="grid lg:grid-cols-[1fr_auto_1fr] gap-6 lg:gap-4 items-start"
              >
                {/* Trusted zone */}
                <div className="rounded-2xl border-2 border-emerald-500/20 bg-emerald-500/[0.02] p-5">
                  <div className="text-[10px] font-mono text-emerald-400/60 mb-4 uppercase tracking-wider flex items-center gap-2">
                    <Lock className="w-3 h-3" />
                    Trusted Zone
                  </div>
                  <div className="space-y-3">
                    {data.trusted.map((item, i) => {
                      const Icon = item.icon
                      return (
                        <motion.div
                          key={`${mode}-t-${i}`}
                          initial={{ opacity: 0, x: -12 }}
                          animate={{ opacity: 1, x: 0 }}
                          transition={{ delay: i * 0.1, duration: 0.3 }}
                          className="flex items-center gap-3 p-3 rounded-xl bg-emerald-500/5 border border-emerald-500/10"
                        >
                          <div className="w-9 h-9 rounded-lg flex items-center justify-center bg-emerald-500/10 shrink-0">
                            <Icon className="w-4 h-4 text-emerald-400" />
                          </div>
                          <div className="min-w-0">
                            <div className="text-sm font-medium text-white">{item.label}</div>
                            <div className="text-xs text-pickle-400 truncate">{item.desc}</div>
                          </div>
                        </motion.div>
                      )
                    })}
                  </div>
                </div>

                {/* Center arrows / flow line */}
                <div className="hidden lg:flex flex-col items-center gap-2 py-8">
                  <div className="w-px h-8 bg-gradient-to-b from-emerald-500/40 to-cyan-500/40" />
                  <motion.div
                    animate={{ y: [0, 4, 0] }}
                    transition={{ duration: 1.5, repeat: Infinity, ease: 'easeInOut' }}
                  >
                    <ArrowRight className="w-5 h-5 text-pickle-500" />
                  </motion.div>
                  <div className="w-px h-8 bg-gradient-to-b from-cyan-500/40 to-emerald-500/40" />
                  <div className="text-[9px] font-mono text-pickle-500 text-center leading-tight max-w-[60px]">
                    encrypted only
                  </div>
                  <div className="w-px h-8 bg-gradient-to-b from-emerald-500/40 to-cyan-500/40" />
                </div>

                {/* Mobile arrow */}
                <div className="flex lg:hidden justify-center py-1">
                  <motion.div animate={{ y: [0, 4, 0] }} transition={{ duration: 1.5, repeat: Infinity, ease: 'easeInOut' }}>
                    <ArrowDown className="w-4 h-4 text-pickle-500" />
                  </motion.div>
                </div>

                {/* Untrusted zone */}
                <div className="rounded-2xl border-2 border-cyan-500/15 bg-cyan-500/[0.01] p-5">
                  <div className="text-[10px] font-mono text-cyan-400/50 mb-4 uppercase tracking-wider flex items-center gap-2">
                    <Database className="w-3 h-3" />
                    Untrusted Zone
                  </div>
                  <div className="space-y-3">
                    {data.untrusted.map((item, i) => {
                      const Icon = item.icon
                      const c = colorMap[item.color]
                      return (
                        <motion.div
                          key={`${mode}-u-${i}`}
                          initial={{ opacity: 0, x: 12 }}
                          animate={{ opacity: 1, x: 0 }}
                          transition={{ delay: 0.15 + i * 0.08, duration: 0.3 }}
                          className={`flex items-center gap-3 p-3 rounded-xl ${c.bg} border ${c.border}`}
                        >
                          <div className="w-9 h-9 rounded-lg flex items-center justify-center bg-pickle-900/60 shrink-0">
                            <Icon className={`w-4 h-4 ${c.text}`} />
                          </div>
                          <div className="min-w-0">
                            <div className="text-sm font-medium text-white">{item.label}</div>
                            <div className="text-xs text-pickle-400 truncate">{item.desc}</div>
                          </div>
                        </motion.div>
                      )
                    })}
                  </div>
                </div>
              </motion.div>
            </AnimatePresence>
          </div>

          {/* Bottom legend */}
          <div className="border-t border-border px-6 lg:px-8 py-4 bg-pickle-900/30">
            <div className="flex flex-wrap items-center gap-4 sm:gap-6">
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 rounded-full bg-emerald-400" />
                <span className="text-[10px] font-mono text-pickle-400 uppercase tracking-wider">Client-side crypto</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 rounded-full bg-cyan-400" />
                <span className="text-[10px] font-mono text-pickle-400 uppercase tracking-wider">Engine (untrusted)</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 rounded-full bg-violet-400" />
                <span className="text-[10px] font-mono text-pickle-400 uppercase tracking-wider">Blind index</span>
              </div>
            </div>
          </div>
          </div>
        </div>

        {/* Takeaway */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          className="mt-8 rounded-2xl border border-border bg-surface p-6 text-center"
        >
          <div className="text-sm text-pickle-300">
            {mode === 'write' ? (
              <>Plaintext enters at <span className="text-emerald-400 font-medium">Application</span> and is immediately encrypted. Only ciphertext reaches the engine, WAL, and disk.</>
            ) : (
              <>Searches use blind tokens — <span className="text-cyan-400 font-medium">no decryption happens in the engine</span>. Only your client SDK decrypts results.</>
            )}
          </div>
        </motion.div>
      </div>
    </section>
  )
}
