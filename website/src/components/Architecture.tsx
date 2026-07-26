import { useState, useRef } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { Lock, Search, Database, FileText, HardDrive, Cpu, Layers, Zap, ArrowDown } from 'lucide-react'

interface Node {
  id: string
  label: string
  desc: string
  x: number
  y: number
  width: number
  height: number
  icon: typeof Lock
  color: string
  bg: string
  border: string
  zone: 'trusted' | 'untrusted'
}

// Two-column layout: trusted left, untrusted right
// Column center x: left=130, right=390
const nodes: Node[] = [
  {
    id: 'app',
    label: 'Application',
    desc: 'Your code calls the PickleDB client SDK. Plaintext data exists only here.',
    x: 25, y: 35, width: 210, height: 55,
    icon: Cpu, color: 'text-emerald-400', bg: 'bg-emerald-500/10', border: 'border-emerald-500/30',
    zone: 'trusted',
  },
  {
    id: 'client',
    label: 'PickleClient',
    desc: 'Client-side SDK. Encrypts data with AES-256-GCM and derives HMAC-SHA256 blind search tokens.',
    x: 25, y: 125, width: 210, height: 65,
    icon: Lock, color: 'text-emerald-400', bg: 'bg-emerald-500/10', border: 'border-emerald-500/20',
    zone: 'trusted',
  },
  {
    id: 'engine',
    label: 'PickleEngine',
    desc: 'Core orchestrator. Coordinates WAL, page manager, index, buffer pool. Never sees encryption keys.',
    x: 285, y: 35, width: 210, height: 55,
    icon: Database, color: 'text-blue-400', bg: 'bg-blue-500/10', border: 'border-blue-500/20',
    zone: 'untrusted',
  },
  {
    id: 'wal',
    label: 'WAL Log',
    desc: 'Append-only write-ahead log. All mutations are journaled before page writes for crash durability.',
    x: 285, y: 205, width: 210, height: 50,
    icon: FileText, color: 'text-purple-400', bg: 'bg-purple-500/10', border: 'border-purple-500/20',
    zone: 'untrusted',
  },
  {
    id: 'page-manager',
    label: 'Page Manager',
    desc: 'Manages slotted 4KB pages. Data grows upward from page bottom, slots grow downward from header.',
    x: 285, y: 280, width: 210, height: 50,
    icon: Layers, color: 'text-blue-400', bg: 'bg-blue-500/10', border: 'border-blue-500/20',
    zone: 'untrusted',
  },
  {
    id: 'buffer-pool',
    label: 'Buffer Pool',
    desc: 'In-memory page cache with FIFO eviction. Caches 1000 recently used 4KB pages. Tracks dirty pages.',
    x: 285, y: 355, width: 210, height: 50,
    icon: Database, color: 'text-emerald-400', bg: 'bg-emerald-500/10', border: 'border-emerald-500/20',
    zone: 'untrusted',
  },
  {
    id: 'index',
    label: 'HashIndex',
    desc: 'In-memory HashMap<SearchToken, Vec<RecordId>>. Enables blind search without decryption.',
    x: 285, y: 118, width: 210, height: 50,
    icon: Search, color: 'text-purple-400', bg: 'bg-purple-500/10', border: 'border-purple-500/20',
    zone: 'untrusted',
  },
  {
    id: 'storage',
    label: 'Encrypted Storage',
    desc: 'On-disk data.db file. Pages stored at fixed offsets: offset = page_id × 4096. Always encrypted.',
    x: 285, y: 440, width: 210, height: 65,
    icon: HardDrive, color: 'text-emerald-400', bg: 'bg-emerald-500/10', border: 'border-emerald-500/20',
    zone: 'untrusted',
  },
]

interface Edge {
  from: string
  to: string
  label: string
  write: boolean
  search: boolean
}

const edges: Edge[] = [
  { from: 'app', to: 'client', label: 'plaintext', write: true, search: true },
  { from: 'client', to: 'engine', label: 'InsertTuple{encrypted,tokens}', write: true, search: false },
  { from: 'engine', to: 'wal', label: 'durable append', write: true, search: false },
  { from: 'wal', to: 'page-manager', label: 'write page', write: true, search: false },
  { from: 'page-manager', to: 'buffer-pool', label: 'cache page', write: true, search: false },
  { from: 'buffer-pool', to: 'storage', label: 'flush dirty pages', write: true, search: false },
  { from: 'engine', to: 'index', label: 'index token', write: true, search: true },
  { from: 'index', to: 'engine', label: 'Vec<RecordId>', write: false, search: true },
  { from: 'engine', to: 'storage', label: 'read payloads', write: false, search: true },
  { from: 'engine', to: 'client', label: 'EncryptedPayload', write: false, search: true },
]

