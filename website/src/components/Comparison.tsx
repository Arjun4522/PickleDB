import { motion } from 'framer-motion'
import { Check, X } from 'lucide-react'

const rows = [
  { feature: 'Encryption', values: [false, false, false, false, true] },
  { feature: 'Searchable Encryption', values: [false, false, false, false, true] },
  { feature: 'Client-side Crypto', values: [false, false, false, false, true] },
  { feature: 'Zero Trust', values: [false, false, false, false, true] },
  { feature: 'Write Ahead Log', values: [true, false, true, true, true] },
  { feature: 'Crash Recovery', values: [true, true, true, true, true] },
  { feature: 'Memory Safe', values: [false, false, false, false, true] },
]
const dbs = ['SQLite', 'DuckDB', 'RocksDB', 'LMDB', 'PickleDB']

export function Comparison() {
  return (
    <section id="comparison" className="relative py-24 lg:py-32">
      <div className="mx-auto max-w-7xl px-6 lg:px-8">
        <div className="max-w-2xl">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
          >
            <span className="text-xs uppercase tracking-widest text-emerald-400/70 font-medium">Why PickleDB</span>
            <h2 className="mt-4 font-heading text-3xl sm:text-4xl lg:text-5xl font-bold text-white tracking-tight">
              The only zero-trust{' '}
              <span className="text-emerald-400">embedded DB</span>
            </h2>
          </motion.div>
        </div>

        <div className="mt-16 overflow-x-auto">
          <motion.table
            initial={{ opacity: 0 }}
            whileInView={{ opacity: 1 }}
            viewport={{ once: true }}
            className="w-full min-w-[500px] border-collapse"
          >
            <thead>
              <tr>
                <th className="text-left py-3 pr-8 text-sm font-medium text-pickle-400">Feature</th>
                {dbs.map((db) => (
                  <th
                    key={db}
                    className={`py-3 px-4 text-sm font-semibold text-center min-w-[90px] ${
                      db === 'PickleDB' ? 'text-emerald-400' : 'text-pickle-300'
                    }`}
                  >
                    {db}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {rows.map((row, i) => (
                <motion.tr
                  key={row.feature}
                  initial={{ opacity: 0, y: 8 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  viewport={{ once: true }}
                  transition={{ delay: i * 0.04 }}
                  className="group"
                >
                  <td className="py-3 pr-8 text-sm text-pickle-200 border-t border-border/50">
                    {row.feature}
                  </td>
                  {row.values.map((val, j) => (
                    <td
                      key={j}
                      className={`py-3 px-4 text-center border-t border-border/50 ${
                        dbs[j] === 'PickleDB' ? 'bg-emerald-500/[0.03]' : ''
                      }`}
                    >
                      {val ? (
                        <Check className={`w-4 h-4 mx-auto ${dbs[j] === 'PickleDB' ? 'text-emerald-400' : 'text-pickle-500'}`} />
                      ) : (
                        <X className="w-4 h-4 mx-auto text-pickle-600" />
                      )}
                    </td>
                  ))}
                </motion.tr>
              ))}
            </tbody>
          </motion.table>
        </div>
      </div>
    </section>
  )
}
