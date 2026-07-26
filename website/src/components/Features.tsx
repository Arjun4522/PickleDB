import { motion } from 'framer-motion'
import { Lock, Search, Shield, FileText, RefreshCw, Database } from 'lucide-react'

const features = [
  {
    icon: Lock,
    title: 'AES-256-GCM Encryption',
    desc: 'Authenticated encryption with hardware acceleration. Data is encrypted on the client side — plaintext never reaches the engine.',
  },
  {
    icon: Search,
    title: 'Searchable Encryption',
    desc: 'Blind search tokens enable queries on encrypted data. Search without ever decrypting — zero information leakage.',
  },
  {
    icon: Shield,
    title: 'Zero Trust Architecture',
    desc: 'Designed for zero-trust environments. No plaintext touches disk or memory outside your application boundary.',
  },
  {
    icon: FileText,
    title: 'Write Ahead Log',
    desc: 'Durable WAL journals every mutation before commitment. Guarantees ACID compliance without sacrificing throughput.',
  },
  {
    icon: RefreshCw,
    title: 'Crash Recovery',
    desc: 'Automatic WAL replay on startup restores database to last consistent state. No corruption, no data loss.',
  },
  {
    icon: Database,
    title: 'Embedded by Design',
    desc: 'Link directly into your application. No server, no separate process. Zero configuration out of the box.',
  },
]

export function Features() {
  return (
    <section id="features" className="relative py-16 lg:py-24">
      <div className="mx-auto max-w-7xl px-6 lg:px-8">
        <div className="max-w-2xl">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
          >
            <span className="text-xs uppercase tracking-widest text-emerald-400/70 font-medium">Features</span>
            <h2 className="mt-4 font-heading text-3xl sm:text-4xl lg:text-5xl font-bold text-white tracking-tight">
              Enterprise security,<br />
              <span className="text-emerald-400">developer experience</span>
            </h2>
          </motion.div>
        </div>

        <div className="mt-16 grid sm:grid-cols-2 lg:grid-cols-3 gap-px bg-border rounded-2xl overflow-hidden">
          {features.map((feature, index) => {
            const Icon = feature.icon
            return (
              <motion.div
                key={feature.title}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ delay: index * 0.05 }}
                className="bg-surface p-8 lg:p-10 hover:bg-surface-elevated transition-colors duration-300 group perspective-card"
              >
                <div className="tilt-card h-full">
                  <div className="w-10 h-10 rounded-xl bg-emerald-500/10 flex items-center justify-center mb-5 group-hover:bg-emerald-500/15 transition-colors group-hover:shadow-[0_0_20px_rgba(16,185,129,0.15)]">
                    <Icon className="w-5 h-5 text-emerald-400" />
                  </div>
                  <h3 className="text-white font-semibold mb-2">{feature.title}</h3>
                  <p className="text-sm text-pickle-400 leading-relaxed">{feature.desc}</p>
                </div>
              </motion.div>
            )
          })}
        </div>
      </div>
    </section>
  )
}