function center(node: Node) {
  return { cx: node.x + node.width / 2, cy: node.y + node.height / 2 }
}

function Particle({ edge, mode, delay }: { edge: Edge; mode: 'write' | 'search'; delay: number }) {
  if ((mode === 'write' && !edge.write) || (mode === 'search' && !edge.search)) return null

  const fromNode = nodes.find(n => n.id === edge.from)!
  const toNode = nodes.find(n => n.id === edge.to)!
  const { cx: fx, cy: fy } = center(fromNode)
  const { cx: tx, cy: ty } = center(toNode)
  const dx = tx - fx, dy = ty - fy
  const dist = Math.sqrt(dx * dx + dy * dy)
  const duration = Math.max(1.5, dist / 60)

  return (
    <motion.div
      className="absolute pointer-events-none z-20"
      initial={{ left: fx, top: fy }}
      animate={{ left: tx, top: ty }}
      transition={{ duration, delay, repeat: Infinity, ease: 'easeInOut', repeatDelay: 1 }}
    >
      <div className={`w-2.5 h-2.5 rounded-full ${mode === 'write' ? 'bg-emerald-400' : 'bg-blue-400'} shadow-lg ${mode === 'write' ? 'shadow-emerald-500/50' : 'shadow-blue-500/50'}`} />
    </motion.div>
  )
}

function EdgeLabel({ edge, mode, index }: { edge: Edge; mode: 'write' | 'search'; index: number }) {
  if ((mode === 'write' && !edge.write) || (mode === 'search' && !edge.search)) return null

  const fromNode = nodes.find(n => n.id === edge.from)!
  const toNode = nodes.find(n => n.id === edge.to)!
  const { cx: fx, cy: fy } = center(fromNode)
  const { cx: tx, cy: ty } = center(toNode)

  return (
    <div
      className="absolute pointer-events-none z-10"
      style={{ left: (fx + tx) / 2 - 55, top: (fy + ty) / 2 - 8, width: 110, textAlign: 'center' }}
    >
      <motion.span
        initial={{ opacity: 0 }}
        animate={{ opacity: [0, 1, 1, 0] }}
        transition={{ duration: 3, delay: index * 0.3 + 0.5, repeat: Infinity, repeatDelay: 1.5 }}
        className="text-[9px] font-mono px-1.5 py-0.5 rounded bg-pickle-800/90 text-pickle-300 whitespace-nowrap inline-block"
      >
        {edge.label}
      </motion.span>
    </div>
  )
}

function EdgeLine({ edge, mode }: { edge: Edge; mode: 'write' | 'search' }) {
  if (!(mode === 'write' ? edge.write : edge.search)) return null

  const fromNode = nodes.find(n => n.id === edge.from)!
  const toNode = nodes.find(n => n.id === edge.to)!
  const { cx: x1, cy: y1 } = center(fromNode)
  const { cx: x2, cy: y2 } = center(toNode)

  const isWriteActive = mode === 'write' && edge.write
  const isSearchActive = mode === 'search' && edge.search
  const isActive = isWriteActive || isSearchActive

  return (
    <svg className="absolute inset-0 pointer-events-none" style={{ width: '100%', height: '100%' }}>
      <line
        x1={x1} y1={y1} x2={x2} y2={y2}
        stroke={isActive ? (isWriteActive ? 'rgba(16, 185, 96, 0.25)' : 'rgba(59, 130, 246, 0.25)') : 'rgba(255,255,255,0.04)'}
        strokeWidth="1.5"
        strokeDasharray="4 3"
      />
    </svg>
  )
}

