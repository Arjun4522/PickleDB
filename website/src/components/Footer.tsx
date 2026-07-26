import { GithubIcon, TwitterIcon } from './ui/Icons'

const links = {
  Product: ['Features', 'Architecture', 'Security', 'Changelog'],
  Docs: ['Get Started', 'API Reference', 'CLI', 'SDK'],
  Community: ['GitHub', 'Discord', 'Twitter', 'Contributing'],
  Legal: ['License (MIT)', 'Privacy'],
}

export function Footer() {
  return (
    <footer className="border-t border-border">
      <div className="mx-auto max-w-7xl px-6 lg:px-8 py-16 lg:py-20">
        <div className="grid grid-cols-2 md:grid-cols-5 gap-8 lg:gap-12">
          <div className="col-span-2 md:col-span-1">
            <a href="#" className="flex items-center gap-2.5 mb-4">
              <div className="w-7 h-7 rounded-lg bg-emerald-500 flex items-center justify-center font-heading font-bold text-white text-xs">
                P
              </div>
              <span className="font-heading font-semibold text-[15px] text-white">PickleDB</span>
            </a>
            <p className="text-sm text-pickle-400 leading-relaxed max-w-xs">
              Embedded zero-trust searchable encrypted database. Built in Rust.
            </p>
            <div className="flex items-center gap-2 mt-6">
              {[
                { icon: GithubIcon, href: 'https://github.com/seladb/pickledb' },
                { icon: TwitterIcon, href: '#' },
              ].map(({ icon: Icon, href }) => (
                <a
                  key={href}
                  href={href}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="p-2 text-pickle-400 hover:text-white transition-colors rounded-lg hover:bg-white/[0.04]"
                >
                  <Icon className="w-4 h-4" />
                </a>
              ))}
            </div>
          </div>

          {Object.entries(links).map(([category, items]) => (
            <div key={category}>
              <h4 className="text-xs font-semibold text-pickle-400 uppercase tracking-wider mb-4">
                {category}
              </h4>
              <ul className="space-y-2.5">
                {items.map((item) => (
                  <li key={item}>
                    <a
                      href="#"
                      className="text-sm text-pickle-400 hover:text-white transition-colors"
                    >
                      {item}
                    </a>
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>

        <div className="mt-12 pt-8 border-t border-border flex flex-col sm:flex-row items-center justify-between gap-4">
          <p className="text-xs text-pickle-500">
            © {new Date().getFullYear()} PickleDB — Open source (MIT)
          </p>
          <span className="text-xs text-pickle-500 flex items-center gap-1">
            Built with <span className="text-emerald-400">Rust</span> and ❤️
          </span>
        </div>
      </div>
    </footer>
  )
}
