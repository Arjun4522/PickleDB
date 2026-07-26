import { useEffect, useState } from 'react'
import { Menu, X } from 'lucide-react'
import { GithubIcon } from './ui/Icons'
import { cn } from '@/lib/utils'

const navItems = [
  { label: 'Features', href: '#features' },
  { label: 'Architecture', href: '#architecture' },
  { label: 'Security', href: '#security' },
  { label: 'Docs', href: 'https://github.com/seladb/pickledb' },
]

export function Navbar() {
  const [scrolled, setScrolled] = useState(false)
  const [mobileOpen, setMobileOpen] = useState(false)

  useEffect(() => {
    const onScroll = () => setScrolled(window.scrollY > 40)
    window.addEventListener('scroll', onScroll, { passive: true })
    return () => window.removeEventListener('scroll', onScroll)
  }, [])

  return (
    <nav
      className={cn(
        'fixed top-0 left-0 right-0 z-50 transition-all duration-500',
        scrolled ? 'glass shadow-sm' : 'bg-transparent',
      )}
    >
      <div className="mx-auto max-w-7xl px-6 lg:px-8">
        <div className="flex items-center justify-between h-16">
          <a href="#" className="flex items-center gap-2.5 group">
            <div className="w-7 h-7 rounded-lg bg-emerald-500 flex items-center justify-center font-heading font-bold text-white text-xs group-hover:shadow-lg group-hover:shadow-emerald-500/25 transition-all duration-300">
              P
            </div>
            <span className="font-heading font-semibold text-[15px] text-white">
              PickleDB
            </span>
          </a>

          <div className="hidden md:flex items-center gap-1">
            {navItems.map((item) => (
              <a
                key={item.label}
                href={item.href}
                className="px-3 py-2 text-sm text-pickle-400 hover:text-white transition-colors rounded-lg hover:bg-white/[0.04]"
              >
                {item.label}
              </a>
            ))}
          </div>

          <div className="hidden md:flex items-center gap-3">
            <a
              href="https://github.com/seladb/pickledb"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-2 px-3 py-2 text-sm text-pickle-400 hover:text-white transition-colors rounded-lg hover:bg-white/[0.04]"
            >
              <GithubIcon className="w-4 h-4" />
              <span>Stars</span>
              <span className="text-emerald-400 font-mono text-xs">2.4k</span>
            </a>
            <a
              href="#"
              className="inline-flex items-center px-4 py-2 text-sm font-medium rounded-lg bg-emerald-500 text-white hover:bg-emerald-400 transition-all hover:shadow-lg hover:shadow-emerald-500/25 active:scale-[0.98]"
            >
              Get Started
            </a>
          </div>

          <button
            onClick={() => setMobileOpen(!mobileOpen)}
            className="md:hidden p-2 text-pickle-400 hover:text-white transition-colors"
          >
            {mobileOpen ? <X className="w-5 h-5" /> : <Menu className="w-5 h-5" />}
          </button>
        </div>
      </div>

      {mobileOpen && (
        <div className="md:hidden glass border-t border-border">
          <div className="px-6 py-4 space-y-1">
            {navItems.map((item) => (
              <a
                key={item.label}
                href={item.href}
                onClick={() => setMobileOpen(false)}
                className="block px-3 py-2.5 text-sm text-pickle-300 hover:text-white rounded-lg hover:bg-white/[0.04] transition-colors"
              >
                {item.label}
              </a>
            ))}
            <hr className="border-border my-3" />
            <a
              href="#"
              className="block text-center px-4 py-2.5 text-sm font-medium rounded-lg bg-emerald-500 text-white hover:bg-emerald-400 transition-colors"
            >
              Get Started
            </a>
          </div>
        </div>
      )}
    </nav>
  )
}