function ComponentCard({ node, isHovered, onHover, mode }: {
  node: Node; isHovered: boolean; onHover: (id: string | null) => void; mode: 'write' | 'search'
}) {
  const Icon = node.icon

  const writeActive = ['client', 'engine', 'wal', 'page-manager', 'buffer-pool', 'storage', 'index'].includes(node.id)
  const searchActive = ['app', 'client', 'engine', 'index', 'storage'].includes(node.id)
  const isInPath = mode === 'write' ? writeActive : searchActive

  return (
    <motion.div
      className={`absolute rounded-xl border transition-all duration-300 cursor-default ${
        isHovered
          ? `${node.border} ${node.bg} shadow-lg z-30`
          : isInPath
            ? `${node.border} ${node.bg} z-20`
            : 'border-pickle-600 bg-pickle-800/30 z-20 opacity-40'
      }`}
      style={{ left: node.x, top: node.y, width: node.width, height: node.height }}
      onMouseEnter={() => onHover(node.id)}
      onMouseLeave={() => onHover(null)}
      whileHover={{ scale: 1.04 }}
    >
      <div className="flex items-center gap-2.5 px-3 h-full">
        <div className={`w-7 h-7 rounded-lg flex items-center justify-center shrink-0 ${node.bg}`}>
          <Icon className={`w-3.5 h-3.5 ${node.color}`} />
        </div>
        <div className="flex-1 min-w-0">
          <div className={`text-xs font-medium truncate ${isInPath ? 'text-white' : 'text-pickle-400'}`}>
            {node.label}
          </div>
          {!isHovered && (
            <div className="text-[9px] text-pickle-500 truncate mt-0.5">{node.zone === 'trusted' ? 'Client-side' : 'Server-side'}</div>
          )}
        </div>
        {isInPath && !isHovered && (
          <div className={`w-1.5 h-1.5 rounded-full ${mode === 'write' ? 'bg-emerald-400' : 'bg-blue-400'}`} />
        )}
      </div>
      <AnimatePresence>
        {isHovered && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: 'auto', opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            className="overflow-hidden px-3 pb-3"
          >
            <p className="text-[9px] text-pickle-400 leading-relaxed">{node.desc}</p>
          </motion.div>
        )}
      </AnimatePresence>
    </motion.div>
  )
}

