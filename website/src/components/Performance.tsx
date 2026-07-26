import { motion } from 'framer-motion'

const headlineStats = [
  { value: '220K+', label: 'Inserts/sec' },
  { value: '0.42ms', label: 'Query latency' },
  { value: 'AES-256', label: 'Encryption' },
  { value: '100%', label: 'Rust' },
]

const benchmarks = [
  { value: '220K+', label: 'Inserts/sec', desc: 'Sustained write throughput with AES-256-GCM encryption' },
  { value: '0.42ms', label: 'Lookup latency', desc: 'Search query time for indexed encrypted fields' },
  { value: '~15%', label: 'Encryption overhead', desc: 'Performance cost of client-side encryption vs plaintext' },
  { value: '4KB', label: 'Page size', desc: 'Optimized slotted page architecture for efficient storage' },
]

export function Performance() {
  return (
    <section id="performance" className="relative py-16 lg:py-24">
      <div className="mx-auto max-w-7xl px-6 lg:px-8">
        <div className="max-w-2xl">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
          >
            <span className="text-xs uppercase tracking-widest text-emerald-400/70 font-medium">Performance</span>
            <h2 className="mt-4 font-heading text-3xl sm:text-4xl lg:text-5xl font-bold text-white tracking-tight">
              Production <span className="text-emerald-400">grade</span>
            </h2>
          </motion.div>
        </div>

        {/* Headline stats */}
        <div className="mt-12 grid grid-cols-2 md:grid-cols-4 gap-8 lg:gap-12">
          {headlineStats.map((stat, index) => (
            <motion.div
              key={stat.label}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ delay: index * 0.1 }}
              className="text-center"
            >
              <div className="text-2xl sm:text-3xl lg:text-4xl font-bold font-heading text-white tracking-tight">
                {stat.value}
              </div>
              <div className="mt-1 text-sm text-pickle-400">{stat.label}</div>
            </motion.div>
          ))}
        </div>

        {/* Benchmark cards */}
        <div className="mt-16 grid sm:grid-cols-2 lg:grid-cols-4 gap-px bg-border rounded-2xl overflow-hidden">
          {benchmarks.map((item, index) => (
              <motion.div
                key={item.label}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ delay: index * 0.08 }}
                className="bg-surface p-8 lg:p-10 perspective-card"
              >
                <div className="tilt-card h-full">
                  <div className="text-3xl sm:text-4xl font-bold font-heading text-white tracking-tight mb-1">
                    {item.value}
                  </div>
                  <div className="text-sm font-medium text-pickle-200 mb-2">{item.label}</div>
                  <div className="text-xs text-pickle-400 leading-relaxed">{item.desc}</div>
                </div>
              </motion.div>
          ))}
        </div>
      </div>
    </section>
  )
}
