import { motion } from 'framer-motion'
import { Star, GitFork } from 'lucide-react'
import { GithubIcon } from './ui/Icons'

export function GitHubCta() {
  return (
    <section className="relative py-24 lg:py-32">
      <div className="mx-auto max-w-7xl px-6 lg:px-8">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          className="max-w-lg mx-auto text-center"
        >
          <div className="inline-flex p-3 rounded-2xl bg-emerald-500/10 border border-emerald-500/20 mb-6">
            <GithubIcon className="w-6 h-6 text-emerald-400" />
          </div>
          <h2 className="font-heading text-3xl sm:text-4xl font-bold text-white tracking-tight mb-3">
            Open source. MIT licensed.
          </h2>
          <p className="text-pickle-400 mb-8 text-sm">
            Star us on GitHub and be part of the zero-trust database revolution.
          </p>
          <div className="flex items-center justify-center gap-4 mb-6">
            <div className="flex items-center gap-1.5">
              <Star className="w-4 h-4 text-yellow-400" fill="currentColor" />
              <span className="text-white font-semibold font-heading">2.4k</span>
              <span className="text-pickle-400 text-xs">Stars</span>
            </div>
            <div className="w-px h-4 bg-border" />
            <div className="flex items-center gap-1.5">
              <GitFork className="w-4 h-4 text-pickle-400" />
              <span className="text-white font-semibold font-heading">186</span>
              <span className="text-pickle-400 text-xs">Forks</span>
            </div>
          </div>
          <div className="flex flex-wrap justify-center gap-3">
            <a
              href="https://github.com/seladb/pickledb"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 px-5 py-2.5 text-sm font-medium rounded-xl bg-emerald-500 text-white hover:bg-emerald-400 transition-all hover:shadow-lg hover:shadow-emerald-500/25 active:scale-[0.98]"
            >
              <GithubIcon className="w-4 h-4" />
              View on GitHub
            </a>
            <a
              href="#"
              className="inline-flex items-center gap-2 px-5 py-2.5 text-sm font-medium rounded-xl border border-border text-pickle-300 hover:text-white hover:border-pickle-400 transition-all active:scale-[0.98]"
            >
              Documentation
            </a>
          </div>
        </motion.div>
      </div>
    </section>
  )
}
