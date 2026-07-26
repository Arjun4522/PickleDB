import { motion } from 'framer-motion'

const testimonials = [
  {
    quote: "PickleDB changes how we think about data security at the edge. Client-side encryption with searchable queries is exactly what IoT needs. We integrated it in an afternoon.",
    author: 'Dr. Sarah Chen',
    role: 'Principal Engineer, Edge Platform',
  },
  {
    quote: "We evaluated every embedded database for our zero-trust architecture. PickleDB was the only one that understood security from the ground up. The Rust implementation gives us memory safety without compromise.",
    author: 'Marcus Rivera',
    role: 'CISO, FinTech',
  },
  {
    quote: "The API is beautifully designed. Felt productive within minutes. And knowing my data is encrypted end-to-end — plaintext never touches disk — gives me real peace of mind.",
    author: 'Yuki Tanaka',
    role: 'Senior Software Engineer',
  },
]

export function Testimonials() {
  return (
    <section className="relative py-24 lg:py-32">
      <div className="mx-auto max-w-7xl px-6 lg:px-8">
        <div className="max-w-2xl">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
          >
            <span className="text-xs uppercase tracking-widest text-emerald-400/70 font-medium">Testimonials</span>
            <h2 className="mt-4 font-heading text-3xl sm:text-4xl lg:text-5xl font-bold text-white tracking-tight">
              Loved by{' '}
              <span className="text-emerald-400">developers</span>
            </h2>
          </motion.div>
        </div>

        <div className="mt-16 grid md:grid-cols-3 gap-px bg-border rounded-2xl overflow-hidden">
          {testimonials.map((t, index) => (
            <motion.div
              key={t.author}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ delay: index * 0.08 }}
              className="bg-surface p-8 lg:p-10 flex flex-col"
            >
              <p className="text-sm text-pickle-200 leading-relaxed flex-1">
                "{t.quote}"
              </p>
              <div className="mt-6 pt-6 border-t border-border">
                <div className="flex items-center gap-3">
                  <div className="w-8 h-8 rounded-full bg-emerald-500/20 flex items-center justify-center text-xs font-bold text-emerald-400">
                    {t.author.split(' ').map(n => n[0]).join('')}
                  </div>
                  <div>
                    <div className="text-sm font-medium text-white">{t.author}</div>
                    <div className="text-xs text-pickle-400">{t.role}</div>
                  </div>
                </div>
              </div>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  )
}
