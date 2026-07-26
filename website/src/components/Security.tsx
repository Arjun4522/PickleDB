import { motion } from 'framer-motion'
import { X, Lock, Eye, HardDrive, Cpu, User } from 'lucide-react'

const traditional = [
  { icon: User, label: 'Application', plaintext: true },
  { icon: Cpu, label: 'Memory', plaintext: true },
  { icon: HardDrive, label: 'Storage', plaintext: false },
]

const pickledb = [
  { icon: User, label: 'Application', plaintext: true },
  { icon: Lock, label: 'Client AES', plaintext: false },
  { icon: Eye, label: 'Search Tokens', plaintext: false },
  { icon: Cpu, label: 'Engine', plaintext: false },
  { icon: HardDrive, label: 'Storage', plaintext: false },
]

export function Security() {
  return (
    <section id="security" className="relative py-24 lg:py-32">
      <div className="mx-auto max-w-7xl px-6 lg:px-8">
        <div className="max-w-2xl">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
          >
            <span className="text-xs uppercase tracking-widest text-emerald-400/70 font-medium">Security</span>
            <h2 className="mt-4 font-heading text-3xl sm:text-4xl lg:text-5xl font-bold text-white tracking-tight">
              Your data never{' '}
              <span className="text-emerald-400">lies plain</span>
            </h2>
          </motion.div>
        </div>

        <div className="mt-16 grid lg:grid-cols-2 gap-6">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
          >
            <div className="rounded-2xl border border-border bg-surface p-8">
              <h3 className="text-white font-semibold mb-6 flex items-center gap-2">
                <X className="w-4 h-4 text-red-400" />
                Traditional Database
              </h3>
              <div className="space-y-3">
                {traditional.map((layer) => (
                  <div key={layer.label} className="flex items-center gap-3">
                    <div className={`w-8 h-8 rounded-lg flex items-center justify-center ${layer.plaintext ? 'bg-red-500/10' : 'bg-emerald-500/10'}`}>
                      <layer.icon className={`w-4 h-4 ${layer.plaintext ? 'text-red-400' : 'text-emerald-400'}`} />
                    </div>
                    <div className="flex-1">
                      <div className="text-sm text-white">{layer.label}</div>
                      <div className="text-xs text-pickle-400">{layer.plaintext ? 'Plaintext exposed' : 'Encrypted'}</div>
                    </div>
                    {layer.plaintext ? (
                      <X className="w-3.5 h-3.5 text-red-400/60" />
                    ) : (
                      <Lock className="w-3.5 h-3.5 text-emerald-400/60" />
                    )}
                  </div>
                ))}
              </div>
            </div>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ delay: 0.1 }}
          >
            <div className="rounded-2xl border border-emerald-500/20 bg-emerald-500/[0.03] p-8">
              <h3 className="text-white font-semibold mb-6 flex items-center gap-2">
                <Lock className="w-4 h-4 text-emerald-400" />
                PickleDB
              </h3>
              <div className="space-y-3">
                {pickledb.map((layer) => (
                  <div key={layer.label} className="flex items-center gap-3">
                    <div className={`w-8 h-8 rounded-lg flex items-center justify-center ${layer.plaintext ? 'bg-emerald-500/10' : 'bg-emerald-500/10'}`}>
                      <layer.icon className={`w-4 h-4 ${layer.plaintext ? 'text-emerald-400' : 'text-emerald-400/80'}`} />
                    </div>
                    <div className="flex-1">
                      <div className="text-sm text-white">{layer.label}</div>
                      <div className="text-xs text-emerald-400/60">{layer.plaintext ? 'Plaintext only here' : 'Ciphertext'}</div>
                    </div>
                    <Lock className="w-3.5 h-3.5 text-emerald-400/60" />
                  </div>
                ))}
              </div>
            </div>
          </motion.div>
        </div>
      </div>
    </section>
  )
}