export function Architecture() {
  const [mode, setMode] = useState<'write' | 'search'>('write')
  const [hoveredNode, setHoveredNode] = useState<string | null>(null)
  const containerRef = useRef<HTMLDivElement>(null)

  const trustedNodes = nodes.filter(n => n.zone === 'trusted')
  const untrustedNodes = nodes.filter(n => n.zone === 'untrusted')

  const trustedRect = {
    x: Math.min(...trustedNodes.map(n => n.x)) - 12,
    y: Math.min(...trustedNodes.map(n => n.y)) - 18,
    width: Math.max(...trustedNodes.map(n => n.x + n.width)) - Math.min(...trustedNodes.map(n => n.x)) + 24,
    height: Math.max(...trustedNodes.map(n => n.y + n.height)) - Math.min(...trustedNodes.map(n => n.y)) + 24,
  }
  const untrustedRect = {
    x: Math.min(...untrustedNodes.map(n => n.x)) - 12,
    y: Math.min(...untrustedNodes.map(n => n.y)) - 18,
    width: Math.max(...untrustedNodes.map(n => n.x + n.width)) - Math.min(...untrustedNodes.map(n => n.x)) + 24,
    height: Math.max(...untrustedNodes.map(n => n.y + n.height)) - Math.min(...untrustedNodes.map(n => n.y)) + 24,
  }

  return (
    <section id="architecture" className="relative py-24 lg:py-32">
      <div className="mx-auto max-w-5xl px-6 lg:px-8">
        <div className="flex flex-col lg:flex-row lg:items-end justify-between gap-6 mb-12">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="max-w-2xl"
          >
            <span className="text-xs uppercase tracking-widest text-emerald-400/70 font-medium">Architecture</span>
            <h2 className="mt-4 font-heading text-3xl sm:text-4xl lg:text-5xl font-bold text-white tracking-tight">
              How it <span className="text-emerald-400">works</span>
            </h2>
            <p className="mt-3 text-pickle-400 text-lg max-w-lg">
              Data is encrypted before it leaves your application. The engine never sees plaintext.
            </p>
          </motion.div>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="flex items-center gap-2 bg-surface rounded-xl border border-border p-1 shrink-0"
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
                mode === 'search' ? 'bg-blue-500/20 text-blue-400 shadow-sm' : 'text-pickle-400 hover:text-white'
              }`}
            >
              <Search className="w-3.5 h-3.5" />
              Search Flow
            </button>
          </motion.div>
        </div>

        {/* Desktop: absolute-positioned interactive diagram */}
        <motion.div
          initial={{ opacity: 0 }}
          whileInView={{ opacity: 1 }}
          viewport={{ once: true }}
          ref={containerRef}
          className="relative hidden lg:block w-full overflow-hidden rounded-2xl border border-border bg-surface/50"
          style={{ height: 545 }}
        >
          {/* Trust boundary zones */}
          <div
            className="absolute rounded-xl border-2 border-emerald-500/20 bg-emerald-500/[0.02] z-0"
            style={{
              left: trustedRect.x,
              top: trustedRect.y,
              width: trustedRect.width,
              height: trustedRect.height,
            }}
          >
            <span className="absolute -top-2 left-3 px-2 text-[9px] font-mono text-emerald-400/50 bg-pickle-900 rounded-sm">
              TRUSTED ZONE — Client-Side Crypto
            </span>
          </div>
          <div
            className="absolute rounded-xl border-2 border-blue-500/15 bg-blue-500/[0.01] z-0"
            style={{
              left: untrustedRect.x,
              top: untrustedRect.y,
              width: untrustedRect.width,
              height: untrustedRect.height,
            }}
          >
            <span className="absolute -top-2 left-3 px-2 text-[9px] font-mono text-blue-400/40 bg-pickle-900 rounded-sm">
              UNTRUSTED ZONE — Engine
            </span>
          </div>

          {/* Edge lines */}
          <div className="absolute inset-0 z-0">
            {edges.map((edge, i) => (
              <EdgeLine key={i} edge={edge} mode={mode} />
            ))}
          </div>

          {/* Edge labels */}
          {edges.map((edge, i) => (
            <EdgeLabel key={i} edge={edge} mode={mode} index={i} />
          ))}

          {/* Particle animations */}
          {edges.filter(e => (mode === 'write' && e.write) || (mode === 'search' && e.search)).map((edge, i) => (
            <Particle key={i} edge={edge} mode={mode} delay={i * 0.25} />
          ))}

          {/* Component cards */}
          {nodes.map(node => (
            <ComponentCard
              key={node.id}
              node={node}
              isHovered={hoveredNode === node.id}
              onHover={setHoveredNode}
              mode={mode}
            />
          ))}

          {/* Key legend */}
          <div className="absolute bottom-3 right-3 flex items-center gap-4 z-10">
            <div className="flex items-center gap-1.5">
              <div className="w-2 h-2 rounded-full bg-emerald-400" />
              <span className="text-[10px] text-pickle-400">Write path</span>
            </div>
            <div className="flex items-center gap-1.5">
              <div className="w-2 h-2 rounded-full bg-blue-400" />
              <span className="text-[10px] text-pickle-400">Search path</span>
            </div>
          </div>
        </motion.div>

        {/* Mobile: stacked component list */}
        <div className="lg:hidden space-y-3">
          <div className="text-xs font-mono text-emerald-400/60 px-1 mb-2">TRUSTED ZONE — Client-side Crypto</div>
          {nodes.filter(n => n.zone === 'trusted').map((node) => {
            const Icon = node.icon
            return (
              <motion.div
                key={node.id}
                initial={{ opacity: 0, x: -10 }}
                whileInView={{ opacity: 1, x: 0 }}
                viewport={{ once: true }}
                className="flex items-center gap-3 p-3 rounded-xl border border-emerald-500/20 bg-emerald-500/[0.03]"
              >
                <div className="w-8 h-8 rounded-lg flex items-center justify-center shrink-0 bg-emerald-500/10">
                  <Icon className="w-4 h-4 text-emerald-400" />
                </div>
                <div>
                  <div className="text-sm font-medium text-white">{node.label}</div>
                  <div className="text-xs text-pickle-400">{node.desc}</div>
                </div>
              </motion.div>
            )
          })}
          <div className="flex justify-center py-1">
            <ArrowDown className="w-3.5 h-3.5 text-pickle-500" />
          </div>
          <div className="text-xs font-mono text-blue-400/60 px-1 mb-2">UNTRUSTED ZONE — Engine</div>
          {nodes.filter(n => n.zone === 'untrusted').map((node) => {
            const Icon = node.icon
            return (
              <motion.div
                key={node.id}
                initial={{ opacity: 0, x: -10 }}
                whileInView={{ opacity: 1, x: 0 }}
                viewport={{ once: true }}
                transition={{ delay: 0.1 }}
                className="flex items-center gap-3 p-3 rounded-xl border border-blue-500/15 bg-blue-500/[0.02]"
              >
                <div className="w-8 h-8 rounded-lg flex items-center justify-center shrink-0 bg-blue-500/10">
                  <Icon className="w-4 h-4 text-blue-400" />
                </div>
                <div>
                  <div className="text-sm font-medium text-white">{node.label}</div>
                  <div className="text-xs text-pickle-400">{node.desc}</div>
                </div>
              </motion.div>
            )
          })}
        </div>

        {/* Step-by-step explanation */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          className="mt-12 max-w-3xl mx-auto"
        >
          <div className="rounded-2xl border border-border bg-surface p-6 lg:p-8">
            <h3 className="text-white font-heading font-semibold mb-6 flex items-center gap-2">
              {mode === 'write' ? (
                <><Zap className="w-4 h-4 text-emerald-400" /> Write Flow — Step by Step</>
              ) : (
                <><Search className="w-4 h-4 text-blue-400" /> Search Flow — Step by Step</>
              )}
            </h3>
            <div className="space-y-4">
              {mode === 'write' ? (
                <>
                  {[
                    ['Application', 'Your app prepares plaintext data and calls the client SDK.'],
                    ['PickleClient', 'Encrypts data with AES-256-GCM. Derives HMAC-SHA256 search tokens for indexed fields. Sends InsertTuple to the engine.'],
                    ['PickleEngine', 'Receives the InsertTuple. Never sees decryption keys or plaintext. Appends operation to WAL for crash durability.'],
                    ['WAL Log', 'Serializes with bincode, writes to wal.log with length-prefixed entries, flushes to disk. Guarantees ACID durability.'],
                    ['Page Manager', 'Finds or allocates a 4KB slotted page. Writes (RecordId, EncryptedPayload) in a slot.'],
                    ['Buffer Pool', 'Caches the dirty page (FIFO eviction, capacity 1000). Periodically flushes to data.db.'],
                    ['HashIndex', 'Maps SearchToken → Vec<RecordId> in memory. Enables future blind searches without decryption.'],
                    ['Encrypted Storage', 'Pages at fixed offsets in data.db. Only ciphertext ever touches disk.'],
                  ].map(([step, desc], i) => (
                    <motion.div
                      key={step}
                      initial={{ opacity: 0, x: -10 }}
                      whileInView={{ opacity: 1, x: 0 }}
                      viewport={{ once: true }}
                      transition={{ delay: i * 0.08 }}
                      className="flex items-start gap-3"
                    >
                      <div className="flex items-center justify-center w-6 h-6 rounded-full bg-emerald-500/20 text-emerald-400 text-xs font-bold font-mono shrink-0 mt-0.5">
                        {i + 1}
                      </div>
                      <div>
                        <span className="text-sm font-medium text-white">{step}</span>
                        <p className="text-xs text-pickle-400 mt-0.5 leading-relaxed">{desc}</p>
                      </div>
                    </motion.div>
                  ))}
                </>
              ) : (
                <>
                  {[
                    ['Application', 'You call db.search("email").eq("alice@example.com"). The SDK prepares a search.'],
                    ['PickleClient', 'Derives search token: HMAC-SHA256(K_search, "email::alice@example.com"). Sends the 32-byte token to the engine.'],
                    ['PickleEngine', 'Receives the SearchToken. Forwards to HashIndex for lookup.'],
                    ['HashIndex', 'Looks up token in HashMap<SearchToken, Vec<RecordId>>. Returns matching RecordIds — no decryption needed.'],
                    ['Encrypted Storage', 'Engine reads EncryptedPayloads from pages via Buffer Pool. Only ciphertext.'],
                    ['PickleClient', 'Decrypts each EncryptedPayload with AES-256-GCM. Verifies authenticity via GCM tag.'],
                    ['Application', 'Receives decrypted plaintext. No component outside your app saw the data unencrypted.'],
                  ].map(([step, desc], i) => (
                    <motion.div
                      key={step}
                      initial={{ opacity: 0, x: -10 }}
                      whileInView={{ opacity: 1, x: 0 }}
                      viewport={{ once: true }}
                      transition={{ delay: i * 0.08 }}
                      className="flex items-start gap-3"
                    >
                      <div className="flex items-center justify-center w-6 h-6 rounded-full bg-blue-500/20 text-blue-400 text-xs font-bold font-mono shrink-0 mt-0.5">
                        {i + 1}
                      </div>
                      <div>
                        <span className="text-sm font-medium text-white">{step}</span>
                        <p className="text-xs text-pickle-400 mt-0.5 leading-relaxed">{desc}</p>
                      </div>
                    </motion.div>
                  ))}
                </>
              )}
            </div>
          </div>
        </motion.div>
      </div>
    </section>
  )
}
