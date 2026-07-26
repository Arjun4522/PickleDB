import { motion } from 'framer-motion'

const stats = [
  { value: '220K+', label: 'Inserts/sec' },
  { value: '0.42ms', label: 'Query latency' },
  { value: 'AES-256', label: 'Encryption' },
  { value: '100%', label: 'Rust' },
]

export function StatsBar() {
  return (
    <section className="relative py-16 lg:py-20">
      <div className="mx-auto max-w-7xl px-6 lg:px-8">
        <div className="grid grid-cols-2 md:grid-cols-4 gap-8 lg:gap-12">
          {stats.map((stat, index) => (
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
      </div>
    </section>
  )
}
